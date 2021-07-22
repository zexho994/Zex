pub mod dfa;
pub mod lexing_flow;
pub mod token;
pub mod utils;
use crate::lexing_flow::*;
use crate::token::token_struct::*;
use crate::token::token_type::*;

/// 解析入口
pub fn lexing(s: String) -> Tokens {
    let mut i: usize = 0;
    let mut tokens = Tokens::new();
    while i < s.chars().count() {
        let (token, state) = get_initial_state(i, s.as_str());
        i = get_full_token(state, i + 1, s.as_str(), token, &mut tokens);
    }
    tokens
}
