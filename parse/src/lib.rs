pub use lexer::token;

pub mod ast_node;
pub mod ast_node_type;
pub mod parse;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_tokens_to_ast_3() {
        let mut tokens = token::new_tokens(String::from("int a = 1 + 3 + 0 + 5 + 1"));
        let res = parse::parse_to_ast(&mut tokens);
        assert_eq!(res, 10)
    }

    #[test]
    fn parse_tokens_to_ast_4() {
        let mut tokens = token::new_tokens(String::from("int a = 1 * 2 * 3 * 4"));
        let res = parse::parse_to_ast(&mut tokens);
        assert_eq!(res, 24)
    }

    #[test]
    fn parse_tokens_to_ast_5() {
        let mut tokens = token::new_tokens(String::from("int a = 1 * 2 + 3 * 4 + 2"));
        let res = parse::parse_to_ast(&mut tokens);
        assert_eq!(res, 16)
    }

    #[test]
    #[ignore]
    fn parse_tokens_to_ast_1() {
        let mut tokens = token::new_tokens(String::from("int a = 1 + 2 * 3"));
        let ast = parse::parse_to_ast(&mut tokens);
        println!("ast is {:?}", ast)
    }

    #[test]
    #[ignore]
    fn parse_tokens_to_ast_2() {
        let mut tokens = token::new_tokens(String::from("int a = 1 + 2 * 3 + 4 * 5"));
        let ast = parse::parse_to_ast(&mut tokens);
        println!("ast is {:?}", ast)
    }

    #[test]
    #[ignore]
    fn match_add_expr() {
        let mut tokens = token::new_tokens(String::from("1 + 2 + 3"));
        let ast = parse::match_add_expr(&mut tokens);
        println!("ast is {:?}", ast)
    }

    #[test]
    #[ignore]
    fn match_mul_expr() {
        let mut tokens = token::new_tokens(String::from("1 * 2 * 3"));
        let ast = parse::match_mul_expr(&mut tokens);
        println!("ast is {:?}", ast)
    }
}
