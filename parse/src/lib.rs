pub use lexer::token;

pub mod ast_node;
pub mod ast_node_type;
pub mod parse;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_to_ast() {
        let s = "int a = 1 + 3 + 0 + 5 + 1;".to_string();
        println!("\n==> parse str {}", s);
        let mut tokens = token::new_tokens(String::from(s));
        let ast = parse::parse_to_ast(&mut tokens);
        println!("==> parse done. ast = {:?}", ast);
        assert_eq!(ast.unwrap(), 10)
    }

    #[test]
    fn match_add_expr() {
        let mut tokens = token::new_tokens(String::from("1 + 2 + 3;"));
        let ast = parse::match_add_expr(&mut tokens);
        println!("ast is {:?}", ast)
    }

    #[test]
    fn match_mul_expr() {
        let mut tokens = token::new_tokens(String::from("1 * 2 * 3;"));
        let ast = parse::match_mul_expr(&mut tokens).unwrap();
        println!("ast is {:?}", ast)
    }

    #[test]
    fn match_assignment_expr() {
        let mut tokens = token::new_tokens(String::from("i = i + 2;"));
        let ast = parse::match_assignment(&mut tokens).unwrap();
        println!("ast is {:?}", ast);
    }

    #[test]
    fn match_express_stm() {
        let mut tokens = token::new_tokens(String::from("1 + 1 + 2;"));
        let ast = parse::match_expr_stm(&mut tokens).unwrap();
        println!("ast is {:?}", ast);
    }

    #[test]
    fn multi_program() {
        let str = String::from("int a = 1;a = 2; a + 1 + 2;");
        let mut tokens = token::new_tokens(str);
        let ast = parse::parse_to_ast(&mut tokens).unwrap();
        println!("ast is {:?}", ast);
    }
}
