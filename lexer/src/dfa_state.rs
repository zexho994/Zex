/// 有限状态机的状态枚举类
#[derive(Debug)]
pub enum DfaState {
    /// 换行符
    EOF,
    /// 空格
    Blank,
    /// ;
    SemiColon,
    /// 初始状态
    Initial,
    /// 字面量状态,数字
    Number,
    /// 标识符状态，数字或者字母
    Identifier,
    ///  等于状态
    EQ,
    /// 大于状态
    GT,
    /// 大于等于状态
    GE,
    /// int_1的状态.首字母为'i'的情况
    Int1,
    /// int_2的状态.int_1后字母为'n'
    Int2,
    /// int_3的状态.int_2后字母为't'
    Int3,
    /// int_ok 的状态.int_3后字符为空格
    IntOK,
    // '+'
    Plus,
    // '-'
    Minus,
    // '*'
    Star,
    // '/'
    Slash,
}
