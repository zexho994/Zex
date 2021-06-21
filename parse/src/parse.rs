use token::*;

use super::*;

#[derive(Debug)]
pub struct AstNode {
    _token: Token,
    _child: Vec<AstNode>,
}

pub fn new_ast_node(root: Token) -> AstNode {
    AstNode {
        _token: root,
        _child: Vec::new(),
    }
}

impl AstNode {
    pub fn get_child(&mut self, i: usize) -> Option<&mut AstNode> {
        return if self._child.len() <= i {
            None
        } else {
            self._child.get_mut(i)
        };
    }

    pub fn add_child(&mut self, child: AstNode) {
        self._child.push(child);
    }
}

pub fn parse_tokens_to_ast(mut tokens: Tokens) -> Option<AstNode> {
    match tokens.peek().unwrap()._type {
        TokenType::Int => int_variable_declaration(&mut tokens),
        _ => {
            panic!("parse_tokens_to_ast failed");
        }
    }
}

/// ## 语法规则
/// int {identifier} {eq} {expr}
fn int_variable_declaration(tokens: &mut Tokens) -> Option<AstNode> {
    let mut ast = new_ast_node(tokens.read().unwrap());
    let child_id = new_ast_node(tokens.read().unwrap());
    let child_eq = new_ast_node(tokens.read().unwrap());
    ast.add_child(child_id);
    ast.add_child(child_eq);
    // match add
    ast.add_child(match_add_expr(tokens).unwrap());
    Option::Some(ast)
}

/// ## BNF
/// add
/// :   mul
/// |   mul Plus add
/// ;
pub fn match_add_expr(tokens: &mut Tokens) -> Option<AstNode> {
    match tokens.peek() {
        Option::Some(p1) => match p1._type {
            TokenType::Number => {
                let c1 = match_mul_expr(tokens);
                match tokens.peek() {
                    Option::Some(p2) => match p2._type {
                        TokenType::Plus => {
                            let mut r = new_ast_node(tokens.read().unwrap());
                            r.add_child(c1.unwrap());
                            r.add_child(match_add_expr(tokens).unwrap());
                            Option::Some(r)
                        }
                        _ => {
                            panic!("match add expr failed,token type is {:?}", p2);
                        }
                    },
                    Option::None => c1,
                }
            }
            _ => {
                panic!("token type is {:?}", p1)
            }
        },
        Option::None => {
            return Option::None;
        }
    }
}

/// ## BNF
/// mul
/// :   int
/// |   mul Star int
/// ;
pub fn match_mul_expr(tokens: &mut Tokens) -> Option<AstNode> {
    match tokens.peek() {
        Option::Some(t1) => match t1._type {
            TokenType::Number => {
                let left = new_ast_node(tokens.read().unwrap());
                match tokens.peek() {
                    Option::Some(t2) => match t2._type {
                        TokenType::Star => {
                            let mut r = new_ast_node(tokens.read().unwrap());
                            r.add_child(left);
                            r.add_child(match_mul_expr(tokens).unwrap());
                            Option::Some(r)
                        }
                        _ => Option::Some(left),
                    },
                    Option::None => return Option::Some(left),
                }
            }
            _ => {
                panic!("match_mul_expr failed")
            }
        },
        _ => Option::None,
    }
}
