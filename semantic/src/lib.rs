pub mod ast_visit;
pub mod scope;
use crate::ast_node::AstNode;
use parse::*;

pub fn semantic(mut ast_root: AstNode) {
    ast_visit::visit_program(&mut ast_root);
}
