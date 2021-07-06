use std::collections::HashMap;

#[derive(Debug)]
pub struct ScopeStack {
	pub seq: u16,
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
		self.stack.get_mut(len-1)
	}
}

#[derive(Debug)]
pub struct Scope {
	pub scope_seq: u16,
	pub scope_name: String,
	// 1.全局，2临时
	pub scope_type: u8,
	pub scope_parent: Option<String>,
	pub scope_children: HashMap<String, Scope>,
}

impl Scope {
	pub fn new_global() -> Scope {
		Scope {
			scope_seq: 0,
			scope_name: "scope_global_".to_string(),
			scope_type: 1,
			scope_parent: Option::None,
			scope_children: HashMap::new(),
		}
	}

	pub fn new_local(parent: String) -> Scope {
		Scope {
			scope_seq: 0,
			scope_name: "scope_local_".to_string(),
			scope_type: 2,
			scope_parent: Option::Some(parent),
			scope_children: HashMap::new(),
		}
	}
}
