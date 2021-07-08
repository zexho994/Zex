use parse::ast_node::AstNode;

pub const SYMBOL_TYPE_VARIABLE: u8 = 1;
pub const SYMBOL_TYPE_FUNCTION: u8 = 2;

/// 符号可以是变量，函数，方法
///
/// ### 创建时机:
/// - declareStmt
/// -
///
/// ### 更新时机:
/// - assignmentStm
///
#[derive(Debug)]
pub struct Symbol {
	/// 符号名称
	symbol_name: String,
	/// 符号类型
	symbol_type: u8,
	/// 符号值,存储AstNode对象
	symbol_val: Option<AstNode>,
}

impl Symbol {
	pub fn new(n: String, t: u8, v: Option<AstNode>) -> Symbol {
		Symbol {
			symbol_name: n,
			symbol_type: t,
			symbol_val: v,
		}
	}

	pub fn get_symbol_name(&self) -> String {
		self.symbol_name.clone()
	}

	pub fn set_symbol_value(&mut self, v: Option<AstNode>) {
		self.symbol_val = v;
	}

	pub fn get_symbol_val(&self) -> &Option<AstNode> {
		&self.symbol_val
	}
}
