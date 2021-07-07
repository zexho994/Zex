pub mod scope;
pub mod symbol;
pub mod visitor;

use crate::ast_node::AstNode;
use parse::*;

pub fn semantic(mut ast_root: AstNode) {
    visitor::visit_program(&mut ast_root);
}
