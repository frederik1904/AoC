use std::time::Duration;

use crate::util::{read_file, RESULT};

const UP: &str = "up";
const DOWN: &str = "down";

pub fn day2part1() -> RESULT {
    let mut x = 0;
    let mut y = 0;
    read_file("day2part1").lines().for_each(|s| {
        let inp: Vec<&str> = s.split_whitespace().into_iter().collect();
        let i: i32 = inp[1].to_string().parse().unwrap();
        match inp[0] {
            UP => y -= i,
            DOWN => y += i,
            _ => x += i,
        }
    });
    RESULT {
        name: "D2P1".to_string(),
        result: (x * y) as i64,
        time: Duration::from_micros(0),
        percentage: 0f32,
    }
}

pub fn day2part2() -> RESULT {
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut a: i64 = 0;

    read_file("day2part1").lines().for_each(|s| {
        let inp: Vec<&str> = s.split_whitespace().into_iter().collect();
        let i: i64 = inp[1].to_string().parse().unwrap();
        match inp[0] {
            UP => {
                a -= i;
            }
            DOWN => {
                a += i;
            }
            _ => {
                x += i;
                y += i * a;
            }
        }
    });
    RESULT {
        name: "D2P2".to_string(),
        result: (x * y) as i64,
        time: Duration::from_micros(0),
        percentage: 0f32,
    }
}
