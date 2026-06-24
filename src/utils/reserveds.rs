/// Java common reserved words.
const RESERVED_WORDS: [&str; 67] = [
    // keywords
    "abstract",
    "assert",
    "boolean",
    "break",
    "byte",
    "case",
    "catch",
    "char",
    "class",
    "const",
    "continue",
    "default",
    "do",
    "double",
    "else",
    "enum",
    "extends",
    "final",
    "finally",
    "float",
    "for",
    "goto",
    "if",
    "implements",
    "import",
    "instanceof",
    "int",
    "interface",
    "long",
    "native",
    "new",
    "package",
    "private",
    "protected",
    "public",
    "return",
    "short",
    "static",
    "strictfp",
    "super",
    "switch",
    "synchronized",
    "this",
    "throw",
    "throws",
    "transient",
    "try",
    "void",
    "volatile",
    "while",
    // literals
    "true",
    "false",
    "null",
    // restricted
    "var",
    "yield",
    "record",
    "sealed",
    "permits",
    "non-sealed",
    // common types
    "string",
    "object",
    "integer",
    "system",
    "thread",
    "exception",
    "error",
    // others
    "java",
];

/// Does the checking of reserved words.
///
/// The actual _'reserved words'_ can be seen at [`RESERVED_WORDS`] constant.
pub struct Reserveds;

impl Reserveds {
    /// If the provided value is recognized as a reserved word.
    ///
    /// This function also trims and lowercases the input value, so it'll work for 'not well formed
    /// strings' too.
    pub fn is_reserved(value: &str) -> bool {
        RESERVED_WORDS.contains(&value.trim().to_lowercase().as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::Reserveds;

    #[test]
    fn is_reserved() {
        assert!(Reserveds::is_reserved("java"));
        assert!(Reserveds::is_reserved("Integer"));
        assert!(!Reserveds::is_reserved("notjava"));
    }
}
