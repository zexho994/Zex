pub mod dfa_core;
pub mod char_help;
pub mod state_handing;
pub mod dfa_state;
pub mod token;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_test_1() {
        let str1 = String::from("age >= 15");
        let tokens = dfa_core::parse_to_tokens(str1);
        assert!(tokens.data.len() == 3);
    }

    #[test]
    fn parse_test_2() {
        let str1 = String::from("num > 0");
        let tokens = dfa_core::parse_to_tokens(str1);
        assert!(tokens.data.len() == 3);
    }

    #[test]
    fn parse_test_3() {
        let str1 = String::from("num > 01");
        let tokens = dfa_core::parse_to_tokens(str1);
        assert!(tokens.data.len() == 3);
    }

    #[test]
    fn parse_test_4() {
        let str1 = String::from("int num > 01");
        let tokens = dfa_core::parse_to_tokens(str1);
        assert!(tokens.data.len() == 4);
    }

    #[test]
    fn parse_test_5() {
        let str1 = String::from("inte num > 01");
        let tokens = dfa_core::parse_to_tokens(str1);
        assert!(tokens.data.len() == 4);
    }

    #[test]
    fn parse_test_6() {
        let str1 = String::from("in num > 01");
        let tokens = dfa_core::parse_to_tokens(str1);
        assert!(tokens.data.len() == 4);
    }

    #[test]
    fn parse_test_7() {
        let str1 = String::from("i int num > 01");
        let tokens = dfa_core::parse_to_tokens(str1);
        assert!(tokens.data.len() == 5);
    }

    /// 测试 + 符号解析
    #[test]
    fn parse_test_8() {
        let str1 = String::from(" + ");
        let tokens = dfa_core::parse_to_tokens(str1);
        assert!(tokens.data.len() == 1);
    }

    /// 测试 + 符号解析
    #[test]
    fn parse_test_9() {
        let str1 = String::from(" - ");
        let tokens = dfa_core::parse_to_tokens(str1);
        assert!(tokens.data.len() == 1);
    }

    /// 测试 + 符号解析
    #[test]
    fn parse_test_10() {
        let str1 = String::from(" * ");
        let tokens = dfa_core::parse_to_tokens(str1);
        assert!(tokens.data.len() == 1);
    }

    /// 测试 + 符号解析
    #[test]
    fn parse_test_11() {
        let str1 = String::from(" / ");
        let tokens = dfa_core::parse_to_tokens(str1);
        assert!(tokens.data.len() == 1);
    }
}