pub use super::ast_node_type::*;

#[derive(Debug)]
pub struct AstNode {
    _text: String,
    _type: AstNodeType,
    _child: Vec<AstNode>,
}

impl AstNode {
    pub fn new(t: AstNodeType, s: &str) -> AstNode {
        AstNode {
            _text: s.to_string(),
            _type: t,
            _child: Vec::new(),
        }
    }

    pub fn get_text(&self) -> String {
        self._text.clone()
    }

    pub fn get_type(&self) -> &AstNodeType {
        &self._type
    }

    pub fn get_child_vec(&self) -> &Vec<AstNode> {
        &self._child
    }

    pub fn get_child_vec_mut(&mut self) -> &mut Vec<AstNode> {
        &mut self._child
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

    // 获取id子节点
    pub fn get_id_child(&self) -> &AstNode {
        match self.get_type() {
            AstNodeType::FnDeclareStmt => self.get_child(0).unwrap(),
            _ => {
                panic!("get id child error")
            }
        }
    }

    pub fn remove_fn_child(&mut self) -> (AstNode, AstNode, AstNode) {
        (
            self.remove_child(0),
            self.remove_child(0),
            self.remove_child(0),
        )
    }

    pub fn get_class_child(&mut self) -> &mut AstNode {
        self.get_child_mut(0).unwrap()
    }

    pub fn child_count(&self) -> usize {
        self._child.len()
    }
}

// trait Calculate {
//     // 计算节点的值
//     // 如果节点为additive，遍历求值
//     fn calculate() -> Option<u32>;
//     fn calculate_sum() -> u32;
//     fn calculate_add() -> u32;
//     fn calculate_expr() -> u32;
// }
impl AstNode {
    /// int a = 1;
    /// a = 2;
    ///
    /// 出发计算的节点的可能类型:
    ///
    /// 1.
    pub fn calculate(node: &AstNode) -> i32 {
        AstNode::print_calculate_info("calculate");
        match node._type {
            AstNodeType::Identifier => {
                panic!("calculate identifier")
            }
            AstNodeType::ExpressionStmt => AstNode::calculate_expr(node),
            AstNodeType::IntLiteral => node._text.parse().unwrap(),
            AstNodeType::Additive => AstNode::calculate_number(node),
            _ => panic!("calculate,{:?}", node),
        }
    }

    /// expr -> add
    fn calculate_expr(node: &AstNode) -> i32 {
        AstNode::print_calculate_info("calculate expression");
        let child = node.get_child(0).unwrap();
        AstNode::calculate_number(child)
    }

    fn calculate_number(node: &AstNode) -> i32 {
        AstNode::print_calculate_info("calculate_number");
        match node._type {
            AstNodeType::Additive => {
                let mut left: i32 = 0;
                let mut right: i32 = 0;
                if let Some(left_child) = node.get_child(0) {
                    left = AstNode::calculate_number(left_child);
                }
                if let Some(right_child) = node.get_child(1) {
                    right = AstNode::calculate_number(right_child);
                }
                left + right
            }
            AstNodeType::Multiplicative => {
                let mut left: i32 = 0;
                let mut right: i32 = 1;
                if let Some(left_child) = node.get_child(0) {
                    left = AstNode::calculate_number(left_child);
                }
                if let Some(right_child) = node.get_child(1) {
                    right = AstNode::calculate_number(right_child);
                }
                left * right
            }
            AstNodeType::Primary => AstNode::calculate_primary(node),
            AstNodeType::IntLiteral => node._text.parse().unwrap(),
            _ => panic!("calculate {:?} failed", node),
        }
    }

    fn calculate_primary(node: &AstNode) -> i32 {
        match node._type {
            AstNodeType::Identifier => {
                panic!("calculate primary,{:?}", node)
            }
            AstNodeType::IntLiteral => {
                return node._text.parse().unwrap();
            }
            _ => panic!("calculate primary,{:?}", node),
        }
    }

    fn print_calculate_info(_msg: &str) {
        // println!("[info][calculate] {}", msg)
    }
}
