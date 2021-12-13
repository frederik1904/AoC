use crate::day1::{day1part1, day1part2};
use crate::day2::{day2part1, day2part2};
use crate::day3::{day3part1, day3part2};
use crate::day4::{day4part1, day4part2};
use crate::day5::{day5part1, day5part2};
use crate::day6::{day6part2, day6part1};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
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

    println!("Day 4");
    println!("Part 1:");
    day4part1();
    println!("Part 2:");
    day4part2();

    println!("Day 5");
    println!("Part 1:");
    day5part1();
    println!("Part 2:");
    day5part2();

    println!("Day 6");
    println!("Part 1:");
    day6part1();
    println!("Part 2:");
    day6part2();
}
