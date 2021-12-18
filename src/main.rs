extern  crate stopwatch;
use stopwatch::Stopwatch;
use util::RESULT;

use crate::day1::{day1part1, day1part2};
use crate::day2::{day2part1, day2part2};
use crate::day3::{day3part1, day3part2};
use crate::day4::{day4part1, day4part2};
use crate::day5::{day5part1, day5part2};
use crate::day6::{day6part2, day6part1};
use crate::day7::{day7part1, day7part2};
use crate::day8::{day8part1, day8part2};
use crate::day9::{day9part1, day9part2};
use crate::day10::{day10part1, day10part2};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod util;

fn main() {
    let days: Vec<fn() -> RESULT> = vec!{
        day1part1,
        day1part2,
        day2part1,
        day2part2,
        day3part1,
        day3part2,
        day4part1,
        day4part2,
        day5part1,
        day5part2,
        day6part1,
        day6part2,
        day7part1,
        day7part2,
        day8part1,
        day8part2,
        day9part1,
        day9part2,
        day10part1,
        day10part2,
    };

    let mut total_time = 0;
    let mut sw = Stopwatch::new();
    let mut results = vec![];
    for d in days {
        sw.reset();
        sw.start();
        let mut r = d();
        sw.stop();
        r.time = sw.elapsed();
        results.push(r);
        total_time += sw.elapsed().as_nanos()
    }

    for mut r in results {
        let p = ((r.time.as_nanos() as f64 / total_time as f64) as f64 * 100f64) as f32;
        r.percentage = p;
        println!("{}", r.to_string());
    }
    
}
