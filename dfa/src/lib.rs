mod dfa_core;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_alpha_test() {
        let s1 = "h1a!";
        assert!(dfa_core::is_alpha(0, &s1));
        assert!(!dfa_core::is_alpha(1, &s1));
        assert!(dfa_core::is_alpha(2, &s1));
        assert!(!dfa_core::is_alpha(3, &s1));
    }

    #[test]
    fn is_alpha_test2() {
        let s1 = "H1A:(`";
        assert!(dfa_core::is_alpha(0, &s1));
        assert!(!dfa_core::is_alpha(1, &s1));
        assert!(dfa_core::is_alpha(2, &s1));
        assert!(!dfa_core::is_alpha(3, &s1));
        assert!(!dfa_core::is_alpha(4, &s1));
        assert!(!dfa_core::is_alpha(5, &s1));
    }

    #[test]
    fn is_digit_test1() {
        let s1 = "019A:(`";
        assert!(dfa_core::is_digit(0, &s1));
        assert!(dfa_core::is_digit(1, &s1));
        assert!(dfa_core::is_digit(2, &s1));
        assert!(!dfa_core::is_digit(3, &s1));
        assert!(!dfa_core::is_digit(4, &s1));
        assert!(!dfa_core::is_digit(5, &s1));
        assert!(!dfa_core::is_digit(6, &s1));
    }

    #[test]
    fn is_gt_test1() {
        let s1 = "><=";
        assert!(dfa_core::is_gt(0, &s1));
        assert!(!dfa_core::is_gt(1, &s1));
        assert!(!dfa_core::is_gt(2, &s1));
    }
}