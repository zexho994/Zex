use super::ast_node::AstNode;
use super::ast_node_type::AstNodeType;
use super::scope::Scope;
use super::scope::ScopeStack;
use super::symbol::*;

// 1. 创建全局域
// 2. 处理流程
// 3. 保存，退出
pub fn visit_program(ast_node: &mut AstNode) {
	print_visit_info("visit program");
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

fn visit_statements(ast_node: &mut parse::ast_node::AstNode, scope_stack:  mut ScopeStack) {
	print_visit_info("visit statements");

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

/// block域的父域是上一层域
fn visit_block_statement(ast_node: &mut parse::ast_node::AstNode, scope_stack: &mut ScopeStack) {
	print_visit_info("visit block statement");
	let pre_scope = scope_stack.current().unwrap();
	let block_scope: Scope = Scope::new_local(pre_scope.scope_name.clone());
	scope_stack.push(block_scope);

	visit_statements(ast_node.get_child_mut(0).unwrap(), scope_stack);

	scope_stack.pop();
}

fn visit_statement(ast_node: &mut parse::ast_node::AstNode, scope_stack: &mut ScopeStack) {
	print_visit_info("visit statement");

	for child in ast_node._child.iter_mut() {
		match child._type {
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
	print_visit_info("visit var declare stmt");
	let var_id: String;
	let mut parent_name: Option<String> = None;

	// 在本域中查找
	{
		let current_scope = scope_stack.current().unwrap();
		var_id = ast_node.get_child_text(1).unwrap();
		if current_scope.current_has_symbol(var_id.clone()) {
			panic_visit_info("变量重复声明")
		}
		parent_name = current_scope.parent_scope_name();
	}

	// 在所有父域中查找
	while parent_name.is_some() {
		let scope = scope_stack.find_scope(&parent_name.unwrap()).unwrap();
		if scope.current_has_symbol(var_id.clone()) {
			panic_visit_info("变量重复声明")
		}
		parent_name = scope.parent_scope_name();
	}

	// 添加变量到本域符号表中
	let current_scope = scope_stack.current().unwrap();
	let val_node_ref = ast_node.get_child(2).unwrap();
	let val_node = Option::Some(*val_node_ref);
	let variable = Symbol::new(var_id, symbol_type_variable, val_node);
	current_scope.push_symbol(variable);
}

/// 赋值语句将更新已有的变量值:
/// 1. 确保变量已经声明
/// 2. 更新变量的值
///
fn visit_assignment_stmt(ast_node: &mut AstNode, scope_stack: &mut ScopeStack) {
	print_visit_info("visit var declare stmt");
	let var_id: String = ast_node.get_child_text(0).unwrap();
	let mut parent_name: Option<String> = scope_stack.current().unwrap().parent_scope_name();

	// 在本域中查找
	let current_scope = scope_stack.current().unwrap();
	if current_scope.current_has_symbol(var_id.clone()) {
		current_scope.update_symbol_val(var_id.clone(), None)
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
		if scope.current_has_variable(var_id.clone()) {
			// 更新
			scope.update_symbol_val(var_id.clone(), None);
		}
		parent_name = scope.parent_scope_name();
	}
}

fn print_visit_info(msg: &str) {
	println!("[info][ast_visit]: {}", msg);
}

fn panic_visit_info(msg: &str) {
	panic!("[error][ast_visit] {}", msg);
}
