use super::parse_flow::*;
use super::flow_block_statement::*;
use crate::ast_node::*;
use crate::utils::print_util::*;
use lexer::token::{token_struct::*, token_type::*};

/// match fn declare,匹配方法声明语句
/// fn foo (int a) -> <type> {}
/// fn <id> (<assignStmt>*) (-> <type>)? {}
pub fn match_fn_declare(tokens: &mut Tokens) -> Option<AstNode> {
	print_parse_more2_info(
		"match fn declare,token is ",
		tokens.peek(),
		tokens.position(),
	);
	let pos = tokens.position();

	if let TokenType::Fn = tokens.read().unwrap().get_type() {
	} else {
		tokens.set_position(pos);
		return None;
	}

	let mut node = AstNode::new(AstNodeType::FnDeclareStmt, "fn");

	// match id
	if let Some(n) = match_id(tokens) {
		node.add_child(n);
	} else {
		tokens.set_position(pos);
		return None;
	}

	if let TokenType::LeftBrace = tokens.read().unwrap().get_type() {}
	// match arguments
	if let TokenType::RightBrace = tokens.read().unwrap().get_type() {}

	// match arrow

	print_parse_more2_info(
		"match fn declare,step 7 match block statement,token is ",
		tokens.peek().unwrap(),
		tokens.position(),
	);
	if let Some(n) = match_block_statement(tokens) {
		node.add_child(n);
	} else {
		tokens.set_position(pos);
		return None;
	}
	return Option::Some(node);
}