use super::parse_flow::*;
use crate::ast_node::*;
use crate::ast_node_type::*;
use crate::utils::print_util::*;
use lexer::token::{token_struct::*, token_type::*};

/// <statements> ::= <blockStm> | <statement> | <statement> <statements>
pub fn match_statements(tokens: &mut Tokens) -> Option<AstNode> {
	print_parse_more2_info(
		"match statements,token is ",
		tokens.peek(),
		tokens.position(),
	);
	let mut ast_node = AstNode::new(AstNodeType::Statements, "");

	while tokens.peek().is_some() {
		if let TokenType::RightBrace = tokens.peek().unwrap().get_type() {
			break;
		}

		if let Some(node) = match_block_statement(tokens) {
			ast_node.add_child(node);
			continue;
		}
		if let Some(node) = match_statement(tokens) {
			ast_node.add_child(node);
			continue;
		}
	}

	Option::Some(ast_node)
}
