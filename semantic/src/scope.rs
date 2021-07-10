use super::symbol::*;
use crate::scope_stack::ScopeStack;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Scope {
	pub scope_seq: u8,
	pub scope_name: String,
	// 1.全局，2临时
	pub scope_type: u8,
	pub scope_parent: Option<String>,
	pub scope_children: HashMap<String, Scope>,
	// 符号表，存储此域下所有符号,K:符号名,V:符号对象
	pub symbol_table: HashMap<String, Symbol>,
}

// 默认的序号为0，实际的序号在stack push时候才会赋值
const DEFAULT_SEQ: u8 = 0;
const GLOBAL_SCOPE_TYPE: u8 = 1;
const LOCAL_SCOPE_TYPE: u8 = 2;
const DEFAULT_GLOBAL_NAME: &str = "scope_global_";
const DEFAULT_LOCAL_NAME: &str = "scope_local_";

impl Scope {
	pub fn new_global() -> Scope {
		Scope {
			scope_seq: DEFAULT_SEQ,
			scope_name: DEFAULT_GLOBAL_NAME.to_string(),
			scope_type: GLOBAL_SCOPE_TYPE,
			scope_parent: Option::None,
			scope_children: HashMap::new(),
			symbol_table: HashMap::new(),
		}
	}

	pub fn new_local(parent: String) -> Scope {
		Scope {
			scope_seq: DEFAULT_SEQ,
			scope_name: DEFAULT_LOCAL_NAME.to_string(),
			scope_type: LOCAL_SCOPE_TYPE,
			scope_parent: Option::Some(parent),
			scope_children: HashMap::new(),
			symbol_table: HashMap::new(),
		}
	}

	pub fn get_scope_name(&self) -> &str {
		self.scope_name.as_str()
	}

	pub fn define_symbol(&mut self, symbol: Symbol) {
		self.symbol_table.insert(symbol.get_name(), symbol);
	}

	pub fn find_symbol(&self, name: &String) -> Option<&Symbol> {
		self.symbol_table.get(name)
	}

	/// 移除符号表一个符号，返回被移除的符号
	pub fn remove_symbol(&mut self, name: String) -> Option<Symbol> {
		self.symbol_table.remove(&name)
	}

	/// 查询域以及所有父域中是否包含目标符号
	/// symbol_id: 要查询的符号名称
	pub fn is_contain_symbol(&self, symbol_id: &String, scope_stack: &ScopeStack) -> bool {
		if self.find_symbol(&symbol_id).is_some() {
			return true;
		}

		let mut parent_name = self.parent_scope_name();
		while let Some(name) = parent_name {
			let scope = scope_stack.find_scope(&name).unwrap();
			if scope.find_symbol(&symbol_id).is_some() {
				return true;
			}
			parent_name = scope.parent_scope_name();
		}
		false
	}

	pub fn update_symbol_val(&mut self, var_name: String, var_val: Symbol) {
		self.symbol_table.insert(var_name, var_val);
	}

	pub fn parent_scope_name(&self) -> Option<String> {
		match &self.scope_parent {
			Some(p) => Option::Some(p.clone()),
			None => None,
		}
	}
}
