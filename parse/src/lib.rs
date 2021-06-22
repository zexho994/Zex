pub use lexer::token;

pub mod ast_node_type;
pub mod parse;
pub mod ast_node;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_tokens_to_ast() {
        let mut tokens = token::new_tokens(String::from("int a = 1 * 3"));
        let ast = parse::parse_to_ast(&mut tokens);
        println!("ast is {:?}", ast)
    }

    #[test]
    fn parse_tokens_to_ast_1() {
        let mut tokens = token::new_tokens(String::from("int a = 1 + 2 * 3"));
        let ast = parse::parse_to_ast(&mut tokens);
        println!("ast is {:?}", ast)
    }

    #[test]
    fn parse_tokens_to_ast_2() {
        let mut tokens = token::new_tokens(String::from("int a = 1 + 2 * 3 + 4 * 5"));
        let ast = parse::parse_to_ast(&mut tokens);
        println!("ast is {:?}", ast)
    }

    #[test]
    fn match_add_expr() {
        let mut tokens = token::new_tokens(String::from("1 + 2 + 3"));
        let ast = parse::match_add_expr(&mut tokens);
        println!("ast is {:?}", ast)
    }

    #[test]
    fn match_mul_expr() {
        let mut tokens = token::new_tokens(String::from("1 * 2 * 3"));
        let ast = parse::match_mul_expr(&mut tokens);
        println!("ast is {:?}", ast)
    }
}
