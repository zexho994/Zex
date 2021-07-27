use parse::ast_node::AstNode;

pub const UN_DEFINED: u8 = 0;
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

	pub fn get_name(&self) -> String {
		self.symbol_name.clone()
	}

	pub fn get_symbol_type(&self) -> u8 {
		self.symbol_type
	}

	pub fn set_symbol_type(&mut self, symbol_type: u8) {
		self.symbol_type = symbol_type;
	}

	pub fn set_ast_node(&mut self, v: Option<AstNode>) {
		self.symbol_val = v;
	}

	pub fn get_ast_node(&self) -> Option<&AstNode> {
		self.symbol_val.as_ref()
	}
}
