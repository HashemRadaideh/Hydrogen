use std::fmt;

/// Struct representing the position of a token in the source code
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Position {
    pub col: usize,
    pub row: usize,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.col, self.row)
    }
}

/// Enum representing different types of tokens
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    LeftParenthesis(Position),
    RightParenthesis(Position),
    LeftBrace(Position),
    RightBrace(Position),
    LeftBracket(Position),
    RightBracket(Position),
    Plus(Position),
    PlusEqual(Position),
    Minus(Position),
    MinusEqual(Position),
    Asterisk(Position),
    AsteriskEqual(Position),
    Slash(Position),
    SlashEqual(Position),
    Equal(Position),
    Equals(Position),
    NotEqual(Position),
    GreaterThan(Position),
    GreaterThanOrEqual(Position),
    LessThan(Position),
    LessThanOrEqual(Position),
    Ampersand(Position),
    And(Position),
    Or(Position),
    DollarSign(Position),
    Hash(Position),
    ExplinationMark(Position),
    QuestionMark(Position),
    Colon(Position),
    Dot(Position),
    Comma(Position),
    At(Position),
    Percent(Position),
    PercentEqual(Position),
    Caret(Position),
    CaretEqual(Position),
    In(Position),
    As(Position),
    Identifier(Position, String),
    Type(Position, String),
    Keyword(Position, String),
    String(Position, String),
    Boolean(Position, String),
    Number(Position, String),
    Unknown(Position, String),
    Eof(Position),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::LeftParenthesis(_) => write!(f, "("),
            Token::RightParenthesis(_) => write!(f, ")"),
            Token::LeftBrace(_) => write!(f, "{{"),
            Token::RightBrace(_) => write!(f, "}}"),
            Token::LeftBracket(_) => write!(f, "["),
            Token::RightBracket(_) => write!(f, "]"),
            Token::Plus(_) => write!(f, "+"),
            Token::PlusEqual(_) => write!(f, "+="),
            Token::Minus(_) => write!(f, "-"),
            Token::MinusEqual(_) => write!(f, "-="),
            Token::Asterisk(_) => write!(f, "*"),
            Token::AsteriskEqual(_) => write!(f, "*="),
            Token::Slash(_) => write!(f, "/"),
            Token::SlashEqual(_) => write!(f, "/="),
            Token::Equal(_) => write!(f, "="),
            Token::Equals(_) => write!(f, "=="),
            Token::NotEqual(_) => write!(f, "!="),
            Token::GreaterThan(_) => write!(f, ">"),
            Token::GreaterThanOrEqual(_) => write!(f, ">="),
            Token::LessThan(_) => write!(f, "<"),
            Token::LessThanOrEqual(_) => write!(f, "<="),
            Token::Ampersand(_) => write!(f, "&"),
            Token::And(_) => write!(f, "and"),
            Token::Or(_) => write!(f, "or"),
            Token::DollarSign(_) => write!(f, "$"),
            Token::Hash(_) => write!(f, "#"),
            Token::ExplinationMark(_) => write!(f, "!"),
            Token::QuestionMark(_) => write!(f, "?"),
            Token::Colon(_) => write!(f, ":"),
            Token::Dot(_) => write!(f, "."),
            Token::At(_) => write!(f, "@"),
            Token::Percent(_) => write!(f, "%"),
            Token::PercentEqual(_) => write!(f, "%="),
            Token::Caret(_) => write!(f, "^"),
            Token::CaretEqual(_) => write!(f, "^="),
            Token::Identifier(_, name) => write!(f, "Identifier({})", name),
            Token::Type(_, ty) => write!(f, "Type({})", ty),
            Token::Keyword(_, kw) => write!(f, "Keyword({})", kw),
            Token::String(_, s) => write!(f, "String(\"{}\")", s),
            Token::Boolean(_, b) => write!(f, "Boolean({})", b),
            Token::Number(_, n) => write!(f, "Number({})", n),
            Token::Unknown(_, u) => write!(f, "Unknown({})", u),
            Token::Eof(_) => write!(f, "EOF"),
            Token::In(_) => todo!(),
            Token::As(_) => todo!(),
            Token::Comma(_) => write!(f, "Comma"),
        }
    }
}
