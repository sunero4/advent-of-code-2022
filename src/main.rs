#![feature(iter_next_chunk)]
#![feature(let_chains)]
#![feature(array_windows)]

mod day10;
mod day6;
mod day7;
mod day9;

fn main() {
    let result = crate::day10::solution::part1();

    println!("{:?}", result);
}
