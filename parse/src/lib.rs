pub use lexer::token;

pub mod ast_node;
pub mod ast_node_type;
pub mod calculate;
pub mod parse;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_to_ast1() {
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
    fn match_assignment_expr1() {
        let s = String::from("i = 1 + 5;");
        println!("\n==> parse str {}", s);
        let mut tokens = token::new_tokens(s);
        let res = parse::parse_to_ast(&mut tokens).unwrap();
        assert_eq!(res, 6)
    }

    #[test]
    fn match_assignment_expr2() {
        let s = String::from("i = 2 * 3 ;");
        println!("\n==> parse str {}", s);
        let mut tokens = token::new_tokens(s);
        let res = parse::parse_to_ast(&mut tokens).unwrap();
        assert_eq!(res, 6)
    }

    #[test]
    fn match_assignment_expr3() {
        let s = String::from("i = 10 + 2 * 3 ;");
        println!("\n==> parse str {}", s);
        let mut tokens = token::new_tokens(s);
        let res = parse::parse_to_ast(&mut tokens).unwrap();
        assert_eq!(res, 16)
    }

    #[test]
    fn match_express_stm() {
        let s = String::from("10 + 1 + 2;");
        println!("\n==> parse str {}", s);
        let mut tokens = token::new_tokens(s);
        let res = parse::parse_to_ast(&mut tokens).unwrap();
        assert_eq!(res, 13)
    }

    #[test]
    fn multi_program() {
        let str = String::from("int a = 1;a = 2; a + 2;");
        println!("\n==> parse str {}", str);
        let mut tokens = token::new_tokens(str);
        let res = parse::parse_to_ast(&mut tokens).unwrap();
        assert_eq!(res, 4)
    }

    #[test]
    fn test_or() {
        let x = Some(1);
        let y = Some(2);
        let z = Some(3);
        assert_eq!(x.or(y).or(z), Some(1));
        assert_eq!(y.or(z).or(x), Some(2));
        assert_eq!(z.or(x).or(y), Some(3));
    }

    fn test_or_fn() {
        // assert_eq!(get(0).or(get(1)).unwrap(), "x".to_string());
        assert_eq!(get(1).or(get(2)).unwrap(), "x".to_string());
    }

    fn get(n: usize) -> Option<String> {
        println!("invoke get fn");
        if n == 1 {
            Some("x".to_string())
        } else if n == 2 {
            Some("y".to_string())
        } else if n == 3 {
            Some("z".to_string())
        } else {
            None
        }
    }
}
