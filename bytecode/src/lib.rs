pub mod bytecode;
pub mod file;

use crate::bytecode::class_file::*;
use parse::ast_node::AstNode;

pub fn to_bytecode(ast: AstNode) {
    println!("hello {:?}", ast);

    let c1: ClassFile = ClassFile::new_jdk8();
    println!("{:?}", c1);
}
