use crate::util;

pub fn day7part1() {
    let mut crab: Vec<i32> = util::read_file("day7part1")
    .split(",")
    .map(|f| f.parse::<i32>().unwrap())
    .collect();
    crab.sort_unstable();
    let median = crab[crab.len() / 2];
    let res: i32 = crab.iter().map(|f| (i32::abs(median - f))).sum();
    
    println!("res: {}", res);
}

pub fn day7part2() {
    let mut crab: Vec<i32> = util::read_file("day7part1")
    .split(",")
    .map(|f| f.parse::<i32>().unwrap())
    .collect();
    crab.sort_unstable();

    let min = crab.iter().min().unwrap().clone();
    let  max = crab.iter().max().unwrap().clone();

    let mut sum: i32 = i32::MAX;

    for i in min..max {
        sum = sum.min(crab
        .iter()
        .map(|f| i32::abs(i - f) *(i32::abs(i - f) + 1) / 2)
        .sum());
    }
    
    println!("res: {}", sum);
}
