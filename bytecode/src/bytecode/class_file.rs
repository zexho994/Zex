use crate::bytecode::constant_pool::ConstantPool;
use crate::bytecode::fields::Fields;
use crate::bytecode::interfaces::Interfaces;
use crate::bytecode::method::Method;

pub type U2 = u16;
pub type U4 = u32;

enum AccessFlags{
	ACC_SUPER = 0x20,
}

/// class file
///
/// ```c
/// {
/// 	u4 magic
/// 	u2 minor_version
/// 	u2 major_version
/// 	u2 constant_pool_count
/// 	cp_info constant_pool[constant_pool_count-1]
/// 	u2 access_flags
/// 	u2 this_class
/// 	u2 super_class
/// 	u2 interfaces_count
/// 	u2 interfaces[interfaces_count]
/// 	u2 fields_count
/// 	field_info fields[fields_count]
/// 	u2 methods_count
/// 	method_info methods[methods_count]
/// 	u2 attributes_count
/// 	attribute_info attributes[attributes_count]
/// }
/// ```
///
#[derive(Debug, Default)]
pub struct ClassFile {
	magic: U4,
	minor_version: U2,
	major_version: U2,
	constant_pool_count: U2,
	constant_pool: ConstantPool,
	access_flags: U2,
	this_class: U2,
	super_class: U2,
	interfaces_count: U2,
	interfaces: Interfaces,
	fields_count: U2,
	fields: Fields,
	methods_count: U2,
	methods: Method,
	attributes_count: U2,
}

impl ClassFile {
	pub fn new_jdk8() -> ClassFile {
		ClassFile {
			magic: 0xCAFEBABE,
			minor_version: 0,
			major_version: 52,
			constant_pool_count: 0,
			constant_pool: ConstantPool {},
			access_flags: 0,
			this_class: 0,
			super_class: 0,
			interfaces_count: 0,
			interfaces: Interfaces {},
			fields_count: 0,
			fields: Fields {},
			methods_count: 0,
			methods: Method {},
			attributes_count: 0,
		}
	}

	pub fn set_interfaces_count(&mut self, count: U2) {
		self.interfaces_count = count;
	}

	pub fn set_fields_count(&mut self, count: U2) {
		self.fields_count = count;
	}
}
