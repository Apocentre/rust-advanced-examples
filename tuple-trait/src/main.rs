mod common;
mod solution1;
mod solution2;

use crate::common::{Trait, Trait2, A, B, Composite, Composite2, Extra};

fn main() {
	println!("Solution 1");
	let a = A {};
	let b = B {};

	let val = (a, b);

	let comp = Composite::<Extra> {val};
	comp.val.run();
	comp.val.exit();

	println!("------------------");
	println!("Solution 2");

	let val = (a, b);
	let comp = Composite2::<Extra> {val};
	comp.val.run2();
}
