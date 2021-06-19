use super::{char_help::*, dfa_state::*, token::*};

pub fn state_int1_handle(i: usize, s: &str, token: &mut Token) -> (usize, DfaState) {
    let ch = s.chars().nth(i).unwrap();
    if ch == 'n' {
        token.text.push(ch);
        (i + 1, DfaState::Int2)
    } else {
        token._type = TokenType::Identifier;
        (i, DfaState::Identifier)
    }
}

pub fn state_int2_handle(i: usize, s: &str, token: &mut Token) -> (usize, DfaState) {
    let ch = s.chars().nth(i).unwrap();
    if ch != 't' {
        token._type = TokenType::Identifier;
        (i, DfaState::Identifier)
    } else {
        token.text.push(ch);
        (i + 1, DfaState::Int3)
    }
}

pub fn state_int3_handle(i: usize, s: &str, token: &mut Token) -> (usize, DfaState) {
    let ch = s.chars().nth(i).unwrap();
    if char_is_blank(ch) {
        (i + 1, DfaState::IntOK)
    } else {
        token._type = TokenType::Identifier;
        (i, DfaState::Identifier)
    }
}

pub fn state_identifier_handle(i: usize, s: &str, token: &mut Token) -> (usize, DfaState) {
    let ch = s.chars().nth(i).unwrap();
    if char_is_alpha(ch) || char_is_digit(ch) {
        token.text.push(ch);
        (i + 1, DfaState::Identifier)
    } else {
        (i, DfaState::Initial)
    }
}

pub fn state_gt_handle(i: usize, s: &str, token: &mut Token) -> (usize, DfaState) {
    let ch = s.chars().nth(i).unwrap();
    if char_is_eq(ch) {
        token._type = TokenType::GE;
        token.text.push(ch);
        (i + 1, DfaState::Initial)
    } else {
        (i, DfaState::Initial)
    }
}

pub fn state_number_handle(i: usize, s: &str, token: &mut Token) -> (usize, DfaState) {
    let ch = s.chars().nth(i).unwrap();
    if char_is_digit(ch) {
        token.text.push(ch);
        (i + 1, DfaState::Number)
    } else {
        (i, DfaState::Initial)
    }
}

pub fn state_algorithm_handle(i: usize) -> (usize, DfaState) {
    (i, DfaState::Initial)
}