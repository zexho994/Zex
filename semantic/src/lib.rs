pub mod ast_visit;
pub mod scope;
use crate::ast_node::AstNode;
use parse::*;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn visit_block_statement() {
        let str = "{int i = 1 + 1;}";
        let mut tokens = lexer::lexing(str.to_string());
        let ast = parse::parsing(&mut tokens);
        println!("ast is {:?}", ast);
    }

    #[test]
    fn calculate() {
        let str = "int i;";
        let mut tokens = lexer::lexing(str.to_string());
        let mut ast = parse::parsing(&mut tokens).unwrap();
        ast_visit::visit_program(&mut ast);
    }
}

pub fn semantic(mut ast_root: AstNode) {
    ast_visit::visit_program(&mut ast_root);
}
