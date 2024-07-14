#[derive(Debug, PartialEq)]
pub enum TokenType {
    RETURN,
    EXIT,
    INTLIT,
    BOOLEAN,
    VARIABLE,
    EQUAL,
    ADD,
    REMOVE,
    SEMICOLON,
    UNKNOWN,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value: Option<String>,
}
