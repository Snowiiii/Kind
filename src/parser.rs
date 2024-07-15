use std::{iter::Peekable, slice::Iter};

use crate::token::{Token, TokenType};

pub enum Node {
    Expr(NodeExpr), // Encapsulate expression details
    Var(NodeVar),
    Exit(NodeExit), // Encapsulate statement details (e.g., return, exit)
}

pub struct NodeVar {
    name: String,
    expr: NodeExpr,
}

impl NodeVar {
    pub fn parse(tokens: &mut Peekable<Iter<Token>>, vars: &Vec<NodeVar>) -> Result<Self, String> {
        if let Some(token) = tokens.next() {
            if token.token_type == TokenType::UNKNOWN {
                // name = save <name>
                let name = token.value.clone().unwrap();
                if let Some(token) = tokens.next() {
                    if token.token_type == TokenType::EQUAL {
                        // equal = save <name> =
                        let expr = NodeExpr::parse(tokens, vars)?;
                        // equal = save <name> = <expr>

                        // Finally check if var is declared twice
                        for var in vars {
                            if var.name == name {
                                return Err(format!("Variable {} is declared twice", name));
                            }
                        }
                        return Ok(Self { name, expr });
                    }
                }
            } else {
                return Err(String::from("Invalid Variable: Expected variable name"));
            }
        }
        Err("Invalid Variable, Usage: save <name> = <var>".to_string())
    }
}

pub struct NodeIntLit {
    pub value: String,
}

impl NodeIntLit {
    pub fn parse(tokens: &mut Peekable<Iter<Token>>, vars: &Vec<NodeVar>) -> Result<Self, String> {
        if let Some(token) = tokens.next() {
            match token.token_type {
                TokenType::INTLIT => {
                    // let value = token.value.as_ref().unwrap().parse::<i32>().unwrap();
                    Ok(NodeIntLit {
                        value: token.value.clone().expect("Failed to get value from Token"),
                    })
                }
                TokenType::UNKNOWN => {
                    let name = token.value.clone().unwrap();
                    for var in vars {
                        if name == var.name {
                            if var.expr.expr_type == ExpressionType::INT {
                                return Ok(Self {
                                    value: var.expr.value.clone(),
                                });
                            } else {
                                return Err(format!(
                                    "Expected Int variable found: {:?}",
                                    var.expr.expr_type
                                ));
                            }
                        }
                    }
                    Err(String::from("Invalid Unknown expression"))
                }
                _ => Err(String::from("Invalid expression: expected integer literal")),
            }
        } else {
            Err(String::from(
                "Unexpected end of input while parsing expression",
            ))
        }
    }
}

#[derive(Clone)]
pub struct NodeExpr {
    pub value: String,
    pub expr_type: ExpressionType,
}

#[derive(Clone, PartialEq, Debug)]
pub enum ExpressionType {
    BOOLEAN,
    INT,
    FLOAT,
    STRING,
}

impl NodeExpr {
    pub fn parse(tokens: &mut Peekable<Iter<Token>>, vars: &Vec<NodeVar>) -> Result<Self, String> {
        if let Some(token) = tokens.next() {
            dbg!(token);
            match token.token_type {
                TokenType::INTLIT => {
                    // let value = token.value.as_ref().unwrap().parse::<i32>().unwrap();
                    Ok(NodeExpr {
                        value: token.value.clone().expect("Failed to get value from Token"),
                        expr_type: ExpressionType::INT,
                    })
                }
                TokenType::BOOLEAN => Ok(NodeExpr {
                    value: token.value.clone().expect("Failed to get value from Token"),
                    expr_type: ExpressionType::BOOLEAN,
                }),
                TokenType::UNKNOWN => {
                    let name = token.value.clone().unwrap();
                    for var in vars {
                        if name == var.name {
                            return Ok(var.expr.clone());
                        }
                    }
                    Err(String::from("Invalid Unknown expression"))
                }
                _ => Err(String::from("Invalid expression")),
            }
        } else {
            Err(String::from(
                "Unexpected end of input while parsing expression",
            ))
        }
    }
}

pub struct NodeExit {
    pub expr: NodeIntLit,
}

impl NodeExit {
    pub fn parse(tokens: &mut Peekable<Iter<Token>>, vars: &Vec<NodeVar>) -> Result<Self, String> {
        let token = tokens.peek().unwrap();
        let expr = if token.token_type == TokenType::SEMICOLON {
            // User can pass in error code, when not we are using 0
            NodeIntLit {
                value: String::from("0"),
            }
        } else {
            NodeIntLit::parse(tokens, vars)?
        };
        Ok(NodeExit { expr })
    }
}

pub fn parse_tokens(tokens: Vec<Token>) -> Result<Vec<Node>, String> {
    let mut tokens = tokens.iter().peekable();
    let mut vars = Vec::new();
    let mut nodes = Vec::new();

    while let Some(token) = tokens.next() {
        match token.token_type {
            TokenType::RETURN => todo!(), // Handle return statement (optional)
            TokenType::EXIT => match NodeExit::parse(&mut tokens, &vars) {
                Ok(exit_node) => nodes.push(Node::Exit(exit_node)),
                Err(err) => return Err(err),
            },
            TokenType::INTLIT => todo!(), // Handle integer literals outside of exit (optional)
            TokenType::SEMICOLON => {}    // Currently ignored, consider handling semicolons
            TokenType::UNKNOWN => return Err(String::from("Encountered invalid token")),
            TokenType::BOOLEAN => todo!(),
            TokenType::VARIABLE => match NodeVar::parse(&mut tokens, &vars) {
                Ok(var_node) => {
                    vars.push(var_node);
                }
                Err(err) => return Err(err),
            },
            TokenType::EQUAL => todo!(),
            TokenType::ADD => todo!(),
            TokenType::REMOVE => todo!(),
        }
    }
    for var in vars {
        nodes.push(Node::Var(var))
    }

    Ok(nodes) // Parsing successful
}
