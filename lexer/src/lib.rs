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
    fn lexing_flow() {
        let s = String::from("i int if num > 01; {} echo ");
        let tokens = lexing(s.as_str().to_string());
        println!("tokens {:?}", tokens);
        assert!(tokens.data.len() == 10);
        match tokens.get_child_idx(0).unwrap()._type {
            TokenType::Identifier => {}
            _ => panic!("parse s = {} failed", s),
        }
        match tokens.get_child_idx(1).unwrap()._type {
            TokenType::Int => {}
            _ => panic!("parse s = {} failed", s),
        }
        match tokens.get_child_idx(2).unwrap()._type {
            TokenType::IF => {}
            _ => panic!("parse s = {} failed", s),
        }
        match tokens.get_child_idx(3).unwrap()._type {
            TokenType::Identifier => {}
            _ => panic!("parse s = {} failed", s),
        }
        match tokens.get_child_idx(4).unwrap()._type {
            TokenType::GT => {}
            _ => panic!("parse s = {} failed", s),
        }
        match tokens.get_child_idx(5).unwrap()._type {
            TokenType::IntLiteral => {}
            _ => panic!("parse s = {} failed", s),
        }
        match tokens.get_child_idx(6).unwrap()._type {
            TokenType::SemiColon => {}
            _ => panic!("parse s = {} failed", s),
        }
        match tokens.get_child_idx(7).unwrap()._type {
            TokenType::LeftBrace => {}
            _ => panic!("parse s = {} failed", s),
        }
        match tokens.get_child_idx(8).unwrap()._type {
            TokenType::RightBrace => {}
            _ => panic!("parse s = {} failed", s),
        }
        match tokens.get_child_idx(9).unwrap()._type {
            TokenType::Echo => {}
            _ => panic!("parse s = {} failed", s),
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
