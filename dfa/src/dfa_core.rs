#[derive(Debug)]
pub enum DfaState {
    // 初始状态
    Initial,
    // 字面量状态,数字
    Number,
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
    Number,
    GT,
    GE,
}

#[derive(Debug)]
pub struct Token {
    _type: TokenType,
    text: String,
}

pub fn parse(s: String) {
    // let mut state: DfaState = DfaState::Initial;
    let mut i: usize = 0;
    let mut tokens: Vec<Token> = Vec::new();

    while i < s.chars().count() {
        let (mut token, mut state) = init_to_other(i, s.as_str());
        i += 1;
        if i == s.chars().count() {
            break;
        }
        let mut ch = s.chars().nth(i).unwrap();
        match state {
            DfaState::Initial => {
                state = DfaState::Initial;
            }
            DfaState::Identifier => {
                while i < s.chars().count() && (char_is_alpha(ch) || char_is_digit(ch)) {
                    token.text.push(ch);
                    i += 1;
                    ch = s.chars().nth(i).unwrap();
                }
                tokens.push(token);
                state = DfaState::Initial;
            }
            DfaState::GT => {
                token._type = TokenType::GT;
                if char_is_eq(ch) {
                    token._type = TokenType::GE;
                    token.text.push(ch);
                    i += 1;
                }
                state = DfaState::Initial;
                tokens.push(token);
            }
            DfaState::Number => {
                while i < s.chars().count() && char_is_digit(ch) {
                    token.text.push(ch);
                    i += 1;
                    ch = s.chars().nth(i).unwrap();
                }
                token._type = TokenType::Number;
                tokens.push(token);
                state = DfaState::Initial;
            }
            _ => { panic!("token type error!") }
        }
        println!("state: {:?}, tokens: {:?}", state, tokens);
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
        token._type = TokenType::Number;
        token.text.push(ch);
        (token, DfaState::Number)
    } else if char_is_gt(ch) {
        token._type = TokenType::GT;
        token.text.push(ch);
        (token, DfaState::GT)
    } else {
        (token, DfaState::Initial)
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