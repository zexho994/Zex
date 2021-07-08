use super::ast_node_type::*;

#[derive(Debug)]
pub struct AstNode {
    pub _text: String,
    pub _type: AstNodeType,
    pub _child: Vec<AstNode>,
}

/// 设置默认构造参数
impl Default for AstNode {
    fn default() -> AstNode {
        AstNode {
            _text: "".to_string(),
            _type: AstNodeType::None,
            _child: Vec::new(),
        }
    }
}

pub fn new_ast() -> AstNode {
    AstNode {
        _child: Vec::new(),
        _type: AstNodeType::Program,
        _text: String::from("Program"),
    }
}

pub fn new_ast_node(t: AstNodeType, s: String) -> AstNode {
    AstNode {
        _child: Vec::new(),
        _type: t,
        _text: s,
    }
}

impl AstNode {
    pub fn new_id_node(text: String) -> AstNode {
        AstNode {
            _type: AstNodeType::Identifier,
            _text: text,
            ..Default::default()
        }
    }

    pub fn new_intliter_node(text: String) -> AstNode {
        AstNode {
            _type: AstNodeType::IntLiteral,
            _text: text,
            ..Default::default()
        }
    }

    /// 移除一个子节点，转移所有权给调用者
    pub fn remove_child(&mut self, i: usize) -> AstNode {
        self._child.remove(i)
    }

    pub fn get_child(&self, i: usize) -> Option<&AstNode> {
        return if self._child.len() <= i {
            None
        } else {
            self._child.get(i)
        };
    }

    pub fn get_child_mut(&mut self, i: usize) -> Option<&mut AstNode> {
        return if self._child.len() <= i {
            None
        } else {
            self._child.get_mut(i)
        };
    }

    pub fn get_child_text(&mut self, i: usize) -> Option<String> {
        return if self._child.len() <= i {
            None
        } else {
            Option::Some(self._child.get(i).unwrap()._text.clone())
        };
    }

    pub fn add_child(&mut self, child: AstNode) {
        self._child.push(child);
    }

    pub fn print_ast(&self) {
        // let stack = Vec::new();
    }
}
