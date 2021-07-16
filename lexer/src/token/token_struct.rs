use super::token_type::*;

#[derive(Debug)]
pub struct Token {
	_type: TokenType,
	pub text: String,
}

#[derive(Debug)]
pub struct Tokens {
	data: Vec<Token>,
	pos: usize,
}

impl Token {
	pub fn new(t: TokenType, s: String) -> Token {
		Token { _type: t, text: s }
	}
	pub fn set_type(&mut self, t: TokenType) {
		self._type = t;
	}

	pub fn get_type(&self) -> &TokenType {
		&self._type
	}
}

impl Tokens {
	pub fn new() -> Tokens {
		Tokens {
			data: Vec::new(),
			pos: 0,
		}
	}

	pub fn add_token(&mut self, t: Token) {
		self.data.push(t);
	}

	pub fn get_child_idx(&self, idx: usize) -> Option<&Token> {
		if idx >= self.count() {
			None
		} else {
			Option::Some(&self.data[idx])
		}
	}

	pub fn peek(&self) -> Option<&Token> {
		if self.pos == self.count() {
			return None;
		}
		Option::Some(&self.data[self.pos])
	}

	pub fn read(&mut self) -> Option<&mut Token> {
		if self.pos == self.count() {
			return None;
		}
		self.pos += 1;
		self.data.get_mut(self.pos - 1)
	}

	pub fn count(&self) -> usize {
		self.data.len()
	}

	pub fn position(&self) -> usize {
		self.pos
	}

	pub fn set_position(&mut self, p: usize) {
		if p >= self.count() {
			panic!("set position error, p is more than count")
		}
		self.pos = p;
	}
}
