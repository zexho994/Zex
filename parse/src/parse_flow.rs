use super::*;
use ast_node::*;
use ast_node_type::*;
use lexer::token::{token_struct::*, token_type::*};

/// <program> ::= <statements> ;
pub fn match_program(tokens: &mut Tokens) -> Option<AstNode> {
    let mut ast_root = new_ast();
    ast_root.add_child(match_statements(tokens).unwrap());
    Option::Some(ast_root)
}

/// <statements> ::= <blockStm> | <statement> | <statement> <statements>
fn match_statements(tokens: &mut Tokens) -> Option<AstNode> {
    let mut ast_node = AstNode {
        _type: AstNodeType::Statements,
        ..Default::default()
    };
    while tokens.peek().is_some() {
        if let TokenType::RightBrace = tokens.peek().unwrap()._type {
            break;
        }

        if let Some(node) = match_block_statement(tokens) {
            ast_node.add_child(node);
            continue;
        }
        if let Some(node) = match_statement(tokens) {
            ast_node.add_child(node);
            continue;
        }
    }

    Option::Some(ast_node)
}

/// <blockStm> ::= { <statements> }
fn match_block_statement(tokens: &mut Tokens) -> Option<AstNode> {
    let mut ast_node = AstNode {
        _type: AstNodeType::BlockStmt,
        ..Default::default()
    };

    if let TokenType::LeftBrace = tokens.peek().unwrap()._type {
        tokens.read();
        if let Some(node) = match_statements(tokens) {
            ast_node.add_child(node);
        }
        if let TokenType::RightBrace = tokens.peek().unwrap()._type {
            tokens.read();
        } else {
            panic!("match block right brace error,tokens : {:?}", tokens);
        }
    } else {
        return None;
    }

    Option::Some(ast_node)
}

/// <statement> -> <intDeclare> | <expressionStm> | <assignmentStm>
fn match_statement(tokens: &mut Tokens) -> Option<AstNode> {
    let mut ast_node: AstNode = AstNode {
        _type: AstNodeType::Statement,
        ..Default::default()
    };

    if let Some(node) = match_int_declare(tokens) {
        ast_node.add_child(node);
    } else if let Some(node) = match_expr_stm(tokens) {
        ast_node.add_child(node);
    } else if let Some(node) = match_assignment(tokens) {
        ast_node.add_child(node);
    } else {
        panic!("match statement error, tokens is {:?}", tokens);
    }

    // println!("match statement success");
    Option::Some(ast_node)
}

/// <intDeclare> ::= int <id> <assignment> <expr> ';' ;
fn match_int_declare(tokens: &mut Tokens) -> Option<AstNode> {
    // println!("match int declare, tokens: {:?}", tokens);
    let mut ast_node: AstNode;
    let pos_cached = tokens.position();

    if let TokenType::Int = tokens.peek().unwrap()._type {
        tokens.read();
    } else {
        return None;
    }

    if let TokenType::Identifier = tokens.peek().unwrap()._type {
        ast_node = new_ast_node(
            AstNodeType::IntDeclaration,
            tokens.read().unwrap().text.clone(),
        )
    } else {
        tokens.set_position(pos_cached);
        return None;
    }

    if let TokenType::Assignment = tokens.peek().unwrap()._type {
        tokens.read();
    } else {
        tokens.set_position(pos_cached);
        return None;
    }
    if tokens.peek().is_none() {
        panic!("match int declaration error,tokens: {:?}", tokens);
    }

    if tokens.peek().is_none() {
        panic!("match int declaration error,tokens: {:?}", tokens);
    }
    match match_add_expr(tokens) {
        Some(t) => ast_node.add_child(t),
        None => return Option::Some(ast_node),
    }

    match tokens.read().unwrap()._type {
        TokenType::SemiColon => {}
        _ => panic!(""),
    }

    // println!(
    //     "match int declaration success, tokens peke is {:?}",
    //     tokens.peek()
    // );

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
        _ => panic!("match expr stm error, token is {:?}", tokens.peek()),
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
                if let Some(t) = match_add_expr(tokens) {
                    let mut r = new_ast_node(AstNodeType::Additive, String::from("+"));
                    r.add_child(child);
                    r.add_child(t);
                    child = r;
                } else {
                    panic!("match add expr failed, the child2 it not be null");
                }
            }
            TokenType::SemiColon | TokenType::RightBrace => {
                break;
            }
            _ => panic!("match add expr failed,token is {:?}", tokens.peek()),
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
                if let Some(p) = match_primary(tokens) {
                    let mut r = new_ast_node(AstNodeType::Multiplicative, "*".to_string());
                    r.add_child(child);
                    r.add_child(p);
                    child = r;
                } else {
                    panic!("match mul expr error");
                }
            }
            TokenType::Plus | TokenType::SemiColon | TokenType::RightBrace => {
                break;
            }
            _ => panic!("match mul expr error,token: {:?}", tokens.peek()),
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
