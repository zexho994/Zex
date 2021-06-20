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
            _ => { panic!("invoke_lexer failed") }
        }
    }

    #[test]
    fn parse_tokens_to_ast() {
        let mut tokens = token::new_tokens(String::from("int a = 1 +  2 * 3"));
        assert_eq!(tokens.count(), 8);
        parse::parse_tokens_to_ast(&mut tokens);
    }

    #[test]
    fn match_mul_expr() {
        let mut tokens = token::new_tokens(String::from("2"));
        parse::match_mul_expr(&mut tokens);
    }

    #[test]
    fn ast(){
        let mut tokens = token::new_tokens(String::from("int a = 1 +  2 * 3"));
        let mut root = parse::new_ast_node(tokens.read().unwrap());
        match root.get_child(0) {
            None => {}
            _ => {panic!("root get child should be None")}
        }
        root.add_child(tokens.read().unwrap());
        let c1 = root.get_child(0).unwrap();
        c1.add_child(tokens.read().unwrap());
        let c2 = c1.get_child(0).unwrap();
        c2.add_child(tokens.read().unwrap());
        match c2.get_child(0) {
            Some(t) => {
                 t.add_child(tokens.read().unwrap());
                 println!("t is {:?}",t); 
            }
            _ => {panic!("ast failed")}
        }
    }
}
