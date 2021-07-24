use crate::ast_node::AstNode;
use crate::ast_node::AstNodeType;
use crate::parse_flow::match_semicolon;
use crate::utils::print_util::print_parse_more2_info;
use lexer::token::token_struct::Tokens;
use lexer::token::token_type::TokenType;

/// <echo> ::= echo ( <id> | <intLiteral> )
pub fn match_echo(tokens: &mut Tokens) -> Option<AstNode> {
	print_parse_more2_info("match echo,token is ", tokens.peek(), tokens.position());
	if let TokenType::Echo = tokens.peek().unwrap().get_type() {
		tokens.read();
		let mut node = AstNode::new(AstNodeType::Echo, "");
		match tokens.peek().unwrap().get_type() {
			TokenType::Identifier => {
				let t = tokens.read().unwrap();
				let child = AstNode::new(AstNodeType::Identifier, t.get_text().as_str());
				node.add_child(child);
			}
			TokenType::IntLiteral => {
				let t = tokens.read().unwrap();
				let child = AstNode::new(AstNodeType::IntLiteral, t.get_text().as_str());
				node.add_child(child);
			}
			_ => panic!(
				"echo error, token is {:?}, pos is {}",
				tokens.peek(),
				tokens.position()
			),
		}
		if !match_semicolon(tokens) {
			panic!("match statement,要以分号结束");
		}
		return Option::Some(node);
	}

	None
}
