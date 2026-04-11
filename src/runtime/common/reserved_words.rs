/// Java reserveds words.
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
    /// Returns `true` if the provided string is reserved (according to [`RESERVEDS`]).
    pub fn is_reserved<T: AsRef<str>>(s: T) -> bool {
        let fixed = s.as_ref().to_lowercase();
        RESERVEDS.into_iter().any(|r| r == fixed.trim())
    }
}
