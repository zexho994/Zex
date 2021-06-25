use super::*;
use ast_node::*;
use ast_node_type::*;
use std::collections::HashMap;

pub fn calculate_prog(ast_root: &mut AstNode) -> Option<i32> {
	let mut var_map: HashMap<String, i32> = HashMap::new();
	let output: i32 = 0;
	for stmt in ast_root._child.iter_mut() {
		// 根据不同的type进行不同的计算
		match stmt._type {
			AstNodeType::IntDeclaration => {
				calculate_int_declare(stmt, &mut var_map);
			}
			AstNodeType::AssignmentStmt => {}
			AstNodeType::ExpressionStmt => {}
			_ => panic!("not impl more type"),
		}
	}
	Option::Some(output)
}

/// 计算Ast的值
fn calculate_int_declare(ast: &mut AstNode, var_map: &mut HashMap<String, i32>) -> i32 {
	match ast._type {
		AstNodeType::IntDeclaration => {
			match var_map.contains_key(&ast._text) {
				// 变量存在
				true => {
					let old = *var_map.get(&ast._text).unwrap();
					let l = match ast.get_child(0) {
						Some(node) => calculate_int_declare(node, var_map),
						None => 0,
					};
					let r = match ast.get_child(1) {
						Some(node) => calculate_int_declare(node, var_map),
						None => 0,
					};
					var_map.insert(ast._text.as_str().to_string(), old + l + r);
					old + l + r
				}
				// 变量不存在
				false => {
					let l = match ast.get_child(0) {
						Some(node) => calculate_int_declare(node, var_map),
						None => 0,
					};
					let r = match ast.get_child(1) {
						Some(node) => calculate_int_declare(node, var_map),
						None => 0,
					};
					var_map.insert(ast._text.as_str().to_string(), l + r);
					l + r
				}
			}
		}
		AstNodeType::Additive => {
			let l = match ast.get_child(0) {
				Some(node) => calculate_int_declare(node, var_map),
				None => 0,
			};
			let r = match ast.get_child(1) {
				Some(node) => calculate_int_declare(node, var_map),
				None => 0,
			};
			l + r
		}
		AstNodeType::Multiplicative => {
			let l = match ast.get_child(0) {
				Some(node) => calculate_int_declare(node, var_map),
				None => 0,
			};
			let r = match ast.get_child(1) {
				Some(node) => calculate_int_declare(node, var_map),
				None => 1,
			};
			l * r
		}
		AstNodeType::IntLiteral => ast._text.parse().unwrap(),
		_ => panic!("calculate error, p is {:?}", ast),
	}
}
