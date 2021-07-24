use crate::ast_node::AstNode;
use crate::ast_node::AstNodeType;
use crate::flow::flow_expression_additive::match_add_expr;
use crate::utils::print_util::print_parse_more2_info;
use lexer::token::token_struct::Tokens;

/// <exprStm> ::= <addExpr>
pub fn match_expr_stm(tokens: &mut Tokens) -> Option<AstNode> {
	print_parse_more2_info("match expr,token is ", tokens.peek(), tokens.position());
	let mut ast_node = AstNode::new(AstNodeType::ExpressionStmt, "");
	if let Some(n) = match_add_expr(tokens) {
		ast_node.add_child(n);
	} else {
		return None;
	}
	Some(ast_node)
}
