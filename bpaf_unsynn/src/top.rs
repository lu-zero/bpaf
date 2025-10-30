//! Top-level structure parsing

use crate::parsing::*;
use crate::field::{analyze_type, Shape};
use crate::attrs::{FieldAttrs, ConsumerType, Post, PostParse, PostDecor, Mode};
use quote::{quote, ToTokens};

/// Parse fields from a brace group
fn parse_fields(group: &proc_macro2::Group) -> Result<Vec<Field>> {
    let mut fields = Vec::new();
    let stream = group.stream();
    let mut iter = unsynn::ToTokens::to_token_iter(&stream);

    loop {
        // Try to parse a field: name : type,
        let field_result = iter.transaction(|t| {
            // Collect bpaf attributes and doc comments
            let mut bpaf_attrs = Vec::new();
            let mut doc_comments = Vec::new();
            loop {
                let attr_result = t.transaction(|t2| {
                    if let Some(TokenTree::Punct(ref p)) = t2.next() {
                        if p.as_char() == '#' {
                            // Get the attribute group
                            if let Some(TokenTree::Group(ref g)) = t2.next() {
                                // Check if this is a #[bpaf(...)] or #[doc = "..."] attribute
                                let attr_stream = g.stream();
                                let attr_str = attr_stream.to_string();
                                if attr_str.starts_with("bpaf") {
                                    // Extract the inner group from bpaf(...)
                                    let mut attr_iter = unsynn::ToTokens::to_token_iter(&attr_stream);
                                    // Skip "bpaf" identifier
                                    let _ = attr_iter.next();
                                    // Get the parenthesized group
                                    if let Some(TokenTree::Group(ref inner)) = attr_iter.next() {
                                        bpaf_attrs.push(inner.clone());
                                    }
                                } else if attr_str.starts_with("doc") {
                                    // Extract doc comment: doc = "text"
                                    doc_comments.push(g.clone());
                                }
                            }
                            return Ok(true);
                        }
                    }
                    Err(Error::no_error())
                });
                if attr_result.is_err() {
                    break;
                }
            }

            // Skip visibility if present
            let _ = t.transaction(|t2| {
                let ident: Ident = t2.parse()?;
                if ident == "pub" {
                    Ok(())
                } else {
                    Err(Error::no_error())
                }
            });

            // Get field name
            let name: Ident = t.parse()?;

            // Expect colon
            match t.next() {
                Some(TokenTree::Punct(ref p)) if p.as_char() == ':' => {}
                _ => return Err(Error::no_error()),
            }

            // Collect type tokens until we hit a comma or end
            let mut ty_tokens = TokenStream::new();
            loop {
                match t.next() {
                    Some(TokenTree::Punct(ref p)) if p.as_char() == ',' => {
                        break; // End of this field
                    }
                    Some(tt) => {
                        ty_tokens.extend(std::iter::once(tt.clone()));
                    }
                    None => {
                        break; // End of fields
                    }
                }
            }

            // Analyze the type to determine its shape
            let (shape, inner_ty) = analyze_type(&ty_tokens);

            // Parse field attributes
            let attrs = FieldAttrs::parse_from_attrs(&bpaf_attrs, &doc_comments).unwrap_or_default();

            Ok(Field {
                name,
                ty: ty_tokens,
                shape,
                inner_ty,
                attrs,
            })
        });

        match field_result {
            Ok(field) => fields.push(field),
            Err(_) => {
                // No more fields
                break;
            }
        }
    }

    Ok(fields)
}

/// Parse enum variants from a brace group
fn parse_enum_variants(group: &proc_macro2::Group) -> Result<Vec<EnumVariant>> {
    let mut variants = Vec::new();
    let stream = group.stream();
    let mut iter = unsynn::ToTokens::to_token_iter(&stream);

    loop {
        // Try to parse a variant
        let variant_result = iter.transaction(|t| {
            // Skip attributes for now
            loop {
                let has_attr = t.transaction(|t2| {
                    if let Some(TokenTree::Punct(ref p)) = t2.next() {
                        if p.as_char() == '#' {
                            let _ = t2.next(); // consume group
                            return Ok(true);
                        }
                    }
                    Err(Error::no_error())
                });
                if has_attr.is_err() {
                    break;
                }
            }

            // Get variant name
            let name: Ident = t.parse()?;

            // Check for variant fields
            let fields = match t.next() {
                // Struct-style variant: Variant { field: Type, ... }
                Some(TokenTree::Group(ref g)) if g.delimiter() == proc_macro2::Delimiter::Brace => {
                    parse_fields(g)?
                }
                // Tuple-style or unit variant - skip fields for now
                _ => Vec::new(),
            };

            Ok(EnumVariant { name, fields })
        });

        match variant_result {
            Ok(variant) => {
                variants.push(variant);
                // Try to consume comma
                let _ = iter.transaction(|t| {
                    match t.next() {
                        Some(TokenTree::Punct(ref p)) if p.as_char() == ',' => Ok(()),
                        _ => Err(Error::no_error()),
                    }
                });
            }
            Err(_) => {
                // No more variants
                break;
            }
        }
    }

    Ok(variants)
}

/// Represents a single field in a struct
#[derive(Clone)]
pub struct Field {
    /// Field name
    pub name: Ident,
    /// Field type (stored as tokens)
    pub ty: TokenStream,
    /// The shape of the field's type
    pub shape: Shape,
    /// Inner type for Option<T> or Vec<T>
    pub inner_ty: TokenStream,
    /// Parsed attributes from #[bpaf(...)]
    pub attrs: FieldAttrs,
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
}

/// The body of the derived item
#[derive(Clone)]
pub enum Body {
    /// Struct with fields
    Struct(Vec<Field>),
    /// Enum with variants
    Enum(Vec<EnumVariant>),
}

/// Represents an enum variant
#[derive(Clone)]
pub struct EnumVariant {
    /// Variant name
    pub name: Ident,
    /// Fields in the variant (if any)
    pub fields: Vec<Field>,
}

/// Parse struct-level attributes from #[bpaf(...)]
/// Parsed struct-level attributes
#[derive(Default)]
struct StructAttrs {
    adjacent: bool,
    mode: Option<Mode>,
}

fn parse_struct_attrs(group: &proc_macro2::Group) -> Result<Option<StructAttrs>> {
    use unsynn::ToTokens;

    let stream = group.stream();
    let mut iter = ToTokens::to_token_iter(&stream);

    // Check if this is a bpaf attribute
    match iter.next() {
        Some(TokenTree::Ident(ref ident)) if *ident == "bpaf" => {
            // Found #[bpaf(...)]
            // Now parse the parentheses group
            match iter.next() {
                Some(TokenTree::Group(ref g)) if g.delimiter() == proc_macro2::Delimiter::Parenthesis => {
                    // Parse the content
                    let inner_stream = g.stream();
                    let inner_iter = ToTokens::to_token_iter(&inner_stream);
                    let mut attrs = StructAttrs::default();

                    for tt in inner_iter {
                        if let TokenTree::Ident(ref ident) = tt {
                            let ident_str = ident.to_string();
                            match ident_str.as_str() {
                                "adjacent" => {
                                    attrs.adjacent = true;
                                }
                                "options" => {
                                    attrs.mode = Some(Mode::Options);
                                }
                                "command" => {
                                    attrs.mode = Some(Mode::Command);
                                }
                                "parser" => {
                                    attrs.mode = Some(Mode::Parser);
                                }
                                _ => {}
                            }
                        }
                    }
                    return Ok(Some(attrs));
                }
                _ => {}
            }
        }
        _ => {}
    }

    Ok(None)
}

impl Parser for Top {
    fn parser(input: &mut TokenIter) -> Result<Self> {
        // Collect struct-level attributes (look for #[bpaf(...)])
        let mut attrs_groups = Vec::new();
        loop {
            let attr = input.transaction(|t| {
                if let Some(TokenTree::Punct(ref p)) = t.next() {
                    if p.as_char() == '#' {
                        // Get the group after #
                        if let Some(TokenTree::Group(ref g)) = t.next() {
                            return Ok(Some(g.clone()));
                        }
                    }
                }
                Err(Error::no_error())
            });

            match attr {
                Ok(Some(g)) => attrs_groups.push(g),
                _ => break,
            }
        }

        // Parse struct-level attributes
        let mut adjacent = false;
        let mut mode = Mode::default();
        for group in &attrs_groups {
            if let Some(attrs) = parse_struct_attrs(group)? {
                if attrs.adjacent {
                    adjacent = true;
                }
                if let Some(m) = attrs.mode {
                    mode = m;
                }
            }
        }

        // Try to skip visibility modifier if present
        let _ = input.transaction(|t| {
            let ident: Ident = t.parse()?;
            if ident == "pub" {
                Ok(())
            } else {
                Err(Error::no_error())
            }
        });

        // Expect "struct" or "enum" keyword
        let keyword: Ident = input.parse()?;
        let keyword_str = keyword.to_string();
        let is_enum = keyword_str == "enum";
        if keyword_str != "struct" && !is_enum {
            return Err(Error::no_error());
        }

        // Get the struct/enum name
        let name: Ident = input.parse()?;

        // Skip generics if present (we'll handle them later)
        // Look for a group (< for generics or { for fields/variants)
        loop {
            match input.next() {
                Some(TokenTree::Punct(ref p)) if p.as_char() == '<' => {
                    // Skip until we find matching >
                    // Simple depth counting for now
                    let mut depth = 1;
                    while depth > 0 {
                        match input.next() {
                            Some(TokenTree::Punct(ref p)) if p.as_char() == '<' => depth += 1,
                            Some(TokenTree::Punct(ref p)) if p.as_char() == '>' => depth -= 1,
                            None => break,
                            _ => {}
                        }
                    }
                }
                Some(TokenTree::Group(ref g)) if g.delimiter() == proc_macro2::Delimiter::Brace => {
                    // Parse the content
                    let body = if is_enum {
                        // Parse enum variants
                        let variants = parse_enum_variants(g)?;
                        Body::Enum(variants)
                    } else {
                        // Parse struct fields
                        let fields = parse_fields(g)?;
                        Body::Struct(fields)
                    };
                    // Consume remaining tokens (semicolon, etc.)
                    while input.next().is_some() {}
                    return Ok(Top { name, body, adjacent, mode });
                }
                None => {
                    // End of input - return with empty body
                    let body = if is_enum {
                        Body::Enum(Vec::new())
                    } else {
                        Body::Struct(Vec::new())
                    };
                    return Ok(Top { name, body, adjacent, mode });
                }
                _ => {
                    // Continue looking for the brace group
                }
            }
        }
    }
}

impl ToTokens for Top {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;

        // Generate parser function
        let mut parser_body = self.generate_parser_body();

        // Apply mode-specific wrappers
        match self.mode {
            Mode::Options => {
                // Add .to_options()
                parser_body = quote! { #parser_body.to_options() };
            }
            Mode::Command => {
                // Add .to_options().command("name")
                let name_lower = name.to_string().to_lowercase();
                parser_body = quote! { #parser_body.to_options().command(#name_lower) };
            }
            Mode::Parser => {
                // No wrapper needed
            }
        }

        // Generate return type based on mode
        let return_type = match self.mode {
            Mode::Options => quote! { ::bpaf::OptionParser<Self> },
            Mode::Parser | Mode::Command => quote! { impl ::bpaf::Parser<Self> },
        };

        tokens.extend(quote! {
            #[allow(unused_imports)]
            impl #name {
                pub fn parse() -> #return_type {
                    use ::bpaf::Parser as _;
                    #parser_body
                }
            }
        });
    }
}

impl Top {
    /// Generate the parser body based on fields or variants
    fn generate_parser_body(&self) -> TokenStream {
        match &self.body {
            Body::Struct(fields) => self.generate_struct_parser(fields),
            Body::Enum(variants) => self.generate_enum_parser(variants),
        }
    }

    /// Generate parser for a struct
    fn generate_struct_parser(&self, fields: &[Field]) -> TokenStream {
        if fields.is_empty() {
            // Empty struct - return a pure value
            let name = &self.name;
            return quote! {
                ::bpaf::pure(#name {})
            };
        }

        // Generate parsers for each field
        let field_parsers: Vec<TokenStream> = fields
            .iter()
            .map(|field| self.generate_field_parser(field))
            .collect();

        // Generate field names for construction
        let field_names: Vec<&Ident> = fields.iter().map(|f| &f.name).collect();
        let field_names_construct = field_names.clone();

        let name = &self.name;

        // Generate: let field1 = parser1; let field2 = parser2; ... construct!(Name { field1, field2 })
        let construct = quote! {
            {
                #( let #field_names = #field_parsers; )*
                ::bpaf::construct!(#name { #( #field_names_construct ),* })
            }
        };

        // Apply .adjacent() if the struct has the adjacent flag
        if self.adjacent {
            quote! { #construct.adjacent() }
        } else {
            construct
        }
    }

    /// Generate parser for an enum (command-style)
    fn generate_enum_parser(&self, variants: &[EnumVariant]) -> TokenStream {
        let enum_name = &self.name;

        // Generate parser for each variant
        let variant_parsers: Vec<TokenStream> = variants
            .iter()
            .map(|variant| {
                let variant_name = &variant.name;
                let command_name = to_kebab_case(&variant_name.to_string());

                if variant.fields.is_empty() {
                    // Unit variant - simple command
                    quote! {
                        ::bpaf::command(
                            #command_name,
                            ::bpaf::pure(#enum_name::#variant_name).to_options()
                        )
                    }
                } else {
                    // Variant with fields - construct from fields
                    let field_parsers: Vec<TokenStream> = variant.fields
                        .iter()
                        .map(|field| self.generate_field_parser(field))
                        .collect();

                    let field_names: Vec<&Ident> = variant.fields.iter().map(|f| &f.name).collect();
                    let field_names_construct = field_names.clone();

                    quote! {
                        ::bpaf::command(
                            #command_name,
                            {
                                #( let #field_names = #field_parsers; )*
                                ::bpaf::construct!(#enum_name::#variant_name { #( #field_names_construct ),* })
                            }.to_options()
                        )
                    }
                }
            })
            .collect();

        // Combine variants - generate named parsers and combine with construct!
        if variant_parsers.is_empty() {
            quote! { ::bpaf::fail("No variants available") }
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
                    ::bpaf::construct!( [ #( #var_names_construct ),* ] )
                }
            }
        }
    }

    /// Generate parser for a single field based on its shape
    fn generate_field_parser(&self, field: &Field) -> TokenStream {
        let field_name = &field.name;
        let field_name_str = field_name.to_string();

        // Determine the names to use based on attributes
        let (short_spec, long_spec) = self.get_name_specs(field);
        let env_spec = &field.attrs.env;

        // Check for explicit consumer type or use shape-based default
        let consumer = field.attrs.consumer.as_ref();

        // Build the base parser with names (for named parsers)
        // If only env is specified (no short/long), we'll use None and handle it later
        let mut base_parser = if let Some(short) = short_spec {
            if let Some(long) = long_spec {
                // Both short and long
                Some(quote! { ::bpaf::short(#short).long(#long) })
            } else {
                // Just short
                Some(quote! { ::bpaf::short(#short) })
            }
        } else if let Some(long) = long_spec {
            // Just long
            Some(quote! { ::bpaf::long(#long) })
        } else if env_spec.is_none() {
            // Neither short/long/env specified - default to long name derived from field
            let default_long = to_kebab_case(&field_name_str);
            Some(quote! { ::bpaf::long(#default_long) })
        } else {
            // Only env specified - will use argument() directly
            None
        };

        // Handle env-only fields (no short/long)
        if base_parser.is_none() && env_spec.is_some() {
            // Use ::bpaf::env() directly for env-only fields
            let env_expr = env_spec.as_ref().unwrap();
            base_parser = Some(quote! { ::bpaf::env(#env_expr) });
        } else if let Some(ref env_expr) = env_spec {
            // Chain .env() onto existing short/long parser
            if let Some(bp) = base_parser {
                base_parser = Some(quote! { #bp.env(#env_expr) });
            }
        }

        // Unwrap base_parser (should always be Some at this point)
        let base_parser = base_parser.unwrap();

        // Generate base parser based on consumer type or shape
        let mut parser = match consumer {
            Some(ConsumerType::Switch) => {
                quote! { #base_parser.switch() }
            }
            Some(ConsumerType::Argument { metavar }) => {
                // Use inner type for Vec/Option shapes, otherwise use field type
                let ty = match field.shape {
                    Shape::Vec | Shape::Option => &field.inner_ty,
                    _ => &field.ty,
                };
                let metavar_str = metavar.as_deref().unwrap_or(&field_name_str);
                quote! { #base_parser.argument::<#ty>(#metavar_str) }
            }
            Some(ConsumerType::Positional { metavar }) => {
                // Use inner type for Vec/Option shapes, otherwise use field type
                let ty = match field.shape {
                    Shape::Vec | Shape::Option => &field.inner_ty,
                    _ => &field.ty,
                };
                let metavar_str = metavar.as_deref().unwrap_or("ARG");
                quote! { ::bpaf::positional::<#ty>(#metavar_str) }
            }
            Some(ConsumerType::Flag { present, absent }) => {
                quote! { #base_parser.flag(#present, #absent) }
            }
            Some(ConsumerType::ReqFlag { present }) => {
                quote! { #base_parser.req_flag(#present) }
            }
            Some(ConsumerType::Any { metavar, ty, check }) => {
                // If we have short/long/env, chain with base_parser
                // Otherwise use standalone any()
                if field.attrs.short.is_some() || field.attrs.long.is_some() || field.attrs.env.is_some() {
                    // Use base_parser.argument().parse() for named parsers
                    // Wrap the check function to convert Option to Result
                    quote! {
                        #base_parser.argument::<String>(#metavar).parse(|s: String| {
                            (#check)(s).ok_or("validation failed")
                        })
                    }
                } else {
                    // Use standalone any() for positional
                    if let Some(ty) = ty {
                        // With explicit type: any::<Type, _, _>(metavar, check)
                        quote! { ::bpaf::any::<#ty, _, _>(#metavar, #check) }
                    } else {
                        // Without type: any(metavar, check)
                        quote! { ::bpaf::any(#metavar, #check) }
                    }
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
                quote! { ::bpaf::pure(#expr) }
            }
            Some(ConsumerType::PureWith { expr }) => {
                quote! { ::bpaf::pure_with(#expr) }
            }
            None => {
                // Infer from shape
                match field.shape {
                    Shape::Bool => {
                        quote! { #base_parser.switch() }
                    }
                    Shape::Direct => {
                        let ty = &field.ty;
                        quote! { #base_parser.argument::<#ty>(#field_name_str) }
                    }
                    Shape::Option => {
                        let inner_ty = &field.inner_ty;
                        quote! { #base_parser.argument::<#inner_ty>(#field_name_str) }
                    }
                    Shape::Vec => {
                        let inner_ty = &field.inner_ty;
                        quote! { #base_parser.argument::<#inner_ty>(#field_name_str) }
                    }
                    Shape::Unit => {
                        quote! { ::bpaf::pure(()) }
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
        let has_explicit_postparse = field.attrs.postpr.iter().any(|p| matches!(p, Post::Parse(_)));

        // Build list of PostParse attributes to apply
        let mut postpr_to_apply = field.attrs.postpr.clone();

        if !has_explicit_postparse {
            // Add implicit PostParse attributes based on type shape
            let is_positional = matches!(consumer, Some(ConsumerType::Positional { .. }));

            if field.shape == Shape::Option && !is_positional {
                // Insert implicit .optional() at position 0
                postpr_to_apply.insert(0, Post::Parse(PostParse::Optional));
            } else if field.shape == Shape::Vec {
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

    /// Get the short and long name specifications from attributes
    fn get_name_specs(&self, field: &Field) -> (Option<char>, Option<String>) {
        let field_name_str = field.name.to_string();

        let short = if let Some(ch) = field.attrs.short {
            // If short char is 'x' (placeholder), derive from field name
            if ch == 'x' {
                Some(field_name_str.chars().next().unwrap_or('x'))
            } else {
                Some(ch)
            }
        } else {
            None
        };

        let long = if let Some(ref name) = field.attrs.long {
            // If name is empty (placeholder), derive from field name
            if name.is_empty() {
                Some(to_kebab_case(&field_name_str))
            } else {
                Some(name.clone())
            }
        } else {
            None
        };

        (short, long)
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

/// Convert identifier to kebab-case
/// Handles both snake_case and camelCase
fn to_kebab_case(s: &str) -> String {
    let mut result = String::new();
    for (i, ch) in s.chars().enumerate() {
        if ch == '_' {
            // Convert underscore to hyphen
            result.push('-');
        } else if ch.is_uppercase() {
            // Insert hyphen before uppercase letters (except at start)
            if i > 0 {
                result.push('-');
            }
            result.push(ch.to_ascii_lowercase());
        } else {
            result.push(ch);
        }
    }
    result
}
