mod dfa_core;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_part_one_test() {
        let str1 = String::from("age >= 15");
        dfa_core::parse(str1);
    }
}