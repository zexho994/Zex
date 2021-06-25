use ast_node::*;
use ast_node_type::*;
use std::collections::HashMap;
use token::*;

use super::*;

/// <program -> <statement>+ ;
/// <statement> -> <intDeclare> | <expressionStm> | <assignmentStm> ;
/// <intDeclare> -> int <id> <assignment> <expr> ';' ;
/// <expressionStm> -> <addExpr>
/// <assignmentStm> -> <id> <assignment> <exprStm>
/// <id> -> ([a-z][A-Z])* ;
/// <addExpr> -> <mulExpr> | <mulExpr> '+' <addExpr> ;
/// <mulExpr> -> <primary> | <primary> '*' <mulExpr> ;
/// <primary> -> <id> | <intLiteral>
pub fn parse_to_ast(tokens: &mut Tokens) -> Option<i32> {
    let mut ast_root = new_ast();
    while tokens.pos < tokens.count() {
        let mut c = match_int_declare(tokens);
        match c {
            None => {
                c = match_assignment(tokens);
                match c {
                    None => {
                        c = match_expr_stm(tokens);
                    }
                    Some(_) => {}
                }
            }
            Some(_) => {}
        }
        ast_root.add_child(c.unwrap());
    }
    println!("====> ast root: {:?}", ast_root);
    calculate_prog(&mut ast_root)
}

fn calculate_prog(ast_root: &mut AstNode) -> Option<i32> {
    let mut var_map: HashMap<String, i32> = HashMap::new();
    let output: i32 = 0;
    for stmt in ast_root._child.iter_mut() {
        // 根据不同的type进行不同的计算
        match stmt._type {
            AstNodeType::IntDeclaration => {
                calculate_int_declare(stmt, &mut var_map);
            }
            AstNodeType::AssignmentStmt => {}
            AstNodeType::ExpressionStmt => {}
            _ => panic!("not impl more type"),
        }
    }
    Option::Some(output)
}

/// 计算Ast的值
fn calculate_int_declare(ast: &mut AstNode, var_map: &mut HashMap<String, i32>) -> i32 {
    match ast._type {
        AstNodeType::IntDeclaration => {
            match var_map.contains_key(&ast._text) {
                // 变量存在
                true => {
                    let old = *var_map.get(&ast._text).unwrap();
                    let l = match ast.get_child(0) {
                        Some(node) => calculate_int_declare(node, var_map),
                        None => 0,
                    };
                    let r = match ast.get_child(1) {
                        Some(node) => calculate_int_declare(node, var_map),
                        None => 0,
                    };
                    var_map.insert(ast._text.as_str().to_string(), old + l + r);
                    old + l + r
                }
                // 变量不存在
                false => {
                    let l = match ast.get_child(0) {
                        Some(node) => calculate_int_declare(node, var_map),
                        None => 0,
                    };
                    let r = match ast.get_child(1) {
                        Some(node) => calculate_int_declare(node, var_map),
                        None => 0,
                    };
                    var_map.insert(ast._text.as_str().to_string(), l + r);
                    l + r
                }
            }
        }
        AstNodeType::Additive => {
            let l = match ast.get_child(0) {
                Some(node) => calculate_int_declare(node, var_map),
                None => 0,
            };
            let r = match ast.get_child(1) {
                Some(node) => calculate_int_declare(node, var_map),
                None => 0,
            };
            l + r
        }
        AstNodeType::Multiplicative => {
            let l = match ast.get_child(0) {
                Some(node) => calculate_int_declare(node, var_map),
                None => 0,
            };
            let r = match ast.get_child(1) {
                Some(node) => calculate_int_declare(node, var_map),
                None => 1,
            };
            l * r
        }
        AstNodeType::IntLiteral => ast._text.parse().unwrap(),
        _ => panic!("calculate error, p is {:?}", ast),
    }
}

/// <intDeclare> ::= int <id> <assignment> <expr> ';' ;
pub fn match_int_declare(tokens: &mut Tokens) -> Option<AstNode> {
    let mut ast_node: AstNode;
    let pos_cached = tokens.position();

    // match 'int'
    match tokens.peek() {
        Some(p) => match p._type {
            TokenType::Int => {
                tokens.read();
            }
            _ => return None,
            // _ => panic!("match int declaration error,tokens: {:?}", tokens),
        },
        // None => return None,
        None => panic!("match int declaration error,tokens: {:?}", tokens),
    }

    // match id
    match tokens.peek() {
        Some(p) => match p._type {
            TokenType::Identifier => {
                ast_node = new_ast_node(
                    AstNodeType::IntDeclaration,
                    tokens.read().unwrap().text.clone(),
                )
            }
            _ => panic!("match id failed"),
        },
        // None => return None,
        None => panic!("match int declaration error,tokens: {:?}", tokens),
    }

    // match '='
    match tokens.peek() {
        Some(p) => match p._type {
            TokenType::Assignment => {
                tokens.read();
            }
            _ => {
                // 因为之前read()了,现在需要回溯归位
                tokens.set_position(pos_cached);
                return None;
            }
        },
        None => panic!("match assignment failed"),
    }

    // match expr
    match tokens.peek() {
        Some(_t) => match match_add_expr(tokens) {
            Some(t) => ast_node.add_child(t),
            None => return Option::Some(ast_node),
        },
        // None => return Option::Some(ast_node),
        None => panic!("match int declaration error,tokens: {:?}", tokens),
    }

    // println!("match int declaration, tokens: {:?}", tokens);
    match tokens.read().unwrap()._type {
        TokenType::SemiColon => {}
        _ => panic!(""),
    }
    Option::Some(ast_node)
}

/// <assignment> ::= <id> <assignment> <exprStm> ;
/// for example:
/// a = 1 + 1 * 2;
///
/// todo: a += 1; a -= 1; a*= 1, a /= 1;
pub fn match_assignment(tokens: &mut Tokens) -> Option<AstNode> {
    let mut ast_node: AstNode;
    let pos_cached = tokens.position();

    // match id
    match tokens.read() {
        Some(t) => match t._type {
            TokenType::Identifier => {
                ast_node = new_ast_node(AstNodeType::AssignmentStmt, t.text.clone());
            }
            _ => {
                tokens.set_position(pos_cached);
                return None;
            }
        },
        None => {
            tokens.set_position(pos_cached);
            return None;
        }
    }

    match tokens.read() {
        Some(t) => match t._type {
            TokenType::Assignment => {}
            _ => {
                tokens.set_position(pos_cached);
                return None;
            }
        },
        None => panic!("match assignment failed"),
    }

    ast_node.add_child(match_add_expr(tokens).unwrap());

    // match ";"
    match tokens.read().unwrap()._type {
        TokenType::SemiColon => {}
        _ => panic!("match semicolon failed"),
    }

    Option::Some(ast_node)
}

/// <exprStm> ::= <addExpr>
pub fn match_expr_stm(tokens: &mut Tokens) -> Option<AstNode> {
    let mut ast_node = new_ast_node(AstNodeType::ExpressionStmt, "".to_string());
    ast_node.add_child(match_add_expr(tokens).unwrap());
    // println!("match int declaration, tokens: {:?}", tokens);
    match tokens.read().unwrap()._type {
        TokenType::SemiColon => Option::Some(ast_node),
        _ => panic!("match expr stm error, token should be semicolon"),
    }
}

/// <addExpr> ::= <mulExpr> | <mulExpr> '+' <addExpr>
pub fn match_add_expr(tokens: &mut Tokens) -> Option<AstNode> {
    // println!("match add expr, tokens: {:?}", tokens);
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
                TokenType::SemiColon => {
                    break;
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
    // println!("match mul expr, tokens: {:?}",tokens);
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
                TokenType::Plus | TokenType::SemiColon => {
                    break;
                }
                _ => panic!("match mul expr error,t1 is {:?}", t1),
            },
            None => break,
        }
    }
    Option::Some(child)
}

/// <primary> ::= int | Identifier
pub fn match_primary(tokens: &mut Tokens) -> Option<AstNode> {
    // println!("====> match primary, tokens: {:?}", tokens);
    let node: AstNode;
    match tokens.peek() {
        Some(t1) => match t1._type {
            TokenType::IntLiteral => {
                let t2 = tokens.read().unwrap();
                node = new_ast_node(AstNodeType::IntLiteral, t2.text.clone());
            }
            TokenType::Identifier => {
                let t2 = tokens.read().unwrap();
                node = new_ast_node(AstNodeType::Identifier, t2.text.clone());
            }
            _ => panic!("match primary failed, type is {:?}", t1),
        },
        None => return None,
    }
    Option::Some(node)
}
