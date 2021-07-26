use crate::ast_node::AstNode;
use crate::ast_node_type::AstNodeType;
use crate::scope::Scope;
use crate::scope_stack::ScopeStack;
use crate::symbol::*;

// 1. 创建全局域
// 2. 处理流程
// 3. 保存，退出
pub fn visit_program(ast_node: &mut AstNode) {
	print_info("visit program");
	// 初始化全局域
	let global_scope = Scope::new_global();
	// 初始化域栈
	let mut scope_stack = ScopeStack::new();
	// 入栈
	scope_stack.push(global_scope);

	// 访问子节点
	visit_program_children(ast_node, &mut scope_stack);

	//出栈
	scope_stack.pop();
}

/// visit program的所有子节点
fn visit_program_children(program_node: &mut AstNode, scope_stack: &mut ScopeStack) {
	for child in program_node.get_child_vec_mut().iter_mut() {
		visit_statements(child, scope_stack);
	}
}

/// ast_node type = AstNodeType::Statements
fn visit_statements(ast_node: &mut parse::ast_node::AstNode, scope_stack: &mut ScopeStack) {
	print_info("visit statements");
	visit_statements_children(ast_node, scope_stack)
}

fn visit_statements_children(ast_node: &mut AstNode, scope_stack: &mut ScopeStack) {
	for child in ast_node.get_child_vec_mut().iter_mut() {
		match child.get_type() {
			AstNodeType::BlockStmt => {
				visit_block_statement(child, scope_stack);
			}
			AstNodeType::Statement => {
				visit_statement(child, scope_stack);
			}
			_ => print_panic_more("visit statements children, child node type error", child),
		}
	}
}

/// ast_node type = AstNodeType::BlockStmt
/// block域的父域是上一层域
fn visit_block_statement(ast_node: &mut parse::ast_node::AstNode, scope_stack: &mut ScopeStack) {
	print_info("visit block statement");
	let current_scope: &Scope = scope_stack.current_scope();
	let new_local_scope: Scope = Scope::new_local(current_scope.get_scope_name());
	scope_stack.push(new_local_scope);

	visit_block_statement_children(ast_node, scope_stack);

	scope_stack.pop();
}

fn visit_block_statement_children(
	ast_node: &mut parse::ast_node::AstNode,
	scope_stack: &mut ScopeStack,
) {
	for child in ast_node.get_child_vec_mut().iter_mut() {
		match child.get_type() {
			AstNodeType::Statements => visit_statements(child, scope_stack),
			_ => print_panic_more(
				"visit block statement children, child node type error",
				child,
			),
		}
	}
}

/// ast_node type = AstNodeType::Statement
///
/// statement 的类型
/// 1. echo 语句
/// 2. varDeclareStmt 声明语句
/// 3. assignmentStmt 赋值语句
/// 4. expressionStmt 表达式语句
fn visit_statement(ast_node: &mut AstNode, scope_stack: &mut ScopeStack) {
	print_info("visit statement");

	// visit children
	for child in ast_node.get_child_vec_mut().iter_mut() {
		match child.get_type() {
			AstNodeType::Echo => visit_echo(child, scope_stack),
			AstNodeType::VarDeclareStmt => visit_var_declare_stmt(child, scope_stack),
			AstNodeType::FnDeclareStmt => visit_fn_declare_stmt(child, scope_stack),
			AstNodeType::AssignmentStmt => visit_assignment_stmt(child, scope_stack),
			AstNodeType::ExpressionStmt => print_panic("visit expr error"),
			_ => print_panic_more("visit statement child error", child),
		}
	}
}

/// <expressionStm> ::= <addExpr>
/// <addExpr> ::= <mulExpr> | <mulExpr> '+' <addExpr>
/// <mulExpr> ::= <primary> | <primary> '*' <mulExpr>
/// <primary> :: <primary> ::= <id> | <intLiteral>
/// fn visit_expression_stmt(child: &mut AstNode, scope_stack: &mut ScopeStack) {}
///
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
fn visit_var_declare_stmt(ast_node: &mut AstNode, scope_stack: &mut ScopeStack) {
	print_info("visit var declare stmt");
	let var_id: String = ast_node.get_child_text(1).unwrap();
	let mut scope = scope_stack.pop().unwrap();

	let ast_node = ast_node.remove_child(3);
	let symbol = Symbol::new(var_id, SYMBOL_TYPE_VARIABLE, Option::Some(ast_node));
	scope.define_symbol(symbol, scope_stack);
	scope_stack.push(scope);
}

/// 处理方法表达式
///
fn visit_fn_declare_stmt(ast_node: &mut AstNode, scope_stack: &mut ScopeStack) {
	print_info_extend("visit fn declare statement", ast_node);
	let id = ast_node.get_text();
	let arguments = ast_node.get_argument_child();
	let returnType = ast_node.get_return_child();
	let block_stmt = ast_node.get_block_statement_child();

	// println!(
	// 	"id = {:?},arguments = {:?},returnType ={:?} ,blockStmt ={:?} ",
	// 	id, arguments, returnType, block_stmt
	// );
}

/// ast_node type = AstNodeType::AssignmentStmt
/// 赋值语句将更新已有的变量值:
/// 1. 确保变量已经声明
/// 2. 更新变量的值
fn visit_assignment_stmt(ast_node: &mut AstNode, scope_stack: &mut ScopeStack) {
	print_info("visit var declare stmt");
	let var_id: String = ast_node.get_child_text(0).unwrap();
	let new_symbol = Symbol::new(
		var_id.clone(),
		SYMBOL_TYPE_VARIABLE,
		Option::Some(ast_node.remove_child(2)),
	);
	let mut current_scope = scope_stack.pop().unwrap();
	current_scope.update_symbol(&var_id, new_symbol, scope_stack);
	scope_stack.push(current_scope);
}

/// <echo> ::= echo (<id> |<intLiteral>),
/// 获取要输出的节点，计算值
/// 要输出的值必需在合法作用域中，且在合法生命周期内
/// 如果要输出的字面值类型，直接打印即可
/// 如果要输出的是其他类型，需要进行计算,计算规则:
/// -
/// - Identifier: 从本域开始向超查找，找到对应变量，然后进行求值计算
/// - ExpressionStmt: todo
/// -
///
/// echo 3;     //intLiteral
/// int a = 1;	// a:primary
/// echo a;	    //identifier a
/// int b = a + 2;  //b:expressionStmt
/// echo b;     //identifier b
///
fn visit_echo(ast_node: &mut AstNode, scope_stack: &mut ScopeStack) {
	print_info_extend("visit echo", &ast_node);
	let target = ast_node.get_child_mut(0).unwrap();
	match target.get_type() {
		// 标识符类型，需要先获取id对应的symbol，然在再获取对应的AstNode
		AstNodeType::Identifier => visit_identifier(target, scope_stack),
		// 字面量类型，可以直接输出
		AstNodeType::IntLiteral => echo_int_literal(target),
		_ => panic!("visit echo"),
	}
}

/// ### identifier可能的场景
/// 符号表中:k=id,v=ast_node
/// ast_node's type = expressionStmt
/// 根据id的text在符号表从下到上遍历寻找对应符号以及值
fn visit_identifier(ast_node: &mut AstNode, scope_stack: &ScopeStack) {
	print_info_extend("visit identifier", ast_node);
	let id = ast_node.get_text();
	let mut target_symbol: Option<&Symbol> = None;

	// 在当前域中查找符号
	if let Some(symbol) = scope_stack.current_scope().find_symbol(&id) {
		target_symbol = Some(symbol);
	} else {
		let mut parent_name = scope_stack.current_scope().parent_scope_name();
		//父域中查找符号
		while parent_name.is_some() {
			let scope = scope_stack.find_scope(&parent_name.unwrap()).unwrap();
			if let Some(symbol) = scope.find_symbol(&id) {
				target_symbol = Some(symbol);
				break;
			}
			parent_name = scope.parent_scope_name();
		}
	}
	let ast_node = target_symbol.unwrap().get_ast_node().unwrap();
	let num = AstNode::calculate(ast_node);
	println!("{}", num);
}

fn echo_int_literal(ast_node: &mut AstNode) {
	println!("{}", ast_node.get_text().clone())
}

fn print_info(_msg: &str) {
	// println!("[info][ast_visit]: {}", _msg);
}

fn print_info_extend<T: std::fmt::Debug>(_msg: &str, _t: &T) {
	// println!("[info][ast_visit]: {},t = {:?}", _msg, _t);
}

fn print_panic(msg: &str) {
	panic!("[error][ast_visit] {}", msg);
}

fn print_panic_more<T: std::fmt::Debug>(msg: &str, t: &T) {
	panic!("[error][visitor] msg = {}, t = {:?}", msg, t);
}
