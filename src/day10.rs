use crate::util::{read_file, RESULT};
use core::panic;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::time::Duration;

pub fn day10part1() -> RESULT {
    let mut pairs = HashMap::new();
    pairs.insert('(', (')', 3));
    pairs.insert(')', ('(', 3));

    pairs.insert('[', (']', 57));
    pairs.insert(']', ('[', 57));

    pairs.insert('}', ('{', 1197));
    pairs.insert('{', ('}', 1197));

    pairs.insert('<', ('>', 25137));
    pairs.insert('>', ('<', 25137));

    let res = read_file("day10")
        .lines()
        .map(|x| {
            let mut stack = VecDeque::new();
            for i in x.chars() {
                match i {
                    '(' | '{' | '[' | '<' => stack.push_front(i),
                    ')' | '}' | ']' | '>' => {
                        let s_pop = stack.pop_front().unwrap_or(i);
                        let pair = pairs.get(&s_pop).unwrap();
                        if pair.0 != i {
                            return pairs.get(&i).unwrap().1;
                        }
                    }
                    _ => panic!("eh, it fucked up mate"),
                }
            }
            return 0;
        })
        .sum();

    RESULT {
        name: "D10P1".to_string(),
        result: res,
        time: Duration::from_micros(0),
        percentage: 0f32,
    }
}

pub fn day10part2() -> RESULT {
    let mut pairs = HashMap::new();
    pairs.insert('(', (')', 1));
    pairs.insert(')', ('(', 1));

    pairs.insert('[', (']', 2));
    pairs.insert(']', ('[', 2));

    pairs.insert('}', ('{', 3));
    pairs.insert('{', ('}', 3));

    pairs.insert('<', ('>', 4));
    pairs.insert('>', ('<', 4));

    let mut res = read_file("day10")
        .lines()
        .map(|x| {
            let mut stack = VecDeque::new();
            for i in x.chars() {
                match i {
                    '(' | '{' | '[' | '<' => stack.push_front(i),
                    ')' | '}' | ']' | '>' => {
                        let s_pop = stack.pop_front().unwrap_or(i);
                        let pair = pairs.get(&s_pop).unwrap();
                        if pair.0 != i {
                            return 0;
                        }
                    }
                    _ => panic!("eh, it fucked up mate"),
                }
            }
            stack
                .iter()
                .map(|f| pairs.get(f).unwrap().1)
                .fold(0i64, |acc, x| acc * 5 + x)
        })
        .filter(|&f| f != 0)
        .collect::<Vec<_>>();

    res.sort_unstable();
    RESULT {
        name: "D10P2".to_string(),
        result: res[res.len() / 2],
        time: Duration::from_micros(0),
        percentage: 0f32,
    }
}
