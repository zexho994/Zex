use super::*;
use super::{
    dfa::dfa_state::*, dfa::dfa_state_handing::*, token::token_struct::*, utils::char_help::*,
};

/// 获取初始dfa状态
pub fn get_initial_state(i: usize, s: &str) -> (Token, DfaState) {
    let ch = s.chars().nth(i).unwrap();
    let mut token = Token {
        _type: TokenType::Blank,
        text: String::from(""),
    };

    if char_is_blank(ch) {
        return (token, DfaState::Blank);
    }
    token.text.push(ch);

    if ch == '{' {
        token._type = TokenType::LeftBrace;
        return (token, DfaState::Initial);
    }
    if ch == '}' {
        token._type = TokenType::RightBrace;
        return (token, DfaState::Initial);
    }

    if char_is_alpha(ch) {
        token._type = TokenType::Identifier;
        return if ch == 'i' {
            (token, DfaState::I)
        } else if ch == 'e' {
            (token, DfaState::E)
        } else if ch == 'f' {
            (token, DfaState::F)
        } else {
            (token, DfaState::Identifier)
        };
    }

    if char_is_digit(ch) {
        token._type = TokenType::IntLiteral;
        return (token, DfaState::Number);
    }

    if char_is_gt(ch) {
        token._type = TokenType::Gt;
        return (token, DfaState::Gt);
    }

    if ch == '=' {
        token._type = TokenType::Assignment;
        return (token, DfaState::Eq);
    }

    if ch == '+' {
        token._type = TokenType::Plus;
        return (token, DfaState::Plus);
    }

    if ch == '-' {
        token._type = TokenType::Minus;
        return (token, DfaState::Minus);
    }

    if ch == '*' {
        token._type = TokenType::Star;
        return (token, DfaState::Star);
    }

    if ch == '/' {
        token._type = TokenType::Slash;
        return (token, DfaState::Slash);
    }

    if ch == ';' {
        token._type = TokenType::SemiColon;
        return (token, DfaState::SemiColon);
    }

    (token, DfaState::EOF)
    // panic!("initial to other error,ch = a{}a", ch);
}

/// # parse_to_token ()
/// 会解析出一个完整的token，并添加到tokens中.
/// ##
/// ## 解析说明:
/// - parse int keyword:  int_1 -> int_2 -> int_3 -> int_ok
pub fn get_full_token(
    mut state: DfaState,
    mut i: usize,
    s: &str,
    mut token: Token,
    tokens: &mut Tokens,
) -> usize {
    let mut handle_res: (usize, DfaState);

    match state {
        DfaState::Blank | DfaState::EOF => {
            return i;
        }
        _ => {}
    }

    while i < s.chars().count() {
        match state {
            // 遇到initial的时候，表示一个token已经解析完，跳出此次循环保存该token
            DfaState::Initial | DfaState::SemiColon => {
                break;
            }
            DfaState::I => {
                handle_res = state_i_handle(i, s, &mut token);
            }
            DfaState::E => {
                handle_res = state_e_handle(i, s, &mut token);
            }
            DfaState::F => {
                handle_res = state_f_handle(i, s, &mut token);
            }
            DfaState::FN => {
                handle_res = state_fn_handle(i, s, &mut token);
            }
            DfaState::EC => {
                handle_res = state_ec_handle(i, s, &mut token);
            }
            DfaState::ECH => {
                handle_res = state_ech_handle(i, s, &mut token);
            }
            DfaState::ECHO => {
                handle_res = state_echo_handle(i, s, &mut token);
            }
            DfaState::If2 => {
                handle_res = state_if2_handle(i, s, &mut token);
            }
            DfaState::Int2 => {
                handle_res = state_int2_handle(i, s, &mut token);
            }
            DfaState::Int3 => {
                handle_res = state_int3_handle(i, s, &mut token);
            }
            DfaState::Identifier => {
                handle_res = state_identifier_handle(i, s, &mut token);
            }
            DfaState::Gt => {
                handle_res = state_gt_handle(i, s, &mut token);
            }
            DfaState::Number => {
                handle_res = state_number_handle(i, s, &mut token);
            }
            DfaState::Plus | DfaState::Minus | DfaState::Star | DfaState::Slash => {
                handle_res = state_algorithm_handle(i);
            }
            DfaState::Eq => {
                handle_res = state_eq_handle(i);
            }
            _ => {
                panic!("token type error!")
            }
        }
        i = handle_res.0;
        state = handle_res.1;
    }
    tokens.add_token(token);
    i
}
