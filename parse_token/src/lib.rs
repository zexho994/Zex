pub mod dfa_core;
pub mod char_help;
pub mod state_handing;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_test_1() {
        let str1 = String::from("age >= 15");
        dfa_core::parse_to_tokens(str1);
    }

    #[test]
    fn parse_test_2() {
        let str1 = String::from("num > 0");
        dfa_core::parse_to_tokens(str1);
    }

    #[test]
    fn parse_test_3() {
        let str1 = String::from("num > 01");
        dfa_core::parse_to_tokens(str1);
    }

    #[test]
    fn parse_test_4() {
        let str1 = String::from("int num > 01");
        dfa_core::parse_to_tokens(str1);
    }

    #[test]
    fn parse_test_5() {
        let str1 = String::from("inte num > 01");
        dfa_core::parse_to_tokens(str1);
    }

    #[test]
    fn parse_test_6() {
        let str1 = String::from("in num > 01");
        dfa_core::parse_to_tokens(str1);
    }

    #[test]
    fn parse_test_7() {
        let str1 = String::from("i int num > 01");
        dfa_core::parse_to_tokens(str1);
    }
}