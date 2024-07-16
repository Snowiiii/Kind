use std::fmt::write;

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

pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub char_info: CharLocationInfo,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} in {}:{}:{}",
            self.value,
            self.char_info.file_path,
            self.char_info.line_number,
            self.char_info.line_col
        )
    }
}

pub struct CharLocationInfo {
    pub file_path: String,
    pub line_number: usize,
    pub line_col: u32,
}
