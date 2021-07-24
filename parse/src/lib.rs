pub mod ast_node;
pub mod ast_node_type;
pub mod flow;
pub mod utils;

use flow::parse_flow;

pub fn parsing(tokens: &mut lexer::token::token_struct::Tokens) -> Option<ast_node::AstNode> {
    flow::flow_program::match_program(tokens)
}
