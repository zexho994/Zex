use token::*;

use super::*;

#[derive(Debug)]
pub struct AstNode {
    _token: Token,
    _child: Vec<AstNode>,
}

pub fn new_ast_node(root: Token) -> AstNode {
    AstNode {_token: root, _child: Vec::new()}
}

pub fn add_child(ast: &mut AstNode,t: Token) {
    let t = new_ast_node(t);
    ast._child.push(t);
}

impl AstNode {
    pub fn get_child(&mut self,i: usize) -> Option<&mut AstNode> {
        return if self._child.len() <= i {
            None
        }else{
            self._child.get_mut(i)
        }
    }

    pub fn add_child(&mut self,child: Token) {
        self._child.push(new_ast_node(child));
    }
}

pub fn parse_tokens_to_ast(tokens: &mut Tokens) {
    match tokens.peek().unwrap()._type {
        TokenType::Int => {
            int_variable_declaration(tokens);
        }
        _ => {
            panic!("parse_tokens_to_ast failed");
        }
    }
}

/// ## 语法规则
/// int {identifier} {eq} {expr}
fn int_variable_declaration(tokens: &mut Tokens) {
    let int = tokens.read().unwrap();
    let id = tokens.read().unwrap();
    let eq = tokens.read().unwrap();
}

/// ## BNF
/// add
/// :   mul
/// |   add Plus mul
/// ;
fn match_add_expr(tokens: &mut Tokens) {
    match_mul_expr(tokens);
}

/// ## BNF
/// mul
/// :   int
/// |   mul Star int
/// ;
pub fn match_mul_expr(tokens: &mut Tokens) {
    match tokens.peek() {
        Option::Some(_token) => {
            match _token._type {
                TokenType::Number => {
                    tokens.read();
                    let next = tokens.peek();
                    println!("next is {:?}", next)
                }
                _ => { panic!("match_mul_expr failed") }
            }
        }
        Option::None => { return; }
        _ => { panic!("match_mul_expr failed") }
    }
}


