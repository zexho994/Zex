use crate::ast_node::*;
use lexer::token::{token_struct::*, token_type::*};

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
