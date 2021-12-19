use std::time::Duration;

use crate::util::{read_file, RESULT};

pub fn day9part1() -> RESULT {
    let res: Vec<Vec<u8>> = read_file("day9")
        .lines()
        .map(|x| {
            x.chars()
                .map(|f| f.to_string().parse::<u8>().unwrap())
                .collect()
        })
        .collect();

    let mut tops: Vec<i32> = vec![];
    for y in 0..res.len() {
        for x in 0..res[y].len() {
            let mut higher = true;
            let top = res[y][x];
            if x != 0 {
                if top >= res[y][x - 1] {
                    higher = false
                }
            }
            if x != res[y].len() - 1 {
                if top >= res[y][x + 1] {
                    higher = false
                }
            }
            if y != 0 {
                if top >= res[y - 1][x] {
                    higher = false
                }
            }
            if y != res.len() - 1 {
                if top >= res[y + 1][x] {
                    higher = false
                }
            }
            if higher {
                tops.push((top + 1) as i32)
            }
        }
    }
    RESULT {
        name: "D9P1".to_string(),
        result: (tops.iter().sum::<i32>()) as i64,
        time: Duration::from_micros(0),
        percentage: 0f32,
    }
}

pub fn day9part2() -> RESULT {
    let res: Vec<Vec<u8>> = read_file("day9")
        .lines()
        .map(|x| {
            x.chars()
                .map(|f| f.to_string().parse::<u8>().unwrap())
                .collect()
        })
        .collect();

    let mut tops: Vec<(usize, usize)> = vec![];
    for y in 0..res.len() {
        for x in 0..res[y].len() {
            let mut higher = true;
            let top = res[y][x];
            if x != 0 {
                if top >= res[y][x - 1] {
                    higher = false
                }
            }
            if x != res[y].len() - 1 {
                if top >= res[y][x + 1] {
                    higher = false
                }
            }
            if y != 0 {
                if top >= res[y - 1][x] {
                    higher = false
                }
            }
            if y != res.len() - 1 {
                if top >= res[y + 1][x] {
                    higher = false
                }
            }
            if higher {
                tops.push((x, y))
            }
        }
    }
    let mut sizes: Vec<i32> = vec![];
    for t in tops {
        let mut size = 0;
        let mut stack = vec![t];
        let mut i = 0;
        while i < stack.len() {
            let (x, y) = stack[i];
            let top = res[y][x];
            if top != 9 {
                if x != 0 {
                    if top < res[y][x - 1] {
                        if !stack.contains(&(x - 1, y)) {
                            stack.push((x - 1, y))
                        }
                    }
                }
                if x != res[y].len() - 1 {
                    if top < res[y][x + 1] {
                        if !stack.contains(&(x + 1, y)) {
                            stack.push((x + 1, y))
                        }
                    }
                }
                if y != 0 {
                    if top < res[y - 1][x] {
                        if !stack.contains(&(x, y - 1)) {
                            stack.push((x, y - 1))
                        }
                    }
                }
                if y != res.len() - 1 {
                    if top < res[y + 1][x] {
                        if !stack.contains(&(x, y + 1)) {
                            stack.push((x, y + 1))
                        }
                    }
                }

                size += 1;
            }
            i += 1;
        }

        sizes.push(size);
    }
    sizes.sort_unstable();
    RESULT {
        name: "D9P2".to_string(),
        result: (sizes.iter().rev().take(3).fold(1, |acc, f| acc * f)) as i64,
        time: Duration::from_micros(0),
        percentage: 0f32,
    }
}
