/// Java reserved words.
const RESERVEDS: [&str; 69] = [
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
    "long",
    "double",
    "system",
    "thread",
    "exception",
    "error",
    // others
    "java",
];

/// Struct designated to detected Java's reserved words.
pub struct ReservedWords;

impl ReservedWords {
    /// Returns an [`Option`] of the `s` string if it's recognized as reserved word.
    ///
    /// Avoided a `fn is_reserved(...) -> bool` since it triggers if reserved, but an extra
    /// allocation is necessary to store the reserved input.
    ///
    /// Just return the word as `&'static str` instead.
    pub fn get_if_contained<T: AsRef<str>>(s: T) -> Option<&'static str> {
        let fixed = s.as_ref().to_lowercase();
        let s = fixed.trim();
        RESERVEDS.into_iter().find(|item| item == &s)
    }
}
