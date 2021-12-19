use std::{time::Duration, vec};

use crate::util::{read_file, RESULT};

pub fn day4part1() -> RESULT {
    let lines: Vec<String> = read_file("day4")
        .lines()
        .map(|f| f.to_string())
        .collect();

    let numbers: Vec<u32> = lines[0]
        .split(",")
        .map(|f| f.parse::<u32>().unwrap())
        .collect();

    let mut eh: Vec<(usize, i128)> = vec![];
    for (i, e) in lines
        .iter()
        .skip(2)
        .collect::<Vec<&String>>()
        .chunks(6)
        .enumerate()
    {
        for t in e {
            if !t.eq_ignore_ascii_case("") {
                let mut n: i128 = 0;
                t.split_whitespace()
                    .for_each(|f| n |= 2i128.pow(f.parse::<u32>().unwrap()));
                eh.push((i, n));
            }
        }
    }
    let mut curr = 0;
    for (_, e) in numbers.iter().enumerate() {
        curr |= 2i128.pow(*e);
        for (_, te) in eh.iter().enumerate() {
            if te.1 & curr == te.1 {
                let mut tmp: i128 = 0;
                eh.iter().filter(|&&f| f.0 == te.0).for_each(|f| {
                    tmp |= f.1;
                });

                let test = (curr & tmp) ^ tmp;
                let mut sum: u32 = 0;
                for j in 0..127 {
                    if test & 2i128.pow(j) != 0 {
                        sum += j;
                    }
                }

                return RESULT {
                    name: "D4P1".to_string(),
                    result: (sum * e) as i64,
                    time: Duration::from_micros(0),
                    percentage: 0f32,
                };
            }
        }
    }
    panic!("I have failed you")
}

pub fn day4part2() -> RESULT {
    let lines: Vec<String> = read_file("day4")
        .lines()
        .map(|f| f.to_string())
        .collect();

    let numbers: Vec<u32> = lines[0]
        .split(",")
        .map(|f| f.parse::<u32>().unwrap())
        .collect();

    let mut eh: Vec<(usize, i128)> = vec![];
    for (i, e) in lines
        .iter()
        .skip(2)
        .collect::<Vec<&String>>()
        .chunks(6)
        .enumerate()
    {
        for t in e {
            if !t.eq_ignore_ascii_case("") {
                let mut n: i128 = 0;
                t.split_whitespace()
                    .for_each(|f| n |= 2i128.pow(f.parse::<u32>().unwrap()));
                eh.push((i, n));
            }
        }

        for t in 0..5 {
            let mut tmp: i128 = 0;
            for j in 0..5 {
                let test: Vec<u32> = e[j]
                    .split_whitespace()
                    .map(|f| f.parse::<u32>().unwrap())
                    .collect();
                tmp |= 2i128.pow(test[t]);
            }
            eh.push((i, tmp));
        }
    }
    let mut curr = 0;
    for (_i, e) in numbers.iter().enumerate() {
        curr |= 2i128.pow(*e);
        let mut g = 0;
        while g < eh.len() {
            if eh[g].1 & curr == eh[g].1 {
                if eh.len() == 10 {
                    let mut tmp: i128 = 0;
                    eh.iter().for_each(|(_, f)| {
                        tmp |= f;
                    });

                    let test = (curr & tmp) ^ tmp;
                    let mut sum: u32 = 0;
                    for j in 0..127 {
                        if test & 2i128.pow(j) != 0 {
                            sum += j;
                        }
                    }

                    return RESULT {
                        name: "D4P2".to_string(),
                        result: (sum * e) as i64,
                        time: Duration::from_micros(0),
                        percentage: 0f32,
                    };
                }
                let h0 = eh[g].0;
                eh = eh.iter().filter(|(f, _)| *f != h0).map(|&f| f).collect();
                g = 0;
            } else {
                g += 1;
            }
        }
    }
    panic!("I have failed you")
}
