use super::flow_statements::*;
use crate::ast_node::*;
use lexer::token::{token_struct::*, token_type::*};

/// <program> ::= <statements>
pub fn match_program(tokens: &mut Tokens) -> Option<AstNode> {
    print_parse_more2_info("match program,token is ", tokens.peek(), tokens.position());
    let mut prog_node = AstNode::new(AstNodeType::Program, "");

    if let Some(n) = match_statements(tokens) {
        prog_node.add_child(n);
    } else {
        return None;
    }

    Option::Some(prog_node)
}

/// <varDeclare> ::= <type> <id> <assign> <exprStm> | <type> <id> ;
pub fn match_var_declare(tokens: &mut Tokens) -> Option<AstNode> {
    print_parse_more2_info(
        "match var declare,token is ",
        tokens.peek(),
        tokens.position(),
    );
    let mut node = AstNode::new(AstNodeType::VarDeclareStmt, "");
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
        panic!("match statement,要以分号结束");
    }
    return Some(node);
}

/// <type> ::= int
pub fn match_type(tokens: &mut Tokens) -> Option<AstNode> {
    print_parse_more2_info("match type,token is ", tokens.peek(), tokens.position());
    match tokens.peek().unwrap().get_type() {
        TokenType::Int => {
            let t = tokens.read().unwrap();
            let node = AstNode::new(AstNodeType::Int, t.get_text().as_str());
            Option::Some(node)
        }
        _ => None,
    }
}

/// <id> = id
pub fn match_id(tokens: &mut Tokens) -> Option<AstNode> {
    print_parse_more2_info(
        "match identifier,token is ",
        tokens.peek(),
        tokens.position(),
    );
    if let TokenType::Identifier = tokens.peek().unwrap().get_type() {
        let t = tokens.read().unwrap();
        let node = AstNode::new(AstNodeType::Identifier, t.get_text().as_str());
        return Option::Some(node);
    }
    None
}

/// a = b
pub fn match_assignment(tokens: &mut Tokens) -> Option<AstNode> {
    print_parse_more2_info(
        "match assignment,token is ",
        tokens.peek(),
        tokens.position(),
    );
    match tokens.peek().unwrap().get_type() {
        TokenType::Assignment => {
            let node = AstNode::new(
                AstNodeType::AssignmentSymbol,
                tokens.read().unwrap().get_text().as_str(),
            );
            Option::Some(node)
        }
        _ => None,
    }
}

/// <assignment> ::= <id> <assignment> <exprStm> ;
/// for example:
/// a = 1 + 1 * 2;
///
/// todo: a += 1; a -= 1; a*= 1, a /= 1;
pub fn match_assignment_stm(tokens: &mut Tokens) -> Option<AstNode> {
    print_parse_more2_info(
        "match assignmentStmt,token is ",
        tokens.peek(),
        tokens.position(),
    );
    let mut node = AstNode::new(AstNodeType::AssignmentStmt, "");
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

    if !match_semicolon(tokens) {
        panic!("match statement,要以分号结束");
    }
    Option::Some(node)
}

/// <exprStm> ::= <addExpr>
pub fn match_expr_stm(tokens: &mut Tokens) -> Option<AstNode> {
    print_parse_more2_info("match expr,token is ", tokens.peek(), tokens.position());
    let mut ast_node = AstNode::new(AstNodeType::ExpressionStmt, "");
    if let Some(n) = match_add_expr(tokens) {
        ast_node.add_child(n);
    } else {
        return None;
    }
    Some(ast_node)
}

/// <addExpr> ::= <mulExpr> | <mulExpr> '+' <addExpr>
pub fn match_add_expr(tokens: &mut Tokens) -> Option<AstNode> {
    print_parse_more2_info("match add,token is ", tokens.peek(), tokens.position());
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
        match tokens.peek().unwrap().get_type() {
            TokenType::Plus => {
                tokens.read();
                if let Some(t) = match_add_expr(tokens) {
                    let mut r = AstNode::new(AstNodeType::Additive, "+");
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
pub fn match_mul_expr(tokens: &mut Tokens) -> Option<AstNode> {
    let pos = tokens.position();
    print_parse_more2_info("match mul,token is ", tokens.peek(), tokens.position());
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
        match tokens.peek().unwrap().get_type() {
            TokenType::Star => {
                tokens.read().unwrap();
                if let Some(p) = match_primary(tokens) {
                    let mut r = AstNode::new(AstNodeType::Multiplicative, "*");
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
            _ => {
                tokens.set_position(pos);
                return None;
            }
        }
    }
    Option::Some(child)
}

/// <primary> ::= int | Identifier
pub fn match_primary(tokens: &mut Tokens) -> Option<AstNode> {
    print_parse_more2_info("match primary,token is ", tokens.peek(), tokens.position());
    let node: AstNode;
    match tokens.peek() {
        Some(t1) => match t1.get_type() {
            TokenType::IntLiteral => {
                let t2 = tokens.read().unwrap();
                node = AstNode::new(AstNodeType::IntLiteral, t2.get_text().as_str());
            }
            TokenType::Identifier => {
                let t2 = tokens.read().unwrap();
                node = AstNode::new(AstNodeType::IntLiteral, t2.get_text().as_str());
            }
            _ => return None,
        },
        None => return None,
    }
    Option::Some(node)
}

pub fn match_semicolon(tokens: &mut Tokens) -> bool {
    print_parse_more2_info(
        "match semicolon,token is ",
        tokens.peek(),
        tokens.position(),
    );
    if let TokenType::SemiColon = tokens.peek().unwrap().get_type() {
        tokens.read();
        true
    } else {
        false
    }
}

pub fn print_parse_more2_info<T: std::fmt::Debug, K: std::fmt::Debug>(msg: &str, t1: T, t2: K) {
    println!("[info][parse] {}, {:?}, {:?}", msg, t1, t2);
}
