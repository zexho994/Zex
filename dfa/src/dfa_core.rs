#[derive(Debug)]
pub enum DfaState {
    // 初始状态
    Initial,
    // 字面量状态,数字
    IntLiteral,
    // 标识符状态，数字或者字母
    Id,
    // 大于状态
    GT,
    // 大于等于状态
    GE,
}

#[derive(Debug)]
pub enum TokenType {
    None,
    Identifier,
    IntLiteral,
    GT,
    GE,
}

#[derive(Debug)]
pub struct Token {
    _type: TokenType,
    text: String,
}

pub fn parse_part_one(s: String) {
    let mut state = DfaState::Initial;
    let mut token = Token { _type: TokenType::None, text: String::from("") };
    let len = s.chars().count();
    let mut i: usize = 0;
    let mut ch: char;
    while i < len {
        ch = s.chars().nth(i).unwrap();
        if char_is_alpha(ch) {
            state = DfaState::Id;
            token._type = TokenType::Identifier;
            token.text.push(ch);
        } else if char_is_digit(ch) {
            state = DfaState::IntLiteral;
            token._type = TokenType::IntLiteral;
            token.text.push(ch);
        } else if char_is_gt(ch) {
            state = DfaState::GT;
            token._type = TokenType::GT;
            token.text.push(ch);
        }
        i += 1;
        if i == len {
            break;
        }
        ch = s.chars().nth(i).unwrap();
        match state {
            DfaState::Initial => {
                state = DfaState::Initial;
            }
            DfaState::Id => {
                if char_is_alpha(ch) | char_is_digit(ch) {
                    token.text.push(ch);
                    i += 1;
                } else {
                    state = DfaState::Initial;
                }
                state = DfaState::Initial;
            }
            DfaState::GT => {
                if char_is_eq(ch) {
                    token._type = TokenType::GE;
                    state = DfaState::GE;
                    token.text.push(ch);
                    i += 1;
                } else {
                    state = DfaState::Initial;
                }
            }
            DfaState::GE => {
                state = DfaState::Initial;
            }
            DfaState::IntLiteral => {
                if char_is_digit(ch) {
                    token.text.push(ch);
                    i += 1;
                } else {
                    state = DfaState::Initial;
                }
            }
            _ => { panic!("token type error!") }
        }
    }

    println!("state: {:?}, token: {:?}", state, token);
}

// 判断字符是否是字母
pub fn char_is_alpha(ch: char) -> bool {
    if ch >= 'a' && ch <= 'z' {
        true
    } else if ch >= 'A' && ch <= 'Z' {
        true
    } else {
        false
    }
}

// 判断字符是否是数字 0~9
pub fn char_is_digit(ch: char) -> bool {
    if ch >= '0' && ch <= '9' {
        true
    } else {
        false
    }
}

// 判断字符是否是'>'符号
pub fn char_is_gt(ch: char) -> bool {
    if ch == '>' {
        true
    } else {
        false
    }
}

// 判断字符是否是'='符号
pub fn char_is_eq(ch: char) -> bool {
    if ch == '=' {
        true
    } else {
        false
    }
}