use super::*;

pub fn hello() {
    println!("Hello parse");
}

pub fn blank() {
    let b = ' ';
    let f = char_is_blank(b);
    assert!(f);
}

fn int_variable_declaration(tokens: &mut Vec<Token>) {}


