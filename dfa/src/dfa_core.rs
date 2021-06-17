/// 有限状态机的状态枚举类
#[derive(Debug)]
enum DfaState {
    /// 初始状态
    Initial = 0x1,
    /// 字面量状态,数字
    Number = 0x2,
    /// 标识符状态，数字或者字母
    Identifier = 0x3,
    /// 大于状态
    GT = 0x4,
    /// 大于等于状态
    GE = 0x5,
    /// int_1的状态.首字母为'i'的情况
    Int1 = 0x6,
    /// int_2的状态.int_1后字母为'n'
    Int2 = 0x7,
    /// int_3的状态.int_2后字母为't'
    Int3 = 0x8,
    /// int_ok 的状态.int_3后字符为空格
    IntOK = 0x9,
}

/// token 的类型枚举
#[derive(Debug)]
enum TokenType {
    /// 空格类型
    Blank = 0x1,
    /// 标识符类型
    Identifier = 0x2,
    /// 数字字面量类型
    Number = 0x3,
    /// > 符号
    GT = 0x4,
    /// >= 符号
    GE = 0x5,
    /// int 关键字
    Int = 0x6,
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
        i = parse_to_token(state, i + 1, s.as_str(), token, &mut tokens);
    }
    println!("the tokens is {:?}", tokens);
}

/// 第一阶段，由Initial状态转化成其他状态
fn initial_to_other(i: usize, s: &str) -> (Token, DfaState) {
    let ch = s.chars().nth(i).unwrap();
    let mut token = Token { _type: TokenType::Blank, text: String::from("") };
    if char_is_blank(ch) {
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

/// parse_to_token 会解析出一个完整的token，并添加到tokens中.
///     parse int keyword:  int_1 -> int_2 -> int_3 -> int_ok
///
fn parse_to_token(state: DfaState, mut i: usize, s: &str, mut token: Token, mut tokens: &mut Vec<Token>) -> usize {
    let count = s.chars().count();
    while i < count {
        match state {
            DfaState::Initial => {
                return i;
            }
            DfaState::Int1 => {
                panic!("not impl Int1");
            }
            DfaState::Identifier => {
                let ch = s.chars().nth(i).unwrap();
                if !(char_is_alpha(ch) || char_is_digit(ch)) { break; }
                i += 1;
                token.text.push(ch);
            }
            DfaState::GT => {
                let ch = s.chars().nth(i).unwrap();
                if char_is_eq(ch) {
                    token._type = TokenType::GE;
                    token.text.push(ch);
                    i += 1;
                }
                break;
            }
            DfaState::Number => {
                let ch = s.chars().nth(i).unwrap();
                if !char_is_digit(ch) {
                    break;
                }
                i += 1;
                token.text.push(ch);
            }
            _ => { panic!("token type error!") }
        }
    }
    tokens.push(token);
    return i;
}

fn char_is_blank(ch: char) -> bool {
    if ch == ' ' {
        true
    } else {
        false
    }
}

/// 判断字符是否是字母
fn char_is_alpha(ch: char) -> bool {
    if ch >= 'a' && ch <= 'z' {
        true
    } else if ch >= 'A' && ch <= 'Z' {
        true
    } else {
        false
    }
}

/// 判断字符是否是数字 0~9
fn char_is_digit(ch: char) -> bool {
    if ch >= '0' && ch <= '9' {
        true
    } else {
        false
    }
}

/// 判断字符是否是'>'符号
fn char_is_gt(ch: char) -> bool {
    if ch == '>' {
        true
    } else {
        false
    }
}

/// 判断字符是否是'='符号
fn char_is_eq(ch: char) -> bool {
    if ch == '=' {
        true
    } else {
        false
    }
}