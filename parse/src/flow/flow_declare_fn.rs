use super::parse_flow::*;
use crate::ast_node::*;
use crate::flow::flow_statement_block::match_block_statement;
use crate::utils::print_util::print_parse_more2_info;
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

	let mut node;
	if let Some(n) = match_id(tokens) {
		node = AstNode::new(AstNodeType::FnDeclareStmt, &n.get_text());
	} else {
		tokens.set_position(pos);
		return None;
	}

	if let TokenType::LeftBrace = tokens.read().unwrap().get_type() {}

	// match arguments
	node.add_child(AstNode::new(AstNodeType::Arguments, ""));
	if let TokenType::RightBrace = tokens.read().unwrap().get_type() {}

	// match arrow
	node.add_child(AstNode::new(AstNodeType::FnReturn, ""));

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
