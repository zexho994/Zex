#[derive(Debug)]
enum DfaState {
    // 初始状态
    Initial = 0x1,
    // 字面量状态,数字
    Number = 0x2,
    // 标识符状态，数字或者字母
    Identifier = 0x3,
    // 大于状态
    GT = 0x4,
    // 大于等于状态
    GE = 0x5,
}

#[derive(Debug)]
enum TokenType {
    Blank = 0x1,
    Identifier = 0x2,
    Number = 0x3,
    GT = 0x4,
    GE = 0x5,
}

#[derive(Debug)]
struct Token {
    _type: TokenType,
    text: String,
}

pub fn parse_to_tokens(s: String) {
    let mut i: usize = 0;
    let mut tokens: Vec<Token> = Vec::new();

    while i < s.chars().count() {
        let (mut token, mut state) = initial_to_other(i, s.as_str());
        i = other_to_token(state, i + 1, s.as_str(), token, &mut tokens);
    }
    println!("the tokens is {:?}", tokens);
}

/// 第一阶段，由Initial状态转化成其他状态
fn initial_to_other(i: usize, s: &str) -> (Token, DfaState) {
    let ch = s.chars().nth(i).unwrap();
    let mut token = Token { _type: TokenType::Blank, text: String::from("") };
    if ch == ' ' {
        (token, DfaState::Initial)
    } else if char_is_alpha(ch) {
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
        panic!("initial to other error");
    }
}

fn other_to_token(state: DfaState, mut i: usize, s: &str, mut token: Token, mut tokens: &mut Vec<Token>) -> usize {
    match state {
        DfaState::Initial => {}
        DfaState::Identifier => {
            while i < s.chars().count() {
                let ch = s.chars().nth(i).unwrap();
                if !(char_is_alpha(ch) || char_is_digit(ch)) { break; }
                token.text.push(ch);
                i += 1;
            }
            tokens.push(token);
        }
        DfaState::GT => {
            if i < s.chars().count() {
                let ch = s.chars().nth(i).unwrap();
                if char_is_eq(ch) {
                    token._type = TokenType::GE;
                    token.text.push(ch);
                    i += 1;
                }
            }
            tokens.push(token);
        }
        DfaState::Number => {
            while i < s.chars().count() {
                let ch = s.chars().nth(i).unwrap();
                if !char_is_digit(ch) { break; }
                token.text.push(ch);
                i += 1;
            }
            tokens.push(token);
        }
        _ => { panic!("token type error!") }
    }
    return i;
}

// 判断字符是否是字母
fn char_is_alpha(ch: char) -> bool {
    if ch >= 'a' && ch <= 'z' {
        true
    } else if ch >= 'A' && ch <= 'Z' {
        true
    } else {
        false
    }
}

// 判断字符是否是数字 0~9
fn char_is_digit(ch: char) -> bool {
    if ch >= '0' && ch <= '9' {
        true
    } else {
        false
    }
}

// 判断字符是否是'>'符号
fn char_is_gt(ch: char) -> bool {
    if ch == '>' {
        true
    } else {
        false
    }
}

// 判断字符是否是'='符号
fn char_is_eq(ch: char) -> bool {
    if ch == '=' {
        true
    } else {
        false
    }
}