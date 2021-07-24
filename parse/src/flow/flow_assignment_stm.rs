use crate::ast_node::AstNode;
use crate::ast_node::AstNodeType;
use crate::parse_flow::match_add_expr;
use crate::parse_flow::match_assignment;
use crate::parse_flow::match_id;
use crate::parse_flow::match_semicolon;
use crate::utils::print_util::print_parse_more2_info;
use lexer::token::token_struct::Tokens;

/// <assignment> ::= <id> <assignment> <exprStm> ;
/// for example:
/// a = 1 + 1 * 2;
///
/// todo: a += 1; a -= 1; a*= 1, a /= 1;
pub fn match_assignment_stm(tokens: &mut Tokens) -> Option<AstNode> {
	print_parse_more2_info(
		"match assignmentStmt,token is ",
		tokens.peek(),
		tokens.position(),
	);
	let mut node = AstNode::new(AstNodeType::AssignmentStmt, "");
	let pos = tokens.position();
	if let Some(n) = match_id(tokens) {
		node.add_child(n)
	} else {
		tokens.set_position(pos);
		return None;
	}

	if let Some(n) = match_assignment(tokens) {
		node.add_child(n);
	} else {
		panic!("match assignment stm error");
	}

	node.add_child(match_add_expr(tokens).unwrap());

	if !match_semicolon(tokens) {
		panic!("match statement,要以分号结束");
	}
	Option::Some(node)
}
