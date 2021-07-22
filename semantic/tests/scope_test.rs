extern crate semantic;

use semantic::*;

#[test]
fn set_seq() {
	let mut local1 = scope::Scope::new_local("parent1".to_string());
	let mut local2 = scope::Scope::new_local("parent2".to_string());
	local1.set_seq(1);
	local2.set_seq(2);

	assert!(local1.get_seq() == 1);
	assert!(local2.get_seq() == 2);
}
