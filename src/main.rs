use std::fs::File;

use one_billion::run;

fn main() {
	let path = std::env::args()
		.nth(1)
		.expect("Expected file path as first program argument");
	let file = File::open(path).expect("Expected file to exist");

	let output = run(file);

	println!("{output}");
}
