use std::{collections::HashMap, hash::Hash, thread, time::Duration};

struct Cacher<P: Eq + Hash, R, F: Fn(&P) -> R> {
    function: F,
    values: HashMap<P, R>,
}

impl<P: Eq + Hash, R, F: Fn(&P) -> R> Cacher<P, R, F> {
    fn new(function: F) -> Cacher<P, R, F> {
        Cacher {
            function,
            values: HashMap::new(),
        }
    }

    fn get(&mut self, arg: P) -> &R {
        self.values.entry(arg).or_insert_with_key(&self.function)
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure(intensity));
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a: &u32| -> u32 { *a });

    let _v1 = c.get(1);
    let v2 = c.get(2);

    assert_eq!(*v2, 2);
}
