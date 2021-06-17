mod dfa_core;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_part_one_test_1() {
        let str1 = String::from("age >= 15");
        dfa_core::parse_to_tokens(str1);
    }

    #[test]
    fn parse_part_one_test_2() {
        let str1 = String::from("num > 0");
        dfa_core::parse_to_tokens(str1);
    }

    #[test]
    fn parse_part_one_test_3() {
        let str1 = String::from("num > 01");
        dfa_core::parse_to_tokens(str1);
    }

}