use add_one;
use add_two;

fn main() {
    let num = 1;
    println!("Added one: {}", add_one::add_one(num));
    println!("Added two: {}", add_two::add_two(num));
}
