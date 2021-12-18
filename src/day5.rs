use crate::util::{read_file, RESULT};
use std::{collections::HashMap, time::Duration};

fn str_to_points(v: &str) -> Vec<i32> {
    v.split(",").map(|f| f.parse::<i32>().unwrap()).collect()
}

fn process_str(s: &str) -> (i32, i32, i32, i32) {
    let tmp: Vec<&str> = s.split(" -> ").collect();
    let points1: Vec<i32> = str_to_points(tmp[0]);
    let points2 = str_to_points(tmp[1]);
    (points1[0], points1[1], points2[0], points2[1])
}

pub fn day5part1() -> RESULT {
    let mut board: HashMap<String, i32> = HashMap::new();
    read_file("day5part1").lines().for_each(|s| {
        let (x1, y1, x2, y2) = process_str(s);
        if x1 == x2 {
            for i in (y1.min(y2) as usize)..=(y2.max(y1) as usize) {
                let key = format!("{},{}", x1, i);
                match &board.get(&key) {
                    None => board.insert(key, 1),
                    Some(&v) => board.insert(key, v + 1),
                };
            }
        } else if y1 == y2 {
            for i in (x1.min(x2) as usize)..=(x2.max(x1) as usize) {
                let key = format!("{},{}", i, y1);
                match &board.get(&key) {
                    None => board.insert(key, 1),
                    Some(&v) => board.insert(key, v + 1),
                };
            }
        }
    });

    let res: Vec<_> = board.values().filter(|&&f| f > 1).collect();
    RESULT {
        name: "D5P1".to_string(),
        result: res.len() as i64,
        time: Duration::from_micros(0),
        percentage: 0f32,
    }
}

pub fn day5part2() -> RESULT {
    let mut board: HashMap<String, i32> = HashMap::new();
    read_file("day5part1").lines().for_each(|s| {
        let (x1, y1, x2, y2) = process_str(s);
        if x1 == x2 {
            for i in (y1.min(y2) as usize)..=(y2.max(y1) as usize) {
                let key = format!("{},{}", x1, i);
                match &board.get(&key) {
                    None => board.insert(key, 1),
                    Some(&v) => board.insert(key, v + 1),
                };
            }
        } else if y1 == y2 {
            for i in (x1.min(x2) as usize)..=(x2.max(x1) as usize) {
                let key = format!("{},{}", i, y1);
                match &board.get(&key) {
                    None => board.insert(key, 1),
                    Some(&v) => board.insert(key, v + 1),
                };
            }
        } else {
            let (x, y, modifier) = if x1 < x2 {
                (x1, y1, if y1 < y2 { 1 } else { -1 })
            } else {
                (x2, y2, if y2 < y1 { 1 } else { -1 })
            };
            for i in 0..=(i32::abs(x1 - x2)) {
                let modified_i = modifier * i;
                let key = format!("{},{}", x + i, y + modified_i);
                match &board.get(&key) {
                    None => board.insert(key, 1),
                    Some(&v) => board.insert(key, v + 1),
                };
            }
        }
    });

    let res: Vec<_> = board.values().filter(|&&f| f > 1).collect();
    RESULT {
        name: "D5P2".to_string(),
        result: res.len() as i64,
        time: Duration::from_micros(0),
        percentage: 0f32,
    }
}
