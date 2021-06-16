#[derive(Debug)]
pub enum DfaState {
    // 初始状态
    Initial,
    // 字面量状态,数字
    IntLiteral,
    // 标识符状态，数字或者字母
    Identifier,
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

type TokenList = Vec<Token>;

#[derive(Debug)]
pub struct Token {
    _type: TokenType,
    text: String,
}

pub fn parse(s: String) {
    // let mut state: DfaState = DfaState::Initial;
    let mut i: usize = 0;
    while i < s.chars().count() {
        let (mut token, mut state) = init_to_other(i, s.as_str());
        i += 1;
        if i == s.chars().count() {
            break;
        }
        let ch = s.chars().nth(i).unwrap();
        match state {
            DfaState::Initial => {
                state = DfaState::Initial;
            }
            DfaState::Identifier => {
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
        println!("state: {:?}, token: {:?}", state, token);
    }
}

/// 第一阶段，由Initial状态转化成其他状态
pub fn init_to_other(i: usize, s: &str) -> (Token, DfaState) {
    let ch = s.chars().nth(i).unwrap();
    let mut token = Token { _type: TokenType::None, text: String::from("") };

    if char_is_alpha(ch) {
        token._type = TokenType::Identifier;
        token.text.push(ch);
        (token, DfaState::Identifier)
    } else if char_is_digit(ch) {
        token._type = TokenType::IntLiteral;
        token.text.push(ch);
        (token, DfaState::IntLiteral)
    } else if char_is_gt(ch) {
        token._type = TokenType::GT;
        token.text.push(ch);
        (token, DfaState::GT)
    } else {
        (token, DfaState::GT)
    }
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