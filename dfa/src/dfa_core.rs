enum DfaState {
    Initial,
    Digital,
    IntLiteral,
    Alpha,
    GT,
    GE,
}

fn core() {
    let mut state = DfaState::Initial;
}

// 判断idx索引的字符是否是字母
pub fn is_alpha(idx: usize, s: &str) -> bool {
    let ch = s.chars().nth(idx).unwrap();
    if ch >= 'a' && ch <= 'z' {
        true
    } else if ch >= 'A' && ch <= 'Z' {
        true
    } else {
        false
    }
}