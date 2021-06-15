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
}