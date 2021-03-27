fn main() {
    let mut number: i32 = get_five();
    println!("Hello, world!");
    println!("Number: {}", number);
    add_4(&mut number);
    println!("{}", number)
}

fn add_4 (mut x: &i32) -> () {
    *x += 4;
}

fn get_five() -> i32 {
    5
}
