use super::ast_node::AstNode;
use super::ast_node_type::AstNodeType;
use super::scope::Scope;
use super::scope_stack::ScopeStack;
use super::symbol::*;

// 1. 创建全局域
// 2. 处理流程
// 3. 保存，退出
pub fn visit_program(ast_node: &mut AstNode) {
	print_info("visit program");
	// 创建作用域栈
	let mut scope_stack = ScopeStack::new();
	// 创建全局域
	let global_scope = Scope::new_global();
	scope_stack.push(global_scope);

	for child in ast_node._child.iter_mut() {
		visit_statements(child, &mut scope_stack);
	}

	// 退出全局域
	scope_stack.pop();
}

/// ast_node type = AstNodeType::Statements
fn visit_statements(ast_node: &mut parse::ast_node::AstNode, scope_stack: &mut ScopeStack) {
	print_info("visit statements");

	for child in ast_node._child.iter_mut() {
		match child._type {
			AstNodeType::BlockStmt => {
				visit_block_statement(child, scope_stack);
			}
			AstNodeType::Statement => {
				visit_statement(child, scope_stack);
			}
			_ => {}
		}
	}
}

/// ast_node type = AstNodeType::BlockStmt
/// block域的父域是上一层域
fn visit_block_statement(ast_node: &mut parse::ast_node::AstNode, scope_stack: &mut ScopeStack) {
	print_info("visit block statement");
	let pre_scope = scope_stack.current().unwrap();
	let block_scope: Scope = Scope::new_local(pre_scope.scope_name.clone());
	scope_stack.push(block_scope);

	visit_statements(ast_node.get_child_mut(0).unwrap(), scope_stack);

	scope_stack.pop();
}

/// ast_node type = AstNodeType::Statement
///
/// statement 的类型
/// 1. echo 语句
/// 2. varDeclareStmt 声明语句
/// 3. assignmentStmt 赋值语句
/// 4. expressionStmt 表达式语句
fn visit_statement(ast_node: &mut parse::ast_node::AstNode, scope_stack: &mut ScopeStack) {
	print_info("visit statement");

	for child in ast_node._child.iter_mut() {
		match child._type {
			AstNodeType::Echo => {
				visit_echo(child, scope_stack);
			}
			AstNodeType::VarDeclareStmt => {
				visit_var_declare_stmt(child, scope_stack);
			}
			AstNodeType::AssignmentStmt => {
				visit_assignment_stmt(child, scope_stack);
			}
			AstNodeType::ExpressionStmt => {}
			_ => panic!("visit statement error"),
		}
	}
}

/// ### varDeclareStmt结构
/// type, id, assignment, expressionStm
///
/// ## 变量声明语句检查规则
/// 1. 变量的作用域存在本scope中
/// 2. 合法scope中不能有重名变量
///    2.1. 本scope中不能有
///    2.2. 递归向上的所有scope中都不能有
/// 3. 保存变量的值的AstNode
///
/// ```text
/// int a = 1;
/// {
/// 	int a = 2;  //error,上级域中已经有a
/// 	int b = 2;  
/// }
/// int b = 3;   //success,块中的b已经失效了
/// ```
///
fn visit_var_declare_stmt(ast_node: &mut parse::ast_node::AstNode, scope_stack: &mut ScopeStack) {
	print_info("visit var declare stmt");
	let var_id: String;
	let mut parent_name: Option<String> = None;

	// 在本域中查找
	{
		let current_scope = scope_stack.current().unwrap();
		var_id = ast_node.get_child_text(1).unwrap();
		if current_scope.current_has_symbol(var_id.clone()) {
			print_panic("变量重复声明")
		}
		parent_name = current_scope.parent_scope_name();
	}

	// 在所有父域中查找
	while parent_name.is_some() {
		let scope = scope_stack.find_scope(&parent_name.unwrap()).unwrap();
		if scope.current_has_symbol(var_id.clone()) {
			print_panic("变量重复声明")
		}
		parent_name = scope.parent_scope_name();
	}

	// 添加变量到本域符号表中
	let current_scope = scope_stack.current().unwrap();
	let ast_node = ast_node.remove_child(2);
	let variable = Symbol::new(var_id, SYMBOL_TYPE_VARIABLE, Option::Some(ast_node));
	current_scope.push_symbol(variable);
}

/// ast_node type = AstNodeType::AssignmentStmt
/// 赋值语句将更新已有的变量值:
/// 1. 确保变量已经声明
/// 2. 更新变量的值
///
fn visit_assignment_stmt(ast_node: &mut AstNode, scope_stack: &mut ScopeStack) {
	print_info("visit var declare stmt");
	let var_id: String = ast_node.get_child_text(0).unwrap();
	let expr_node = ast_node.remove_child(2);
	let mut parent_name: Option<String> = scope_stack.current().unwrap().parent_scope_name();

	// 在本域中查找
	let mut current_scope = scope_stack.current().unwrap();
	if current_scope.current_has_symbol(var_id.clone()) {
		let mut symbol = current_scope.remove_symbol(var_id.clone()).unwrap();
		symbol.set_symbol_value(Option::Some(expr_node));
		current_scope.update_symbol_val(var_id.clone(), Option::Some(symbol));
		return;
	}

	// 在所有父域中查找
	//
	// 流程：
	// 查找当前域
	// if 当前域中存在
	// 		then -> update 变量
	// else 当前域中不存在
	// 		then -> 到父级域中找
	//
	while parent_name.is_some() {
		// 获取父域
		let scope = scope_stack.find_scope(&parent_name.unwrap()).unwrap();
		// 当前域中是否存在该变量
		if scope.current_has_symbol(var_id.clone()) {
			let mut symbol = scope.remove_symbol(var_id.clone()).unwrap();
			symbol.set_symbol_value(Option::Some(expr_node));
			scope.update_symbol_val(var_id.clone(), None);
			return;
		}
		parent_name = scope.parent_scope_name();
	}
}

/// echo hack
/// 获取要输出的节点，计算值
/// 要输出的值必需在合法作用域中，且在合法生命周期内
/// 如果要输出的字面值类型，直接打印即可
/// 如果要输出的是其他类型，需要进行计算,计算规则:
/// - Identifier: 从本域开始向超查找，找到对应变量，然后进行求值计算
/// - ExpressionStmt: todo
fn visit_echo(ast_node: &mut AstNode, scope_stack: &mut ScopeStack) {
	let target = ast_node.get_child_mut(0).unwrap();

	match target._type {
		AstNodeType::Identifier => {
			visit_identifier(target, scope_stack);
		}
		AstNodeType::IntLiteral => {
			println!("{}", target._text.clone());
		}
		_ => print_panic_extend("todo", target),
	}
}

/// ### identifier可能的场景
/// 符号表中:k=id,v=ast_node
/// ast_node's type = expressionStmt
/// 根据id的text在符号表从下到上遍历寻找对应符号以及值
fn visit_identifier(id_node: &mut AstNode, scope_stack: &mut ScopeStack) {
	print_info("visit identifier");
	// println!("id_node: {:?}", id_node);
	let id = id_node._text.clone();
	let current = scope_stack.current().unwrap();
	// println!("id: {:?}", id);
	// println!("current scope: {:?}", current);
	if current.current_has_symbol(id.clone()) {
		let symbol = current.get_symbol(id.clone()).as_ref().unwrap();
		let node = symbol.get_symbol_val().as_ref().unwrap();
		// println!("symbol val is {:?}", node);
		let text = node._text.clone();
		println!("{}", text);
	} else {
		let mut parent_name = current.parent_scope_name();
		while parent_name.is_some() {
			// 获取父域
			let scope = scope_stack.find_scope(&parent_name.unwrap()).unwrap();
			// 当前域中是否存在该变量
			if scope.current_has_symbol(id.clone()) {}
			parent_name = scope.parent_scope_name();
		}
	}
}

fn print_info(msg: &str) {
	println!("[info][ast_visit]: {}", msg);
}

// fn print_info_extend<T: std::fmt::Debug>(msg: &str, t: &T) {
// 	println!("[info][ast_visit]: {},t = {:?}", msg, t);
// }

fn print_panic(msg: &str) {
	panic!("[error][ast_visit] {}", msg);
}

fn print_panic_extend<T: std::fmt::Debug>(msg: &str, t: &T) {
	panic!("[error][ast_visit] {}, t = {:?}", msg, t);
}
