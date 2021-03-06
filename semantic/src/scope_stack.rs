use super::scope::*;

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
		scope.set_seq(seq);
		scope.append_scope_name(&seq.to_string());
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
			if scope.get_scope_name() == name {
				return Option::Some(scope);
			}
		}
		None
	}

	pub fn find_scope_mut(&mut self, scope_name: String) -> Option<&mut Scope> {
		for scope in self.stack.iter_mut() {
			if scope.get_scope_name() == scope_name {
				return Option::Some(scope);
			}
		}
		None
	}
}
