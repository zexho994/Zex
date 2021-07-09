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

	/// stack len
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

	pub fn current_scope(&self) -> Option<&Scope> {
		let len = self.stack.len();
		self.stack.get(len - 1)
	}

	pub fn current_scope_mut(&mut self) -> Option<&mut Scope> {
		let len = self.stack.len();
		self.stack.get_mut(len - 1)
	}

	pub fn find_scope(&self, scope_name: String) -> Option<&Scope> {
		for scope in self.stack.iter() {
			if scope.scope_name == scope_name {
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
}
