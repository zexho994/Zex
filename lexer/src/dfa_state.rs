
/// 有限状态机的状态枚举类
#[derive(Debug)]
pub enum DfaState {
    Blank = 0x0,
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
    // '+'
    Plus = 0xa,
    // '-'
    Minux = 0xb,
    // '*'
    Star = 0xc,
    // '/'
    Slash = 0xd,
}
