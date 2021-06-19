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
}

#[derive(Debug)]
pub struct Token {
    pub _type: TokenType,
    pub text: String,
}

