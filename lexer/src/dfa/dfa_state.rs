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
    Eq,
    /// 大于状态
    Gt,
    /// 大于等于状态
    Ge,
    /// 首字母为 'i'
    I,
    /// int_2的状态.int_1后字母为'n'
    Int2,
    /// int_3的状态.int_2后字母为't'
    Int3,
    // '+'
    Plus,
    // '-'
    Minus,
    // '*'
    Star,
    // '/'
    Slash,
    // 'if '
    If2,
    // 'e'
    E,
    // 'ec'
    EC,
    // 'ech'
    ECH,
    // 'echo'
    ECHO,

    F,
    FN,
}
