use crate::utils::MayFrom;
use std::sync::LazyLock;
use strum::IntoEnumIterator;

/// Permutation for [`JavaReserveds`] variants.
static RESERVED_WORDS_PERMUTATION: LazyLock<Vec<JavaReserveds>> =
    LazyLock::new(|| JavaReserveds::iter().collect());

/// `Java` reserved words.
#[derive(strum::Display, strum::EnumIter, PartialEq, Clone, Debug)]
pub enum JavaReserveds {
    // keywords
    #[strum(to_string = "abstract")]
    Abstract,
    #[strum(to_string = "assert")]
    Assert,
    #[strum(to_string = "boolean")]
    Boolean,
    #[strum(to_string = "break")]
    Break,
    #[strum(to_string = "byte")]
    Byte,
    #[strum(to_string = "case")]
    Case,
    #[strum(to_string = "catch")]
    Catch,
    #[strum(to_string = "char")]
    Char,
    #[strum(to_string = "class")]
    Class,
    #[strum(to_string = "const")]
    Const,
    #[strum(to_string = "continue")]
    Continue,
    #[strum(to_string = "default")]
    Default,
    #[strum(to_string = "do")]
    Do,
    #[strum(to_string = "double")]
    Double,
    #[strum(to_string = "else")]
    Else,
    #[strum(to_string = "enum")]
    Enum,
    #[strum(to_string = "extends")]
    Extends,
    #[strum(to_string = "final")]
    Final,
    #[strum(to_string = "finally")]
    Finally,
    #[strum(to_string = "float")]
    Float,
    #[strum(to_string = "for")]
    For,
    #[strum(to_string = "goto")]
    Goto,
    #[strum(to_string = "if")]
    If,
    #[strum(to_string = "implements")]
    Implements,
    #[strum(to_string = "import")]
    Import,
    #[strum(to_string = "instanceof")]
    Instanceof,
    #[strum(to_string = "int")]
    Int,
    #[strum(to_string = "interface")]
    Interface,
    #[strum(to_string = "long")]
    Long,
    #[strum(to_string = "native")]
    Native,
    #[strum(to_string = "new")]
    New,
    #[strum(to_string = "package")]
    Package,
    #[strum(to_string = "private")]
    Private,
    #[strum(to_string = "protected")]
    Protected,
    #[strum(to_string = "public")]
    Public,
    #[strum(to_string = "return")]
    Return,
    #[strum(to_string = "short")]
    Short,
    #[strum(to_string = "static")]
    Static,
    #[strum(to_string = "strictfp")]
    Strictfp,
    #[strum(to_string = "super")]
    Super,
    #[strum(to_string = "switch")]
    Switch,
    #[strum(to_string = "synchronized")]
    Synchronized,
    #[strum(to_string = "this")]
    This,
    #[strum(to_string = "throw")]
    Throw,
    #[strum(to_string = "throws")]
    Throws,
    #[strum(to_string = "transient")]
    Transient,
    #[strum(to_string = "try")]
    Try,
    #[strum(to_string = "void")]
    Void,
    #[strum(to_string = "volatile")]
    Volatile,
    #[strum(to_string = "while")]
    While,
    // literals
    #[strum(to_string = "true")]
    True,
    #[strum(to_string = "false")]
    False,
    #[strum(to_string = "null")]
    Null,
    // restricted
    #[strum(to_string = "var")]
    Var,
    #[strum(to_string = "yield")]
    Yield,
    #[strum(to_string = "record")]
    Record,
    #[strum(to_string = "sealed")]
    Sealed,
    #[strum(to_string = "permits")]
    Permits,
    #[strum(to_string = "non-sealed")]
    NonSealed,
    // common types
    #[strum(to_string = "string")]
    String,
    #[strum(to_string = "object")]
    Object,
    #[strum(to_string = "integer")]
    Integer,
    #[strum(to_string = "system")]
    System,
    #[strum(to_string = "thread")]
    Thread,
    #[strum(to_string = "exception")]
    Exception,
    #[strum(to_string = "error")]
    Error,
    // others
    #[strum(to_string = "java")]
    Java,
}

impl MayFrom<&str> for JavaReserveds {
    fn may_from(value: &str) -> Option<Self> {
        RESERVED_WORDS_PERMUTATION
            .iter()
            .find(|x| x.to_string() == value)
            .cloned()
    }
}
