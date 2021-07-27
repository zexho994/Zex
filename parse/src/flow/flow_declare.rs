use super::flow_declare_fn::*;
use crate::ast_node::*;
use crate::flow::flow_declare_class::match_class_declare;
use crate::flow::flow_declare_variable::match_var_declare;
use crate::utils::print_util::print_parse_more2_info;
use lexer::token::token_struct::*;

/// todo 声明语句现在提供变量声明，以后还有方法声明、类声明
/// <declare> ::= <varDeclare> | <fnDeclare>
pub fn match_declare(tokens: &mut Tokens) -> Option<AstNode> {
    print_parse_more2_info("match declare,token is ", tokens.peek(), tokens.position());
    let pos = tokens.position();
    if let Some(n) = match_var_declare(tokens) {
        Some(n)
    } else if let Some(n) = match_fn_declare(tokens) {
        Some(n)
    } else if let Some(n) = match_class_declare(tokens) {
        Some(n)
    } else {
        tokens.set_position(pos);
        None
    }
}
