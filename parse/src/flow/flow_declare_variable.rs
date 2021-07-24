use crate::flow::flow_statement_expression::match_expr_stm;
use crate::ast_node::AstNode;
use crate::ast_node_type::AstNodeType;
use crate::parse_flow::match_assignment;
use crate::parse_flow::match_id;
use crate::parse_flow::match_semicolon;
use crate::parse_flow::match_type;
use crate::utils::print_util::print_parse_more2_info;
use lexer::token::token_struct::Tokens;

/// <varDeclare> ::= <type> <id> <assign> <exprStm> | <type> <id> ;
pub fn match_var_declare(tokens: &mut Tokens) -> Option<AstNode> {
	print_parse_more2_info(
		"match var declare,token is ",
		tokens.peek(),
		tokens.position(),
	);
	let mut node = AstNode::new(AstNodeType::VarDeclareStmt, "");
	if let Some(t) = match_type(tokens) {
		node.add_child(t);
	} else {
		return None;
	}

	if let Some(t) = match_id(tokens) {
		node.add_child(t);
	} else {
		return None;
	}

	if let Some(t) = match_assignment(tokens) {
		node.add_child(t);
		if let Some(t) = match_expr_stm(tokens) {
			node.add_child(t);
		} else {
			panic!("match var declare error, assignment 符号后面expr stm 不能为空");
		}
	}

	if !match_semicolon(tokens) {
		panic!("match statement,要以分号结束");
	}
	return Some(node);
}
