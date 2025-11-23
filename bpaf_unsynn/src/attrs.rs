//! Attribute parsing for #[bpaf(...)] annotations

use crate::parsing::*;
use crate::utils::to_kebab_case;

/// Attributes that can be applied to a field
#[derive(Debug, Clone, Default)]
pub struct FieldAttrs {
    /// Short flag name (already resolved from field name if needed)
    pub short: Option<char>,
    /// Long flag name (already resolved from field name if needed)
    pub long: Option<String>,
    /// Environment variable name expression
    pub env: Option<TokenStream>,
    /// Help text for the field
    pub help: Option<String>,
    /// Fallback value expression (as tokens)
    pub fallback: Option<TokenStream>,
    /// Explicit consumer type
    pub consumer: Option<ConsumerType>,
    /// Post-processing attributes (map, parse, guard, etc.)
    pub postpr: Vec<Post>,
    /// Ignore rustdoc comments for help text
    pub ignore_rustdoc: bool,
}

/// The type of consumer to use for parsing
#[derive(Debug, Clone)]
pub enum ConsumerType {
    /// Use as a switch (bool, no value)
    Switch,
    /// Use as a flag with values (bool with present/absent values)
    Flag {
        present: TokenStream,
        absent: TokenStream,
    },
    /// Parse as an argument with a value
    Argument { metavar: Option<String> },
    /// Parse as a positional argument
    Positional { metavar: Option<String> },
    /// Required flag with value
    ReqFlag { present: TokenStream },
    /// Custom validation with metavar
    Any {
        metavar: String,
        ty: Option<TokenStream>,
        check: TokenStream,
    },
    /// Reference external parser
    External { ident: Option<TokenStream> },
    /// Pure value (no parsing)
    Pure { expr: TokenStream },
    /// Pure value with function
    PureWith { expr: TokenStream },
}

/// Post-processing attribute (can change type or just behavior)
#[derive(Debug, Clone)]
pub enum Post {
    /// Type-changing post-processing (map, parse, optional, etc.)
    Parse(PostParse),
    /// Behavior-changing decorators (guard, hide, fallback_with, etc.)
    Decor(PostDecor),
}

/// Type-changing post-processing attributes
#[derive(Debug, Clone)]
pub enum PostParse {
    /// map - transform the parsed value
    Map { f: TokenStream },
    /// parse - custom parsing function
    Parse { f: TokenStream },
    /// optional - make the parser optional
    Optional,
    /// many - parse multiple occurrences (explicit)
    Many,
    /// some - require at least one with error message
    Some { msg: TokenStream },
    /// catch - catch parse errors
    Catch,
    /// collect - collect into collection
    Collect,
    /// count - count occurrences
    Count,
    /// anywhere - parse anywhere in arguments
    Anywhere,
    /// adjacent - parse adjacent values
    Adjacent,
    /// strict - strict parsing
    Strict,
    /// non_strict - non-strict parsing
    NonStrict,
}

/// Behavior-changing decorator attributes
#[derive(Debug, Clone)]
pub enum PostDecor {
    /// guard - validation with error message
    Guard {
        check: TokenStream,
        msg: TokenStream,
    },
    /// hide - hide from help
    Hide,
    /// hide_usage - hide from usage line
    HideUsage,
    /// custom_usage - custom usage text
    CustomUsage { usage: TokenStream },
    /// fallback_with - dynamic fallback
    FallbackWith { f: TokenStream },
    /// group_help - help for option group
    GroupHelp { doc: TokenStream },
    /// debug_fallback - debug format fallback
    DebugFallback,
    /// display_fallback - display format fallback
    DisplayFallback,
    /// format_fallback - custom format fallback
    FormatFallback { formatter: TokenStream },
    /// last - use last occurrence
    Last,
    /// complete - shell completion function
    Complete { f: TokenStream },
    /// group - completion group
    CompleteGroup { group: String },
    /// complete_shell - shell-specific completion
    CompleteShell { f: TokenStream },
}

/// Helper function to parse and extract two comma-separated arguments
/// The TwoArgs grammar requires exactly 2 non-empty arguments with a comma separator
/// Returns an error if parsing fails (missing comma, empty arguments, etc.)
fn parse_two_args(
    group: &proc_macro2::Group,
    attr_name: &str,
    arg_description: &str,
) -> unsynn::Result<(TokenStream, TokenStream)> {
    use unsynn::IParse;

    let stream = group.stream();
    let mut iter = unsynn::ToTokens::to_token_iter(&stream);

    match iter.parse::<crate::parsing::TwoArgs>() {
        Ok(two_args) => Ok(two_args.into_streams()),
        Err(_) => unsynn::Error::other(
            None,
            &iter,
            format!(
                "{}() requires exactly 2 comma-separated arguments ({})",
                attr_name, arg_description
            ),
        ),
    }
}

/// Helper function to parse any() arguments: string literal and check function
/// Returns an error if metavar is not a string literal or arguments are malformed
fn parse_any_args(group: &proc_macro2::Group) -> unsynn::Result<(String, TokenStream)> {
    use unsynn::IParse;

    let stream = group.stream();
    let mut iter = unsynn::ToTokens::to_token_iter(&stream);

    match iter.parse::<crate::parsing::AnyArgs>() {
        Ok(any_args) => {
            let metavar = any_args.metavar_str().to_string();
            let check = any_args.check_fn_tokens();
            Ok((metavar, check))
        }
        Err(_) => unsynn::Error::other(
            None,
            &iter,
            "any() requires a string literal for metavar and a check function: any(\"METAVAR\", check_fn)".to_string(),
        ),
    }
}

impl FieldAttrs {
    /// Parse attributes from a field's attribute list
    /// Uses unsynn grammar to parse BpafAttr and DocInner structures directly
    /// The field_name is used to derive short/long names when not explicitly specified
    pub fn parse_from_attrs(
        field_name: &str,
        bpaf_attrs: &[BpafAttr],
        doc_attrs: &[DocInner],
    ) -> Result<Self> {
        use crate::parsing::*;

        let mut field_attrs = FieldAttrs::default();

        // Extract doc comment strings (already parsed)
        // Remove leading space from each line (rustdoc convention)
        let doc_strings: Vec<String> = doc_attrs
            .iter()
            .map(|doc| {
                let s = doc.value.as_str();
                s.strip_prefix(' ').unwrap_or(s).to_string()
            })
            .collect();

        // Process bpaf attributes (already parsed)
        for bpaf_attr in bpaf_attrs {
            // Iterate through the inner attributes
            for delimited in bpaf_attr.inner.content.iter() {
                let inner = &delimited.value;

                match inner {
                    // Name attributes - resolve immediately using field_name
                    BpafInner::Short(si) => {
                        field_attrs.short = Some(
                            si.ch
                                .as_ref()
                                .map(|g| g.content.value())
                                .unwrap_or_else(|| field_name.chars().next().unwrap_or('_')),
                        );
                    }
                    BpafInner::Long(li) => {
                        field_attrs.long = Some(
                            li.name
                                .as_ref()
                                .map(|g| g.content.as_str().to_string())
                                .unwrap_or_else(|| to_kebab_case(field_name)),
                        );
                    }
                    BpafInner::Env(ei) => {
                        field_attrs.env = Some(ei.expr.0.stream());
                    }

                    // Consumer attributes
                    BpafInner::Switch(_) => {
                        field_attrs.consumer = Some(ConsumerType::Switch);
                    }
                    BpafInner::Flag(fi) => {
                        let (present, absent) =
                            parse_two_args(&fi.values.0, "flag", "present value, absent value")?;
                        field_attrs.consumer = Some(ConsumerType::Flag { present, absent });
                    }
                    BpafInner::Argument(ai) => {
                        let metavar = ai.metavar.as_ref().map(|g| g.content.as_str().to_string());
                        field_attrs.consumer = Some(ConsumerType::Argument { metavar });
                    }
                    BpafInner::Positional(pi) => {
                        let metavar = pi.metavar.as_ref().map(|g| g.content.as_str().to_string());
                        field_attrs.consumer = Some(ConsumerType::Positional { metavar });
                    }
                    BpafInner::ReqFlag(ri) => {
                        field_attrs.consumer = Some(ConsumerType::ReqFlag {
                            present: ri.value.0.stream(),
                        });
                    }
                    BpafInner::Any(ai) => {
                        let ty = ai.turbofish.as_ref().map(|t| {
                            let mut ts = proc_macro2::TokenStream::new();
                            unsynn::ToTokens::to_tokens(&t.ty, &mut ts);
                            ts
                        });

                        // Parse and validate any() arguments
                        let (metavar, check) = parse_any_args(&ai.args.0)?;

                        field_attrs.consumer = Some(ConsumerType::Any { metavar, ty, check });
                    }
                    BpafInner::External(ei) => {
                        field_attrs.consumer = Some(ConsumerType::External {
                            ident: ei.parser.as_ref().map(|p| p.0.stream()),
                        });
                    }
                    BpafInner::Pure(pi) => {
                        field_attrs.consumer = Some(ConsumerType::Pure {
                            expr: pi.value.0.stream(),
                        });
                    }
                    BpafInner::PureWith(pwi) => {
                        field_attrs.consumer = Some(ConsumerType::PureWith {
                            expr: pwi.func.0.stream(),
                        });
                    }

                    // PostParse attributes
                    BpafInner::Map(mi) => {
                        field_attrs.postpr.push(Post::Parse(PostParse::Map {
                            f: mi.func.0.stream(),
                        }));
                    }
                    BpafInner::Parse(pi) => {
                        field_attrs.postpr.push(Post::Parse(PostParse::Parse {
                            f: pi.func.0.stream(),
                        }));
                    }
                    BpafInner::Optional(_) => {
                        field_attrs.postpr.push(Post::Parse(PostParse::Optional));
                    }
                    BpafInner::Many(_) => {
                        field_attrs.postpr.push(Post::Parse(PostParse::Many));
                    }
                    BpafInner::Some(si) => {
                        field_attrs.postpr.push(Post::Parse(PostParse::Some {
                            msg: si.msg.0.stream(),
                        }));
                    }
                    BpafInner::Catch(_) => {
                        field_attrs.postpr.push(Post::Parse(PostParse::Catch));
                    }
                    BpafInner::Collect(_) => {
                        field_attrs.postpr.push(Post::Parse(PostParse::Collect));
                    }
                    BpafInner::Count(_) => {
                        field_attrs.postpr.push(Post::Parse(PostParse::Count));
                    }
                    BpafInner::Anywhere(_) => {
                        field_attrs.postpr.push(Post::Parse(PostParse::Anywhere));
                    }
                    BpafInner::Adjacent(_) => {
                        field_attrs.postpr.push(Post::Parse(PostParse::Adjacent));
                    }
                    BpafInner::Strict(_) => {
                        field_attrs.postpr.push(Post::Parse(PostParse::Strict));
                    }
                    BpafInner::NonStrict(_) => {
                        field_attrs.postpr.push(Post::Parse(PostParse::NonStrict));
                    }

                    // PostDecor attributes
                    BpafInner::Guard(gi) => {
                        let (check, msg) =
                            parse_two_args(&gi.args.0, "guard", "check_fn, error_message")?;
                        field_attrs
                            .postpr
                            .push(Post::Decor(PostDecor::Guard { check, msg }));
                    }
                    BpafInner::Hide(_) => {
                        field_attrs.postpr.push(Post::Decor(PostDecor::Hide));
                    }
                    BpafInner::HideUsage(_) => {
                        field_attrs.postpr.push(Post::Decor(PostDecor::HideUsage));
                    }
                    BpafInner::CustomUsage(cui) => {
                        field_attrs.postpr.push(Post::Decor(PostDecor::CustomUsage {
                            usage: cui.text.0.stream(),
                        }));
                    }
                    BpafInner::FallbackWith(fwi) => {
                        field_attrs
                            .postpr
                            .push(Post::Decor(PostDecor::FallbackWith {
                                f: fwi.func.0.stream(),
                            }));
                    }
                    BpafInner::GroupHelp(ghi) => {
                        field_attrs.postpr.push(Post::Decor(PostDecor::GroupHelp {
                            doc: ghi.text.0.stream(),
                        }));
                    }
                    BpafInner::DebugFallback(_) => {
                        field_attrs
                            .postpr
                            .push(Post::Decor(PostDecor::DebugFallback));
                    }
                    BpafInner::DisplayFallback(_) => {
                        field_attrs
                            .postpr
                            .push(Post::Decor(PostDecor::DisplayFallback));
                    }
                    BpafInner::FormatFallback(ffi) => {
                        field_attrs
                            .postpr
                            .push(Post::Decor(PostDecor::FormatFallback {
                                formatter: ffi.formatter.0.stream(),
                            }));
                    }
                    BpafInner::Last(_) => {
                        field_attrs.postpr.push(Post::Decor(PostDecor::Last));
                    }
                    BpafInner::Complete(ci) => {
                        field_attrs.postpr.push(Post::Decor(PostDecor::Complete {
                            f: ci.func.0.stream(),
                        }));
                    }
                    BpafInner::Group(gi) => {
                        let group = gi.name.content.as_str().to_string();
                        field_attrs
                            .postpr
                            .push(Post::Decor(PostDecor::CompleteGroup { group }));
                    }
                    BpafInner::CompleteShell(csi) => {
                        field_attrs
                            .postpr
                            .push(Post::Decor(PostDecor::CompleteShell {
                                f: csi.expr.0.stream(),
                            }));
                    }

                    // Other attributes
                    BpafInner::Help(hi) => {
                        field_attrs.help = Some(hi.text.content.as_str().to_string());
                    }
                    BpafInner::Fallback(fi) => {
                        field_attrs.fallback = Some(fi.expr.0.stream());
                    }
                    BpafInner::IgnoreRustdoc(_) => {
                        field_attrs.ignore_rustdoc = true;
                    }

                    // Mode attributes (shouldn't appear on fields, but handle gracefully)
                    BpafInner::Options(_)
                    | BpafInner::Parser(_)
                    | BpafInner::Command(_)
                    | BpafInner::Skip(_)
                    | BpafInner::FallbackToUsage(_)
                    | BpafInner::Path(_)
                    | BpafInner::Generate(_)
                    | BpafInner::Private(_)
                    | BpafInner::Boxed(_)
                    | BpafInner::Descr(_)
                    | BpafInner::Footer(_)
                    | BpafInner::Header(_)
                    | BpafInner::Usage(_)
                    | BpafInner::Version(_)
                    | BpafInner::MaxWidth(_)
                    | BpafInner::CargoHelper(_) => {
                        // These are enum/struct-level attributes, not field-level
                        // Ignore them here
                    }

                    // Unknown attributes
                    BpafInner::Unknown(u) => {
                        let mut iter = u.to_token_iter();
                        return unsynn::Error::unexpected_token(iter.next(), &iter);
                    }
                }
            }
        }
        // Use doc comments as help text if:
        // 1. ignore_rustdoc is false (default)
        // 2. No explicit help attribute was provided
        // 3. We have doc strings
        if !field_attrs.ignore_rustdoc && field_attrs.help.is_none() && !doc_strings.is_empty() {
            field_attrs.help = Some(doc_strings.join("\n"));
        }

        Ok(field_attrs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsing::BpafAttr;

    fn parse_bpaf_attr(content: &str) -> BpafAttr {
        let tokens: TokenStream = format!("bpaf({})", content).parse().unwrap();
        let mut iter = unsynn::ToTokens::to_token_iter(&tokens);
        iter.parse::<BpafAttr>().unwrap()
    }

    #[test]
    fn test_parse_short_attr() {
        // Test parsing: #[bpaf(short)] - derive from field name
        let bpaf_attr = parse_bpaf_attr("short");
        let attrs = FieldAttrs::parse_from_attrs("verbose", &[bpaf_attr], &[]).unwrap();
        assert_eq!(attrs.short, Some('v')); // Derived from field name
    }

    #[test]
    fn test_parse_long_attr() {
        // Test parsing: #[bpaf(long)] - derive from field name
        let bpaf_attr = parse_bpaf_attr("long");
        let attrs = FieldAttrs::parse_from_attrs("my_field", &[bpaf_attr], &[]).unwrap();
        assert_eq!(attrs.long, Some("my-field".to_string())); // Derived from field name
    }

    #[test]
    fn test_parse_short_with_value() {
        // Test parsing: #[bpaf(short('v'))] - explicit char
        let bpaf_attr = parse_bpaf_attr("short('v')");
        let attrs = FieldAttrs::parse_from_attrs("other", &[bpaf_attr], &[]).unwrap();
        assert_eq!(attrs.short, Some('v'));
    }

    #[test]
    fn test_parse_long_with_value() {
        // Test parsing: #[bpaf(long("verbose"))] - explicit name
        let bpaf_attr = parse_bpaf_attr(r#"long("verbose")"#);
        let attrs = FieldAttrs::parse_from_attrs("other", &[bpaf_attr], &[]).unwrap();
        assert_eq!(attrs.long, Some("verbose".to_string()));
    }

    #[test]
    fn test_parse_multiple_attrs() {
        // Test parsing: #[bpaf(short('v'), long("verbose"))]
        let bpaf_attr = parse_bpaf_attr(r#"short('v'), long("verbose")"#);
        let attrs = FieldAttrs::parse_from_attrs("other", &[bpaf_attr], &[]).unwrap();
        assert_eq!(attrs.short, Some('v'));
        assert_eq!(attrs.long, Some("verbose".to_string()));
    }
}
