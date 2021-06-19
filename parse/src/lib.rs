pub use lexer::token;

mod parse;

// pub use lexer::Tokens;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invoke_lexer() {
        let mut _tokens = token::new_tokens(String::from("a"));
        match _tokens.read() {
            Option::Some(_token) => {
                assert_eq!(_token.text, "a");
                assert_eq!(_tokens.count(), 0);
            }
            _ => { panic!("invoke_lexer failed") }
        }
    }
}
