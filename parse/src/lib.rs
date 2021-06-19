mod parse;

pub use lexer::char_help::*;
// pub use lexer::Tokens;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        parse::blank();
    }

    #[test]
    fn lexer_test(){
        assert!(char_is_blank(' '));
        assert!(!char_is_blank('a'));
    }
}
