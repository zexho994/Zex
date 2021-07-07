pub const symbol_type_variable: u8 = 1;
pub const symbol_type_function: u8 = 2;

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
	/// 符号值
	symbol_val: Option<i32>,
}

impl Symbol {
	pub fn new(n: String, t: u8, v: Option<i32>) -> Symbol {
		Symbol {
			symbol_name: n,
			symbol_type: t,
			symbol_val: v,
		}
	}

	pub fn getSymbolName(&self) -> String {
		self.symbol_name.clone()
	}
}
