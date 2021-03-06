use super::super::{token::token_struct::*, token::token_type::*, utils::char_help::*};
use super::dfa_state::*;

/// if  
///     int 第二个字符为n => 继续验证第三个字符n
/// else
///     为一个标识符i
pub fn state_i_handle(i: usize, s: &str, token: &mut Token) -> (usize, DfaState) {
    let ch = s.chars().nth(i).unwrap();
    if ch == 'n' {
        token.text_append_char(ch);
        (i + 1, DfaState::Int2)
    } else if ch == 'f' {
        token.text_append_char(ch);
        (i + 1, DfaState::If2)
    } else {
        (i, DfaState::Identifier)
    }
}

pub fn state_e_handle(i: usize, s: &str, token: &mut Token) -> (usize, DfaState) {
    let ch = s.chars().nth(i).unwrap();
    if ch == 'c' {
        token.text_append_char(ch);
        (i + 1, DfaState::EC)
    } else {
        (i, DfaState::Identifier)
    }
}
pub fn state_f_handle(i: usize, s: &str, token: &mut Token) -> (usize, DfaState) {
    let ch = s.chars().nth(i).unwrap();
    if ch == 'n' {
        token.text_append_char(ch);
        (i + 1, DfaState::FN)
    } else {
        (i, DfaState::Identifier)
    }
}
pub fn state_fn_handle(i: usize, s: &str, token: &mut Token) -> (usize, DfaState) {
    let ch = s.chars().nth(i).unwrap();
    if ch == ' ' {
        token.set_type(TokenType::Fn);
        token.text_append_char(ch);
        (i + 1, DfaState::Initial)
    } else {
        (i, DfaState::Identifier)
    }
}

pub fn state_ec_handle(i: usize, s: &str, token: &mut Token) -> (usize, DfaState) {
    let ch = s.chars().nth(i).unwrap();
    if ch == 'h' {
        token.text_append_char(ch);
        (i + 1, DfaState::ECH)
    } else {
        (i, DfaState::Identifier)
    }
}

pub fn state_ech_handle(i: usize, s: &str, token: &mut Token) -> (usize, DfaState) {
    let ch = s.chars().nth(i).unwrap();
    if ch == 'o' {
        token.text_append_char(ch);
        (i + 1, DfaState::ECHO)
    } else {
        (i, DfaState::Identifier)
    }
}

pub fn state_echo_handle(i: usize, s: &str, token: &mut Token) -> (usize, DfaState) {
    let ch = s.chars().nth(i).unwrap();
    if ch == ' ' {
        token.set_type(TokenType::Echo);
        (i + 1, DfaState::Initial)
    } else {
        (i, DfaState::Identifier)
    }
}

// 'i' -> 'if',最后需要讲过ifOk判断为‘ ’，才能确定是if
pub fn state_if2_handle(i: usize, s: &str, token: &mut Token) -> (usize, DfaState) {
    let ch = s.chars().nth(i).unwrap();
    if ch == ' ' {
        token.set_type(TokenType::If);
        (i + 1, DfaState::Initial)
    } else {
        (i, DfaState::Identifier)
    }
}

pub fn state_int2_handle(i: usize, s: &str, token: &mut Token) -> (usize, DfaState) {
    let ch = s.chars().nth(i).unwrap();
    if ch != 't' {
        (i, DfaState::Identifier)
    } else {
        token.text_append_char(ch);
        (i + 1, DfaState::Int3)
    }
}

pub fn state_int3_handle(i: usize, s: &str, token: &mut Token) -> (usize, DfaState) {
    let ch = s.chars().nth(i).unwrap();
    if char_is_blank(ch) {
        token.set_type(TokenType::Int);
        (i + 1, DfaState::Initial)
    } else {
        (i, DfaState::Identifier)
    }
}

pub fn state_identifier_handle(i: usize, s: &str, token: &mut Token) -> (usize, DfaState) {
    let ch = s.chars().nth(i).unwrap();
    if char_is_alpha(ch) || char_is_digit(ch) {
        token.text_append_char(ch);
        (i + 1, DfaState::Identifier)
    } else {
        (i, DfaState::Initial)
    }
}

pub fn state_gt_handle(i: usize, s: &str, token: &mut Token) -> (usize, DfaState) {
    let ch = s.chars().nth(i).unwrap();
    if char_is_eq(ch) {
        token.set_type(TokenType::Ge);
        token.text_append_char(ch);
        (i + 1, DfaState::Initial)
    } else {
        (i, DfaState::Initial)
    }
}

pub fn state_number_handle(i: usize, s: &str, token: &mut Token) -> (usize, DfaState) {
    let ch = s.chars().nth(i).unwrap();
    if char_is_digit(ch) {
        token.text_append_char(ch);
        (i + 1, DfaState::Number)
    } else {
        (i, DfaState::Initial)
    }
}

/// cross '-'
/// Arrow : -> xx
/// minus : - xx
pub fn state_cross_handle(i: usize, s: &str, token: &mut Token) -> (usize, DfaState) {
    let ch = s.chars().nth(i).unwrap();
    if ch == '>' {
        token.text_append_char(ch);
        token.set_type(TokenType::Arrow);
        (i + 1, DfaState::Initial)
    } else {
        token.set_type(TokenType::Minus);
        (i, DfaState::Initial)
    }
}

pub fn state_c_handle(i: usize, s: &str, token: &mut Token) -> (usize, DfaState) {
    let ch = s.chars().nth(i).unwrap();
    if ch == 'l' {
        token.text_append_char(ch);
        (i + 1, DfaState::Cl)
    } else {
        (i, DfaState::Identifier)
    }
}
pub fn state_cl_handle(i: usize, s: &str, token: &mut Token) -> (usize, DfaState) {
    let ch = s.chars().nth(i).unwrap();
    if ch == 'a' {
        token.text_append_char(ch);
        (i + 1, DfaState::Cla)
    } else {
        (i, DfaState::Identifier)
    }
}
pub fn state_cla_handle(i: usize, s: &str, token: &mut Token) -> (usize, DfaState) {
    let ch = s.chars().nth(i).unwrap();
    if ch == 's' {
        token.text_append_char(ch);
        (i + 1, DfaState::Clas)
    } else {
        (i, DfaState::Identifier)
    }
}

pub fn state_clas_handle(i: usize, s: &str, token: &mut Token) -> (usize, DfaState) {
    let ch = s.chars().nth(i).unwrap();
    if ch == 's' {
        token.text_append_char(ch);
        (i + 1, DfaState::Class)
    } else {
        (i, DfaState::Identifier)
    }
}

pub fn state_class_handle(i: usize, s: &str, token: &mut Token) -> (usize, DfaState) {
    let ch = s.chars().nth(i).unwrap();
    if ch == ' ' {
        token.text_append_char(ch);
        token.set_type(TokenType::Class);
        (i + 1, DfaState::Initial)
    } else {
        (i, DfaState::Identifier)
    }
}

/// 遇到算数运算符，已经是一个完整的token
/// 直接返回Initial进行下一轮解析
pub fn state_algorithm_handle(i: usize) -> (usize, DfaState) {
    (i, DfaState::Initial)
}

pub fn state_eq_handle(i: usize) -> (usize, DfaState) {
    (i, DfaState::Initial)
}

pub fn state_semicolon_handle(i: usize) -> (usize, DfaState) {
    (i, DfaState::SemiColon)
}
