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
    pub fn get_child(&mut self, i: usize) -> Option<&mut AstNode> {
        return if self._child.len() <= i {
            None
        } else {
            self._child.get_mut(i)
        };
    }

    pub fn add_child(&mut self, child: AstNode) {
        self._child.push(child);
    }

    pub fn print_ast(&self) {
        // let stack = Vec::new();
    }
}
