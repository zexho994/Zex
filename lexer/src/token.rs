use super::dfa_core;

/// token 的类型枚举
#[derive(Debug)]
pub enum TokenType {
    // 空格类型
    Blank = 0x1,
    // 标识符类型
    Identifier = 0x2,
    // 数字字面量类型
    Number = 0x3,
    // > 符号
    GT = 0x4,
    // >= 符号
    GE = 0x5,
    // int 关键字
    Int = 0x6,
    // '+'
    Plus = 0x7,
    // '-'
    Minux = 0x8,
    // '*'
    Star = 0x9,
    // '/'
    Slash = 0xc,
}

#[derive(Debug)]
pub struct Tokens {
    pub data: Vec<Token>,
}

impl Tokens {
    pub fn peek(&self) -> Option<&Token> {
        if self.data.len() == 0 {
            return Option::None;
        }
        Option::Some(&self.data[0])
    }

    pub fn read(&mut self) -> Option<Token> {
        if self.is_empty() {
            return Option::None;
        }
        Option::Some(self.data.remove(0))
    }

    pub fn count(&self) -> usize {
        self.data.len()
    }

    /// 判断token.data的元素数量是否为0
    fn is_empty(&self) -> bool {
        if self.data.len() == 0 {
            true
        } else {
            false
        }
    }
}

pub fn new_tokens(text: String) -> Tokens {
    dfa_core::parse_to_tokens(text)
}

#[derive(Debug)]
pub struct Token {
    pub _type: TokenType,
    pub text: String,
}

