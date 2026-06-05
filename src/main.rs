mod adder;
use adder::adder;
use adder::multiplier;
fn main() {
	adder(42, 10);
	println!("multplier: {}",multiplier(42, 10));
}
