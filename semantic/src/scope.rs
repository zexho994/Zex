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

	fn add_symbol(&mut self, symbol: Symbol) {
		self.symbol_table.insert(symbol.get_name(), symbol);
	}

	/// 在域中定义符号
	/// 前提条件: 该域中还未存在相同名该的符号，即一个域中不允许相同名称的符号
	pub fn define_symbol(&mut self, symbol: Symbol, scope_stack: &mut ScopeStack) {
		let symbol_id = symbol.get_name();

		if self.is_contain_symbol(&symbol_id, scope_stack) {
			panic!("变量重复声明,symbol = {:?}", symbol);
		}

		self.add_symbol(symbol);
	}

	/// 更新符号
	/// 在域中找到名称为name的符号,并替换成symbol
	/// symbol_id: 被更新的符号名称
	/// symbol: 更新的符号
	/// 约束条件:
	/// 1. 符号以及被声明过
	pub fn update_symbol(
		&mut self,
		name: &String,
		mut symbol: Symbol,
		scope_stack: &mut ScopeStack,
	) {
		// 在本域中查找
		let current_scope = scope_stack.current_scope_mut();
		if current_scope.symbol_table.contains_key(name) {
			let t = current_scope
				.symbol_table
				.get(name)
				.unwrap()
				.get_symbol_type();
			symbol.set_symbol_type(t);
			current_scope.symbol_table.insert(name.to_string(), symbol);
			return;
		}

		let mut parent_name = current_scope.parent_scope_name();
		while parent_name.is_some() {
			let scope = scope_stack
				.find_scope_mut(parent_name.unwrap().clone())
				.unwrap();
			if scope.symbol_table.contains_key(name) {
				let t = scope.symbol_table.get(name).unwrap().get_symbol_type();
				symbol.set_symbol_type(t);
				scope.symbol_table.insert(name.to_string(), symbol);
				return;
			} else {
				parent_name = scope.parent_scope_name();
			}
		}
	}

	pub fn find_symbol(&self, name: &String) -> Option<&Symbol> {
		self.symbol_table.get(name)
	}

	pub fn find_symbol_mut(&mut self, name: &String) -> Option<&mut Symbol> {
		self.symbol_table.get_mut(name)
	}

	/// 移除符号表一个符号，返回被移除的符号
	pub fn remove_symbol(&mut self, name: String) -> Option<Symbol> {
		self.symbol_table.remove(&name)
	}

	/// 查询域以及所有父域中是否包含目标符号
	/// symbol_id: 要查询的符号名称
	/// return
	fn is_contain_symbol(&self, symbol_id: &String, scope_stack: &ScopeStack) -> bool {
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

	pub fn parent_scope_name(&self) -> Option<String> {
		match &self.scope_parent {
			Some(p) => Option::Some(p.clone()),
			None => None,
		}
	}
}
