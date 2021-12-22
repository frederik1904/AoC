use std::{collections::HashMap, time::Duration};

use crate::util::{read_file, RESULT};

pub fn day14part1() -> RESULT {
    let lines: Vec<String> = read_file("day14").lines().map(|s| s.to_string()).collect();

    let mut inp: Vec<char> = lines[0].chars().collect();

    let mut mapping: HashMap<String, char> = HashMap::new();

    for i in 2..lines.len() {
        let sp: Vec<String> = lines[i].split(" -> ").map(|f| f.to_string()).collect();
        let c1: char = sp[1].chars().collect::<Vec<char>>()[0];
        mapping.insert(sp[0].clone(), c1);
    }

    for _ in 0..10 {
        let mut n_inp = vec![];
        for j in 0..inp.len() - 1 {
            n_inp.push(inp[j]);
            let cmb = format!("{}{}", inp[j], inp[j + 1]);
            n_inp.push(*mapping.get(&cmb).unwrap())
        }
        n_inp.push(inp[inp.len() - 1]);
        inp = n_inp;
    }

    inp.sort_unstable();

    let mut sums = vec![];
    let mut last = inp[0];
    let mut sum = 0;
    for i in inp {
        if i != last {
            sums.push(sum);
            sum = 0;
            last = i;
        }

        sum += 1;
    }
    sums.push(sum);
    let min: i64 = *sums.iter().min().unwrap();
    let max: i64 = *sums.iter().max().unwrap();

    RESULT {
        name: "D14P1".to_string(),
        result: max - min,
        time: Duration::from_micros(0),
        percentage: 0f32,
    }
}

pub fn day14part2() -> RESULT {
    let lines: Vec<String> = read_file("day14").lines().map(|s| s.to_string()).collect();

    let mut inp: Vec<char> = lines[0].chars().collect();

    let mut mapping: HashMap<String, char> = HashMap::new();

    for i in 2..lines.len() {
        let sp: Vec<String> = lines[i].split(" -> ").map(|f| f.to_string()).collect();
        let c1: char = sp[1].chars().collect::<Vec<char>>()[0];
        mapping.insert(sp[0].clone(), c1);
    }

    let mut combined: HashMap<String, i64> = HashMap::new();
    let mut combined_count: HashMap<char, i64> = HashMap::new();
    for i in 0..inp.len() - 1 {
        let k = format!("{}{}", inp[i], inp[i + 1]);
        let v: i64 = *combined.get(&k).unwrap_or(&0);
        combined.insert(k, v + 1);

        let val: i64 = *combined_count.get(&inp[i]).unwrap_or(&0);
        combined_count.insert(inp[i], val + 1);
    }

    let val: i64 = *combined_count.get(&inp[inp.len() - 1]).unwrap_or(&0);
    combined_count.insert(inp[inp.len() - 1], val + 1);

    for i in 0..40 {
        let mut n_combined: HashMap<String, i64> = HashMap::new();
        for (_, (k, v)) in combined.iter().enumerate() {
            let c = *mapping.get(k).unwrap();
            let chars = k.chars().collect::<Vec<char>>();
            let left = format!("{}{}", chars[0], c);
            let right = format!("{}{}", c, chars[1]);

            let l_val = *n_combined.get(&left).unwrap_or(&0);
            let r_val = *n_combined.get(&right).unwrap_or(&0);

            n_combined.insert(left, v + l_val);
            n_combined.insert(right, v + r_val);
            let val: i64 = *combined_count.get(&c).unwrap_or(&0);
            combined_count.insert(c, val + v);
        }
        combined = n_combined
    }

    let (mut min, mut max) = (i64::MAX, i64::MIN);
    for (_, v ) in combined_count {
        if v < min {min = v}
        if v > max {max = v}
    }

    RESULT {
        name: "D14P2".to_string(),
        result: max - min,
        time: Duration::from_micros(0),
        percentage: 0f32,
    }
}
