//! Attribute parsing for #[bpaf(...)] annotations

use crate::parsing::*;

/// Attributes that can be applied to a field
#[derive(Debug, Clone, Default)]
pub struct FieldAttrs {
    /// Short flag name like 'v' for -v
    pub short: Option<char>,
    /// Long flag name like "verbose" for --verbose
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

/// Mode for the generated parser function
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Default)]
pub enum Mode {
    /// Generate `impl Parser<T>` - basic parser (default)
    #[default]
    Parser,
    /// Generate `OptionParser<T>` - includes help/version handling
    Options,
    /// Generate `impl Parser<T>` with command wrapper
    Command,
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

impl FieldAttrs {
    /// Parse attributes from a field's attribute list
    pub fn parse_from_attrs(
        attrs_tokens: &[proc_macro2::Group],
        doc_attrs: &[proc_macro2::Group],
    ) -> Result<Self> {
        let mut field_attrs = FieldAttrs::default();

        // Extract doc comments if we don't have ignore_rustdoc set yet
        // We'll check ignore_rustdoc later after parsing all attributes
        let mut doc_strings = Vec::new();
        for group in doc_attrs {
            // Parse: doc = "text"
            let stream = group.stream();
            let mut iter = unsynn::ToTokens::to_token_iter(&stream);

            // Try to parse "doc = "text""
            let doc_text = iter.transaction(|t| {
                // Expect "doc"
                let ident: Ident = t.parse()?;
                if ident != "doc" {
                    return Err(Error::no_error());
                }

                // Expect "="
                match t.next() {
                    Some(TokenTree::Punct(ref p)) if p.as_char() == '=' => {}
                    _ => return Err(Error::no_error()),
                }

                // Expect string literal
                match t.next() {
                    Some(TokenTree::Literal(ref lit)) => {
                        let lit_str = lit.to_string();
                        // Remove surrounding quotes
                        if lit_str.starts_with('"') && lit_str.ends_with('"') {
                            let mut content = lit_str[1..lit_str.len() - 1].to_string();
                            // Remove leading space if present (rustdoc convention)
                            if content.starts_with(' ') {
                                content = content[1..].to_string();
                            }
                            Ok(content)
                        } else {
                            Err(Error::no_error())
                        }
                    }
                    _ => Err(Error::no_error()),
                }
            });

            if let Ok(text) = doc_text {
                doc_strings.push(text);
            }
        }

        for group in attrs_tokens {
            // Each group is the content of #[bpaf(...)]
            let stream = group.stream();
            let mut iter = unsynn::ToTokens::to_token_iter(&stream);

            // Parse comma-separated attributes within the group
            loop {
                let attr_result = iter.transaction(|t| {
                    // Try to parse an attribute
                    let ident: Ident = t.parse()?;
                    let ident_str = ident.to_string();

                    match ident_str.as_str() {
                        "short" => {
                            // Parse: short or short('c')
                            let ch = parse_optional_char_arg(t)?;
                            field_attrs.short = Some(ch.unwrap_or({
                                // Default to first character if not specified
                                'x' // Placeholder, will be derived from field name
                            }));
                            Ok(())
                        }
                        "long" => {
                            // Parse: long or long("name")
                            let name = parse_optional_string_arg(t)?;
                            field_attrs.long = Some(name.unwrap_or_else(|| {
                                // Placeholder, will be derived from field name
                                String::new()
                            }));
                            Ok(())
                        }
                        "env" => {
                            // Parse: env("VAR_NAME") or env(expression)
                            let env_expr = parse_optional_expr_arg(t)?;
                            if let Some(expr) = env_expr {
                                field_attrs.env = Some(expr);
                            }
                            Ok(())
                        }
                        "help" => {
                            // Parse: help("description")
                            let help_text = parse_optional_string_arg(t)?;
                            if let Some(text) = help_text {
                                field_attrs.help = Some(text);
                            }
                            Ok(())
                        }
                        "fallback" => {
                            // Parse: fallback(expr)
                            let fallback_expr = parse_optional_expr_arg(t)?;
                            if let Some(expr) = fallback_expr {
                                field_attrs.fallback = Some(expr);
                            }
                            Ok(())
                        }
                        "switch" => {
                            field_attrs.consumer = Some(ConsumerType::Switch);
                            Ok(())
                        }
                        "flag" => {
                            // Parse: flag(present, absent)
                            let args = parse_two_expr_args(t)?;
                            if let Some((present, absent)) = args {
                                field_attrs.consumer = Some(ConsumerType::Flag { present, absent });
                            } else {
                                // flag requires two arguments
                                return Err(Error::no_error());
                            }
                            Ok(())
                        }
                        "argument" => {
                            // Parse: argument or argument("METAVAR")
                            let metavar = parse_optional_string_arg(t)?;
                            field_attrs.consumer = Some(ConsumerType::Argument { metavar });
                            Ok(())
                        }
                        "positional" => {
                            // Parse: positional or positional("METAVAR")
                            let metavar = parse_optional_string_arg(t)?;
                            field_attrs.consumer = Some(ConsumerType::Positional { metavar });
                            Ok(())
                        }
                        // Phase 5: Additional consumer types
                        "req_flag" => {
                            // Parse: req_flag(present_value)
                            let present = parse_optional_expr_arg(t)?;
                            if let Some(present) = present {
                                field_attrs.consumer = Some(ConsumerType::ReqFlag { present });
                            }
                            Ok(())
                        }
                        "any" => {
                            // Parse: any("METAVAR", check_function) or any::<Type>("METAVAR", check_function)
                            let ty = parse_turbofish(t)?;
                            let args = parse_two_expr_args(t)?;
                            if let Some((metavar_expr, check)) = args {
                                // Extract string from metavar_expr
                                let metavar_str = metavar_expr.to_string();
                                // Remove quotes if it's a string literal
                                let metavar =
                                    if metavar_str.starts_with('"') && metavar_str.ends_with('"') {
                                        metavar_str[1..metavar_str.len() - 1].to_string()
                                    } else {
                                        metavar_str
                                    };
                                field_attrs.consumer =
                                    Some(ConsumerType::Any { metavar, ty, check });
                            }
                            Ok(())
                        }
                        "external" => {
                            // Parse: external or external(parser_fn)
                            let ident = parse_optional_expr_arg(t)?;
                            field_attrs.consumer = Some(ConsumerType::External { ident });
                            Ok(())
                        }
                        "pure" => {
                            // Parse: pure(value)
                            let expr = parse_optional_expr_arg(t)?;
                            if let Some(expr) = expr {
                                field_attrs.consumer = Some(ConsumerType::Pure { expr });
                            }
                            Ok(())
                        }
                        "pure_with" => {
                            // Parse: pure_with(function)
                            let expr = parse_optional_expr_arg(t)?;
                            if let Some(expr) = expr {
                                field_attrs.consumer = Some(ConsumerType::PureWith { expr });
                            }
                            Ok(())
                        }
                        // PostDecor attributes
                        "guard" => {
                            // Parse: guard(check_fn, "error message")
                            let args = parse_two_expr_args(t)?;
                            if let Some((check, msg)) = args {
                                field_attrs
                                    .postpr
                                    .push(Post::Decor(PostDecor::Guard { check, msg }));
                            }
                            Ok(())
                        }
                        "hide" => {
                            field_attrs.postpr.push(Post::Decor(PostDecor::Hide));
                            Ok(())
                        }
                        "hide_usage" => {
                            field_attrs.postpr.push(Post::Decor(PostDecor::HideUsage));
                            Ok(())
                        }
                        "custom_usage" => {
                            // Parse: custom_usage("usage text")
                            let usage = parse_optional_expr_arg(t)?;
                            if let Some(usage) = usage {
                                field_attrs
                                    .postpr
                                    .push(Post::Decor(PostDecor::CustomUsage { usage }));
                            }
                            Ok(())
                        }
                        // PostParse attributes
                        "map" => {
                            // Parse: map(function)
                            let f = parse_optional_expr_arg(t)?;
                            if let Some(f) = f {
                                field_attrs.postpr.push(Post::Parse(PostParse::Map { f }));
                            }
                            Ok(())
                        }
                        "parse" => {
                            // Parse: parse(function)
                            let f = parse_optional_expr_arg(t)?;
                            if let Some(f) = f {
                                field_attrs.postpr.push(Post::Parse(PostParse::Parse { f }));
                            }
                            Ok(())
                        }
                        "optional" => {
                            field_attrs.postpr.push(Post::Parse(PostParse::Optional));
                            Ok(())
                        }
                        "catch" => {
                            field_attrs.postpr.push(Post::Parse(PostParse::Catch));
                            Ok(())
                        }
                        "collect" => {
                            field_attrs.postpr.push(Post::Parse(PostParse::Collect));
                            Ok(())
                        }
                        "count" => {
                            field_attrs.postpr.push(Post::Parse(PostParse::Count));
                            Ok(())
                        }
                        "many" => {
                            field_attrs.postpr.push(Post::Parse(PostParse::Many));
                            Ok(())
                        }
                        "some" => {
                            // Parse: some("error message")
                            let msg = parse_optional_expr_arg(t)?;
                            if let Some(msg) = msg {
                                field_attrs
                                    .postpr
                                    .push(Post::Parse(PostParse::Some { msg }));
                            }
                            Ok(())
                        }
                        "anywhere" => {
                            field_attrs.postpr.push(Post::Parse(PostParse::Anywhere));
                            Ok(())
                        }
                        "adjacent" => {
                            field_attrs.postpr.push(Post::Parse(PostParse::Adjacent));
                            Ok(())
                        }
                        "strict" => {
                            field_attrs.postpr.push(Post::Parse(PostParse::Strict));
                            Ok(())
                        }
                        "non_strict" => {
                            field_attrs.postpr.push(Post::Parse(PostParse::NonStrict));
                            Ok(())
                        }
                        // Phase 4: Advanced PostDecor attributes
                        "fallback_with" => {
                            // Parse: fallback_with(function)
                            let f = parse_optional_expr_arg(t)?;
                            if let Some(f) = f {
                                field_attrs
                                    .postpr
                                    .push(Post::Decor(PostDecor::FallbackWith { f }));
                            }
                            Ok(())
                        }
                        "group_help" => {
                            // Parse: group_help("doc text")
                            let doc = parse_optional_expr_arg(t)?;
                            if let Some(doc) = doc {
                                field_attrs
                                    .postpr
                                    .push(Post::Decor(PostDecor::GroupHelp { doc }));
                            }
                            Ok(())
                        }
                        "debug_fallback" => {
                            field_attrs
                                .postpr
                                .push(Post::Decor(PostDecor::DebugFallback));
                            Ok(())
                        }
                        "display_fallback" => {
                            field_attrs
                                .postpr
                                .push(Post::Decor(PostDecor::DisplayFallback));
                            Ok(())
                        }
                        "format_fallback" => {
                            // Parse: format_fallback(formatter)
                            let formatter = parse_optional_expr_arg(t)?;
                            if let Some(formatter) = formatter {
                                field_attrs
                                    .postpr
                                    .push(Post::Decor(PostDecor::FormatFallback { formatter }));
                            }
                            Ok(())
                        }
                        "last" => {
                            field_attrs.postpr.push(Post::Decor(PostDecor::Last));
                            Ok(())
                        }
                        "group" => {
                            // Parse: group("completion_group")
                            let group = parse_optional_string_arg(t)?;
                            if let Some(group) = group {
                                field_attrs
                                    .postpr
                                    .push(Post::Decor(PostDecor::CompleteGroup { group }));
                            }
                            Ok(())
                        }
                        "complete" => {
                            // Parse: complete(completion_fn)
                            let f = parse_optional_expr_arg(t)?;
                            if let Some(f) = f {
                                field_attrs
                                    .postpr
                                    .push(Post::Decor(PostDecor::Complete { f }));
                            }
                            Ok(())
                        }
                        "complete_shell" => {
                            // Parse: complete_shell(shell_comp_expr)
                            let f = parse_optional_expr_arg(t)?;
                            if let Some(f) = f {
                                field_attrs
                                    .postpr
                                    .push(Post::Decor(PostDecor::CompleteShell { f }));
                            }
                            Ok(())
                        }
                        "ignore_rustdoc" => {
                            // Parse: ignore_rustdoc
                            // Note: Currently a no-op since we don't extract doc comments,
                            // but supported for compatibility with bpaf_derive
                            field_attrs.ignore_rustdoc = true;
                            Ok(())
                        }
                        _ => {
                            // Unknown attribute - skip for now
                            Ok(())
                        }
                    }
                });

                match attr_result {
                    Ok(()) => {
                        // Try to consume optional comma
                        let _ = iter.transaction(|t| match t.next() {
                            Some(TokenTree::Punct(ref p)) if p.as_char() == ',' => Ok(()),
                            _ => Err(Error::no_error()),
                        });
                    }
                    Err(_) => {
                        // No more attributes to parse
                        break;
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

/// Parse optional char argument like ('c')
fn parse_optional_char_arg(iter: &mut TokenIter) -> Result<Option<char>> {
    iter.transaction(|t| {
        // Look for opening paren
        match t.next() {
            Some(TokenTree::Group(ref g))
                if g.delimiter() == proc_macro2::Delimiter::Parenthesis =>
            {
                // Parse the char literal inside
                let stream = g.stream();
                let mut inner = unsynn::ToTokens::to_token_iter(&stream);
                match inner.next() {
                    Some(TokenTree::Literal(ref lit)) => {
                        // Parse char from literal
                        let lit_str = lit.to_string();
                        if lit_str.starts_with('\'')
                            && lit_str.ends_with('\'')
                            && lit_str.len() == 3
                        {
                            let ch = lit_str.chars().nth(1).unwrap();
                            Ok(Some(ch))
                        } else {
                            Err(Error::no_error())
                        }
                    }
                    _ => Err(Error::no_error()),
                }
            }
            _ => Err(Error::no_error()),
        }
    })
    .or(Ok(None))
}

/// Parse optional string argument like ("name")
fn parse_optional_string_arg(iter: &mut TokenIter) -> Result<Option<String>> {
    iter.transaction(|t| {
        // Look for opening paren
        match t.next() {
            Some(TokenTree::Group(ref g))
                if g.delimiter() == proc_macro2::Delimiter::Parenthesis =>
            {
                // Parse the string literal inside
                let stream = g.stream();
                let mut inner = unsynn::ToTokens::to_token_iter(&stream);
                match inner.next() {
                    Some(TokenTree::Literal(ref lit)) => {
                        // Parse string from literal
                        let lit_str = lit.to_string();
                        if lit_str.starts_with('"') && lit_str.ends_with('"') {
                            let s = lit_str[1..lit_str.len() - 1].to_string();
                            Ok(Some(s))
                        } else {
                            Err(Error::no_error())
                        }
                    }
                    _ => Err(Error::no_error()),
                }
            }
            _ => Err(Error::no_error()),
        }
    })
    .or(Ok(None))
}

/// Parse optional expression argument like (value)
fn parse_optional_expr_arg(iter: &mut TokenIter) -> Result<Option<TokenStream>> {
    iter.transaction(|t| {
        // Look for opening paren
        match t.next() {
            Some(TokenTree::Group(ref g))
                if g.delimiter() == proc_macro2::Delimiter::Parenthesis =>
            {
                // Return the entire content as TokenStream
                Ok(Some(g.stream()))
            }
            _ => Err(Error::no_error()),
        }
    })
    .or(Ok(None))
}

/// Parse optional turbofish type like ::<Type>
fn parse_turbofish(iter: &mut TokenIter) -> Result<Option<TokenStream>> {
    iter.transaction(|t| {
        // Look for :: followed by < Type >
        match (t.next(), t.next()) {
            (Some(TokenTree::Punct(ref p1)), Some(TokenTree::Punct(ref p2)))
                if p1.as_char() == ':' && p2.as_char() == ':' =>
            {
                // Now expect < Type >
                match t.next() {
                    Some(TokenTree::Punct(ref p)) if p.as_char() == '<' => {
                        // Collect tokens until we find the closing >
                        let mut ty = TokenStream::new();
                        let mut depth = 1;

                        loop {
                            match t.next() {
                                Some(TokenTree::Punct(ref p)) if p.as_char() == '<' => {
                                    depth += 1;
                                    ty.extend(std::iter::once(TokenTree::Punct(p.clone())));
                                }
                                Some(TokenTree::Punct(ref p)) if p.as_char() == '>' => {
                                    depth -= 1;
                                    if depth == 0 {
                                        break;
                                    }
                                    ty.extend(std::iter::once(TokenTree::Punct(p.clone())));
                                }
                                Some(tt) => {
                                    ty.extend(std::iter::once(tt));
                                }
                                None => return Err(Error::no_error()),
                            }
                        }

                        Ok(Some(ty))
                    }
                    _ => Err(Error::no_error()),
                }
            }
            _ => Err(Error::no_error()),
        }
    })
    .or(Ok(None))
}

/// Parse two comma-separated expression arguments like (expr1, expr2)
fn parse_two_expr_args(iter: &mut TokenIter) -> Result<Option<(TokenStream, TokenStream)>> {
    iter.transaction(|t| {
        // Look for opening paren
        match t.next() {
            Some(TokenTree::Group(ref g))
                if g.delimiter() == proc_macro2::Delimiter::Parenthesis =>
            {
                // Parse the content: expr, expr
                let stream = g.stream();
                let mut inner = unsynn::ToTokens::to_token_iter(&stream);

                // Collect tokens until comma
                let mut first = TokenStream::new();
                let mut found_comma = false;

                loop {
                    match inner.next() {
                        Some(TokenTree::Punct(ref p)) if p.as_char() == ',' => {
                            found_comma = true;
                            break;
                        }
                        Some(tt) => {
                            first.extend(std::iter::once(tt));
                        }
                        None => break,
                    }
                }

                if !found_comma {
                    return Err(Error::no_error());
                }

                // Collect remaining tokens as second expression
                let mut second = TokenStream::new();
                for tt in inner {
                    second.extend(std::iter::once(tt));
                }

                if first.is_empty() || second.is_empty() {
                    return Err(Error::no_error());
                }

                Ok(Some((first, second)))
            }
            _ => Err(Error::no_error()),
        }
    })
    .or(Ok(None))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_short_attr() {
        // Test parsing: #[bpaf(short)]
        let tokens: TokenStream = "short".parse().unwrap();
        let group = proc_macro2::Group::new(proc_macro2::Delimiter::Parenthesis, tokens);
        let attrs = FieldAttrs::parse_from_attrs(&[group], &[]).unwrap();
        assert!(attrs.short.is_some());
    }

    #[test]
    fn test_parse_long_attr() {
        // Test parsing: #[bpaf(long)]
        let tokens: TokenStream = "long".parse().unwrap();
        let group = proc_macro2::Group::new(proc_macro2::Delimiter::Parenthesis, tokens);
        let attrs = FieldAttrs::parse_from_attrs(&[group], &[]).unwrap();
        assert!(attrs.long.is_some());
    }

    #[test]
    fn test_parse_short_with_value() {
        // Test parsing: #[bpaf(short('v'))]
        let tokens: TokenStream = "short('v')".parse().unwrap();
        let group = proc_macro2::Group::new(proc_macro2::Delimiter::Parenthesis, tokens);
        let attrs = FieldAttrs::parse_from_attrs(&[group], &[]).unwrap();
        assert_eq!(attrs.short, Some('v'));
    }

    #[test]
    fn test_parse_long_with_value() {
        // Test parsing: #[bpaf(long("verbose"))]
        let tokens: TokenStream = r#"long("verbose")"#.parse().unwrap();
        let group = proc_macro2::Group::new(proc_macro2::Delimiter::Parenthesis, tokens);
        let attrs = FieldAttrs::parse_from_attrs(&[group], &[]).unwrap();
        assert_eq!(attrs.long, Some("verbose".to_string()));
    }

    #[test]
    fn test_parse_multiple_attrs() {
        // Test parsing: #[bpaf(short('v'), long("verbose"))]
        let tokens: TokenStream = r#"short('v'), long("verbose")"#.parse().unwrap();
        let group = proc_macro2::Group::new(proc_macro2::Delimiter::Parenthesis, tokens);
        let attrs = FieldAttrs::parse_from_attrs(&[group], &[]).unwrap();
        assert_eq!(attrs.short, Some('v'));
        assert_eq!(attrs.long, Some("verbose".to_string()));
    }
}
