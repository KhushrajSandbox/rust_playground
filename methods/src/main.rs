struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

fn main() {
    let me = Person {
        first_name: String::from("Khushraj"),
        last_name: String::from("Rathod"),
    };

    println!("My name is {}", me.full_name());
}