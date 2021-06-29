pub mod dfa;
pub mod lexing_flow;
pub mod token;
pub mod utils;
use crate::lexing_flow::*;
use crate::token::token_struct::*;
use crate::token::token_type::*;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_test_1() {
        let str1 = String::from("age >= 15 ;");
        let tokens = lexing(str1);
        assert_eq!(tokens.count(), 4)
    }

    #[test]
    fn parse_test_5() {
        let str1 = String::from("inte num > 01;");
        let tokens = lexing(str1);
        assert!(tokens.data.len() == 5);
    }

    #[test]
    fn parse_test_7() {
        let s = String::from("i int if num > 01;");
        let tokens = lexing(s.as_str().to_string());
        println!("tokens {:?}", tokens);
        assert!(tokens.data.len() == 7);
        match tokens.get_child_idx(1).unwrap()._type {
            TokenType::Int => {}
            _ => panic!("parse s = {} failed", s),
        }
        match tokens.get_child_idx(2).unwrap()._type {
            TokenType::IF => {}
            _ => panic!("parse s = {} failed", s),
        }
    }

    #[test]
    fn parse_test_12() {
        let str1 = String::from("int num=1; ");
        let tokens = lexing(str1);
        println!("tokens is {:?}", tokens);
        assert!(tokens.data.len() == 5);
    }

    #[test]
    fn assignment() {
        let str1 = String::from("=");
        let tokens = lexing(str1);
        assert!(tokens.data.len() == 1);
        match tokens.peek() {
            Some(t) => match t._type {
                TokenType::Assignment => {
                    println!("");
                }
                _ => panic!("token is not an Assignment"),
            },
            None => panic!("parse test fail"),
        }
    }

    #[test]
    fn semi_colon() {
        let str1 = String::from(";");
        let tokens = lexing(str1);
        assert!(tokens.data.len() == 1);
        match tokens.peek() {
            Some(t) => match t._type {
                TokenType::SemiColon => {
                    println!("");
                }
                _ => panic!("token is not a SemiColon"),
            },
            None => panic!("parse test fail"),
        }
    }

    #[test]
    fn parse_test_13() {
        let str1 = String::from(" ");
        let tokens = lexing(str1);
        assert!(tokens.data.len() == 0);
        match tokens.peek() {
            Option::None => {}
            _ => {
                panic!("parse_test_12 failure!")
            }
        }
    }

    #[test]
    fn tokens_read() {
        let str1 = String::from("a");
        let mut tokens = lexing(str1.clone());
        assert!(tokens.count() == 1);
        let ot = tokens.read();
        match ot {
            Some(t) => match t._type {
                TokenType::Identifier => {}
                _ => panic!("token type mismatch, type is {:?}", t),
            },
            None => panic!("token missing"),
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
}

/// 解析入口
pub fn lexing(s: String) -> Tokens {
    let mut i: usize = 0;
    let mut tokens = Tokens {
        data: Vec::new(),
        pos: 0,
    };
    while i < s.chars().count() {
        let (token, state) = get_initial_state(i, s.as_str());
        i = get_full_token(state, i + 1, s.as_str(), token, &mut tokens);
    }
    tokens
}
