use crate::util;

const UP: &str = "up";
const DOWN: &str = "down";

pub fn day2part1() {
    let mut x = 0;
    let mut y = 0;
    util::read_file("day2part1")
    .lines()
    .for_each(|s| {
        let inp: Vec<&str> = s.split_whitespace().into_iter().collect();
        let i: i32 = inp[1].to_string().parse().unwrap();
        match inp[0] {
            UP => y -= i,
            DOWN => y += i,
            _ => x += i,
        }
    });

    println!("x: {}, y: {}, res: {}", x, y, x*y)
}

pub fn day2part2() {
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut a: i64 = 0;

    util::read_file("day2part1")
    .lines()
    .for_each(|s| {
        let inp: Vec<&str> = s.split_whitespace().into_iter().collect();
        let i: i64 = inp[1].to_string().parse().unwrap();
        match inp[0] {
            UP => {
                a -= i;
            },
            DOWN => {
                a += i;
            },
            _ => {
                x += i;
                y += i * a;
            },
        }
    });

    println!("x: {}, y: {}, res: {}", x, y, x*y)
}