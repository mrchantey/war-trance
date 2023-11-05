use sweet::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

fn hash_values<T: Hash, U: Hash>(a: T, b: U) -> u64 {
	let mut hasher = DefaultHasher::new();
	a.hash(&mut hasher);
	b.hash(&mut hasher);
	hasher.finish()
}


#[sweet_test]
pub fn works() -> Result<()> {
	let a = "Hello";
	let b = "World";
	expect(hash_values(a, b)).to_be(915208533284079590)?;
	expect(hash_values(b, a)).not().to_be(915208533284079590)?;
	expect(hash_values(b, a)).to_be(14628430809838058366)?;
	Ok(())
}
