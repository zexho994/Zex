use crate::ast_node::AstNode;

pub fn echo_int_literal(_ast_node: &mut AstNode) {
	println!("\n[info]<visitor>: {}", _ast_node.get_text().clone())
}

pub fn print_info(_msg: &str) {
	println!("\n[info]<visitor>: {}", _msg);
}

pub fn print_info_extend<T: std::fmt::Debug>(_msg: &str, _t: &T) {
	println!("\n[info]<visitor>: {},t = {:?}", _msg, _t);
}

pub fn print_panic(msg: &str) {
	panic!("\n[error]<visitor> {}", msg);
}

pub fn print_panic_more<T: std::fmt::Debug>(msg: &str, t: &T) {
	panic!("\n[error]<visitor> msg = {}, t = {:?}", msg, t);
}
