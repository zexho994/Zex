use super::flow_echo::*;
use crate::ast_node::AstNode;
use crate::ast_node_type::AstNodeType;
use crate::flow::flow_declare::match_declare;
use crate::flow::flow_statement_assignment::match_assignment_stm;
use crate::parse_flow::*;
use crate::utils::print_util::print_parse_more2_info;
use lexer::token::token_struct::Tokens;

/// 语句类型：分配声明语句，表达式语句，赋值语句
/// <statement> ::= <declare> | <expressionStm> | <assignmentStm>
pub fn match_statement(tokens: &mut Tokens) -> Option<AstNode> {
	print_parse_more2_info(
		"match statement,token is ",
		tokens.peek().unwrap(),
		tokens.position(),
	);
	let mut ast_node = AstNode::new(AstNodeType::Statement, "");
	if let Some(node) = match_echo(tokens) {
		ast_node.add_child(node);
	} else if let Some(node) = match_declare(tokens) {
		ast_node.add_child(node);
	} else if let Some(node) = match_expr_stm(tokens) {
		ast_node.add_child(node);
	} else if let Some(node) = match_assignment_stm(tokens) {
		ast_node.add_child(node);
	} else {
		panic!("match statement error, tokens is {:?}", tokens);
	}

	Option::Some(ast_node)
}
