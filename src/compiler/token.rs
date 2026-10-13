#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // --- Control Flow Keywords ---
    Import,
    Return,
    If,
    Else,
    While,
    Do,
    For,
    Break,
    Continue,
    Goto,

    // --- Built-in Data Types ---
    Void,
    Int,
    Double,
    Char,
    Bool,

    // --- Boolean Literals ---
    True,
    False,

    // --- Operators ---
    Assign, // =
    Plus,   // +
    Minus,  // -
    Star,   // *
    Slash,  // /
    Modulo, // %

    // --- Compound Assignment Operators ---
    PlusAssign,  // +=
    MinusAssign, // -=
    StarAssign,  // *=
    SlashAssign, // /=
    ModAssign,   // %=
    AndAssign,   // &=
    OrAssign,    // |=
    XorAssign,   // ^=
    ShlAssign,   // <<=
    ShrAssign,   // >>=

    // --- Increment & Decrement Operators
    Increment, // ++
    Decrement, // --

    // --- Logical Operators ---
    And, // &&
    Or,  // ||
    Not, // !

    // --- Relational Operators ---
    Eq,    // ==
    NotEq, // !=
    Lt,    // <
    Gt,    // >
    LtEq,  // <=
    GtEq,  // >=

    // --- Bitwise Operators ---
    BitAnd, // &
    BitOr,  // |
    BitXor, // ^
    BitNot, // ~
    Shl,    // << (Shift left)
    Shr,    // >> (Shift right)

    // --- Punctuation & Delimitators ---
    Semi,         // ;
    Comma,        // ,
    Dot,          // .
    Colon,        // :
    OpenParen,    // (
    CloseParen,   // )
    OpenCurly,    // {
    CloseCurly,   // }
    OpenBracket,  // [
    CloseBracket, // ]

    // --- Comments ---
    LineComment,  // //
    BlockComment, // /* */

    // --- Dynamic Literals and Identifiers ---
    IntLit,    // e.g., 42
    DoubleLit, // e.g., 3.14
    CharLit,   // e.g., 'a'
    StringLit, // e.g., "hello world"
    Ident,     // Variable or function names, e.g., "my_variable"

    // --- Special & Error Handling ---
    Invalid,
}

#[derive(Debug, Clone)]
pub struct Token {
    kind: TokenType,
    value: Option<String>,
    line: usize,
    column: usize,
    //file_id: usize, // Reservado para uso futuro (quando tiver imports)
}

impl Token {
    pub const fn new(value: Option<String>, kind: TokenType, line: usize, column: usize) -> Self {
        Token {
            kind,
            value,
            line,
            column,
            //file_id: 0,
        }
    }

    pub const fn kind(&self) -> &TokenType {
        &self.kind
    }

    pub const fn value(&self) -> Option<&String> {
        self.value.as_ref()
    }

    pub const fn line(&self) -> &usize {
        &self.line
    }

    pub const fn column(&self) -> &usize {
        &self.column
    }
}
