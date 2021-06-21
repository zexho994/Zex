pub use lexer::token;

pub mod parse;

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
            _ => {
                panic!("invoke_lexer failed")
            }
        }
    }

    #[test]
    fn parse_tokens_to_ast() {
        let tokens = token::new_tokens(String::from("int a = 1 * 3"));
        let ast = parse::parse_tokens_to_ast(tokens);
        println!("parse tokens to ast {:?}", ast);
    }

    #[test]
    fn match_add_expr() {
        let mut tokens = token::new_tokens(String::from("1 + 2 * 3"));
        let ast = parse::match_add_expr(&mut tokens);
        println!("ast is -> {:?}", ast.unwrap());
    }

    #[test]
    fn match_mul_expr() {
        let mut tokens = token::new_tokens(String::from("2 * 3"));
        let ast = parse::match_mul_expr(&mut tokens);
        println!("ast is -> {:?}", ast.unwrap());
    }

    #[test]
    fn ast() {
        let mut tokens = token::new_tokens(String::from("2 * 3"));
        let mut root = parse::new_ast_node(tokens.read().unwrap());
        match root.get_child(0) {
            None => {}
            _ => {
                panic!("root get child should be None")
            }
        }
    }
}
