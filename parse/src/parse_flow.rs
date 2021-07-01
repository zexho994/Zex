use super::*;
use ast_node::*;
use ast_node_type::*;
use lexer::token::{token_struct::*, token_type::*};

/// <program> ::= <statements>
pub fn match_program(tokens: &mut Tokens) -> Option<AstNode> {
    let mut prog_node = AstNode {
        _type: AstNodeType::Program,
        ..Default::default()
    };

    if let Some(n) = match_statements(tokens) {
        prog_node.add_child(n);
    } else {
        return None;
    }

    Option::Some(prog_node)
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
    println!("match block statement");
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
        println!("match block statement failed");
        return None;
    }

    Option::Some(ast_node)
}

/// 语句类型：分配声明语句，表达式语句，赋值语句
/// <statement> ::= <declare> | <expressionStm> | <assignmentStm>
fn match_statement(tokens: &mut Tokens) -> Option<AstNode> {
    println!(
        "match statement, token is {:?}, pos is {}",
        tokens.peek(),
        tokens.pos
    );
    let mut ast_node: AstNode = AstNode {
        _type: AstNodeType::Statement,
        ..Default::default()
    };

    if let Some(node) = match_declare(tokens) {
        ast_node.add_child(node);
        return Option::Some(ast_node);
    }

    if let Some(node) = match_expr_stm(tokens) {
        ast_node.add_child(node);
        return Option::Some(ast_node);
    }

    if let Some(node) = match_assignment_stm(tokens) {
        ast_node.add_child(node);
        return Option::Some(ast_node);
    }
    panic!("match statement error, tokens is {:?}", tokens);
}

/// todo 声明语句现在提供变量声明，以后还有方法声明、类声明
/// <declare> ::= <varDeclare>
fn match_declare(tokens: &mut Tokens) -> Option<AstNode> {
    println!("match declare");
    let pos = tokens.position();
    if let Some(n) = match_var_declare(tokens) {
        Some(n)
    } else {
        println!("match declare failed");
        tokens.set_position(pos);
        None
    }
}

/// <varDeclare> ::= <type> <id> <assign> <exprStm> ; | <type> <id> ;
fn match_var_declare(tokens: &mut Tokens) -> Option<AstNode> {
    println!("match var declare");
    let mut node = AstNode {
        _type: AstNodeType::VarDeclare,
        ..Default::default()
    };
    if let Some(t) = match_type(tokens) {
        node.add_child(t);
    } else {
        return None;
    }

    if let Some(t) = match_id(tokens) {
        node.add_child(t);
    } else {
        return None;
    }

    if let Some(t) = match_assignment(tokens) {
        node.add_child(t);
        if let Some(t) = match_expr_stm(tokens) {
            node.add_child(t);
        } else {
            panic!("match var declare error, assignment 符号后面expr stm 不能为空");
        }
    }

    if !match_semicolon(tokens) {
        panic!("match var declare error, expr stm后面以分号;结尾");
    }

    return Some(node);
}

/// <type> ::= int
fn match_type(tokens: &mut Tokens) -> Option<AstNode> {
    println!("match type");
    match tokens.peek().unwrap()._type {
        TokenType::Int => {
            let t = tokens.read().unwrap();
            Option::Some(AstNode {
                _type: AstNodeType::Int,
                _text: t.text.clone(),
                ..Default::default()
            })
        }
        _ => None,
    }
}

/// <id> = id
fn match_id(tokens: &mut Tokens) -> Option<AstNode> {
    println!("match id");
    if let TokenType::Identifier = tokens.peek().unwrap()._type {
        let t = tokens.read().unwrap();
        return Option::Some(AstNode {
            _type: AstNodeType::Identifier,
            _text: t.text.clone(),
            ..Default::default()
        });
    }
    None
}

fn match_assignment(tokens: &mut Tokens) -> Option<AstNode> {
    println!("match assignment");
    match tokens.peek().unwrap()._type {
        TokenType::Assignment => Option::Some(AstNode {
            _type: AstNodeType::AssignmentSymbol,
            _text: tokens.read().unwrap().text.to_string(),
            ..Default::default()
        }),
        _ => {
            println!("match assignment failed");
            None
        }
    }
}

/// <assignment> ::= <id> <assignment> <exprStm> ;
/// for example:
/// a = 1 + 1 * 2;
///
/// todo: a += 1; a -= 1; a*= 1, a /= 1;
fn match_assignment_stm(tokens: &mut Tokens) -> Option<AstNode> {
    println!("match assignment stm");
    let mut node = AstNode {
        _type: AstNodeType::AssignmentStmt,
        ..Default::default()
    };
    let pos = tokens.position();
    if let Some(n) = match_id(tokens) {
        node.add_child(n)
    } else {
        tokens.set_position(pos);
        return None;
    }

    if let Some(n) = match_assignment(tokens) {
        node.add_child(n);
    } else {
        panic!("match assignment stm error");
    }

    node.add_child(match_add_expr(tokens).unwrap());
    if let TokenType::SemiColon = tokens.read().unwrap()._type {}

    Option::Some(node)
}

/// <exprStm> ::= <addExpr>
fn match_expr_stm(tokens: &mut Tokens) -> Option<AstNode> {
    println!("match expr stm");
    let mut ast_node = AstNode {
        _type: AstNodeType::ExpressionStmt,
        ..Default::default()
    };
    if let Some(n) = match_add_expr(tokens) {
        ast_node.add_child(n);
    } else {
        return None;
    }
    Some(ast_node)
}

/// <addExpr> ::= <mulExpr> | <mulExpr> '+' <addExpr>
fn match_add_expr(tokens: &mut Tokens) -> Option<AstNode> {
    println!("match add expr");
    let mut child: AstNode;
    if let Some(n) = match_mul_expr(tokens) {
        child = n;
    } else {
        return None;
    }
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
    println!("match mul expr");
    let mut child: AstNode;
    if let Some(n) = match_primary(tokens) {
        child = n;
    } else {
        return None;
    }
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
    println!(
        "match primary, token: {:?}, pos is {}",
        tokens.peek(),
        tokens.pos
    );
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
            _ => return None,
        },
        None => return None,
    }
    Option::Some(node)
}

fn match_semicolon(tokens: &mut Tokens) -> bool {
    println!(
        "match semicolon, token is {:?}, pos is {}",
        tokens.peek(),
        tokens.pos
    );
    if let TokenType::SemiColon = tokens.peek().unwrap()._type {
        tokens.read();
        true
    } else {
        false
    }
}
