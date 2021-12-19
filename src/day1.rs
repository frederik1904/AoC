use std::time::Duration;

use crate::util::{read_file, RESULT};

pub fn day1part1() -> RESULT {
    let res = read_file("day1")
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .fold((0, 0), |acc, x| (acc.0 + if x > acc.1 { 1 } else { 0 }, x));

    RESULT {
        name: "D1P1".to_string(),
        result: res.0,
        time: Duration::from_micros(0),
        percentage: 0f32,
    }
}

pub fn day1part2() -> RESULT {
    let str = read_file("day1");
    let nums = str
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut a = nums[0] + nums[1] + nums[2];
    let mut sum = 0;
    for i in 1..nums.len() - 2 {
        let b = nums[i] + nums[i + 1] + nums[i + 2];
        if a < b {
            sum += 1;
        }
        a = b;
    }
    RESULT {
        name: "D1P2".to_string(),
        result: sum,
        time: Duration::from_micros(0),
        percentage: 0f32,
    }
}
