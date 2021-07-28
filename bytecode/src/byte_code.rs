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
#[derive(Debug)]
pub struct ClassFile {
	magic: u32,
	minor_version: u16,
	major_version: u16,
	// constant_pool_count: u16,
	// constant_pool: u32,
	// access_flags: u16,
	// this_class: u16,
	// super_class: u16,
	// interfaces_count: u16,
	// interfaces:
	// fields_count: u16,
	// fields[fields_count]
	// methods_count: u16,
	// methods[methods_count]
	// attributes_count: u16,
}

impl ClassFile {
	pub fn new() -> ClassFile {
		ClassFile {
			magic: 0xCAFEBABE,
			minor_version: 0,
			major_version: 52,
		}
	}
}
