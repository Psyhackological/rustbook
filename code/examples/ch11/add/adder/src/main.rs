use rand::prelude::*;

fn main() {
    let num: i32 = rand::thread_rng().gen_range(1..=6);
    println!("{num}+1={}", add_one::add_one(num));

    let num: i32 = rand::thread_rng().gen_range(1..=6);
    println!("{num}+2={}", add_two::add_two(num));
}
