use crate::util;

pub fn day1part1() {
    let res = util::read_file("day1part1")
    .lines()
    .map(|x| x.parse::<i32>().unwrap())
    .fold((0,0), |acc, x| (acc.0 + if x > acc.1 {1} else {0}, x));

    println!("{}", res.0);
}

pub fn day1part2() {
    let str = util::read_file("day1part1");
    let nums = str
    .lines()
    .map(|x| x.parse::<i32>().unwrap())
    .collect::<Vec<i32>>();
    
    let mut a = nums[0]+nums[1]+nums[2];
    let mut sum = 0;
    for i in 1..nums.len()-2 {
        let b = nums[i]+nums[i+1]+nums[i+2];
        if a < b {
            sum += 1;
        }
        a = b;
    }

    println!("{}", sum);
}