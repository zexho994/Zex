pub mod char_help;
pub mod dfa_core;
pub mod dfa_state;
pub mod state_handing;
pub mod token;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_test_1() {
        let str1 = String::from("age >= 15 ;");
        let tokens = dfa_core::parse_to_tokens(str1);
        assert!(tokens.data.len() == 4);
    }

    #[test]
    fn parse_test_5() {
        let str1 = String::from("inte num > 01;");
        let tokens = dfa_core::parse_to_tokens(str1);
        assert!(tokens.data.len() == 5);
    }

    #[test]
    fn parse_test_7() {
        let str1 = String::from("i int num > 01;");
        let tokens = dfa_core::parse_to_tokens(str1);
        assert!(tokens.data.len() == 6);
    }

    #[test]
    fn parse_test_12() {
        let str1 = String::from("int num = 1; ");
        let tokens = dfa_core::parse_to_tokens(str1);
        assert!(tokens.data.len() == 5);
    }

    #[test]
    fn assignment() {
        let str1 = String::from("=");
        let tokens = dfa_core::parse_to_tokens(str1);
        assert!(tokens.data.len() == 1);
        match tokens.peek() {
            Some(t) => match t._type {
                token::TokenType::Assignment => {
                    println!("");
                }
                _ => panic!("token is not an Assignment"),
            },
            None => panic!("parse test fail"),
        }
    }
    
    #[test]
    fn SemiColon(){
        let str1 = String::from(";");
        let tokens = dfa_core::parse_to_tokens(str1);
        assert!(tokens.data.len() == 1);
        match tokens.peek() {
            Some(t) => match t._type {
                token::TokenType::SemiColon => {
                    println!("");
                }
                _ => panic!("token is not a SemiColon"),
            },
            None => panic!("parse test fail"),
        }
    }

    #[test]
    fn parse_test_13() {
        let str1 = String::from(" ");
        let tokens = dfa_core::parse_to_tokens(str1);
        assert!(tokens.data.len() == 0);
        match tokens.peek() {
            Option::None => {}
            _ => {
                panic!("parse_test_12 failure!")
            }
        }
    }

    #[test]
    fn tokens_read() {
        let str1 = String::from("a");
        let mut tokens = dfa_core::parse_to_tokens(str1);
        assert!(tokens.data.len() == 1);
        match tokens.read() {
            Option::Some(_token) => {
                assert!(tokens.count() == 0);
                assert_eq!(_token.text, "a");
            }
            _ => {
                panic!("parse_test_12 failure!")
            }
        }
    }
}
