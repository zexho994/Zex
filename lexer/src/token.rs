use super::dfa_core;

/// token 的类型枚举
#[derive(Debug)]
#[warn(unused_variables)]
pub enum TokenType {
    // 空格类型
    Blank,
    // 标识符类型
    Identifier,
    // 数字字面量类型
    IntLiteral,
    // == 符号
    EQ,
    // > 符号
    GT,
    // >= 符号
    GE,
    // =
    Assignment,
    // int 关键字
    Int,
    // '+'
    Plus,
    // '-'
    Minus,
    // '*'
    Star,
    // '/'
    Slash,
    // ;
    SemiColon,
    // if
    IF,
}

#[derive(Debug)]
pub struct Tokens {
    pub data: Vec<Token>,
    pub pos: usize,
}

impl Tokens {
    pub fn add_token(&mut self, t: Token) {
        self.data.push(t);
    }

    pub fn get_child_idx(&self, idx: usize) -> Option<&Token> {
        if idx >= self.count() {
            None
        } else {
            Option::Some(&self.data[idx])
        }
    }

    pub fn peek(&self) -> Option<&Token> {
        if self.pos == self.count() {
            return None;
        }
        Option::Some(&self.data[self.pos])
    }

    pub fn read(&mut self) -> Option<&mut Token> {
        if self.pos == self.count() {
            return None;
        }
        self.pos += 1;
        self.data.get_mut(self.pos - 1)
    }

    pub fn count(&self) -> usize {
        self.data.len()
    }

    pub fn position(&self) -> usize {
        self.pos
    }

    pub fn set_position(&mut self, p: usize) {
        if p >= self.count() {
            panic!("set position error, p is more than count")
        }
        self.pos = p;
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
