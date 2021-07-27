use super::symbol::*;
use crate::scope_stack::ScopeStack;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Scope {
	scope_seq: u8,
	scope_name: String,
	// 1.全局，2临时
	scope_type: u8,
	scope_parent: Option<String>,
	scope_children: HashMap<String, Scope>,
	// 符号表，存储此域下所有符号,K:符号名,V:符号对象
	symbol_table: HashMap<String, Symbol>,
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

	pub fn set_seq(&mut self, s: u8) {
		self.scope_seq = s;
	}

	pub fn get_seq(&self) -> u8 {
		if self.scope_seq == DEFAULT_SEQ {
			panic!("scope un initial");
		}
		return self.scope_seq;
	}

	pub fn get_scope_name(&self) -> String {
		self.scope_name.clone()
	}

	pub fn set_scope_name(&mut self, str: String) {
		self.scope_name = str;
	}

	pub fn append_scope_name(&mut self, str: &String) {
		self.scope_name.push_str(str);
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
		symbol_name: &String,
		new_symbol: Symbol,
		scope_stack: &mut ScopeStack,
	) {
		println!("update symbol");

		if !self.is_contain_symbol(symbol_name, scope_stack) {
			panic!("变量未声明,symbol= {}", symbol_name);
		}

		if self.find_symbol(symbol_name).is_some() {
			Scope::update_in_current(self, symbol_name, new_symbol);
		} else {
			Scope::update_in_parent(self, symbol_name, new_symbol, scope_stack)
		}
	}

	fn update_in_current(scope: &mut Scope, symbol_name: &String, mut new_symbol: Symbol) {
		if let Some(symbol) = scope.find_symbol(symbol_name) {
			let origin_type = symbol.get_symbol_type();
			new_symbol.set_symbol_type(origin_type);
			scope.add_symbol(new_symbol);
			return;
		}
	}

	fn update_in_parent(
		scope: &mut Scope,
		symbol_name: &String,
		mut new_symbol: Symbol,
		scope_stack: &mut ScopeStack,
	) {
		let mut parent_name = scope.parent_scope_name();
		while parent_name.is_some() {
			let scope = scope_stack
				.find_scope_mut(parent_name.unwrap().clone())
				.unwrap();
			if let Some(s) = scope.find_symbol(&symbol_name) {
				let origin_type = s.get_symbol_type();
				new_symbol.set_symbol_type(origin_type);
				scope.add_symbol(new_symbol);
				return;
			}
			parent_name = scope.parent_scope_name();
		}
	}

	pub fn loop_find_symbol(id: String, scope_stack: &ScopeStack) -> Option<&Symbol> {
		let mut target_symbol: Option<&Symbol> = None;

		if let Some(symbol) = scope_stack.current_scope().find_symbol(&id) {
			target_symbol = Some(symbol);
		} else {
			let mut parent_name = scope_stack.current_scope().parent_scope_name();
			//父域中查找符号
			while parent_name.is_some() {
				let scope = scope_stack.find_scope(&parent_name.unwrap()).unwrap();
				if let Some(symbol) = scope.find_symbol(&id) {
					target_symbol = Some(symbol);
					break;
				}
				parent_name = scope.parent_scope_name();
			}
		}

		target_symbol
	}

	fn find_symbol(&self, name: &String) -> Option<&Symbol> {
		self.symbol_table.get(name)
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
