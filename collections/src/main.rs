fn main() {
    // Vectors

    let vec = vec![
        String::from("1"),
        String::from("2"),
        String::from("3"),
        String::from("4"),
    ];

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
    scores.insert(String::from("Khushraj"), 100);

    scores.entry(String::from("Khushraj2")).or_insert(50);

    println!("{:#?}", scores);
}
