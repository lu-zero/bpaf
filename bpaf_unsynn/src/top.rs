//! Top-level structure parsing

use crate::attrs::{ConsumerType, FieldAttrs, Post, PostDecor, PostParse};
use crate::mode::Mode;
use crate::parsing::{
    Attribute, BpafAttr, BpafInner, DocInner, KEnum, KStruct, TokenIter, TupleFields, TypeShape,
    VerbatimUntilComma, Visibility,
};
use crate::utils::to_kebab_case;
use proc_macro2::{Ident, TokenStream, TokenTree};
use quote::quote;
use unsynn::{
    BraceGroup, BracketGroupContaining, Colon, Comma, Cons, IParse, Many, Parser, Pound, Result,
    ToTokens, Transaction,
};

// Constants for magic strings
const DEFAULT_POSITIONAL_METAVAR: &str = "ARG";
const TUPLE_FIELD_NAME_PREFIX: &str = "field_";

/// Create a compile_error! with span pointing to a specific identifier
fn spanned_compile_error(ident: &Ident, msg: String) -> TokenStream {
    let span = ident.span();
    quote::quote_spanned! {span=> compile_error!(#msg) }
}

/// Apply command variant modifiers in the correct order
/// This includes: help, fallback_to_usage, command, long, short, and hide
fn apply_command_modifiers(
    base: TokenStream,
    help_text: Option<String>,
    variant: &EnumBranch,
    command_name: &str,
) -> TokenStream {
    let with_help = if let Some(help) = help_text {
        quote! { #base.descr(#help) }
    } else {
        base
    };

    let with_fallback = if variant.fallback_to_usage {
        quote! { #with_help.fallback_to_usage() }
    } else {
        with_help
    };

    let with_command = quote! {
        #with_fallback.command(#command_name)
    };

    // Chain all long aliases
    let long_aliases = &variant.long_aliases;
    let with_long = if !long_aliases.is_empty() {
        quote! { #with_command #(.long(#long_aliases))* }
    } else {
        with_command
    };

    // Chain all short aliases
    let short_aliases = &variant.short_aliases;
    let with_short = if !short_aliases.is_empty() {
        quote! { #with_long #(.short(#short_aliases))* }
    } else {
        with_long
    };

    if variant.hide {
        quote! { #with_short.hide() }
    } else {
        with_short
    }
}

/// A single outer attribute: #[...]
type OuterAttr = Cons<Pound, BracketGroupContaining<Attribute>>;

/// Collect bpaf attributes and doc comments from a token stream
/// Uses unsynn grammar to parse attributes directly
fn collect_attributes(iter: &mut TokenIter) -> (Vec<BpafAttr>, Vec<DocInner>) {
    // Parse all attributes at once
    let attrs: Option<Many<OuterAttr>> = iter.parse().ok();

    let mut bpaf_attrs = Vec::new();
    let mut doc_comments = Vec::new();

    if let Some(attrs) = attrs {
        for attr in attrs.into_iter() {
            match attr.value.second.content {
                Attribute::Bpaf(bpaf_attr) => bpaf_attrs.push(bpaf_attr),
                Attribute::Doc(doc_inner) => doc_comments.push(doc_inner),
                Attribute::Other(_) => {} // Ignore
            }
        }
    }

    (bpaf_attrs, doc_comments)
}

/// Parse fields from a brace group
fn parse_fields(group: &proc_macro2::Group) -> Result<Vec<StructField>> {
    let mut fields = Vec::new();
    let stream = group.stream();
    let mut iter = stream.to_token_iter();

    loop {
        // Try to parse a field: name : type,
        let field_result = iter.transaction(|t| {
            // Collect bpaf attributes and doc comments
            let (bpaf_attrs, doc_comments) = collect_attributes(t);

            // Skip visibility if present
            let _ = t.parse::<Visibility>();

            // Get field name
            let name: Ident = t.parse()?;

            // Expect colon
            let _: Colon = t.parse()?;

            // Collect type tokens until comma or end using VerbatimUntilComma
            let ty_verbatim: VerbatimUntilComma = t.parse()?;

            // Consume optional trailing comma
            let _ = t.parse::<Comma>();

            // Parse type shape directly from tokens
            let mut ty_iter = ty_verbatim.to_token_iter();
            let shape: TypeShape = ty_iter.parse()?;

            // Convert to TokenStream for storage
            let ty_tokens = ty_verbatim.to_token_stream();

            // Return the parsed field structure and attributes for validation
            Ok((name, ty_tokens, shape, bpaf_attrs, doc_comments))
        });

        match field_result {
            Ok((name, ty_tokens, shape, bpaf_attrs, doc_comments)) => {
                // Parse and validate field attributes AFTER the transaction succeeds
                // This ensures validation errors are propagated, not swallowed
                let attrs =
                    FieldAttrs::parse_from_attrs(&name.to_string(), &bpaf_attrs, &doc_comments)?;

                fields.push(StructField {
                    name,
                    ty: ty_tokens,
                    shape,
                    attrs,
                });
            }
            Err(_) => {
                // No more fields
                break;
            }
        }
    }

    Ok(fields)
}

/// Convert parsed tuple fields to StructField vec
/// Takes already-parsed TupleFields from unsynn
fn convert_tuple_fields(tuple_fields: TupleFields) -> Result<Vec<StructField>> {
    tuple_fields
        .content
        .into_iter()
        .enumerate()
        .map(|(field_index, delimited)| {
            let field = delimited.value;

            // Parse type shape directly from the verbatim tokens
            let mut ty_iter = field.ty.to_token_iter();
            let shape: TypeShape = ty_iter.parse()?;

            // Convert to TokenStream for storage
            let ty_tokens = field.ty.to_token_stream();

            // Defensive check: unsynn's DelimitedVec should never create empty elements
            // from trailing commas, so ty_tokens should never be empty with valid Rust syntax
            debug_assert!(
                !ty_tokens.is_empty(),
                "field_index {} has empty ty_tokens - this should be unreachable",
                field_index
            );

            // In release builds where debug_assert is removed, gracefully skip if empty
            if ty_tokens.is_empty() {
                return Ok(None);
            }

            // Extract bpaf attrs and doc comments
            let (bpaf_attrs, doc_comments) = field.extract_attrs();

            let name = quote::format_ident!("{}{}", TUPLE_FIELD_NAME_PREFIX, field_index);

            // Parse field attributes, using positional as default for tuple fields without attrs
            let attrs = if bpaf_attrs.is_empty() && doc_comments.is_empty() {
                // No attributes - use default positional
                FieldAttrs {
                    consumer: Some(ConsumerType::Positional {
                        metavar: Some(DEFAULT_POSITIONAL_METAVAR.to_string()),
                    }),
                    ..Default::default()
                }
            } else {
                // Parse attributes from the collected bpaf attrs
                // Let the shape-based logic determine the consumer if not explicitly set
                FieldAttrs::parse_from_attrs(&name.to_string(), &bpaf_attrs, &doc_comments)?
            };

            Ok(Some(StructField {
                name,
                ty: ty_tokens,
                shape,
                attrs,
            }))
        })
        .filter_map(|r| r.transpose())
        .collect()
}

/// Parse enum variants from a brace group
fn parse_enum_variants(group: &proc_macro2::Group) -> Result<Vec<EnumBranch>> {
    let mut variants = Vec::new();
    let stream = group.stream();

    let mut iter = stream.to_token_iter();

    loop {
        // Try to parse a variant
        let variant_result = iter.transaction(|t| {
            // Collect bpaf attributes and doc comments
            let (bpaf_attrs, doc_comments) = collect_attributes(t);

            // Extract doc comments
            let doc_comment_strings = extract_doc_comments(&doc_comments);

            // Get variant name
            let name: Ident = t.parse()?;

            // Check for variant fields using unsynn parsing
            let (fields, is_tuple) = if let Ok(tuple_fields) = t.parse::<TupleFields>() {
                // Tuple-style variant: Variant(Type)
                (convert_tuple_fields(tuple_fields)?, true)
            } else if let Ok(brace_group) = t.parse::<BraceGroup>() {
                // Struct-style variant: Variant { field: Type, ... }
                (parse_fields(&brace_group.0)?, false)
            } else {
                // Unit variant
                (Vec::new(), false)
            };

            Ok((name, fields, is_tuple, bpaf_attrs, doc_comment_strings))
        });

        match variant_result {
            Ok((name, fields, is_tuple, bpaf_attrs, doc_comment_strings)) => {
                // Parse and validate variant attributes AFTER the transaction succeeds
                // This ensures validation errors are propagated, not swallowed
                let variant_attrs = parse_variant_attrs(&bpaf_attrs)?;

                variants.push(EnumBranch {
                    name,
                    fields,
                    doc_comments: doc_comment_strings,
                    is_command: variant_attrs.is_command,
                    command_name: variant_attrs.command_name,
                    skip: variant_attrs.skip,
                    hide: variant_attrs.hide,
                    fallback_to_usage: variant_attrs.fallback_to_usage,
                    is_tuple,
                    long_aliases: variant_attrs.long_aliases,
                    short_aliases: variant_attrs.short_aliases,
                    help: variant_attrs.help,
                });
                let _ = iter.parse::<Comma>();
            }

            Err(_) => {
                // No more variants
                break;
            }
        }
    }

    Ok(variants)
}

/// Enum Decor - variant-level attributes
#[derive(Default)]
struct Ed {
    is_command: bool,
    command_name: Option<String>,
    skip: bool,
    hide: bool,
    fallback_to_usage: bool,
    long_aliases: Vec<String>,
    short_aliases: Vec<char>,
    help: Option<String>,
}

/// Parse variant-level attributes from #[bpaf(...)]
/// Accepts parsed BpafAttr structures from the unsynn grammar
fn parse_variant_attrs(attrs: &[BpafAttr]) -> unsynn::Result<Ed> {
    let mut result = Ed::default();

    for bpaf_attr in attrs {
        // Access the inner DelimitedVec directly from the parsed structure
        for delimited in bpaf_attr.inner.content.iter() {
            match &delimited.value {
                BpafInner::Command(cmd) => {
                    // Check if we've already seen a command attribute
                    if result.is_command {
                        let mut iter = bpaf_attr.to_token_iter();
                        return unsynn::Error::other(
                            iter.next(),
                            &iter,
                            "Multiple 'command' attributes are not allowed. Use 'short' and 'long' for command aliases.".to_string(),
                        );
                    }
                    result.is_command = true;
                    result.command_name = cmd.name.as_ref().map(|g| g.content.as_str().to_string());
                }
                BpafInner::Skip(_) => {
                    result.skip = true;
                }
                BpafInner::Hide(_) => {
                    result.hide = true;
                }
                BpafInner::FallbackToUsage(_) => {
                    result.fallback_to_usage = true;
                }
                BpafInner::Long(long) => {
                    if let Some(name) = long.name.as_ref() {
                        result.long_aliases.push(name.content.as_str().to_string());
                    }
                }
                BpafInner::Short(short) => {
                    if let Some(ch) = short.ch.as_ref() {
                        result.short_aliases.push(ch.content.value());
                    }
                }
                BpafInner::Help(h) => {
                    let val = h.text.content.value();
                    let stripped = val.trim_matches('"');
                    result.help = Some(stripped.to_string());
                }
                _ => {}
            }
        }
    }

    Ok(result)
}

/// Parse doc comments into paragraphs directly from DocInner structures
///
/// Groups consecutive non-empty doc comments, using empty lines as paragraph separators.
/// Returns (first_paragraph, second_paragraph, remaining_paragraphs)
fn parse_doc_paragraphs(
    doc_attrs: &[DocInner],
) -> (Option<String>, Option<String>, Option<String>) {
    let mut paragraphs = Vec::new();
    let mut current = String::new();
    let mut prev_empty = false;

    for doc_inner in doc_attrs {
        let s = doc_inner.value.as_str();
        let content = s.strip_prefix(' ').unwrap_or(s);

        if content.is_empty() {
            if prev_empty && !current.is_empty() {
                // Double empty line - paragraph separator
                current.truncate(current.trim_end().len());
                paragraphs.push(current.clone());
                current.clear();
                prev_empty = false;
            } else {
                prev_empty = true;
            }
        } else {
            if prev_empty && !current.is_empty() {
                current.push('\n');
            }
            current.push_str(&content);
            current.push('\n');
            prev_empty = false;
        }
    }

    // Push final paragraph if any
    if !current.is_empty() {
        current.truncate(current.trim_end().len());
        paragraphs.push(current);
    }

    let first = paragraphs.get(0).cloned();
    let second = paragraphs.get(1).cloned().filter(|s| !s.is_empty());
    let rest = if paragraphs.len() > 2 {
        Some(paragraphs[2..].join("\n"))
    } else {
        None
    };

    (first, second, rest)
}

/// Split doc comments into descr/header/footer for Options mode
///
/// Parses doc comment paragraphs and assigns them to descr/header/footer:
/// - First paragraph -> descr
/// - Second paragraph -> header
/// - Third and subsequent paragraphs -> footer
fn split_options_help(doc_attrs: &[DocInner], opts: &mut crate::mode::OptionsCfg) {
    let (first, second, rest) = parse_doc_paragraphs(doc_attrs);

    if let Some(descr) = first {
        if opts.descr.is_none() {
            opts.descr = Some(quote! { #descr });
        }
    }

    if let Some(header) = second {
        if opts.header.is_none() {
            opts.header = Some(quote! { #header });
        }
    }

    if let Some(footer) = rest {
        if opts.footer.is_none() {
            opts.footer = Some(quote! { #footer });
        }
    }
}

/// Extract all doc comments as individual strings
fn extract_doc_comments(doc_attrs: &[DocInner]) -> Vec<String> {
    doc_attrs
        .iter()
        .map(|doc_inner| {
            let s = doc_inner.value.as_str();
            s.strip_prefix(' ').unwrap_or(s).to_string()
        })
        .collect()
}

/// Represents a single field in a struct
#[derive(Clone)]
pub struct StructField {
    /// Field name
    pub name: Ident,
    /// Field type (stored as tokens)
    pub ty: TokenStream,
    /// The shape of the field's type (includes inner type for Option/Vec)
    pub shape: TypeShape,
    /// Parsed attributes from #[bpaf(...)]
    pub attrs: FieldAttrs,
}

impl StructField {
    /// Get the appropriate type for the parser based on field shape
    /// Returns a reference to the inner type for Vec/Option, otherwise the field type
    fn get_parser_type(&self) -> &TokenStream {
        self.shape.inner_type().unwrap_or(&self.ty)
    }
}

/// Represents the top-level item being derived
pub struct Top {
    /// Name of the struct or enum
    pub name: Ident,
    /// Body: either struct fields or enum variants
    pub body: Body,
    /// Whether the struct/enum has #[bpaf(adjacent)] attribute
    pub adjacent: bool,
    /// Mode for the generated parser function
    pub mode: Mode,
    /// Fallback value for the parser (from #[bpaf(fallback(...))])
    pub fallback: Option<proc_macro2::TokenStream>,
    /// Custom path to the bpaf crate (from #[bpaf(path(...))])
    pub bpaf_path: Option<proc_macro2::TokenStream>,
    /// Custom function name (from #[bpaf(generate(...))])
    pub custom_name: Option<Ident>,
    /// Generate private function (from #[bpaf(private)])
    pub private: bool,
    /// Add .boxed() wrapper (from #[bpaf(boxed)])
    pub boxed: bool,
    /// Top-level PostDecor attributes (guard, hide, etc.)
    pub attrs: Vec<Post>,
}

/// The body of the derived item
#[derive(Clone)]
pub enum Body {
    /// Struct with fields
    Struct(Vec<StructField>),
    /// Enum with variants
    Enum(Vec<EnumBranch>),
}

/// Represents an enum variant
#[derive(Clone)]
pub struct EnumBranch {
    /// Variant name
    pub name: Ident,
    /// Fields in the variant (if any)
    pub fields: Vec<StructField>,
    /// Doc comments extracted from #[doc = "..."]
    pub doc_comments: Vec<String>,
    /// Whether this variant has #[bpaf(command)]
    pub is_command: bool,
    /// Custom command name from #[bpaf(command("name"))]
    pub command_name: Option<String>,
    /// Whether this variant has #[bpaf(skip)]
    pub skip: bool,
    /// Whether this variant has #[bpaf(hide)]
    pub hide: bool,
    /// Whether this variant has #[bpaf(fallback_to_usage)]
    pub fallback_to_usage: bool,
    /// Whether this is a tuple variant (e.g., Variant(Type))
    pub is_tuple: bool,
    /// Long aliases from #[bpaf(long("alias"))]
    pub long_aliases: Vec<String>,
    /// Short aliases from #[bpaf(short('x'))]
    pub short_aliases: Vec<char>,
    /// Explicit help text from #[bpaf(help("..."))]
    pub help: Option<String>,
}

/// Target for field construction - used by emit_fields_construct
enum ConstructTarget<'a> {
    /// Construct a struct: StructName { field1, field2 }
    Struct,
    /// Construct a tuple enum variant: EnumName::Variant(field1, field2)
    EnumTupleVariant {
        enum_name: &'a Ident,
        variant_name: &'a Ident,
    },
    /// Construct a struct enum variant: EnumName::Variant { field1, field2 }
    EnumStructVariant {
        enum_name: &'a Ident,
        variant_name: &'a Ident,
    },
}

/// Parse struct-level attributes from #[bpaf(...)]
/// Top-level struct/enum attributes
#[derive(Default)]
struct TopInfo {
    adjacent: bool,
    mode: Mode,
    fallback: Option<proc_macro2::TokenStream>,
    bpaf_path: Option<proc_macro2::TokenStream>,
    custom_name: Option<Ident>,
    private: bool,
    boxed: bool,
    ignore_rustdoc: bool,
    attrs: Vec<Post>,
}

/// Helper function to apply a change to OptionsCfg if it exists
fn with_options<F>(options: &mut Option<crate::mode::OptionsCfg>, f: F)
where
    F: FnOnce(&mut crate::mode::OptionsCfg),
{
    if let Some(opts) = options.as_mut() {
        f(opts);
    }
}

/// Helper function to apply a change to ParserCfg if it exists
fn with_parser<F>(parser: &mut Option<crate::mode::ParserCfg>, f: F)
where
    F: FnOnce(&mut crate::mode::ParserCfg),
{
    if let Some(p) = parser.as_mut() {
        f(p);
    }
}

/// Helper function to apply a change to CommandCfg if it exists
fn with_command<F>(command: &mut Option<crate::mode::CommandCfg>, f: F)
where
    F: FnOnce(&mut crate::mode::CommandCfg),
{
    if let Some(cmd) = command.as_mut() {
        f(cmd);
    }
}

fn parse_struct_attrs(bpaf_attr: &BpafAttr) -> Result<Option<TopInfo>> {
    use crate::mode::{CommandCfg, OptionsCfg, ParserCfg};

    // Mode configuration tracking (following bpaf_derive pattern)
    let mut command: Option<CommandCfg> = None;
    let mut options: Option<OptionsCfg> = None;
    let mut parser: Option<ParserCfg> = Some(ParserCfg::default());

    // Other top-level attributes
    let mut adjacent = false;
    let mut fallback = None;
    let mut bpaf_path = None;
    let mut custom_name = None;
    let mut private = false;
    let mut boxed = false;
    let mut ignore_rustdoc = false;
    let mut top_attrs = Vec::new();

    let mut first = true;

    // Access the inner DelimitedVec directly from the parsed structure
    for delimited in bpaf_attr.inner.content.iter() {
        match &delimited.value {
            // Mode keywords (must be first)
            BpafInner::Options(_) if first => {
                options = Some(OptionsCfg::default());
                parser = None;
            }
            BpafInner::Command(cmd) if first => {
                command = Some(CommandCfg {
                    name: cmd.name.as_ref().map(|g| g.content.as_str().to_string()),
                    ..CommandCfg::default()
                });
                options = Some(OptionsCfg::default());
                parser = None;
            }
            BpafInner::Parser(_) if first => {
                // Already default
            }

            // Non-mode attributes
            BpafInner::Adjacent(_) => {
                adjacent = true;
            }
            BpafInner::Fallback(fb) => {
                fallback = Some(fb.expr.0.stream());
            }
            BpafInner::Path(p) => {
                bpaf_path = Some(p.path.0.stream());
            }
            BpafInner::Generate(gen) => {
                custom_name = Some(gen.name.content.clone());
            }
            BpafInner::Private(_) => {
                private = true;
            }
            BpafInner::Boxed(_) => {
                boxed = true;
            }
            BpafInner::IgnoreRustdoc(_) => {
                ignore_rustdoc = true;
            }

            // Options mode attributes
            BpafInner::Descr(d) => {
                with_options(&mut options, |opts| {
                    opts.descr = Some(d.expr.0.stream());
                });
            }
            BpafInner::Footer(f) => {
                with_options(&mut options, |opts| {
                    opts.footer = Some(f.expr.0.stream());
                });
            }
            BpafInner::Header(h) => {
                with_options(&mut options, |opts| {
                    opts.header = Some(h.expr.0.stream());
                });
            }
            BpafInner::Usage(u) => {
                with_options(&mut options, |opts| opts.usage = Some(u.expr.0.stream()));
            }
            BpafInner::Version(v) => {
                with_options(&mut options, |opts| opts.version = Some(v.expr.0.stream()));
            }
            BpafInner::MaxWidth(mw) => {
                with_options(&mut options, |opts| {
                    opts.max_width = Some(mw.expr.0.stream())
                });
            }
            BpafInner::CargoHelper(ch) => {
                with_options(&mut options, |opts| {
                    opts.cargo_helper = Some(ch.name.content.value().to_string());
                });
            }
            BpafInner::FallbackToUsage(_) => {
                with_options(&mut options, |opts| opts.fallback_usage = true);
            }

            // Command mode attributes (short, long, help for commands)
            BpafInner::Short(s) => {
                with_command(&mut command, |cmd| {
                    if let Some(ch) = &s.ch {
                        cmd.short.push(ch.content.value());
                    }
                });
            }
            BpafInner::Long(l) => {
                with_command(&mut command, |cmd| {
                    if let Some(name) = &l.name {
                        // unsynn's LiteralString includes quotes, so strip them
                        let val = name.content.value();
                        let stripped = val.trim_matches('"');
                        cmd.long.push(stripped.to_string());
                    }
                });
            }
            BpafInner::Help(h) => {
                with_command(&mut command, |cmd| {
                    // unsynn's LiteralString includes quotes, so strip them
                    let val = h.text.content.value();
                    let stripped = val.trim_matches('"');
                    cmd.help = Some(quote! { #stripped });
                });
            }

            // PostDecor attributes at struct/enum level
            BpafInner::Guard(gi) => {
                let (check, msg) =
                    crate::attrs::parse_two_args(&gi.args.0, "guard", "check_fn, error_message")?;
                top_attrs.push(Post::Decor(PostDecor::Guard { check, msg }));
            }
            BpafInner::Hide(_) => {
                top_attrs.push(Post::Decor(PostDecor::Hide));
            }
            BpafInner::HideUsage(_) => {
                top_attrs.push(Post::Decor(PostDecor::HideUsage));
            }
            BpafInner::CustomUsage(cu) => {
                top_attrs.push(Post::Decor(PostDecor::CustomUsage {
                    usage: cu.text.0.stream(),
                }));
            }
            BpafInner::FallbackWith(fw) => {
                top_attrs.push(Post::Decor(PostDecor::FallbackWith {
                    f: fw.func.0.stream(),
                }));
            }
            BpafInner::GroupHelp(gh) => {
                with_parser(&mut parser, |p| {
                    p.group_help = Some(gh.text.0.stream());
                });
            }
            BpafInner::Complete(c) => {
                top_attrs.push(Post::Decor(PostDecor::Complete {
                    f: c.func.0.stream(),
                }));
            }
            BpafInner::Group(g) => {
                top_attrs.push(Post::Decor(PostDecor::CompleteGroup {
                    group: g.name.content.value().to_string(),
                }));
            }
            _ => {}
        }

        first = false;
    }

    // Construct Mode from parsed configurations
    let mode = match (options, command) {
        (Some(options), Some(command)) => Mode::Command { command, options },
        (Some(options), None) => Mode::Options { options },
        _ => Mode::Parser {
            parser: parser.unwrap_or_default(),
        },
    };

    Ok(Some(TopInfo {
        adjacent,
        mode,
        fallback,
        bpaf_path,
        custom_name,
        private,
        boxed,
        ignore_rustdoc,
        attrs: top_attrs,
    }))
}

impl Parser for Top {
    fn parser(input: &mut TokenIter) -> Result<Self> {
        // Collect struct-level attributes using unsynn grammar
        let mut bpaf_attrs = Vec::new();
        let mut doc_comments = Vec::new();
        loop {
            let attr_result = input.transaction(|t| {
                let _: Pound = t.parse()?;
                let attr_group: BracketGroupContaining<Attribute> = t.parse()?;

                match attr_group.content {
                    Attribute::Bpaf(bpaf_attr) => {
                        bpaf_attrs.push(bpaf_attr);
                    }
                    Attribute::Doc(doc_inner) => {
                        doc_comments.push(doc_inner);
                    }
                    Attribute::Other(_) => {
                        // Ignore other attributes
                    }
                }
                Ok(())
            });
            if attr_result.is_err() {
                break;
            }
        }

        // Parse struct-level attributes from parsed BpafAttr structures
        let mut adjacent = false;
        let mut mode = Mode::default();
        let mut fallback = None;
        let mut bpaf_path = None;
        let mut custom_name = None;
        let mut private = false;
        let mut boxed = false;
        let mut ignore_rustdoc = false;
        let mut top_attrs = Vec::new();
        for bpaf_attr in &bpaf_attrs {
            if let Some(attrs) = parse_struct_attrs(bpaf_attr)? {
                if attrs.adjacent {
                    adjacent = true;
                }
                // Always take the mode from parsed attributes
                mode = attrs.mode;
                if attrs.ignore_rustdoc {
                    ignore_rustdoc = true;
                }
                top_attrs.extend(attrs.attrs);
                if let Some(fb) = attrs.fallback {
                    fallback = Some(fb);
                }
                if let Some(bp) = attrs.bpaf_path {
                    bpaf_path = Some(bp);
                }
                if let Some(cn) = attrs.custom_name {
                    custom_name = Some(cn);
                }
                if attrs.private {
                    private = true;
                }
                if attrs.boxed {
                    boxed = true;
                }
            }
        }

        // Apply struct-level doc comments if ignore_rustdoc is false
        if !ignore_rustdoc && !doc_comments.is_empty() {
            // Split and apply based on mode
            match &mut mode {
                Mode::Options { options } => {
                    split_options_help(&doc_comments, options);
                }
                Mode::Command { options, .. } => {
                    split_options_help(&doc_comments, options);
                }
                Mode::Parser { parser } => {
                    // Parser mode uses all doc comments joined as group_help
                    if parser.group_help.is_none() {
                        let (first, second, rest) = parse_doc_paragraphs(&doc_comments);
                        let mut parts = Vec::new();
                        if let Some(f) = first {
                            parts.push(f);
                        }
                        if let Some(s) = second {
                            parts.push(s);
                        }
                        if let Some(r) = rest {
                            parts.push(r);
                        }
                        let doc_text = parts.join("\n");
                        parser.group_help = Some(quote! { #doc_text });
                    }
                }
            }
        }

        // Try to skip visibility modifier if present
        let _ = input.parse::<Visibility>();

        // Expect "struct" or "enum" keyword
        let first_token = input.clone().next();
        let is_enum = match input.parse::<unsynn::Either<KStruct, KEnum>>() {
            Ok(unsynn::Either::First(_)) => false,
            Ok(unsynn::Either::Second(_)) => true,
            Ok(_) => unreachable!(),
            Err(_) => {
                return unsynn::Error::other(
                    first_token,
                    &input,
                    "Only structs and enums are supported".to_string(),
                );
            }
        };

        // Get the struct/enum name
        let name: Ident = input.parse()?;

        // Skip generics and where clauses until we hit the body ({ or ;)
        // LazyVecUntil stops before consuming the terminator
        let _generics_and_where: unsynn::LazyVecUntil<
            TokenTree,
            unsynn::Either<unsynn::BraceGroup, unsynn::Semicolon>,
        > = input.parse()?;

        // Parse the body: either a brace group with fields/variants, or semicolon for unit struct
        let body = match input.parse::<unsynn::Either<unsynn::BraceGroup, unsynn::Semicolon>>()? {
            unsynn::Either::First(body_group) => {
                if is_enum {
                    let variants = parse_enum_variants(&body_group.0)?;
                    if variants.is_empty() {
                        return unsynn::Error::other(
                            Some(TokenTree::Group(body_group.0)),
                            &input,
                            "Enums must have at least one variant".to_string(),
                        );
                    }
                    Body::Enum(variants)
                } else {
                    Body::Struct(parse_fields(&body_group.0)?)
                }
            }
            unsynn::Either::Second(_semicolon) => {
                // Unit struct is fine; unit enum (enum Foo;) is invalid Rust
                // syntax so the compiler will reject it anyway
                Body::Struct(Vec::new())
            }
            _ => unreachable!(),
        };

        Ok(Top {
            name,
            body,
            adjacent,
            mode,
            fallback,
            bpaf_path,
            custom_name,
            private,
            boxed,
            attrs: top_attrs,
        })
    }
}

impl ToTokens for Top {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;

        // Generate parser function
        let mut parser_body = self.emit_body();

        // Apply fallback if specified (must be before mode wrappers)
        if let Some(ref fallback_expr) = self.fallback {
            parser_body = quote! { #parser_body.fallback(#fallback_expr) };
        }

        // Apply top-level PostDecor attributes (guard, hide, etc.)
        for attr in &self.attrs {
            parser_body = self.apply_post(parser_body, attr);
        }

        // Get the bpaf crate path
        let bpaf = self.bpaf_crate();

        // Apply mode-specific wrappers
        match &self.mode {
            Mode::Options { options } => {
                // Apply cargo_helper BEFORE .to_options() (wraps the Parser)
                if let Some(ref cargo_helper) = options.cargo_helper {
                    parser_body = quote! { #bpaf::cargo_helper(#cargo_helper, #parser_body) };
                }
                // Add .to_options()
                parser_body = quote! { #parser_body.to_options() };
                // Apply options configuration (excluding cargo_helper which was already applied)
                parser_body = self.apply_options_cfg(parser_body, options, &bpaf);
            }
            Mode::Command { command, options } => {
                // Add .to_options() first
                parser_body = quote! { #parser_body.to_options() };
                // Apply options configuration BEFORE calling .command()
                // Note: cargo_helper is ignored in command mode (same as bpaf_derive)
                parser_body = self.apply_options_cfg(parser_body, options, &bpaf);
                // Then add .command("name")
                // ParseCommand<T> implements Parser<T> so we can return it as impl Parser<T>
                let default_name = name.to_string().to_lowercase();
                let cmd_name = command.name.as_deref().unwrap_or(&default_name);
                parser_body = quote! { #parser_body.command(#cmd_name) };

                // Apply command aliases (short, long, help)
                for short_char in &command.short {
                    parser_body = quote! { #parser_body.short(#short_char) };
                }
                for long_name in &command.long {
                    parser_body = quote! { #parser_body.long(#long_name) };
                }
                if let Some(ref help) = command.help {
                    parser_body = quote! { #parser_body.help(#help) };
                }

                // Apply .boxed() AFTER .command() for command mode
                if self.boxed {
                    parser_body = quote! { #parser_body.boxed() };
                }
            }
            Mode::Parser { parser: parser_cfg } => {
                // Apply parser configuration (group_help)
                if let Some(ref group_help) = parser_cfg.group_help {
                    parser_body = quote! { #parser_body.group_help(#group_help) };
                }

                // Apply .boxed() at the end for parser mode
                if self.boxed {
                    parser_body = quote! { #parser_body.boxed() };
                }
            }
        }

        // Generate return type based on mode (boxed changes return type for parser/command mode)
        let return_type = match (&self.mode, self.boxed) {
            (Mode::Options { .. }, _) => quote! { #bpaf::OptionParser<Self> },
            (Mode::Command { .. }, true) | (Mode::Parser { .. }, true) => {
                quote! { Box<dyn #bpaf::Parser<Self>> }
            }
            (Mode::Command { .. }, false) | (Mode::Parser { .. }, false) => {
                quote! { impl #bpaf::Parser<Self> }
            }
        };

        // Determine function name
        let default_fn_name = quote::format_ident!("parse");
        let fn_name = self.custom_name.as_ref().unwrap_or(&default_fn_name);

        // Determine visibility
        let visibility = if self.private {
            quote! {}
        } else {
            quote! { pub }
        };

        tokens.extend(quote! {
            #[allow(unused_imports)]
            impl #name {
                #visibility fn #fn_name() -> #return_type {
                    use #bpaf::Parser as _;
                    #parser_body
                }
            }
        });
    }
}

impl Top {
    /// Get the bpaf crate path to use (either custom or default ::bpaf)
    fn bpaf_crate(&self) -> TokenStream {
        if let Some(custom_path) = &self.bpaf_path {
            quote! { #custom_path }
        } else {
            quote! { ::bpaf }
        }
    }

    /// Apply OptionsCfg configuration to the parser
    fn apply_options_cfg(
        &self,
        mut parser: TokenStream,
        cfg: &crate::mode::OptionsCfg,
        _bpaf: &TokenStream,
    ) -> TokenStream {
        // Apply descr
        if let Some(ref descr) = cfg.descr {
            parser = quote! { #parser.descr(#descr) };
        }
        // Apply footer
        if let Some(ref footer) = cfg.footer {
            parser = quote! { #parser.footer(#footer) };
        }
        // Apply header
        if let Some(ref header) = cfg.header {
            parser = quote! { #parser.header(#header) };
        }
        // Apply usage
        if let Some(ref usage) = cfg.usage {
            parser = quote! { #parser.usage(#usage) };
        }
        // Apply version
        if let Some(ref version) = cfg.version {
            parser = quote! { #parser.version(#version) };
        }
        // Apply max_width
        if let Some(ref max_width) = cfg.max_width {
            parser = quote! { #parser.max_width(#max_width) };
        }
        // Apply fallback_usage
        if cfg.fallback_usage {
            parser = quote! { #parser.fallback_to_usage() };
        }
        // Note: cargo_helper is handled in to_tokens() before .to_options()
        // It's ignored in command mode (same as bpaf_derive)
        parser
    }

    /// Emit the parser body based on fields or variants
    fn emit_body(&self) -> TokenStream {
        match &self.body {
            Body::Struct(fields) => self.emit_struct(fields),
            Body::Enum(variants) => self.emit_enum(variants),
        }
    }

    /// Emit tokens for a struct parser
    fn emit_struct(&self, fields: &[StructField]) -> TokenStream {
        let bpaf = self.bpaf_crate();

        if fields.is_empty() {
            // Empty struct - return a pure value
            let name = &self.name;
            return quote! {
                #bpaf::pure(#name {})
            };
        }

        // Generate field parsers and construction code
        let construct = self.emit_fields_construct(fields, ConstructTarget::Struct);

        // Apply .adjacent() if the struct has the adjacent flag
        if self.adjacent {
            quote! { #construct.adjacent() }
        } else {
            construct
        }
    }

    /// Emit tokens for an enum parser
    fn emit_enum(&self, variants: &[EnumBranch]) -> TokenStream {
        let bpaf = self.bpaf_crate();
        let enum_name = &self.name;

        // Check if any variant is a command
        let has_commands = variants.iter().any(|v| v.is_command);

        // Generate parser for each variant (skip variants with skip=true)
        let variant_parsers: Vec<TokenStream> = variants
            .iter()
            .filter(|v| !v.skip)
            .map(|variant| {
                let variant_name = &variant.name;
                let command_name = variant.command_name.clone().unwrap_or_else(|| to_kebab_case(&variant_name.to_string()));

                // Get help text - prefer explicit help() over doc comments
                let help_text = variant.help.clone().or_else(|| {
                    if !variant.doc_comments.is_empty() {
                        Some(variant.doc_comments.join("\n"))
                    } else {
                        None
                    }
                });

                if has_commands && variant.is_command {
                    // Command-based variant
                    if variant.fields.is_empty() {
                        // Unit variant - simple command
                        let base = quote! {
                            #bpaf::pure(#enum_name::#variant_name).to_options()
                        };
                        apply_command_modifiers(base, help_text, variant, &command_name)
                    } else {
                        // Variant with fields - construct from fields
                        let construct_target = if variant.is_tuple {
                            ConstructTarget::EnumTupleVariant { enum_name, variant_name }
                        } else {
                            ConstructTarget::EnumStructVariant { enum_name, variant_name }
                        };
                        let construct = self.emit_fields_construct(&variant.fields, construct_target);
                        let base = quote! { #construct.to_options() };
                        apply_command_modifiers(base, help_text, variant, &command_name)
                    }
                } else {
                    // Regular flag-based variant (no command attribute)
                    if has_commands && !variant.fields.is_empty() {
                        // Non-command variants with fields in mixed enums are not supported
                        let error_msg = format!(
                            "Variant '{}' has fields but no #[bpaf(command)] attribute. \
                             In enums with command variants, non-command variants must be unit variants.",
                            variant_name
                        );
                        return spanned_compile_error(&variant.name, error_msg);
                    }

                    if variant.fields.is_empty() {
                        // Unit variant - use long/short flags
                        // Note: .help() must be called BEFORE .req_flag() (on NamedArg, not Parser)
                        if let Some(help) = help_text {
                            quote! {
                                #bpaf::long(#command_name).help(#help).req_flag(#enum_name::#variant_name)
                            }
                        } else {
                            quote! {
                                #bpaf::long(#command_name).req_flag(#enum_name::#variant_name)
                            }
                        }
                    } else {
                        // Pure flag-based enum with variant fields - construct from fields
                        let construct_target = if variant.is_tuple {
                            ConstructTarget::EnumTupleVariant { enum_name, variant_name }
                        } else {
                            ConstructTarget::EnumStructVariant { enum_name, variant_name }
                        };
                        self.emit_fields_construct(&variant.fields, construct_target)
                    }
                }
            })
            .collect();

        // Combine variants - generate named parsers and combine with construct!
        if variant_parsers.is_empty() {
            quote! { #bpaf::fail("No variants available") }
        } else {
            // Generate variable names for each variant parser
            let var_names: Vec<Ident> = (0..variant_parsers.len())
                .map(|i| quote::format_ident!("variant_{}", i))
                .collect();

            let parsers_iter = variant_parsers.iter();
            let var_names_construct = var_names.iter();
            let var_names_bind = var_names.iter();

            quote! {
                {
                    #( let #var_names_bind = #parsers_iter; )*
                    #bpaf::construct!( [ #( #var_names_construct ),* ] )
                }
            }
        }
    }

    /// Emit tokens for a field's parser
    fn emit_field(&self, field: &StructField) -> TokenStream {
        let bpaf = self.bpaf_crate();
        let field_name = &field.name;
        let field_name_str = field_name.to_string();

        // Determine the names to use based on attributes
        let short_spec = field.attrs.short;
        let long_spec = field.attrs.long.as_ref();
        let env_spec = &field.attrs.env;

        // Check for explicit consumer type or use shape-based default
        let consumer = field.attrs.consumer.as_ref();

        // Build the base parser with names (for named parsers)
        // If only env is specified (no short/long), we'll use None and handle it later
        // For Any/Positional consumers without names, we also use None (they're standalone)
        let mut base_parser = if let Some(short) = short_spec {
            if let Some(long) = long_spec {
                // Both short and long
                Some(quote! { #bpaf::short(#short).long(#long) })
            } else {
                // Just short
                Some(quote! { #bpaf::short(#short) })
            }
        } else if let Some(long) = long_spec {
            // Just long
            Some(quote! { #bpaf::long(#long) })
        } else if env_spec.is_none() {
            // Neither short/long/env specified
            // Check if this is a standalone Any or Positional consumer
            let is_standalone = matches!(
                consumer,
                Some(ConsumerType::Any { .. }) | Some(ConsumerType::Positional { .. })
            );

            if !is_standalone {
                // Default to long name derived from field
                let default_long = to_kebab_case(&field_name_str);
                Some(quote! { #bpaf::long(#default_long) })
            } else {
                // Standalone Any/Positional - no base parser
                None
            }
        } else {
            // Only env specified - will use argument() directly
            None
        };

        // Handle env-only fields (no short/long)
        if let (None, Some(ref env_expr)) = (&base_parser, &env_spec) {
            // Use #bpaf::env() directly for env-only fields
            base_parser = Some(quote! { #bpaf::env(#env_expr) });
        } else if let Some(ref env_expr) = env_spec {
            // Chain .env() onto existing short/long parser
            if let Some(bp) = base_parser {
                base_parser = Some(quote! { #bp.env(#env_expr) });
            }
        }

        // Generate base parser based on consumer type or shape
        let mut parser = match consumer {
            // Handle standalone Positional first (when base_parser is None)
            Some(ConsumerType::Positional { metavar }) if base_parser.is_none() => {
                let ty = field.get_parser_type();
                let metavar_str = metavar.as_deref().unwrap_or(DEFAULT_POSITIONAL_METAVAR);
                quote! { #bpaf::positional::<#ty>(#metavar_str) }
            }

            // Handle standalone Any first (when base_parser is None)
            Some(ConsumerType::Any { metavar, ty, check }) if base_parser.is_none() => {
                if let Some(ty) = ty {
                    // With explicit type: any::<Type, _, _>(metavar, check)
                    quote! { #bpaf::any::<#ty, _, _>(#metavar, #check) }
                } else {
                    // Without type: any(metavar, check)
                    quote! { #bpaf::any(#metavar, #check) }
                }
            }

            // All other cases need base_parser
            _ => {
                let base_parser = match base_parser {
                    Some(bp) => bp,
                    None => {
                        // This should be unreachable with current logic, but provide a helpful error
                        let error_msg = format!(
                            "Internal error: field '{}' requires a base parser (short/long/env), but none was generated. \
                             This is a bug in the derive macro. Please report this at https://github.com/pacak/bpaf",
                            field_name_str
                        );
                        return quote! { compile_error!(#error_msg) };
                    }
                };

                match consumer {
                    Some(ConsumerType::Switch) => {
                        quote! { #base_parser.switch() }
                    }
                    Some(ConsumerType::Argument { metavar }) => {
                        let ty = field.get_parser_type();
                        let metavar_str = metavar.as_deref().unwrap_or(&field_name_str);
                        quote! { #base_parser.argument::<#ty>(#metavar_str) }
                    }
                    Some(ConsumerType::Positional { .. }) => {
                        // This case is unreachable due to validation in attrs.rs
                        // Positional consumer cannot be combined with base_parser (short/long)
                        unreachable!("Positional consumer with naming attributes should be rejected during parsing")
                    }
                    Some(ConsumerType::Flag { present, absent }) => {
                        quote! { #base_parser.flag(#present, #absent) }
                    }
                    Some(ConsumerType::ReqFlag { present }) => {
                        quote! { #base_parser.req_flag(#present) }
                    }
                    Some(ConsumerType::Any {
                        metavar,
                        ty: _,
                        check,
                    }) => {
                        // Named any - use base_parser.argument().parse()
                        quote! {
                            #base_parser.argument::<String>(#metavar).parse(|s: String| {
                                (#check)(s).ok_or("validation failed")
                            })
                        }
                    }
                    Some(ConsumerType::External { ident }) => {
                        if let Some(ref path) = ident {
                            quote! { #path() }
                        } else {
                            // Use default based on field name
                            let fn_name = &field.name;
                            quote! { #fn_name() }
                        }
                    }
                    Some(ConsumerType::Pure { expr }) => {
                        quote! { #bpaf::pure(#expr) }
                    }
                    Some(ConsumerType::PureWith { expr }) => {
                        quote! { #bpaf::pure_with(#expr) }
                    }
                    None => {
                        // Infer from shape
                        match &field.shape {
                            TypeShape::Bool => {
                                quote! { #base_parser.switch() }
                            }
                            TypeShape::Direct(ty) => {
                                quote! { #base_parser.argument::<#ty>(#field_name_str) }
                            }
                            TypeShape::Optional(inner_ty) => {
                                quote! { #base_parser.argument::<#inner_ty>(#field_name_str) }
                            }
                            TypeShape::Multiple(inner_ty) => {
                                quote! { #base_parser.argument::<#inner_ty>(#field_name_str) }
                            }
                            TypeShape::Unit => {
                                quote! { #bpaf::pure(()) }
                            }
                        }
                    }
                }
            }
        };

        // Add help if specified and supported by this consumer type
        // External, Pure, and PureWith don't support .help()
        let help_supported = !matches!(
            consumer,
            Some(ConsumerType::External { .. })
                | Some(ConsumerType::Pure { .. })
                | Some(ConsumerType::PureWith { .. })
        );

        if help_supported {
            if let Some(ref help_text) = field.attrs.help {
                parser = quote! { #parser.help(#help_text) };
            }
        }

        // Add fallback if specified
        if let Some(ref fallback_expr) = field.attrs.fallback {
            parser = quote! { #parser.fallback(#fallback_expr) };
        }

        // Determine if we need implicit PostParse attributes
        // Only add implicit .optional() or .many() if there are NO explicit PostParse attributes
        let has_explicit_postparse = field
            .attrs
            .postpr
            .iter()
            .any(|p| matches!(p, Post::Parse(_)));

        // Build list of PostParse attributes to apply
        let mut postpr_to_apply = field.attrs.postpr.clone();

        // Don't apply implicit post-processing for Pure or PureWith - they specify exact values
        let is_pure = matches!(
            consumer,
            Some(ConsumerType::Pure { .. }) | Some(ConsumerType::PureWith { .. })
        );

        if !has_explicit_postparse && !is_pure {
            // Add implicit PostParse attributes based on type shape
            if field.shape.is_optional() {
                // Insert implicit .optional() at position 0
                // This works for both positional and named arguments
                postpr_to_apply.insert(0, Post::Parse(PostParse::Optional));
            } else if field.shape.is_multiple() {
                // Insert implicit .many() at position 0
                postpr_to_apply.insert(0, Post::Parse(PostParse::Many));
            }
        }

        // Apply post-processing attributes in order
        for post in &postpr_to_apply {
            parser = self.apply_post(parser, post);
        }

        parser
    }

    /// Generate field parsers and construction code for a set of fields
    /// Used by structs and enum variants to build parsers from fields
    fn emit_fields_construct(
        &self,
        fields: &[StructField],
        construct_target: ConstructTarget,
    ) -> TokenStream {
        let bpaf = self.bpaf_crate();

        // Generate parsers for each field
        let field_parsers: Vec<TokenStream> =
            fields.iter().map(|field| self.emit_field(field)).collect();

        // Extract field names
        let field_names: Vec<&Ident> = fields.iter().map(|f| &f.name).collect();

        // Generate construct based on target type
        let construct = match construct_target {
            ConstructTarget::Struct => {
                let struct_name = &self.name;
                quote! { #bpaf::construct!(#struct_name { #( #field_names ),* }) }
            }
            ConstructTarget::EnumTupleVariant {
                enum_name,
                variant_name,
            } => {
                quote! { #bpaf::construct!(#enum_name::#variant_name( #( #field_names ),* )) }
            }
            ConstructTarget::EnumStructVariant {
                enum_name,
                variant_name,
            } => {
                quote! { #bpaf::construct!(#enum_name::#variant_name { #( #field_names ),* }) }
            }
        };

        quote! {
            {
                #( let #field_names = #field_parsers; )*
                #construct
            }
        }
    }

    /// Apply a post-processing attribute to the parser
    fn apply_post(&self, parser: TokenStream, post: &Post) -> TokenStream {
        use Post::*;
        match post {
            Parse(pp) => self.apply_post_parse(parser, pp),
            Decor(pd) => self.apply_post_decor(parser, pd),
        }
    }

    /// Apply a PostParse attribute (type-changing)
    fn apply_post_parse(&self, parser: TokenStream, post: &PostParse) -> TokenStream {
        use PostParse::*;
        match post {
            Map { f } => quote! { #parser.map(#f) },
            Parse { f } => quote! { #parser.parse(#f) },
            Optional => quote! { #parser.optional() },
            Many => quote! { #parser.many() },
            Some { msg } => quote! { #parser.some(#msg) },
            Catch => quote! { #parser.catch() },
            Collect => quote! { #parser.collect() },
            Count => quote! { #parser.count() },
            Anywhere => quote! { #parser.anywhere() },
            Adjacent => quote! { #parser.adjacent() },
            Strict => quote! { #parser.strict() },
            NonStrict => quote! { #parser.non_strict() },
        }
    }

    /// Apply a PostDecor attribute (behavior-changing)
    fn apply_post_decor(&self, parser: TokenStream, post: &PostDecor) -> TokenStream {
        use PostDecor::*;
        match post {
            Guard { check, msg } => quote! { #parser.guard(#check, #msg) },
            Hide => quote! { #parser.hide() },
            HideUsage => quote! { #parser.hide_usage() },
            CustomUsage { usage } => quote! { #parser.custom_usage(#usage) },
            FallbackWith { f } => quote! { #parser.fallback_with(#f) },
            GroupHelp { doc } => quote! { #parser.group_help(#doc) },
            DebugFallback => quote! { #parser.debug_fallback() },
            DisplayFallback => quote! { #parser.display_fallback() },
            FormatFallback { formatter } => quote! { #parser.format_fallback(#formatter) },
            Last => quote! { #parser.last() },
            Complete { f } => quote! { #parser.complete(#f) },
            CompleteGroup { group } => quote! { #parser.group(#group) },
            CompleteShell { f } => quote! { #parser.complete_shell(#f) },
        }
    }
}
