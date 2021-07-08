pub mod ast_node;
pub mod ast_node_type;
pub mod parse_flow;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_block_stmt() {
        let s = String::from("{ int a = 1 + 3 * 2;{int b = 1;} }");
        println!("\n==> parse str {}", s);
        let mut tokens = lexer::lexing(s);
        let ast = parsing(&mut tokens);
        println!("\n==> parse str to ast , ast is {:?}", ast);
    }

    #[test]
    fn parse_assignment() {
        let s = String::from("int a;a=1;echo a;");
        println!("\n===> parse assignment {}", s);
        let mut tokens = lexer::lexing(s.clone());
        let ast = parsing(&mut tokens);
        println!("\n==> parse str to ast , ast is {:?}", ast);
    }

    #[test]
    fn parse_echo() {
        let s = String::from("echo 1;echo a;");
        println!("\n===> parse assignment {}", s);
        let mut tokens = lexer::lexing(s.clone());
        let ast = parsing(&mut tokens);
        println!("\n==> parse str to ast , ast is {:?}", ast);
    }
}

/// // 程序入口
/// <program> ::= <statements>
///
/// // 语句集：块语句，普通语句，
/// <statements> ::= <blockStm> | <statement> | <statements> <statement>
///
/// // 块语句
/// <blockStm> ::= { <statements> }
///
/// // 语句类型：分配声明语句，表达式语句，赋值语句
/// <statement> ::=  <echo> | <declare> | <expressionStm> | <assignmentStm> ;
///
/// // 声明语句现在提供变量声明，以后还有方法声明、类声明
/// <declare> ::= <varDeclare>
///
/// // 变量声明有两种，有无初始化
/// <varDeclare> ::= <varDefine> | <varDefine> <assignment> <expressionStm>
///
/// // 变量定义
/// <varDefine> ::= <type> <id>
///
/// //变量类型，暂时提供int
/// <type> ::= int
///
/// // 表达式语句
/// <expressionStm> ::= <addExpr> ;
///
/// // 赋值语句
/// <assignmentStm> ::= <identifier> <assignment> <expressionStm> ;
///
/// // 赋值符号
/// <assignment> ::= =
///
/// <addExpr> ::= <mulExpr> | <mulExpr> '+' <addExpr>
///
/// <mulExpr> ::= <primary> | <primary> '*' <mulExpr>
///
/// <primary> ::= <id> | <intLiteral>
///
/// <id> ::= ([a-z][A-Z])*
///
/// <intLiteral> ::= [1-9][0-9]*
///
/// <echo> ::= echo <id> | echo <intLiteral>
pub fn parsing(tokens: &mut lexer::token::token_struct::Tokens) -> Option<ast_node::AstNode> {
    parse_flow::match_program(tokens)
}
