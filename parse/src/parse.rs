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

impl AstNode {
    pub fn get_child(&mut self,i: usize) -> Option<&mut AstNode> {
        return if self._child.len() <= i {
            None
        }else{
            self._child.get_mut(i)
        }
    }

    pub fn add_child(&mut self,child: AstNode) {
        self._child.push(child);
    }
}

pub fn parse_tokens_to_ast(mut tokens: Tokens) -> AstNode{
    match tokens.peek().unwrap()._type {
        TokenType::Int => {
            int_variable_declaration(&mut tokens)
        }
        _ => {
            panic!("parse_tokens_to_ast failed");
        }
    }
}

/// ## 语法规则
/// int {identifier} {eq} {expr}
fn int_variable_declaration(tokens: &mut Tokens) -> AstNode {
    let int = tokens.read().unwrap();
    let id = tokens.read().unwrap();
    let eq = tokens.read().unwrap();
    // match add
    match_add_expr(tokens)
}

/// ## BNF
/// add
/// :   mul
/// |   add Plus mul
/// ;
fn match_add_expr(tokens: &mut Tokens) -> AstNode {
    match_mul_expr(tokens)
}

/// ## BNF
/// mul
/// :   int
/// |   mul Star int
/// ;
pub fn match_mul_expr(tokens: &mut Tokens) -> AstNode {
    // 预读
    match tokens.peek() {
        Option::Some(t1) => {
            // 当前token的类型
            match t1._type {
                TokenType::Number => {
                    let left = new_ast_node(tokens.read().unwrap());
                    let next = tokens.peek();
                    match next {
                        Option::Some(t2) => {
                            match t2._type {
                                TokenType::Star => {
                                    let mut r = new_ast_node(tokens.read().unwrap());
                                    r.add_child(left);
                                    r.add_child(match_mul_expr(tokens));
                                    return r;
                                }
                                _ => {panic!("match_mul_expr failed")}
                            }
                        }
                        Option::None =>{return left}
                    }
                }
                _ => { panic!("match_mul_expr failed") }
            }
        }
        _ => { panic!("match_mul_expr failed") }
    }
}


