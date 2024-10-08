use std::path::PathBuf;

use crate::token::{CharLocationInfo, DataType, Token};

pub fn read_file(file: PathBuf) -> Vec<Token> {
    let path = file.display().to_string();
    let content = std::fs::read_to_string(file).expect("Failed to read file");
    let mut final_vec = Vec::new();
    for (line_number, line) in content.lines().enumerate() {
        final_vec.append(&mut tokennize_line(line.to_owned(), line_number, &path))
    }
    final_vec
}

// We tokennize line by line so we can do better error handling/better compiler errors
pub fn tokennize_line(string: String, line_number: usize, file: &String) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = string.chars().peekable();

    let mut line_col = 0;
    while let Some(c) = chars.next() {
        line_col += 1;
        let char_info = CharLocationInfo {
            file_path: file.clone(),
            line_number: line_number + 1, // We want to start from line 1 and not 0
            line_col,
        };
        if c.is_alphabetic() {
            let mut ident = String::new();
            ident.push(c);
            while let Some(next) = chars.peek() {
                if next.is_alphanumeric() {
                    ident.push(chars.next().unwrap());
                } else {
                    break;
                }
            }
            tokens.push(parse_token(&ident, char_info));
        } else if c.is_numeric() {
            let mut num = String::new();
            num.push(c);
            while let Some(next) = chars.peek() {
                if next.is_numeric() {
                    num.push(chars.next().unwrap());
                } else {
                    break;
                }
            }
            //   let int_value = num.parse::<i32>().unwrap();
            tokens.push(Token {
                token_type: crate::token::TokenType::INTLIT,
                value: num,
                char_info,
            });
        } else if c == ';' {
            tokens.push(Token {
                token_type: crate::token::TokenType::SEMICOLON,
                value: ";".to_string(),
                char_info,
            })
        } else if c.is_whitespace() {
            continue;
        } else if let Some(token) = parse_expr(c, char_info) {
            tokens.push(token)
        } else {
            println!("Unexpected char")
        }
    }
    tokens
}

pub fn parse_token(string: &str, char_info: CharLocationInfo) -> Token {
    if string == "ret" {
        return Token {
            token_type: crate::token::TokenType::RETURN,
            value: "ret".to_string(),
            char_info,
        };
    } else if string == "exit" {
        return Token {
            token_type: crate::token::TokenType::EXIT,
            value: "exit".to_string(),
            char_info,
        };
    } else if string == "true" {
        return Token {
            token_type: crate::token::TokenType::DATATYPE(DataType::BOOLEAN),
            value: "true".to_string(),
            char_info,
        };
    } else if string == "false" {
        return Token {
            token_type: crate::token::TokenType::DATATYPE(DataType::BOOLEAN),
            value: "false".to_string(),
            char_info,
        };
    } else if string == "save" {
        return Token {
            token_type: crate::token::TokenType::VARIABLE,
            value: "save".to_string(),
            char_info,
        };
    }
    Token {
        token_type: crate::token::TokenType::UNKNOWN,
        value: string.to_string(),
        char_info,
    }
}

pub fn parse_expr(string: char, char_info: CharLocationInfo) -> Option<Token> {
    match string {
        '=' => Some(Token {
            token_type: crate::token::TokenType::EQUAL,
            value: "=".to_string(),
            char_info,
        }),
        '-' => Some(Token {
            token_type: crate::token::TokenType::REMOVE,
            value: "-".to_string(),
            char_info,
        }),
        '+' => Some(Token {
            token_type: crate::token::TokenType::ADD,
            value: "+".to_string(),
            char_info,
        }),
        _ => None,
    }
}
