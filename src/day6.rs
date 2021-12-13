use crate::util;

struct FISH {
    amount: i64,
    state: i32
}

impl FISH {
    pub fn end_day(&mut self) -> Option<FISH> {
        self.state -= 1;
        if self.state == -1 {
            self.state = 6;
            Some(FISH{state:9, amount: self.amount})
        } else {
            None
        }
    }
}

pub fn day6part1() {
    let mut fishes: Vec<FISH> = vec![];
    util::read_file("day6part1")
    .split(",")
    .for_each(|f| fishes.push(FISH{state: f.parse().unwrap(), amount: 1}));

    for _ in 0..80 {
        let mut index = 0;
        while index < fishes.len() {
           match fishes[index].end_day() {
               None => (),
               Some(f) => fishes.push(f)
           }
           index += 1;
        }
    }
    println!("res: {}", fishes.len());
}

pub fn day6part2() {
    let mut fishes: Vec<FISH> = vec![];
    let mut numbers = vec![0; 6];
    util::read_file("day6part1")
    .split(",")
    .for_each(|f| {
        let i = f.parse::<usize>().unwrap();
        numbers[i] += 1;
    });
    for i in 0..numbers.len() {
        let fish = FISH{state: i as i32, amount: numbers[i]};
        fishes.push(fish);
    }

    for _ in 0..256 {
        let mut index = 0;
        let mut amounts: i64 = 0;
        while index < fishes.len() {
           match fishes[index].end_day() {
               None => (),
               Some(f) => amounts += f.amount
           }
           index += 1;
        }
        fishes.push(FISH{state: 8, amount: amounts})
    }
    println!("res: {}", fishes.iter().map(|f| f.amount).sum::<i64>());
}
