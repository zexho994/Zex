use crate::ast_node::AstNode;
use crate::ast_node::AstNodeType;
use crate::flow::flow_statements::match_statements;
use crate::utils::print_util::print_parse_more2_info;
use lexer::token::token_struct::Tokens;

/// <program> ::= <statements>
pub fn match_program(tokens: &mut Tokens) -> Option<AstNode> {
	print_parse_more2_info("match program,token is ", tokens.peek(), tokens.position());
	let mut prog_node = AstNode::new(AstNodeType::Program, "");

	if let Some(n) = match_statements(tokens) {
		prog_node.add_child(n);
	} else {
		return None;
	}

	Option::Some(prog_node)
}
