//! Core parsing primitives using unsynn
//!
//! This module defines basic building blocks for parsing Rust syntax

pub use unsynn::*;

// Re-export commonly used types
pub use proc_macro2::{Ident, TokenStream, TokenTree};

// Type alias for the iterator type we use
pub type TokenIter<'a> = unsynn::TokenIter<'a>;

// Keywords we'll need for parsing structs and enums
keyword! {
    pub KStruct = "struct";
    pub KEnum = "enum";
    pub KPub = "pub";
}

// Bpaf-specific keywords
keyword! {
    /// The "bpaf" keyword for attributes
    pub KBpaf = "bpaf";
    /// The "doc" keyword for documentation
    pub KDoc = "doc";

    // Name attributes
    /// The "short" keyword
    pub KShort = "short";
    /// The "long" keyword
    pub KLong = "long";
    /// The "env" keyword
    pub KEnv = "env";

    // Consumer attributes
    /// The "switch" keyword
    pub KSwitch = "switch";
    /// The "flag" keyword
    pub KFlag = "flag";
    /// The "argument" keyword
    pub KArgument = "argument";
    /// The "positional" keyword
    pub KPositional = "positional";
    /// The "req_flag" keyword
    pub KReqFlag = "req_flag";
    /// The "any" keyword
    pub KAny = "any";
    /// The "external" keyword
    pub KExternal = "external";
    /// The "pure" keyword
    pub KPure = "pure";
    /// The "pure_with" keyword
    pub KPureWith = "pure_with";

    // PostParse attributes
    /// The "map" keyword
    pub KMap = "map";
    /// The "parse" keyword
    pub KParse = "parse";
    /// The "optional" keyword
    pub KOptional = "optional";
    /// The "many" keyword
    pub KMany = "many";
    /// The "some" keyword
    pub KSome = "some";
    /// The "catch" keyword
    pub KCatch = "catch";
    /// The "collect" keyword
    pub KCollect = "collect";
    /// The "count" keyword
    pub KCount = "count";
    /// The "anywhere" keyword
    pub KAnywhere = "anywhere";
    /// The "adjacent" keyword
    pub KAdjacent = "adjacent";
    /// The "strict" keyword
    pub KStrict = "strict";
    /// The "non_strict" keyword
    pub KNonStrict = "non_strict";

    // PostDecor attributes
    /// The "guard" keyword
    pub KGuard = "guard";
    /// The "hide" keyword
    pub KHide = "hide";
    /// The "hide_usage" keyword
    pub KHideUsage = "hide_usage";
    /// The "custom_usage" keyword
    pub KCustomUsage = "custom_usage";
    /// The "fallback_with" keyword
    pub KFallbackWith = "fallback_with";
    /// The "group_help" keyword
    pub KGroupHelp = "group_help";
    /// The "debug_fallback" keyword
    pub KDebugFallback = "debug_fallback";
    /// The "display_fallback" keyword
    pub KDisplayFallback = "display_fallback";
    /// The "format_fallback" keyword
    pub KFormatFallback = "format_fallback";
    /// The "last" keyword
    pub KLast = "last";
    /// The "complete" keyword
    pub KComplete = "complete";
    /// The "group" keyword
    pub KGroup = "group";
    /// The "complete_shell" keyword
    pub KCompleteShell = "complete_shell";

    // Enum/variant attributes
    /// The "command" keyword
    pub KCommand = "command";
    /// The "skip" keyword
    pub KSkip = "skip";
    /// The "fallback_to_usage" keyword
    pub KFallbackToUsage = "fallback_to_usage";

    // Other attributes
    /// The "help" keyword
    pub KHelp = "help";
    /// The "fallback" keyword
    pub KFallback = "fallback";
    /// The "ignore_rustdoc" keyword
    pub KIgnoreRustdoc = "ignore_rustdoc";
    /// The "path" keyword
    pub KPath = "path";
    /// The "generate" keyword
    pub KGenerate = "generate";
    /// The "private" keyword
    pub KPrivate = "private";
    /// The "boxed" keyword
    pub KBoxed = "boxed";

    // Options mode attributes
    /// The "descr" keyword
    pub KDescr = "descr";
    /// The "footer" keyword
    pub KFooter = "footer";
    /// The "header" keyword
    pub KHeader = "header";
    /// The "usage" keyword
    pub KUsage = "usage";
    /// The "version" keyword
    pub KVersion = "version";
    /// The "max_width" keyword
    pub KMaxWidth = "max_width";
    /// The "cargo_helper" keyword
    pub KCargoHelper = "cargo_helper";

    // Mode keywords
    /// The "options" keyword
    pub KOptions = "options";
    /// The "parser" keyword
    pub KParser = "parser";
}

// Common operators
operator! {
    pub Comma = ",";
    pub Colon = ":";
    pub Semi = ";";
    pub Eq = "=";
    pub Lt = "<";
    pub Gt = ">";
}

// A token tree that properly handles `<...>` as a single unit (recursive).
// Either `< ... >` containing more AngleTokenTrees, or a plain TokenTree.
unsynn! {
    #[derive(Clone)]
    pub struct AngleTokenTree(
        pub Either<Cons<Lt, Vec<Cons<Except<Gt>, AngleTokenTree>>, Gt>, TokenTree>,
    );
}

/// Parses tokens until terminator C, properly handling nested `<...>`.
/// Use this for type parsing where generics like `HashMap<K, V>` must stay together.
pub type VerbatimUntil<C> = Many<Cons<Except<C>, AngleTokenTree>>;

/// Parses tokens until a comma is found (handles nested angle brackets for types)
pub type VerbatimUntilComma = VerbatimUntil<Comma>;

/// Parses tokens until > is found (handles nesting)
pub type VerbatimUntilGt = VerbatimUntil<Gt>;

/// Simple verbatim until terminator (no angle bracket handling)
/// Use this for non-type contexts where `<>` don't need special treatment.
pub type SimpleVerbatimUntil<C> = Many<Cons<Except<C>, TokenTree>>;

/// Simple verbatim until comma (no angle bracket handling)
pub type SimpleVerbatimUntilComma = SimpleVerbatimUntil<Comma>;

// Type keywords for field analysis
keyword! {
    pub KOption = "Option";
    pub KVec = "Vec";
    pub KBool = "bool";
}

unsynn! {
    /// Option<T> type with proper nesting support
    pub struct OptionType {
        pub _kw: KOption,
        pub _lt: Lt,
        pub inner: VerbatimUntilGt,
        pub _gt: Gt,
    }

    /// Vec<T> type with proper nesting support
    pub struct VecType {
        pub _kw: KVec,
        pub _lt: Lt,
        pub inner: VerbatimUntilGt,
        pub _gt: Gt,
    }

    /// Empty parentheses ()
    pub struct UnitType(pub ParenthesisGroup);
}

/// The shape of a field's type with inner type included
#[derive(Debug, Clone)]
pub enum TypeShape {
    /// bool type
    Bool,
    /// () unit type
    Unit,
    /// Option<T>
    Optional(TokenStream),
    /// Vec<T>
    Multiple(TokenStream),
    /// Any other type T
    Direct(TokenStream),
}

impl Parser for TypeShape {
    fn parser(input: &mut TokenIter) -> Result<Self> {
        // Try Option<T>
        if let Ok(opt) = input.parse::<OptionType>() {
            return Ok(TypeShape::Optional(opt.inner.to_token_stream()));
        }

        // Try Vec<T>
        if let Ok(vec) = input.parse::<VecType>() {
            return Ok(TypeShape::Multiple(vec.inner.to_token_stream()));
        }

        // Try bool
        if input.parse::<KBool>().is_ok() {
            return Ok(TypeShape::Bool);
        }

        // Try unit ()
        if let Ok(unit) = input.parse::<UnitType>() {
            if unit.0 .0.stream().is_empty() {
                return Ok(TypeShape::Unit);
            }
            // Non-empty parens, fall through to Direct with the whole type
        }

        // Collect remaining as Direct type
        let tokens: Vec<TokenTree> = input.collect();
        if tokens.is_empty() {
            return Err(Error::no_error());
        }
        Ok(TypeShape::Direct(tokens.into_iter().collect()))
    }
}

impl TypeShape {
    /// Get the inner type for Optional/Multiple/Direct, or empty for Bool/Unit
    pub fn inner_type(&self) -> TokenStream {
        match self {
            TypeShape::Bool | TypeShape::Unit => TokenStream::new(),
            TypeShape::Optional(inner) | TypeShape::Multiple(inner) | TypeShape::Direct(inner) => {
                inner.clone()
            }
        }
    }

    /// Returns true if this is an Optional<T> type
    pub fn is_optional(&self) -> bool {
        matches!(self, TypeShape::Optional(_))
    }

    /// Returns true if this is a Vec<T> type
    pub fn is_multiple(&self) -> bool {
        matches!(self, TypeShape::Multiple(_))
    }

    /// Returns true if this is a bool type
    #[allow(dead_code)]
    pub fn is_bool(&self) -> bool {
        matches!(self, TypeShape::Bool)
    }

    /// Returns true if this is a () unit type
    #[allow(dead_code)]
    pub fn is_unit(&self) -> bool {
        matches!(self, TypeShape::Unit)
    }
}

unsynn! {
    /// Represents a bpaf attribute: #[bpaf(...)]
    pub struct BpafAttr {
        /// The "bpaf" keyword
        pub _kw_bpaf: KBpaf,
        /// The inner content in parentheses
        pub inner: ParenthesisGroupContaining<CommaDelimitedVec<BpafInner>>,
    }

    /// Inner content of #[bpaf(...)] attributes
    pub enum BpafInner {
        // Variant attributes
        /// command or command("name")
        Command(CommandInner),
        /// skip
        Skip(KSkip),
        /// fallback_to_usage
        FallbackToUsage(KFallbackToUsage),

        // Name attributes
        /// short or short('c')
        Short(ShortInner),
        /// long or long("name")
        Long(LongInner),
        /// env("VAR") or env(expr)
        Env(EnvInner),

        // Consumer attributes
        /// switch
        Switch(KSwitch),
        /// flag(present, absent)
        Flag(FlagInner),
        /// argument or argument("METAVAR")
        Argument(ArgumentInner),
        /// positional or positional("METAVAR")
        Positional(PositionalInner),
        /// req_flag(value)
        ReqFlag(ReqFlagInner),
        /// any("METAVAR", check_fn) or any::<Type>("METAVAR", check_fn)
        Any(AnyInner),
        /// external or external(parser)
        External(ExternalInner),
        /// pure(value)
        Pure(PureInner),
        /// pure_with(function)
        PureWith(PureWithInner),

        // PostParse attributes
        /// map(function)
        Map(MapInner),
        /// parse(function)
        Parse(ParseInner),
        /// optional
        Optional(KOptional),
        /// many
        Many(KMany),
        /// some("error msg")
        Some(SomeInner),
        /// catch
        Catch(KCatch),
        /// collect
        Collect(KCollect),
        /// count
        Count(KCount),
        /// anywhere
        Anywhere(KAnywhere),
        /// adjacent
        Adjacent(KAdjacent),
        /// strict
        Strict(KStrict),
        /// non_strict
        NonStrict(KNonStrict),

        // PostDecor attributes
        /// guard(check_fn, "error msg")
        Guard(GuardInner),
        /// hide
        Hide(KHide),
        /// hide_usage
        HideUsage(KHideUsage),
        /// custom_usage("text")
        CustomUsage(CustomUsageInner),
        /// fallback_with(function)
        FallbackWith(FallbackWithInner),
        /// group_help("text")
        GroupHelp(GroupHelpInner),
        /// debug_fallback
        DebugFallback(KDebugFallback),
        /// display_fallback
        DisplayFallback(KDisplayFallback),
        /// format_fallback(formatter)
        FormatFallback(FormatFallbackInner),
        /// last
        Last(KLast),
        /// complete(function)
        Complete(CompleteInner),
        /// group("name")
        Group(GroupInner),
        /// complete_shell(expr)
        CompleteShell(CompleteShellInner),

        // Other attributes
        /// help("text")
        Help(HelpInner),
        /// fallback(expr)
        Fallback(FallbackInner),
        /// ignore_rustdoc
        IgnoreRustdoc(KIgnoreRustdoc),
        /// path(::custom::bpaf)
        Path(PathInner),
        /// generate(custom_name)
        Generate(GenerateInner),
        /// private
        Private(KPrivate),
        /// boxed
        Boxed(KBoxed),

        // Options mode attributes
        /// descr(...)
        Descr(DescrInner),
        /// footer(...)
        Footer(FooterInner),
        /// header(...)
        Header(HeaderInner),
        /// usage(...)
        Usage(UsageInner),
        /// version(...)
        Version(VersionInner),
        /// max_width(...)
        MaxWidth(MaxWidthInner),
        /// cargo_helper(...)
        CargoHelper(CargoHelperInner),

        // Mode attributes
        /// options
        Options(KOptions),
        /// parser
        Parser(KParser),

        /// Any other unknown attribute (for forward compatibility)
        Unknown(SimpleVerbatimUntilComma),
    }

    /// command or command("name")
    pub struct CommandInner {
        /// The "command" keyword
        pub _kw: KCommand,
        /// Optional custom name
        pub name: Option<ParenthesisGroupContaining<LiteralString>>,
    }

    /// short or short('c')
    pub struct ShortInner {
        /// The "short" keyword
        pub _kw: KShort,
        /// Optional character
        pub ch: Option<ParenthesisGroupContaining<LiteralCharacter>>,
    }

    /// long or long("name")
    pub struct LongInner {
        /// The "long" keyword
        pub _kw: KLong,
        /// Optional name
        pub name: Option<ParenthesisGroupContaining<LiteralString>>,
    }

    /// env("VAR") or env(expr)
    pub struct EnvInner {
        /// The "env" keyword
        pub _kw: KEnv,
        /// Expression in parentheses (could be string literal or other expr)
        pub expr: ParenthesisGroup,
    }

    /// flag(present, absent)
    pub struct FlagInner {
        /// The "flag" keyword
        pub _kw: KFlag,
        /// Two comma-separated expressions
        pub values: ParenthesisGroup,
    }

    /// argument or argument("METAVAR")
    pub struct ArgumentInner {
        /// The "argument" keyword
        pub _kw: KArgument,
        /// Optional metavar
        pub metavar: Option<ParenthesisGroupContaining<LiteralString>>,
    }

    /// positional or positional("METAVAR")
    pub struct PositionalInner {
        /// The "positional" keyword
        pub _kw: KPositional,
        /// Optional metavar
        pub metavar: Option<ParenthesisGroupContaining<LiteralString>>,
    }

    /// req_flag(value)
    pub struct ReqFlagInner {
        /// The "req_flag" keyword
        pub _kw: KReqFlag,
        /// The value expression
        pub value: ParenthesisGroup,
    }

    /// any("METAVAR", check_fn) or any::<Type>("METAVAR", check_fn)
    pub struct AnyInner {
        /// The "any" keyword
        pub _kw: KAny,
        /// Optional turbofish type
        pub turbofish: Option<TurbofishType>,
        /// Arguments - parsed separately to provide better error messages
        pub args: ParenthesisGroup,
    }

    /// Represents ::<Type>
    pub struct TurbofishType {
        /// ::
        pub _double_colon: Cons<Colon, Colon>,
        /// <
        pub _lt: Lt,
        /// Type tokens
        pub ty: VerbatimUntilGt,
        /// >
        pub _gt: Gt,
    }

    /// external or external(parser)
    pub struct ExternalInner {
        /// The "external" keyword
        pub _kw: KExternal,
        /// Optional parser expression
        pub parser: Option<ParenthesisGroup>,
    }

    /// pure(value)
    pub struct PureInner {
        /// The "pure" keyword
        pub _kw: KPure,
        /// The value expression
        pub value: ParenthesisGroup,
    }

    /// pure_with(function)
    pub struct PureWithInner {
        /// The "pure_with" keyword
        pub _kw: KPureWith,
        /// The function expression
        pub func: ParenthesisGroup,
    }

    /// map(function)
    pub struct MapInner {
        /// The "map" keyword
        pub _kw: KMap,
        /// The function expression
        pub func: ParenthesisGroup,
    }

    /// parse(function)
    pub struct ParseInner {
        /// The "parse" keyword
        pub _kw: KParse,
        /// The function expression
        pub func: ParenthesisGroup,
    }

    /// some("error msg")
    pub struct SomeInner {
        /// The "some" keyword
        pub _kw: KSome,
        /// The error message
        pub msg: ParenthesisGroup,
    }

    /// guard(check_fn, "error msg")
    pub struct GuardInner {
        /// The "guard" keyword
        pub _kw: KGuard,
        /// Arguments
        pub args: ParenthesisGroup,
    }

    /// custom_usage("text")
    pub struct CustomUsageInner {
        /// The "custom_usage" keyword
        pub _kw: KCustomUsage,
        /// The usage text
        pub text: ParenthesisGroup,
    }

    /// fallback_with(function)
    pub struct FallbackWithInner {
        /// The "fallback_with" keyword
        pub _kw: KFallbackWith,
        /// The function expression
        pub func: ParenthesisGroup,
    }

    /// group_help("text")
    pub struct GroupHelpInner {
        /// The "group_help" keyword
        pub _kw: KGroupHelp,
        /// The help text
        pub text: ParenthesisGroup,
    }

    /// format_fallback(formatter)
    pub struct FormatFallbackInner {
        /// The "format_fallback" keyword
        pub _kw: KFormatFallback,
        /// The formatter expression
        pub formatter: ParenthesisGroup,
    }

    /// complete(function)
    pub struct CompleteInner {
        /// The "complete" keyword
        pub _kw: KComplete,
        /// The function expression
        pub func: ParenthesisGroup,
    }

    /// group("name")
    pub struct GroupInner {
        /// The "group" keyword
        pub _kw: KGroup,
        /// The group name
        pub name: ParenthesisGroupContaining<LiteralString>,
    }

    /// complete_shell(expr)
    pub struct CompleteShellInner {
        /// The "complete_shell" keyword
        pub _kw: KCompleteShell,
        /// The expression
        pub expr: ParenthesisGroup,
    }

    /// help("text")
    pub struct HelpInner {
        /// The "help" keyword
        pub _kw: KHelp,
        /// The help text
        pub text: ParenthesisGroupContaining<LiteralString>,
    }

    /// fallback(expr)
    pub struct FallbackInner {
        /// The "fallback" keyword
        pub _kw: KFallback,
        /// The fallback expression
        pub expr: ParenthesisGroup,
    }

    /// path(::custom::bpaf)
    pub struct PathInner {
        /// The "path" keyword
        pub _kw: KPath,
        /// The custom path expression
        pub path: ParenthesisGroup,
    }

    /// generate(custom_name)
    pub struct GenerateInner {
        /// The "generate" keyword
        pub _kw: KGenerate,
        /// The custom function name
        pub name: ParenthesisGroupContaining<Ident>,
    }

    /// descr(...)
    pub struct DescrInner {
        /// The "descr" keyword
        pub _kw: KDescr,
        /// The description expression
        pub expr: ParenthesisGroup,
    }

    /// footer(...)
    pub struct FooterInner {
        /// The "footer" keyword
        pub _kw: KFooter,
        /// The footer expression
        pub expr: ParenthesisGroup,
    }

    /// header(...)
    pub struct HeaderInner {
        /// The "header" keyword
        pub _kw: KHeader,
        /// The header expression
        pub expr: ParenthesisGroup,
    }

    /// usage(...)
    pub struct UsageInner {
        /// The "usage" keyword
        pub _kw: KUsage,
        /// The usage expression
        pub expr: ParenthesisGroup,
    }

    /// version(...)
    pub struct VersionInner {
        /// The "version" keyword
        pub _kw: KVersion,
        /// The version expression
        pub expr: ParenthesisGroup,
    }

    /// max_width(...)
    pub struct MaxWidthInner {
        /// The "max_width" keyword
        pub _kw: KMaxWidth,
        /// The max width expression
        pub expr: ParenthesisGroup,
    }

    /// cargo_helper(...)
    pub struct CargoHelperInner {
        /// The "cargo_helper" keyword
        pub _kw: KCargoHelper,
        /// The cargo helper name
        pub name: ParenthesisGroupContaining<LiteralString>,
    }

    /// Represents a documentation attribute: #[doc = "text"]
    pub struct DocInner {
        /// The "doc" keyword
        pub _kw: KDoc,
        /// The equals sign '='
        pub _eq: Eq,
        /// The documentation text as a literal string
        pub value: LiteralString,
    }

    /// Represents an attribute annotation on an item
    pub enum Attribute {
        /// A bpaf attribute: #[bpaf(...)]
        Bpaf(BpafAttr),
        /// A documentation attribute: #[doc = "..."]
        Doc(DocInner),
        /// Any other attribute (skipped)
        Other(Vec<TokenTree>),
    }

    /// Visibility modifier for fields
    pub struct Visibility {
        pub _kw: KPub,
        pub restriction: Option<ParenthesisGroup>,
    }

    /// Two comma-separated arguments (both non-empty)
    /// Used for attributes like flag(present, absent) or guard(check, msg)
    /// Requires at least 1 token before comma, a comma, and at least 1 token after comma
    pub struct TwoArgs {
        /// First argument (at least 1 token before comma)
        pub first: Many<Cons<Except<Comma>, TokenTree>>,
        /// Required comma separator
        pub _comma: Comma,
        /// Second argument (at least 1 token after comma)
        pub second: Many<TokenTree>,
    }

    /// Arguments for any() attribute: string literal and check function
    /// Example: any("METAVAR", check_fn)
    pub struct AnyArgs {
        /// String literal for metavar (must be quoted)
        pub metavar: LiteralString,
        /// Required comma separator
        pub _comma: Comma,
        /// Check function (at least 1 token)
        pub check_fn: Many<TokenTree>,
    }
}

impl TwoArgs {
    /// Convert to pair of TokenStreams
    pub fn into_streams(self) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
        let first: proc_macro2::TokenStream = self
            .first
            .0
            .into_iter()
            .map(|delimited| delimited.value.second) // Extract TokenTree from Cons<Except<Comma>, TokenTree>
            .collect();
        let second: proc_macro2::TokenStream = self
            .second
            .0
            .into_iter()
            .map(|delimited| delimited.value)
            .collect();
        (first, second)
    }
}

impl AnyArgs {
    /// Get the metavar string (without quotes)
    pub fn metavar_str(&self) -> &str {
        self.metavar.as_str()
    }

    /// Get the check function as TokenStream
    pub fn check_fn_tokens(&self) -> proc_macro2::TokenStream {
        use unsynn::ToTokens;
        let mut ts = proc_macro2::TokenStream::new();
        for delimited in &self.check_fn.0 {
            delimited.value.to_tokens(&mut ts);
        }
        ts
    }
}
