use std::path::PathBuf;

use crate::token::Token;

pub fn read_file(file: PathBuf) -> Vec<Token> {
    let content = std::fs::read_to_string(file).expect("Failed to read file");
    tokennize(content)
}

pub fn tokennize(string: String) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = string.chars().peekable();

    while let Some(c) = chars.next() {
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
            tokens.push(parse_token(&ident));
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
                value: Some(num),
            });
        } else if c == ';' {
            tokens.push(Token {
                token_type: crate::token::TokenType::SEMICOLON,
                value: None,
            })
        } else if c.is_whitespace() {
            continue;
        } else if let Some(token) = parse_expr(c) {
            tokens.push(token)
        } else {
            println!("Unexpected char")
        }
    }
    tokens
}

pub fn parse_token(string: &str) -> Token {
    if string == "ret" {
        return Token {
            token_type: crate::token::TokenType::RETURN,
            value: None,
        };
    } else if string == ";" {
        return Token {
            token_type: crate::token::TokenType::SEMICOLON,
            value: None,
        };
    } else if string == "exit" {
        return Token {
            token_type: crate::token::TokenType::EXIT,
            value: None,
        };
    } else if string == "true" {
        return Token {
            token_type: crate::token::TokenType::BOOLEAN,
            value: Some("true".to_string()),
        };
    } else if string == "false" {
        return Token {
            token_type: crate::token::TokenType::BOOLEAN,
            value: Some("false".to_string()),
        };
    } else if string == "save" {
        return Token {
            token_type: crate::token::TokenType::VARIABLE,
            value: None,
        };
    } else if string == "=" {
        return Token {
            token_type: crate::token::TokenType::VARIABLE,
            value: None,
        };
    }
    Token {
        token_type: crate::token::TokenType::UNKNOWN,
        value: Some(string.to_string()),
    }
}

pub fn parse_expr(string: char) -> Option<Token> {
    match string {
        '=' => Some(Token {
            token_type: crate::token::TokenType::EQUAL,
            value: None,
        }),
        '-' => Some(Token {
            token_type: crate::token::TokenType::REMOVE,
            value: None,
        }),
        '+' => Some(Token {
            token_type: crate::token::TokenType::ADD,
            value: None,
        }),
        _ => None,
    }
}
