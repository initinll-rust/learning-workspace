use lib_add_one::add_one;
use lib_add_two::add_two;
use rand;

fn main() {
    let num1 = 10;
    let num2 = 20;
    println!("{num1} + 1 = {}", add_one(num1));
    println!("{num2} + 2 = {}", add_two(num2));
}
