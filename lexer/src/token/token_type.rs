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
    Eq,
    // > 符号
    Gt,
    // >= 符号
    Ge,
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
    If,
    // {
    LeftBrace,
    // }
    RightBrace,
    // echo
    Echo,
    // fn
    Fn,
}
