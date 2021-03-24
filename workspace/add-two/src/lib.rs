mod constants;
use constants::two;

pub fn add_two(number: i32) -> i32 {
    number + two::NUMBER
}
