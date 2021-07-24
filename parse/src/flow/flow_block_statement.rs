use crate::ast_node::AstNode;
use crate::ast_node::AstNodeType;
use crate::flow::flow_statements::match_statements;
use crate::utils::print_util::print_parse_more2_info;
use lexer::token::token_struct::Tokens;
use lexer::token::token_type::TokenType;

/// <blockStm> ::= { <statements> }
pub fn match_block_statement(tokens: &mut Tokens) -> Option<AstNode> {
	print_parse_more2_info(
		"match block statement,token is ",
		tokens.peek().unwrap(),
		tokens.position(),
	);
	let mut ast_node = AstNode::new(AstNodeType::BlockStmt, "");
	if let TokenType::LeftBrace = tokens.peek().unwrap().get_type() {
		tokens.read();
		if let Some(node) = match_statements(tokens) {
			ast_node.add_child(node);
		}
		if let TokenType::RightBrace = tokens.peek().unwrap().get_type() {
			tokens.read();
		} else {
			panic!("match block right brace error,tokens : {:?}", tokens);
		}
	} else {
		return None;
	}

	Option::Some(ast_node)
}
