//
use parse::ast_node::AstNode;
pub mod byte_code;

pub fn to_bytecode(ast: AstNode) {
    println!("hello {:?}", ast);
    let c = byte_code::ClassFile::new();
    println!("{:?}",c)
}
