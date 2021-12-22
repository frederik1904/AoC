use crate::util::{read_file, RESULT};
use std::time::Duration;
struct POINT {
    x: usize,
    y: usize,
}

pub fn day13part1() -> RESULT {
    let lines: Vec<String> = read_file("day13").lines().map(|s| s.to_string()).collect();

    let mut i = 0;
    let (mut max_x, mut max_y) = (0, 0);
    let mut points: Vec<POINT> = vec![];
    while lines[i] != "" {
        let tmp: Vec<i32> = lines[i].split(",").map(|f| f.parse().unwrap()).collect();
        let (x, y) = (tmp[0], tmp[1]);
        if x > max_x {
            max_x = x
        }
        if y > max_y {
            max_y = y
        }

        points.push(POINT {
            x: x as usize,
            y: y as usize,
        });

        i += 1;
    }
    i += 1;

    let mut matrix = vec![vec!['.'; max_x as usize + 1]; max_y as usize + 1];

    points.iter().for_each(|f| matrix[f.y][f.x] = '#');

    let fold: Vec<String> = lines[i]
        .replace("fold along ", "")
        .split("=")
        .map(|f| f.to_string())
        .collect();

    let split = fold[1].parse().unwrap();
    match fold[0].as_str() {
        "y" => {
            let bot: Vec<_> = matrix.iter().take(split).collect();
            let mut top: Vec<_> = matrix.iter().skip(split).collect();
            top.reverse();
            matrix = bot
                .iter()
                .zip(top.iter())
                .map(|(v1, v2)| {
                    v1.iter()
                        .zip(v2.iter())
                        .map(|(&f1, &f2)| if f1 == '#' || f2 == '#' { '#' } else { '.' })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<Vec<char>>>();
        }
        "x" => {
            let mut n_matrix: Vec<Vec<char>> = vec![vec![]; matrix.len()];
            for y in 0..matrix.len() {
                let left: Vec<_> = matrix[y].iter().take(split).collect();
                let mut right: Vec<_> = matrix[y].iter().skip(split).collect();
                right.reverse();
                n_matrix[y] = left
                    .iter()
                    .zip(right.iter())
                    .map(|(&&f1, &&f2)| if f1 == '#' || f2 == '#' { '#' } else { '.' })
                    .collect();
            }
            matrix = n_matrix;
        }
        _ => panic!("Somthing aint right"),
    }

    let sum = matrix
        .iter()
        .map(|f| {
            f.iter()
                .fold(0, |acc, &x| acc + if x == '#' { 1 } else { 0 })
        })
        .sum();

    RESULT {
        name: "D13P1".to_string(),
        result: sum,
        time: Duration::from_micros(0),
        percentage: 0f32,
    }
}

pub fn day13part2() -> RESULT {
    let lines: Vec<String> = read_file("day13").lines().map(|s| s.to_string()).collect();

    let mut i = 0;
    let (mut max_x, mut max_y) = (0, 0);
    let mut points: Vec<POINT> = vec![];
    while lines[i] != "" {
        let tmp: Vec<i32> = lines[i].split(",").map(|f| f.parse().unwrap()).collect();
        let (x, y) = (tmp[0], tmp[1]);
        if x > max_x {
            max_x = x
        }
        if y > max_y {
            max_y = y
        }

        points.push(POINT {
            x: x as usize,
            y: y as usize,
        });

        i += 1;
    }
    i += 1;

    let mut matrix = vec![vec!['.'; max_x as usize + 1]; max_y as usize + 1];

    points.iter().for_each(|f| matrix[f.y][f.x] = '#');
    while i < lines.len() {
        let fold: Vec<String> = lines[i]
            .replace("fold along ", "")
            .split("=")
            .map(|f| f.to_string())
            .collect();

        let split = fold[1].parse().unwrap();
        match fold[0].as_str() {
            "y" => {
                let bot: Vec<_> = matrix.iter().take(split).collect();
                let mut top: Vec<_> = matrix.iter().skip(split).collect();
                top.reverse();
                matrix = bot
                    .iter()
                    .zip(top.iter())
                    .map(|(v1, v2)| {
                        v1.iter()
                            .zip(v2.iter())
                            .map(|(&f1, &f2)| if f1 == '#' || f2 == '#' { '#' } else { '.' })
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<Vec<char>>>();
            }
            "x" => {
                let mut n_matrix: Vec<Vec<char>> = vec![vec![]; matrix.len()];
                for y in 0..matrix.len() {
                    let left: Vec<_> = matrix[y].iter().take(split).collect();
                    let mut right: Vec<_> = matrix[y].iter().skip(split).collect();
                    right.reverse();
                    n_matrix[y] = left
                        .iter()
                        .zip(right.iter())
                        .map(|(&&f1, &&f2)| if f1 == '#' || f2 == '#' { '#' } else { '.' })
                        .collect();
                }
                matrix = n_matrix;
            }
            _ => panic!("Somthing aint right"),
        }
        i += 1;
    }

    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            let t = matrix[y][x];
            print!("{}", t)
        }
        println!()
    }

    RESULT {
        name: "D13P2: TODO".to_string(),
        result: 0,
        time: Duration::from_micros(0),
        percentage: 0f32,
    }
}
