use add_one::add_one;
use add_two::add_two;

fn main() {
    let num = 5;
    println!("Hello, World! {num} plus one is {}", add_one(num));
    println!("Hello, World! {num} plus two is {}", add_two(num));
}
