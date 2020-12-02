pub mod exercises;
pub mod helpers;

use std::env;

use exercises::ex01::ex01;
use exercises::ex02::ex02;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please enter an exercise number");
    } else {
        match args[1].parse::<u32>() {
            Ok(n) => match n {
                1 => ex01(),
                2 => ex02(),
                _ => println!("This exercise does not exist"),
            },
            _ => println!("Error reading exercise number"),
        }
    }
}
