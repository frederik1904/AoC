use std::time::Duration;

use crate::util::{read_file, RESULT};

pub fn day7part1() -> RESULT {
    let mut crab: Vec<i32> = read_file("day7part1")
        .split(",")
        .map(|f| f.parse::<i32>().unwrap())
        .collect();
    crab.sort_unstable();
    let median = crab[crab.len() / 2];
    let res: i32 = crab.iter().map(|f| (i32::abs(median - f))).sum();

    RESULT {
        name: "D7P1".to_string(),
        result: (res) as i64,
        time: Duration::from_micros(0),
        percentage: 0f32,
    }
}

pub fn day7part2() -> RESULT {
    let mut crab: Vec<i32> = read_file("day7part1")
        .split(",")
        .map(|f| f.parse::<i32>().unwrap())
        .collect();
    crab.sort_unstable();

    let min = crab.iter().min().unwrap().clone();
    let max = crab.iter().max().unwrap().clone();

    let mut sum: i32 = i32::MAX;

    for i in min..max {
        sum = sum.min(
            crab.iter()
                .map(|f| i32::abs(i - f) * (i32::abs(i - f) + 1) / 2)
                .sum(),
        );
    }

    RESULT {
        name: "D7P2".to_string(),
        result: (sum) as i64,
        time: Duration::from_micros(0),
        percentage: 0f32,
    }
}
