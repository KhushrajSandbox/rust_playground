fn main() {
    const NUMBERS: (f64, i32) = (5.2, 4);
    let difference = NUMBERS.0 as i32 - NUMBERS.1;
    println!("Difference, {}", difference);

    let boolean: bool = true;
    println!("Boolean, {}", boolean);

    const NUMBERS_ARRAY: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{}", NUMBERS_ARRAY[1])
}
