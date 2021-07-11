use super::scope::*;
use crate::symbol::Symbol;

/// 存储Scope的栈
/// - 在进入一个作用域的时候，执行push，压入新的Scope
/// - 在退出一个作用域的时候,执行pop,弹出顶部的Scope
///
/// current() 表示栈顶，也等同于当前域
#[derive(Debug)]
pub struct ScopeStack {
	seq: u8,
	//  直接管理生命周期，在pop时候就可以直接进行引用回收了
	stack: Vec<Scope>,
}

impl ScopeStack {
	pub fn new() -> ScopeStack {
		ScopeStack {
			seq: 0,
			stack: Vec::new(),
		}
	}

	pub fn len(&self) -> usize {
		self.stack.len()
	}

	pub fn push(&mut self, mut scope: Scope) {
		let seq = self.seq;
		scope.scope_seq = seq;
		scope.scope_name.push_str(seq.to_string().as_str());
		self.stack.push(scope);
		self.seq += 1;
	}

	pub fn pop(&mut self) -> Option<Scope> {
		self.stack.pop()
	}

	pub fn current_scope(&self) -> &Scope {
		let len = self.stack.len();
		self.stack.get(len - 1).unwrap()
	}

	pub fn current_scope_mut(&mut self) -> &mut Scope {
		let len = self.stack.len();
		self.stack.get_mut(len - 1).unwrap()
	}

	pub fn find_scope(&self, scope_name: &String) -> Option<&Scope> {
		let name = scope_name.as_str();
		for scope in self.stack.iter() {
			if scope.scope_name == name {
				return Option::Some(scope);
			}
		}
		None
	}

	pub fn find_scope_mut(&mut self, scope_name: String) -> Option<&mut Scope> {
		for scope in self.stack.iter_mut() {
			if scope.scope_name == scope_name {
				return Option::Some(scope);
			}
		}
		None
	}

	pub fn update_symbol(&mut self, name: &String, mut symbol: Symbol) {
		// 在本域中查找
		let current_scope = self.current_scope_mut();
		if let Some(k) = current_scope.symbol_table.get(name) {
			let t = k.get_symbol_type();
			symbol.set_symbol_type(t);
			current_scope.symbol_table.insert(name.to_string(), symbol);
			return;
		}

		let mut parent_name = current_scope.parent_scope_name();
		while parent_name.is_some() {
			let scope = self.find_scope_mut(parent_name.unwrap().clone()).unwrap();
			if let Some(k) = scope.symbol_table.get(name) {
				let t = k.get_symbol_type();
				symbol.set_symbol_type(t);
				scope.symbol_table.insert(name.to_string(), symbol);
				return;
			} else {
				parent_name = scope.parent_scope_name();
			}
		}
		
		panic!("update symbol failure");
	}
}
