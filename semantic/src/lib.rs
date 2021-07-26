pub mod scope;
pub mod scope_stack;
pub mod symbol;
pub mod visitor;

use crate::ast_node::AstNode;
use crate::visitor::visit_program::visit_program;
use parse::*;

pub fn semantic(mut ast_root: AstNode) {
    visit_program(&mut ast_root);
}
