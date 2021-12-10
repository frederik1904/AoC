use crate::day1::{day1part1, day1part2};
use crate::day2::{day2part1, day2part2};
use crate::day3::{day3part1, day3part2};

mod day1;
mod day2;
mod day3;
mod util;

fn main() {
    println!("Day 1");
    println!("Part 1:");
    day1part1();
    println!("Part 2:");
    day1part2();

    println!("Day 2");
    println!("Part 1:");
    day2part1();
    println!("Part 2:");
    day2part2();

    println!("Day 3");
    println!("Part 1:");
    day3part1();
    println!("Part 2:");
    day3part2();

}
