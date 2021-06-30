pub mod ast_node;
pub mod ast_node_type;
pub mod calculate;
pub mod parse_flow;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_to_ast1() {
        let s = String::from("int a = 1 + 3 + 1;");
        println!("\n==> parse str {}", s);
        let mut tokens = lexer::lexing(s);
        assert_eq!(9, tokens.count());
        let ast = parsing(&mut tokens);
        println!("==> parse done. ast = {:?}", ast);
        // assert_eq!(ast.unwrap(), 5)
    }

    #[test]
    fn parse_block_stmt() {
        let s = String::from("{ int a = 1 + 3 + 1; }");
        let mut tokens = lexer::lexing(s);
        let ast = parsing(&mut tokens);
        println!("\n==> parse str to ast , ast is {:?}", ast);
    }

    // #[test]
    // fn parse_to_ast2() {
    //     let s = String::from("int a = 2 * 3 + 1 + 3 * 10;");
    //     println!("\n==> parse str {}", s);
    //     let mut tokens = lexer::lexing(s);
    //     assert_eq!(13, tokens.count());
    //     let ast = parse::parsing(&mut tokens);
    //     println!("==> parse done. ast = {:?}", ast);
    //     assert_eq!(ast.unwrap(), 37)
    // }

    // #[test]
    // fn match_assignment_expr1() {
    //     let s = String::from("i = 1 + 5;");
    //     println!("\n==> parse str {}", s);
    //     let mut tokens = lexer::lexing(s);
    //     let res = parse::parsing(&mut tokens).unwrap();
    //     assert_eq!(res, 6)
    // }

    // #[test]
    // fn match_assignment_expr2() {
    //     let s = String::from("i = 2 * 3 ;");
    //     println!("\n==> parse str {}", s);
    //     let mut tokens = lexer::lexing(s);
    //     let res = parse::parsing(&mut tokens).unwrap();
    //     assert_eq!(res, 6)
    // }

    // #[test]
    // fn match_assignment_expr3() {
    //     let s = String::from("i = 10 + 2 * 3 ;");
    //     println!("\n==> parse str {}", s);
    //     let mut tokens = lexer::lexing(s);
    //     let res = parse::parsing(&mut tokens).unwrap();
    //     assert_eq!(res, 16)
    // }

    // #[test]
    // fn match_express_stm() {
    //     let s = String::from("10 + 1 + 2;");
    //     println!("\n==> parse str {}", s);
    //     let mut tokens = lexer::lexing(s);
    //     let res = parse::parsing(&mut tokens).unwrap();
    //     assert_eq!(res, 13)
    // }

    // #[test]
    // fn multi_program() {
    //     let str = String::from("int a = 1;a = 2; a + 2;");
    //     println!("\n==> parse str {}", str);
    //     let mut tokens = lexer::lexing(str);
    //     let res = parse::parsing(&mut tokens).unwrap();
    //     assert_eq!(res, 4)
    // }

    #[test]
    fn test_or() {
        let x = Some(1);
        let y = Some(2);
        let z = Some(3);
        assert_eq!(x.or(y).or(z), Some(1));
        assert_eq!(y.or(z).or(x), Some(2));
        assert_eq!(z.or(x).or(y), Some(3));
    }

    fn test_or_fn() {
        // assert_eq!(get(0).or(get(1)).unwrap(), "x".to_string());
        assert_eq!(get(1).or(get(2)).unwrap(), "x".to_string());
    }

    fn get(n: usize) -> Option<String> {
        println!("invoke get fn");
        if n == 1 {
            Some("x".to_string())
        } else if n == 2 {
            Some("y".to_string())
        } else if n == 3 {
            Some("z".to_string())
        } else {
            None
        }
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
/// <statement> ::=  <declare> | <expressionStm> | <assignmentStm>
/// 
/// // 声明语句现在提供变量声明，以后还有方法声明、类声明
/// <declare> ::= <varDeclare>
/// 
/// // 变量声明有两种，有无初始化
/// <varDeclare> ::= <varDefine> | <varDefine> <aggignment> <expressionStm>
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
pub fn parsing(tokens: &mut lexer::token::token_struct::Tokens) -> Option<ast_node::AstNode> {
    // println!("parsing tokens:{:?} -> AST", tokens);
    parse_flow::match_program(tokens)
}
