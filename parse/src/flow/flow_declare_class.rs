use crate::ast_node::*;
use crate::flow::flow_statement_block::match_block_statement;
use crate::parse_flow::match_id;
use crate::utils::print_util::print_parse_more2_info;
use lexer::token::{token_struct::*, token_type::*};

/// match class
///
/// flow:
/// 1. match 'class'
/// 2. match <id>
/// 3. match <blockStmt>
pub fn match_class_declare(tokens: &mut Tokens) -> Option<AstNode> {
	let pos = tokens.position();
	print_parse_more2_info("match class declare,token is ", tokens.peek(), pos);

	// 1.
	if let TokenType::Class = tokens.read().unwrap().get_type() {
	} else {
		tokens.set_position(pos);
		return None;
	}

	// 2
	let mut node;
	if let Some(n) = match_id(tokens) {
		node = AstNode::new(AstNodeType::FnDeclareStmt, &n.get_text());
	} else {
		tokens.set_position(pos);
		return None;
	}

	// 3.
	if let Some(n) = match_block_statement(tokens) {
		node.add_child(n);
	} else {
		tokens.set_position(pos);
		return None;
	}

	println!("\n--------debug mark--------");

	Option::Some(node)
}
