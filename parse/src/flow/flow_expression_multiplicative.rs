use crate::ast_node::AstNode;
use crate::ast_node::AstNodeType;
use crate::parse_flow::match_primary;
use crate::utils::print_util::print_parse_more2_info;
use lexer::token::token_struct::Tokens;
use lexer::token::token_type::TokenType;

/// <mulExpr> ::= <primary> | <primary> '*' <mulExpr>
pub fn match_mul_expr(tokens: &mut Tokens) -> Option<AstNode> {
	let pos = tokens.position();
	print_parse_more2_info("match mul,token is ", tokens.peek(), tokens.position());
	let mut child: AstNode;

	if let Some(n) = match_primary(tokens) {
		child = n;
	} else {
		return None;
	}

	loop {
		if tokens.peek().is_none() {
			break;
		}
		match tokens.peek().unwrap().get_type() {
			TokenType::Star => {
				tokens.read().unwrap();
				if let Some(p) = match_primary(tokens) {
					let mut r = AstNode::new(AstNodeType::Multiplicative, "*");
					r.add_child(child);
					r.add_child(p);
					child = r;
				} else {
					panic!("match mul expr error");
				}
			}
			TokenType::Plus | TokenType::SemiColon | TokenType::RightBrace => {
				break;
			}
			_ => {
				tokens.set_position(pos);
				return None;
			}
		}
	}
	Option::Some(child)
}
