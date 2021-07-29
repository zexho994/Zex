use super::class_file::U2;

#[derive(Debug, Default)]
pub struct ConstantPool {}

/// 常量池中存储的基本单元
#[derive(Debug, Default)]
pub struct ConstantPoolUnit {
	default: U2,
}
