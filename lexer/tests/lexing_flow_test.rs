extern crate lexer;

use lexer::lexing;
use lexer::token::token_type::*;

#[test]
fn lexing_flow() {
    let s = String::from("i int if num > 01; {} echo - -> ( ) class cla clas fn ");
    let tokens = lexing(s.as_str().to_string());
    assert!(tokens.count() == 18);
    match tokens.get_child_idx(0).unwrap().get_type() {
        TokenType::Identifier => {}
        _ => panic!("parse s = {} failed", s),
    }
    match tokens.get_child_idx(1).unwrap().get_type() {
        TokenType::Int => {}
        _ => panic!("parse s = {} failed", s),
    }
    match tokens.get_child_idx(2).unwrap().get_type() {
        TokenType::If => {}
        _ => panic!("parse s = {} failed", s),
    }
    match tokens.get_child_idx(3).unwrap().get_type() {
        TokenType::Identifier => {}
        _ => panic!("parse s = {} failed", s),
    }
    match tokens.get_child_idx(4).unwrap().get_type() {
        TokenType::Gt => {}
        _ => panic!("parse s = {} failed", s),
    }
    match tokens.get_child_idx(5).unwrap().get_type() {
        TokenType::IntLiteral => {}
        _ => panic!("parse s = {} failed", s),
    }
    match tokens.get_child_idx(6).unwrap().get_type() {
        TokenType::SemiColon => {}
        _ => panic!("parse s = {} failed", s),
    }
    match tokens.get_child_idx(7).unwrap().get_type() {
        TokenType::LeftBrace => {}
        _ => panic!("parse s = {} failed", s),
    }
    match tokens.get_child_idx(8).unwrap().get_type() {
        TokenType::RightBrace => {}
        _ => panic!("parse s = {} failed", s),
    }
    match tokens.get_child_idx(9).unwrap().get_type() {
        TokenType::Echo => {}
        _ => panic!("parse s = {} failed", s),
    }
    match tokens.get_child_idx(10).unwrap().get_type() {
        TokenType::Minus => {}
        _ => panic!("parse s = {} failed", s),
    }
    match tokens.get_child_idx(11).unwrap().get_type() {
        TokenType::Arrow => {}
        _ => panic!("parse s = {} failed", s),
    }
    match tokens.get_child_idx(12).unwrap().get_type() {
        TokenType::LeftBracket => {}
        _ => panic!("parse s = {} failed", s),
    }
    match tokens.get_child_idx(13).unwrap().get_type() {
        TokenType::RightBracket => {}
        _ => panic!("parse s = {} failed", s),
    }
    match tokens.get_child_idx(14).unwrap().get_type() {
        TokenType::Class => {}
        _ => panic!("parse s = {} failed", s),
    }
    match tokens.get_child_idx(15).unwrap().get_type() {
        TokenType::Identifier => {}
        _ => panic!("parse s = {} failed", s),
    }
    match tokens.get_child_idx(16).unwrap().get_type() {
        TokenType::Identifier => {}
        _ => panic!("parse s = {} failed", s),
    }
    match tokens.get_child_idx(17).unwrap().get_type() {
        TokenType::Fn => {}
        _ => panic!("parse s = {:?} failed", tokens.get_child_idx(17)),
    }
}

#[test]
fn tokens_set_position() {
    let str1 = String::from("int num=1; ");
    let mut tokens = lexing(str1);
    assert!(tokens.position() == 0);
    tokens.set_position(1);
    assert!(tokens.position() == 1);
    tokens.set_position(4);
    assert!(tokens.position() == 4);
}
