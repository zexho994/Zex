use ast_node_type::*;
use token::*;
use ast_node::*;

use super::*;

pub fn parse_tokens_to_ast(tokens: &mut Tokens) -> Option<AstNode> {
    match_program(tokens)
}

fn match_program(tokens: &mut Tokens) -> Option<AstNode> {
    let mut ast_root = new_ast();
    match tokens.peek() {
        Some(t) => match t._type {
            TokenType::Int => {
                ast_root.add_child(match_int_declare(tokens).unwrap());
                Option::Some(ast_root)
            }
            _ => panic!("match program failed"),
        },
        None => Some(ast_root),
    }
}

fn match_int_declare(tokens: &mut Tokens) -> Option<AstNode> {
    let mut ast_node: AstNode;

    // match 'int'
    match tokens.peek() {
        Some(_t) => match _t._type {
            TokenType::Int => {
                tokens.read();
            }
            _ => panic!("match int failed"),
        },
        None => return None,
    }

    // match id
    match tokens.peek() {
        Some(_t) => match _t._type {
            TokenType::Identifier => {
                ast_node = new_ast_node(AstNodeType::IntDeclaration, tokens.read().unwrap().text)
            }
            _ => panic!("match id failed"),
        },
        None => return None,
    }

    // match '='
    match tokens.peek() {
        Some(_t) => match _t._type {
            TokenType::Assignment => {
                tokens.read();
            }
            _ => panic!("match assignment failed"),
        },
        None => return None,
    }

    // match expr
    match tokens.peek() {
        Some(_t) => match match_add_expr(tokens) {
            Some(t) => ast_node.add_child(t),
            None => return Option::Some(ast_node),
        },
        None => return Option::Some(ast_node),
    }

    Option::Some(ast_node)
}

/// <addExpr> ::= <mulExpr> | <mulExpr> '+' <addExpr>
pub fn match_add_expr(tokens: &mut Tokens) -> Option<AstNode> {
    let mut child = match_mul_expr(tokens).unwrap();
    loop {
        match tokens.peek() {
            Some(t1) => match t1._type {
                TokenType::Plus => {
                    tokens.read().unwrap(); // t3 = '+'
                    match match_add_expr(tokens) {
                        Some(t2) => {
                            let mut r = new_ast_node(AstNodeType::Additive, String::from("+"));
                            r.add_child(child);
                            r.add_child(t2);
                            child = r;
                        }
                        None => panic!("match add expr failed, the child2 it not be null"),
                    }
                }
                _ => panic!("match add expr failed,_t1 is {:?}", t1),
            },
            None => break,
        }
    }
    Option::Some(child)
}

/// <mulExpr> ::= <primary> | <primary> '*' <mulExpr>
pub fn match_mul_expr(tokens: &mut Tokens) -> Option<AstNode> {
    let mut child = match_primary(tokens).unwrap();
    loop {
        match tokens.peek() {
            Some(t1) => match t1._type {
                TokenType::Star => {
                    tokens.read().unwrap();
                    match match_primary(tokens) {
                        Some(t2) => {
                            let mut r = new_ast_node(AstNodeType::Multiplicative, "*".to_string());
                            r.add_child(child);
                            r.add_child(t2);
                            child = r;
                        }
                        None => panic!("match mul expr error"),
                    }
                }
                _ => break,
            },
            None => break,
        }
    }
    Option::Some(child)
}

/// <primary> ::= int | Identifier
pub fn match_primary(tokens: &mut Tokens) -> Option<AstNode> {
    let node: AstNode;
    match tokens.peek() {
        Some(t1) => match t1._type {
            TokenType::Number => {
                let t2 = tokens.read().unwrap();
                node = new_ast_node(AstNodeType::IntLiteral, t2.text);
            }
            TokenType::Identifier => {
                let t2 = tokens.read().unwrap();
                node = new_ast_node(AstNodeType::Identifier, t2.text);
            }
            _ => panic!("match primary failed, type is {:?}", t1),
        },
        None => return None,
    }
    Option::Some(node)
}
