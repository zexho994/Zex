use crate::ast_node::AstNode;
use crate::ast_node::AstNodeType;
use crate::flow::flow_expression_multiplicative::match_mul_expr;
use crate::utils::print_util::print_parse_more2_info;
use lexer::token::token_struct::Tokens;
use lexer::token::token_type::TokenType;

/// <addExpr> ::= <mulExpr> | <mulExpr> '+' <addExpr>
pub fn match_add_expr(tokens: &mut Tokens) -> Option<AstNode> {
	print_parse_more2_info("match add,token is ", tokens.peek(), tokens.position());
	let mut child: AstNode;
	if let Some(n) = match_mul_expr(tokens) {
		child = n;
	} else {
		return None;
	}
	loop {
		if tokens.peek().is_none() {
			break;
		}
		match tokens.peek().unwrap().get_type() {
			TokenType::Plus => {
				tokens.read();
				if let Some(t) = match_add_expr(tokens) {
					let mut r = AstNode::new(AstNodeType::Additive, "+");
					r.add_child(child);
					r.add_child(t);
					child = r;
				} else {
					panic!("match add expr failed, the child2 it not be null");
				}
			}
			TokenType::SemiColon | TokenType::RightBrace => {
				break;
			}
			_ => panic!("match add expr failed,token is {:?}", tokens.peek()),
		}
	}
	Option::Some(child)
}
