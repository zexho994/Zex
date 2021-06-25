use super::*;
use ast_node::*;
use ast_node_type::*;
use std::collections::HashMap;

/// 执行program，支持表达式计算
pub fn calculate_prog(ast_root: &mut AstNode) -> Option<i32> {
	let mut var_map: HashMap<String, i32> = HashMap::new();
	let mut output: i32 = 0;
	for stmt in ast_root._child.iter_mut() {
		// 根据不同的type进行不同的计算
		match stmt._type {
			AstNodeType::IntDeclaration => {
				output = calculate_int_declare(stmt, &mut var_map);
			}
			AstNodeType::AssignmentStmt => {
				output = calculate_assignment(stmt, &mut var_map);
			}
			AstNodeType::ExpressionStmt => {}
			_ => panic!("not impl more type"),
		}
	}
	Option::Some(output)
}

/// 计算int声明语句中的值变量的值
fn calculate_int_declare(ast: &mut AstNode, var_map: &mut HashMap<String, i32>) -> i32 {
	let id = ast._text.clone();
	let old = *var_map.get(&id).or(Option::Some(&0)).unwrap();
	let l = match ast.get_child(0) {
		Some(node) => calculate_sum(node, var_map),
		None => 0,
	};
	let r = match ast.get_child(1) {
		Some(node) => calculate_sum(node, var_map),
		None => 0,
	};
	var_map.insert(id, old + l + r);
	old + l + r
}

/// 计算赋值语句中变量的值
fn calculate_assignment(ast: &mut AstNode, var_map: &mut HashMap<String, i32>) -> i32 {
	let id = ast._text.clone();
	let l = match ast.get_child(0) {
		Some(node) => calculate_sum(node, var_map),
		None => 0,
	};
	let r = match ast.get_child(1) {
		Some(node) => calculate_sum(node, var_map),
		None => 0,
	};
	var_map.insert(id, l + r);
	l + r
}

fn calculate_sum(ast: &mut AstNode, var_map: &mut HashMap<String, i32>) -> i32 {
	match ast._type {
		AstNodeType::Additive => {
			let l = match ast.get_child(0) {
				Some(node) => calculate_sum(node, var_map),
				None => 0,
			};
			let r = match ast.get_child(1) {
				Some(node) => calculate_sum(node, var_map),
				None => 0,
			};
			l + r
		}
		AstNodeType::Multiplicative => {
			let l = match ast.get_child(0) {
				Some(node) => calculate_sum(node, var_map),
				None => 1,
			};
			let r = match ast.get_child(1) {
				Some(node) => calculate_sum(node, var_map),
				None => 1,
			};
			l * r
		}
		AstNodeType::IntLiteral => ast._text.parse().unwrap(),
		_ => panic!("calculate error, p is {:?}", ast),
	}
}
