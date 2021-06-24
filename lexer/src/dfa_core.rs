use super::{char_help::*, dfa_state::*, state_handing::*, token::*};

pub fn parse_to_tokens(s: String) -> Tokens {
    let mut i: usize = 0;
    let mut tokens: Tokens = Tokens { data: Vec::new() };
    while i < s.chars().count() {
        let (token, state) = initial_to_other(i, s.as_str());
        i = parse_to_token(state, i + 1, s.as_str(), token, &mut tokens.data);
    }
    tokens
}

/// 第一阶段，由Initial状态转化成其他状态
fn initial_to_other(i: usize, s: &str) -> (Token, DfaState) {
    let ch = s.chars().nth(i).unwrap();
    let mut token = Token { _type: TokenType::Blank, text: String::from("") };
    if char_is_blank(ch) {
        return (token, DfaState::Blank);
    }
    token.text.push(ch);

    if char_is_alpha(ch) {
        return if ch == 'i' {
            token._type = TokenType::Int;
            (token, DfaState::Int1)
        } else {
            token._type = TokenType::Identifier;
            (token, DfaState::Identifier)
        };
    }

    if char_is_digit(ch) {
        token._type = TokenType::IntLiteral;
        return (token, DfaState::Number);
    }

    if char_is_gt(ch) {
        token._type = TokenType::GT;
        return (token, DfaState::GT);
    }

    if ch == '=' {
        token._type = TokenType::Assignment;
        return (token, DfaState::EQ);
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

    panic!("initial to other error");
}

/// # parse_to_token ()
/// 会解析出一个完整的token，并添加到tokens中.
/// ##
/// ## 解析说明:
/// - parse int keyword:  int_1 -> int_2 -> int_3 -> int_ok
///
fn parse_to_token(mut state: DfaState, mut i: usize, s: &str, mut token: Token, tokens: &mut Vec<Token>) -> usize {
    let mut handle_res: (usize, DfaState);

    match state {
        DfaState::Blank => { return i; }
        _ => {}
    }

    while i < s.chars().count() {
        match state {
            // 遇到initial的时候，表示一个token已经解析完，跳出此次循环保存该token
            DfaState::Initial | DfaState::IntOK | DfaState::SemiColon => { break; }
            DfaState::Int1 => { handle_res = state_int1_handle(i, s, &mut token); }
            DfaState::Int2 => { handle_res = state_int2_handle(i, s, &mut token); }
            DfaState::Int3 => { handle_res = state_int3_handle(i, s, &mut token); }
            DfaState::Identifier => { handle_res = state_identifier_handle(i, s, &mut token); }
            DfaState::GT => { handle_res = state_gt_handle(i, s, &mut token); }
            DfaState::Number => { handle_res = state_number_handle(i, s, &mut token); }
            DfaState::Plus | DfaState::Minus | DfaState::Star | DfaState::Slash => { handle_res = state_algorithm_handle(i); }
            DfaState::EQ => { handle_res = state_eq_handle(i); }
            _ => { panic!("token type error!") }
        }
        i = handle_res.0;
        state = handle_res.1;
    }
    tokens.push(token);
    i
}