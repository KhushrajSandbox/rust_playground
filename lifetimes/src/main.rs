use rand::Rng;

fn main() {
    println!("{}", choose("hi", "bye"));
}

fn choose<'a>(a: &'a str, b: &'a str) -> &'a str {
    if rand::thread_rng().gen_range(0..2) == 0 {
        return a;
    } else {
        return b;
    }
}
