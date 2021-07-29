//
use parse::ast_node::AstNode;
pub mod byte_code;
pub mod file;

pub fn to_bytecode(ast: AstNode) {
    println!("hello {:?}", ast);
    let c: byte_code::ClassFile = Default::default();
    println!("{:?}", c);

    let c1: byte_code::ClassFile = byte_code::ClassFile::new();
    println!("{:?}", c1);
}
