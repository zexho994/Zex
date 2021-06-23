use ast_node::*;
use ast_node_type::*;
use std::collections::HashMap;
use token::*;

use super::*;

pub fn parse_to_ast(tokens: &mut Tokens) -> i32 {
    let mut ast_root = new_ast();
    tokens.check_peek().expect("Error parsing");
    match tokens.peek().unwrap()._type {
        TokenType::Int => {
            ast_root.add_child(match_int_declare(tokens).unwrap());
        }
        _ => panic!("match program failed"),
    }
    println!("ast root: {:?}", ast_root);
    let mut var_map: HashMap<String, i32> = HashMap::new();
    calculate(&mut ast_root.get_child(0).unwrap(), &mut var_map)
}

pub fn calculate(ast: &mut AstNode, var_map: &mut HashMap<String, i32>) -> i32 {
    match ast._type {
        AstNodeType::IntDeclaration => {
            match var_map.contains_key(&ast._text) {
                // 变量存在
                true => {
                    let old = *var_map.get(&ast._text).unwrap();
                    let l = match ast.get_child(0) {
                        Some(node) => calculate(node, var_map),
                        None => 0,
                    };
                    let r = match ast.get_child(1) {
                        Some(node) => calculate(node, var_map),
                        None => 0,
                    };
                    var_map.insert(ast._text.as_str().to_string(), old + l + r);
                    old + l + r
                }
                // 变量不存在
                false => {
                    let l = match ast.get_child(0) {
                        Some(node) => calculate(node, var_map),
                        None => 0,
                    };
                    let r = match ast.get_child(1) {
                        Some(node) => calculate(node, var_map),
                        None => 0,
                    };
                    var_map.insert(ast._text.as_str().to_string(), l + r);
                    l + r
                }
            }
        }
        AstNodeType::Additive => {
            let l = match ast.get_child(0) {
                Some(node) => calculate(node, var_map),
                None => 0,
            };
            let r = match ast.get_child(1) {
                Some(node) => calculate(node, var_map),
                None => 0,
            };
            l + r
        }
        AstNodeType::Multiplicative => {
            let l = match ast.get_child(0) {
                Some(node) => calculate(node, var_map),
                None => 0,
            };
            let r = match ast.get_child(1) {
                Some(node) => calculate(node, var_map),
                None => 1,
            };
            l * r
        }
        AstNodeType::IntLiteral => ast._text.parse().unwrap(),
        _ => panic!("calculate error, p is {:?}", ast),
    }
}

/// <intDeclare> :== int <id> <assignment> <expr>
fn match_int_declare(tokens: &mut Tokens) -> Option<AstNode> {
    let mut ast_node: AstNode;

    // match 'int'
    tokens.check_peek().expect("Error match int");
    match tokens.peek().unwrap()._type {
        TokenType::Int => {
            tokens.read();
        }
        _ => panic!("match int failed"),
    }

    // match id
    tokens.check_peek().expect("Error match id");
    match tokens.peek().unwrap()._type {
        TokenType::Identifier => {
            ast_node = new_ast_node(AstNodeType::IntDeclaration, tokens.read().unwrap().text)
        }
        _ => panic!("match id failed"),
    }

    // match '='
    tokens.check_peek().expect("Error match =");
    match tokens.peek().unwrap()._type {
        TokenType::Assignment => {
            tokens.read();
        }
        _ => panic!("match assignment failed"),
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
            TokenType::IntLiteral => {
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
