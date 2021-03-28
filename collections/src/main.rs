fn main() {
    // Vectors

    let vec = vec!["1", "2", "3", "4"];

    match vec.get(2) {
        Some(third) => println!("Third element: {}", third),
        None => println!("Third element of vector does not exist"),
    }

    let third = &vec[2];

    println!("Third element of vector: {}", third);

    // Strings

    let name = "Khushraj";

    for c in name.chars() {
        print!("{}", c)
    }

    // Hashmaps
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert("Khushraj", 100);

    scores.entry("Khushraj2").or_insert(50);

    println!("{:#?}", scores);
}
