use super::symbol::*;
use std::collections::HashMap;

/// 存储Scope的栈
/// - 在进入一个作用域的时候，执行push，压入新的Scope
/// - 在退出一个作用域的时候,执行pop,弹出顶部的Scope
///
/// current() 表示栈顶，也等同于当前域
#[derive(Debug)]
pub struct ScopeStack {
	pub seq: u8,
	//  直接管理生命周期，在pop时候就可以直接进行引用回收了
	pub stack: Vec<Scope>,
}

impl ScopeStack {
	pub fn new() -> ScopeStack {
		ScopeStack {
			seq: 0,
			stack: Vec::new(),
		}
	}

	pub fn push(&mut self, mut scope: Scope) {
		let seq = self.seq;
		scope.scope_seq = seq;
		scope.scope_name.push_str(seq.to_string().as_str());
		self.stack.push(scope);
		self.seq += 1;
	}

	pub fn pop(&mut self) {
		self.stack.pop();
	}

	pub fn current(&mut self) -> Option<&mut Scope> {
		let len = self.stack.len();
		self.stack.get_mut(len - 1)
	}

	pub fn find_scope(&mut self, scope_name: &str) -> Option<&mut Scope> {
		for scope in self.stack.iter_mut() {
			if scope.scope_name == scope_name {
				return Option::Some(scope);
			}
		}
		None
	}
}

#[derive(Debug)]
pub struct Scope {
	pub scope_seq: u8,
	pub scope_name: String,
	// 1.全局，2临时
	pub scope_type: u8,
	pub scope_parent: Option<String>,
	pub scope_children: HashMap<String, Scope>,
	// 符号表，存储此域下所有符号,K:符号名,V:符号对象
	pub symbol_table: HashMap<String, Option<Symbol>>,
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

	// 压入一个符号
	pub fn push_symbol(&mut self, symbol: Symbol) {
		self.symbol_table
			.insert(symbol.get_symbol_name(), Option::Some(symbol));
	}

	pub fn current_has_symbol(&self, k: String) -> bool {
		self.symbol_table.contains_key(&k)
	}

	pub fn update_symbol_val(&mut self, var_name: String, var_val: Option<Symbol>) {
		self.symbol_table.insert(var_name, var_val);
	}

	pub fn parent_scope_name(&self) -> Option<String> {
		match &self.scope_parent {
			Some(p) => Option::Some(p.clone()),
			None => None,
		}
	}
}
