use crate::ast_node::*;
use crate::ast_node_type::*;
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

/// <statements> ::= <blockStm> | <statement> | <statement> <statements>
fn match_statements(tokens: &mut Tokens) -> Option<AstNode> {
    print_parse_more2_info(
        "match statements,token is ",
        tokens.peek(),
        tokens.position(),
    );
    let mut ast_node = AstNode::new(AstNodeType::Statements, "");

    while tokens.peek().is_some() {
        if let TokenType::RightBrace = tokens.peek().unwrap().get_type() {
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
    print_parse_more2_info(
        "match block statement,token is ",
        tokens.peek(),
        tokens.position(),
    );
    let mut ast_node = AstNode::new(AstNodeType::BlockStmt, "");
    if let TokenType::LeftBrace = tokens.peek().unwrap().get_type() {
        tokens.read();
        if let Some(node) = match_statements(tokens) {
            ast_node.add_child(node);
        }
        if let TokenType::RightBrace = tokens.peek().unwrap().get_type() {
            tokens.read();
        } else {
            panic!("match block right brace error,tokens : {:?}", tokens);
        }
    } else {
        return None;
    }

    Option::Some(ast_node)
}

/// 语句类型：分配声明语句，表达式语句，赋值语句
/// <statement> ::= <declare> | <expressionStm> | <assignmentStm>
fn match_statement(tokens: &mut Tokens) -> Option<AstNode> {
    print_parse_more2_info(
        "match statement,token is ",
        tokens.peek(),
        tokens.position(),
    );
    let mut ast_node = AstNode::new(AstNodeType::Statement, "");
    if let Some(node) = match_echo(tokens) {
        ast_node.add_child(node);
    } else if let Some(node) = match_declare(tokens) {
        ast_node.add_child(node);
    } else if let Some(node) = match_expr_stm(tokens) {
        ast_node.add_child(node);
    } else if let Some(node) = match_assignment_stm(tokens) {
        ast_node.add_child(node);
    } else {
        panic!("match statement error, tokens is {:?}", tokens);
    }

    if !match_semicolon(tokens) {
        panic!("match statement,要以分号结束");
    }
    Option::Some(ast_node)
}

/// <echo> ::= echo ( <id> | <intLiteral> )
fn match_echo(tokens: &mut Tokens) -> Option<AstNode> {
    print_parse_more2_info("match echo,token is ", tokens.peek(), tokens.position());
    if let TokenType::Echo = tokens.peek().unwrap().get_type() {
        tokens.read();
        let mut node = AstNode::new(AstNodeType::Echo, "");
        match tokens.peek().unwrap().get_type() {
            TokenType::Identifier => {
                let t = tokens.read().unwrap();
                let child = AstNode::new(AstNodeType::Identifier, t.get_text().as_str());
                node.add_child(child);
            }
            TokenType::IntLiteral => {
                let t = tokens.read().unwrap();
                let child = AstNode::new(AstNodeType::IntLiteral, t.get_text().as_str());
                node.add_child(child);
            }
            _ => panic!(
                "echo error, token is {:?}, pos is {}",
                tokens.peek(),
                tokens.position()
            ),
        }

        return Option::Some(node);
    }

    None
}

/// todo 声明语句现在提供变量声明，以后还有方法声明、类声明
/// <declare> ::= <varDeclare> ;
fn match_declare(tokens: &mut Tokens) -> Option<AstNode> {
    print_parse_more2_info("match declare,token is ", tokens.peek(), tokens.position());
    let pos = tokens.position();
    if let Some(n) = match_var_declare(tokens) {
        Some(n)
    } else {
        tokens.set_position(pos);
        None
    }
}

/// <varDeclare> ::= <type> <id> <assign> <exprStm> | <type> <id>
fn match_var_declare(tokens: &mut Tokens) -> Option<AstNode> {
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

    return Some(node);
}

/// <type> ::= int
fn match_type(tokens: &mut Tokens) -> Option<AstNode> {
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
fn match_id(tokens: &mut Tokens) -> Option<AstNode> {
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

fn match_assignment(tokens: &mut Tokens) -> Option<AstNode> {
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

    Option::Some(node)
}

/// <exprStm> ::= <addExpr>
fn match_expr_stm(tokens: &mut Tokens) -> Option<AstNode> {
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
fn match_add_expr(tokens: &mut Tokens) -> Option<AstNode> {
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
fn match_mul_expr(tokens: &mut Tokens) -> Option<AstNode> {
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
fn match_primary(tokens: &mut Tokens) -> Option<AstNode> {
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

fn match_semicolon(tokens: &mut Tokens) -> bool {
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

// fn print_parse_info(msg: &str) {
//     println!("[info][parse] {} ", msg);
// }

// fn print_parse_more1_info<T: std::fmt::Debug>(msg: &str, t: T) {
//     println!("[info][parse] {}, {:?}", msg, t);
// }

fn print_parse_more2_info<T: std::fmt::Debug, K: std::fmt::Debug>(_msg: &str, _t1: T, _t2: K) {
    // println!("[info][parse] {}, {:?}, {:?}", msg, t1, t2);
}
