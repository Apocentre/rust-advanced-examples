fn main() {
	let mut bitmap = [0; 100_000];

	let byte = bitmap.len() / 8;
	let bit = bitmap.len() % 8;
	let mask = 1 << bit;

	println!("Original value {}", bitmap[byte] & mask);

	bitmap[byte] ^= mask;
	println!("After flip value {}", bitmap[byte]);
}
