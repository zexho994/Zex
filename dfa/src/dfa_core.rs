#[derive(Debug)]
enum DfaState {
    Initial,
    IntLiteral,
    Id,
    GT,
    GE,
}

#[derive(Debug)]
enum TokenType {
    None,
    Identifier,
    IntLiteral,
    GT,
}

struct Token{
    _type : TokenType,
    text : String,
}

pub fn parse_part_one(s: String) {
    let mut state = DfaState::Initial;
    let mut token = Token{_type: TokenType::None, text: String::from("")};
    let ch = s.chars().nth(0).unwrap();
    println!("s is {}",s);
    if char_is_alpha(ch){
        state = DfaState::Id;
        token._type = TokenType::Identifier;
        token.text.push(ch);
    }else if char_is_digit(ch) {
        state = DfaState::IntLiteral;
        token._type =  TokenType::IntLiteral;
        token.text.push(ch);
    }else if char_is_gt(ch) {
        state = DfaState::GT;
        token._type = TokenType::GT;
        token.text.push(ch);
    }
    println!("=====> state is {:?}, token._type is {:?}, text is {}",state, token._type, token.text)
}

// 判断idx索引的字符是否是字母
// pub fn str_is_alpha(idx: usize, s: &str) -> bool {
//     let ch = s.chars().nth(idx).unwrap();
//     if ch >= 'a' && ch <= 'z' {
//         true
//     } else if ch >= 'A' && ch <= 'Z' {
//         true
//     } else {
//         false
//     }
// }

pub fn char_is_alpha(ch: char) -> bool {
    if ch >= 'a' && ch <= 'z' {
        true
    } else if ch >= 'A' && ch <= 'Z' {
        true
    } else {
        false
    }
}

// 判断idx索引的字符是否是数字 0~9
// pub fn str_is_digit(idx: usize, s: &str) -> bool {
//     let ch = s.chars().nth(idx).unwrap();
//     if ch >= '0' && ch <= '9' {
//         true
//     } else {
//         false
//     }
// }

pub fn char_is_digit(ch: char) -> bool {
    if ch >= '0' && ch <= '9' {
        true
    } else {
        false
    }
}

// 判断idx索引的字符是否是'>'符号
// pub fn str_is_gt(idx: usize, s: &str) -> bool {
//     match s.chars().nth(idx) {
//         Some('>') => true,
//         _ => false
//     }
// }

pub fn char_is_gt(ch: char) -> bool {
    match ch {
        '>' => true,
        _ => false
    }
}