pub mod exercises;
pub mod helpers;

use std::env;

use exercises::ex01::ex01;
use exercises::ex02::ex02;
use exercises::ex03::ex03;
use exercises::ex04::ex04;
use exercises::ex05::ex05;
use exercises::ex06::ex06;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please enter an exercise number");
    } else {
        match args[1].parse::<u32>() {
            Ok(n) => match n {
                1 => ex01(),
                2 => ex02(),
                3 => ex03(),
                4 => ex04(),
                5 => ex05(),
                6 => ex06(),
                _ => println!("This exercise does not exist"),
            },
            _ => println!("Error reading exercise number"),
        }
    }
}
