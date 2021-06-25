pub use lexer::token;

pub mod ast_node;
pub mod ast_node_type;
pub mod calculate;
pub mod parse;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_to_ast() {
        let s = String::from("int a = 1 + 3 + 1;");
        println!("\n==> parse str {}", s);
        let mut tokens = token::new_tokens(s);
        assert_eq!(9, tokens.count());
        let ast = parse::parse_to_ast(&mut tokens);
        println!("==> parse done. ast = {:?}", ast);
        assert_eq!(ast.unwrap(), 5)
    }

    #[test]
    fn parse_to_ast2() {
        let s = String::from("int a = 2 * 3 + 1 + 3 * 10;");
        println!("\n==> parse str {}", s);
        let mut tokens = token::new_tokens(s);
        assert_eq!(13, tokens.count());
        let ast = parse::parse_to_ast(&mut tokens);
        println!("==> parse done. ast = {:?}", ast);
        assert_eq!(ast.unwrap(), 37)
    }

    #[test]
    fn match_add_expr() {
        let s = String::from("1 + 2 + 3");
        println!("\n==> parse str {}", s);
        let mut tokens = token::new_tokens(s);
        let ast = parse::match_add_expr(&mut tokens);
        println!("ast is {:?}", ast)
    }

    #[test]
    fn match_mul_expr() {
        let s = String::from("1 * 2 * 3");
        println!("\n==> parse str {}", s);
        let mut tokens = token::new_tokens(s);
        let ast = parse::match_mul_expr(&mut tokens).unwrap();
        println!("ast is {:?}", ast)
    }

    #[test]
    fn match_assignment_expr1() {
        let s = String::from("i = 1 + 2;");
        println!("\n==> parse str {}", s);
        let mut tokens = token::new_tokens(s);
        let res = parse::parse_to_ast(&mut tokens).unwrap();
        println!("ast is {}", res);
        assert_eq!(res, 3)
    }

    #[test]
    fn match_assignment_expr2() {
        let s = String::from("i = 2 * 3 ;");
        println!("\n==> parse str {}", s);
        let mut tokens = token::new_tokens(s);
        let res = parse::parse_to_ast(&mut tokens).unwrap();
        println!("ast is {}", res);
        assert_eq!(res, 6)
    }

    #[test]
    fn match_assignment_expr3() {
        let s = String::from("i = 10 + 2 * 3 ;");
        println!("\n==> parse str {}", s);
        let mut tokens = token::new_tokens(s);
        let res = parse::parse_to_ast(&mut tokens).unwrap();
        println!("ast is {}", res);
        assert_eq!(res, 16)
    }
    
    #[test]
    fn match_express_stm() {
        let s = String::from("1 + 1 + 2;");
        println!("\n==> parse str {}", s);
        let mut tokens = token::new_tokens(s);
        let ast = parse::match_expr_stm(&mut tokens).unwrap();
        println!("ast is {:?}", ast);
    }

    #[test]
    fn multi_program() {
        let str = String::from("int a = 1;a = 2; a + 1 + 2;");
        println!("\n==> parse str {}", str);
        let mut tokens = token::new_tokens(str);
        let ast = parse::parse_to_ast(&mut tokens).unwrap();
        println!("ast is {:?}", ast);
    }

    #[test]
    fn map_or_test() {
        let n: Option<&str> = None;
        assert!(n.map_or(10, |_| 5) == 10);
    }
}
