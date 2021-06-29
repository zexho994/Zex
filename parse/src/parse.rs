use super::*;
use ast_node::*;
use ast_node_type::*;
use calculate;
use std::collections::HashMap;
use token::{token_struct::*, token_type::*};

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
    let mut var_map: HashMap<String, i32> = HashMap::new();
    parse_tokens(tokens, &mut var_map)
}

pub fn parse_tokens(tokens: &mut Tokens, var_map: &mut HashMap<String, i32>) -> Option<i32> {
    let mut ast_root = new_ast();
    while tokens.pos < tokens.count() {
        let mut c = match_int_declare(tokens);
        if c.is_none() {
            c = match_assignment(tokens);
        }
        if c.is_none() {
            c = match_expr_stm(tokens);
        }
        ast_root.add_child(c.unwrap());
    }
    calculate::calculate_prog(&mut ast_root, var_map)
}

/// <intDeclare> ::= int <id> <assignment> <expr> ';' ;
fn match_int_declare(tokens: &mut Tokens) -> Option<AstNode> {
    // println!("match int declare, tokens: {:?}", tokens);
    let mut ast_node: AstNode;
    let pos_cached = tokens.position();

    // match 'int'
    if tokens.peek().is_none() {
        panic!("match int declaration error,tokens: {:?}", tokens);
    }
    match tokens.peek().unwrap()._type {
        TokenType::Int => {
            tokens.read();
        }
        _ => return None,
    }

    // match <id>
    if tokens.peek().is_none() {
        panic!("match int declaration error,tokens: {:?}", tokens);
    }
    match tokens.peek().unwrap()._type {
        TokenType::Identifier => {
            ast_node = new_ast_node(
                AstNodeType::IntDeclaration,
                tokens.read().unwrap().text.clone(),
            )
        }
        _ => panic!("match id failed"),
    }

    // match <assignment>
    if tokens.peek().is_none() {
        panic!("match int declaration error,tokens: {:?}", tokens);
    }
    match tokens.peek().unwrap()._type {
        TokenType::Assignment => {
            tokens.read();
        }
        _ => {
            tokens.set_position(pos_cached);
            return None;
        }
    }

    // match <addExpr>
    if tokens.peek().is_none() {
        panic!("match int declaration error,tokens: {:?}", tokens);
    }
    match match_add_expr(tokens) {
        Some(t) => ast_node.add_child(t),
        None => return Option::Some(ast_node),
    }

    // match semicolon
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
fn match_assignment(tokens: &mut Tokens) -> Option<AstNode> {
    if tokens.pos == tokens.count() {
        return None;
    }
    // println!("match assignment, tokens: {:?}", tokens);
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

    match tokens.read().unwrap()._type {
        TokenType::SemiColon => {}
        _ => panic!("match semicolon failed"),
    }

    Option::Some(ast_node)
}

/// <exprStm> ::= <addExpr>
fn match_expr_stm(tokens: &mut Tokens) -> Option<AstNode> {
    // println!("match expression statement, tokens: {:?}", tokens);
    let mut ast_node = new_ast_node(AstNodeType::ExpressionStmt, "".to_string());
    ast_node.add_child(match_add_expr(tokens).unwrap());
    match tokens.read().unwrap()._type {
        TokenType::SemiColon => Option::Some(ast_node),
        _ => panic!("match expr stm error, token should be semicolon"),
    }
}

/// <addExpr> ::= <mulExpr> | <mulExpr> '+' <addExpr>
fn match_add_expr(tokens: &mut Tokens) -> Option<AstNode> {
    let mut child = match_mul_expr(tokens).unwrap();
    loop {
        if tokens.peek().is_none() {
            break;
        }
        match tokens.peek().unwrap()._type {
            TokenType::Plus => {
                tokens.read();
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
            _ => panic!("match add expr failed,tokens is {:?}", tokens),
        }
    }
    Option::Some(child)
}

/// <mulExpr> ::= <primary> | <primary> '*' <mulExpr>
fn match_mul_expr(tokens: &mut Tokens) -> Option<AstNode> {
    let mut child = match_primary(tokens).unwrap();
    loop {
        if tokens.peek().is_none() {
            break;
        }
        match tokens.peek().unwrap()._type {
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
            _ => panic!("match mul expr error,tokens: {:?}", tokens),
        }
    }
    Option::Some(child)
}

/// <primary> ::= int | Identifier
fn match_primary(tokens: &mut Tokens) -> Option<AstNode> {
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
