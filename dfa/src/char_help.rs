pub fn char_is_blank(ch: char) -> bool {
    if ch == ' ' {
        true
    } else {
        false
    }
}

/// 判断字符是否是字母
pub fn char_is_alpha(ch: char) -> bool {
    if ch >= 'a' && ch <= 'z' {
        true
    } else if ch >= 'A' && ch <= 'Z' {
        true
    } else {
        false
    }
}

/// 判断字符是否是数字 0~9
pub fn char_is_digit(ch: char) -> bool {
    if ch >= '0' && ch <= '9' {
        true
    } else {
        false
    }
}

/// 判断字符是否是'>'符号
pub fn char_is_gt(ch: char) -> bool {
    if ch == '>' {
        true
    } else {
        false
    }
}


/// 判断字符是否是'='符号
pub fn char_is_eq(ch: char) -> bool {
    if ch == '=' {
        true
    } else {
        false
    }
}