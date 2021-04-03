use add_one;
use add_two;
use adder;

fn main() {
    println!("Added one: {}", add_one::add_one(adder::NUM));
    println!("Added two: {}", add_two::add_two(adder::NUM));
}
