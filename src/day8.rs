use crate::util::{read_file, RESULT};
use std::{
    collections::{HashMap, HashSet},
    time::Duration,
};

pub fn day8part1() -> RESULT {
    let unique_lens = vec![2, 4, 3, 7];
    let sum: usize = read_file("day8part1")
        .lines()
        .map(|f| f.split(" | ").last().unwrap())
        .map(|f| {
            f.split(" ")
                .filter(|s| unique_lens.contains(&s.len()))
                .collect::<Vec<_>>()
                .len()
        })
        .sum();

    RESULT {
        name: "D8P1".to_string(),
        result: (sum) as i64,
        time: Duration::from_micros(0),
        percentage: 0f32,
    }
}

// Transform char to digit
fn d(d: u8) -> u8 {
    2u8.pow((d - b'a') as u32)
}

pub fn day8part2() -> RESULT {
    let unique_lens = vec![
        (6, d(b'a') + d(b'b') + d(b'c') + d(b'e') + d(b'f') + d(b'g')), // 0
        (2, d(b'c') + d(b'f')),                                         // 1
        (5, d(b'a') + d(b'c') + d(b'd') + d(b'e') + d(b'g')),           // 2
        (5, d(b'a') + d(b'c') + d(b'd') + d(b'f') + d(b'g')),           // 3
        (4, d(b'b') + d(b'c') + d(b'd') + d(b'f')),                     // 4
        (5, d(b'a') + d(b'b') + d(b'd') + d(b'f') + d(b'g')),           // 5
        (6, d(b'a') + d(b'b') + d(b'd') + d(b'e') + d(b'f') + d(b'g')), // 6
        (3, d(b'a') + d(b'c') + d(b'f')),                               // 7
        (
            7,
            d(b'a') + d(b'b') + d(b'c') + d(b'd') + d(b'e') + d(b'f') + d(b'g'),
        ), // 8
        (6, d(b'a') + d(b'b') + d(b'c') + d(b'd') + d(b'f') + d(b'g')), // 9
    ];
    let result: i64 = read_file("day8part1")
        .lines()
        .map(|f| {
            let mut segment_map: HashMap<u8, u8> = HashMap::new();
            let mut undecided: HashMap<usize, HashSet<u8>> = HashMap::new();

            let last = f.split(" | ").last().unwrap();
            f.replace("| ", "").split(" ").for_each(|s| {
                s.split(" ").for_each(|s| {
                    let c = s
                        .chars()
                        .map(|t| t as u8 - b'a')
                        .fold(0u8, |acc, t| acc + 2u8.pow(t as u32));
                    if s.len() == unique_lens[1].0 {
                        segment_map.insert(unique_lens[1].1, c);
                    } else if s.len() == unique_lens[4].0 {
                        segment_map.insert(unique_lens[4].1, c);
                    } else if s.len() == unique_lens[7].0 {
                        segment_map.insert(unique_lens[7].1, c);
                    } else if s.len() == unique_lens[8].0 {
                        segment_map.insert(unique_lens[8].1, c);
                    } else {
                        let n_hmap = HashSet::new();
                        let mut u: HashSet<_> = undecided.get(&s.len()).unwrap_or(&n_hmap).clone();
                        u.insert(c);
                        undecided.insert(s.len(), u.clone());
                    };
                })
            });

            let one = segment_map.get(&unique_lens[1].1).unwrap().clone();
            let four = segment_map.get(&unique_lens[4].1).unwrap().clone();
            let seven = segment_map.get(&unique_lens[7].1).unwrap().clone();
            let eight = segment_map.get(&unique_lens[8].1).unwrap().clone();

            let six = undecided
                .get(&6)
                .unwrap()
                .iter()
                .filter(|&&f| (f & one) != one)
                .last()
                .unwrap();

            let nine = undecided
                .get(&6)
                .unwrap()
                .iter()
                .filter(|&&f| (four & f) == four)
                .last()
                .unwrap();

            let zero = undecided
                .get(&6)
                .unwrap()
                .iter()
                .filter(|&&f| f != *nine && f != *six)
                .last()
                .unwrap();

            let f = six & one;
            let c = one ^ f;

            let three = undecided
                .get(&5)
                .unwrap()
                .iter()
                .filter(|&&s| (s & one) == one)
                .last()
                .unwrap();

            let two = undecided
                .get(&5)
                .unwrap()
                .iter()
                .filter(|&&f| (f & c) == c && f != *three)
                .last()
                .unwrap();

            let five = undecided
                .get(&5)
                .unwrap()
                .iter()
                .filter(|&&s| s != *two && s != *three)
                .last()
                .unwrap();

            segment_map = HashMap::new();
            segment_map.insert(*zero, 0);
            segment_map.insert(one, 1);
            segment_map.insert(*two, 2);
            segment_map.insert(*three, 3);
            segment_map.insert(four, 4);
            segment_map.insert(*five, 5);
            segment_map.insert(*six, 6);
            segment_map.insert(seven, 7);
            segment_map.insert(eight, 8);
            segment_map.insert(*nine, 9);

            last.split(" ")
                .map(|st| {
                    segment_map
                        .get(
                            &st.chars()
                                .map(|t| t as u8 - b'a')
                                .fold(0u8, |acc, t| (acc + 2u8.pow(t as u32))),
                        )
                        .unwrap()
                })
                .fold(0i64, |acc, &f| acc * 10 + f as i64)
        })
        .sum();
    RESULT {
        name: "D8P2".to_string(),
        result: (result) as i64,
        time: Duration::from_micros(0),
        percentage: 0f32,
    }
}
