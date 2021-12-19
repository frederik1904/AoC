use std::time::Duration;

use crate::util::{read_file, RESULT};

#[derive(Clone, Copy)]
struct OCT {
    pub energy: u32,
    pub used: bool,
}

pub fn day11part1() -> RESULT {
    let mut matrix: Vec<Vec<OCT>> = read_file("day11")
        .lines()
        .map(|x| {
            x.chars()
                .map(|f| {
                    let e = f.to_string().parse::<u32>().unwrap();
                    let o = OCT {
                        energy: e,
                        used: false,
                    };
                    o
                })
                .collect::<Vec<OCT>>()
        })
        .collect();
        let dir: Vec<(i8, i8)> = vec![
            (-1,-1), (-1,0), (-1,1),
            (0,-1), (0, 1),
            (1, -1), (1, 0), (1, 1),
        ];
    let mut flashes = 0;
    for _ in 1..=100 {
        let mut rewalk: Vec<(usize, usize)> = vec![];
        for y in 0..matrix.len() {
            for x in 0..matrix[y].len() {
                matrix[y][x].energy += 1;
                if matrix[y][x].energy > 9 {
                    rewalk.push((x, y));
                }
            }
        }

        let mut index = 0;
        while index < rewalk.len() {
            let (x, y) = rewalk[index];
            if matrix[y][x].energy > 9 && !matrix[y][x].used {
                flashes += 1;
                matrix[y][x].used = true;
                for (nx, ny) in dir.clone() {
                    if (y != 0 || ny >= 0)
                    && (y < matrix.len() -1 || ny <= 0)
                    && (x != 0 || nx >= 0)
                    && (x < matrix[y].len() - 1 || nx <= 0)
                {
                    let (nny, nnx) = ((y as i8+ny) as usize, (x as i8 +nx) as usize);
                    matrix[nny][nnx].energy += 1;
                    if matrix[nny][nnx].energy > 9 && !matrix[nny][nnx].used {
                        rewalk.push((nnx, nny));
                    }
                }
                }
            }
            index += 1;
        }
        for y in 0..matrix.len() {
            for x in 0..matrix[y].len() {
                if matrix[y][x].used {
                    matrix[y][x].used = false;
                    matrix[y][x].energy = 0;
                }
            }
        }
    }

    RESULT {
        name: "D11P1".to_string(),
        result: flashes,
        time: Duration::from_micros(0),
        percentage: 0f32,
    }
}

pub fn day11part2() -> RESULT {
    let mut matrix: Vec<Vec<OCT>> = read_file("day11")
        .lines()
        .map(|x| {
            x.chars()
                .map(|f| {
                    let e = f.to_string().parse::<u32>().unwrap();
                    let o = OCT {
                        energy: e,
                        used: false,
                    };
                    o
                })
                .collect::<Vec<OCT>>()
        })
        .collect();
        let dir: Vec<(i8, i8)> = vec![
            (-1,-1), (-1,0), (-1,1),
            (0,-1), (0, 1),
            (1, -1), (1, 0), (1, 1),
        ];
    let mut round = 0;
    loop {
        let mut rewalk: Vec<(usize, usize)> = vec![];
        for y in 0..matrix.len() {
            for x in 0..matrix[y].len() {
                matrix[y][x].energy += 1;
                if matrix[y][x].energy > 9 {
                    rewalk.push((x, y));
                }
            }
        }

        let mut index = 0;
        let all = matrix.len() * matrix[0].len();
        while index < rewalk.len() {
            let (x, y) = rewalk[index];
            if matrix[y][x].energy > 9 && !matrix[y][x].used {
                matrix[y][x].used = true;
                for (nx, ny) in dir.clone() {
                    if (y != 0 || ny >= 0)
                    && (y < matrix.len() -1 || ny <= 0)
                    && (x != 0 || nx >= 0)
                    && (x < matrix[y].len() - 1 || nx <= 0)
                {
                    let (nny, nnx) = ((y as i8+ny) as usize, (x as i8 +nx) as usize);
                    matrix[nny][nnx].energy += 1;
                    if matrix[nny][nnx].energy > 9 && !matrix[nny][nnx].used {
                        rewalk.push((nnx, nny));
                    }
                }
                }
            }
            index += 1;
        }
        let mut reset = 0;
        for y in 0..matrix.len() {
            for x in 0..matrix[y].len() {
                if matrix[y][x].used {
                    matrix[y][x].used = false;
                    matrix[y][x].energy = 0;
                    reset += 1;
                }
            }
        }
        round += 1;
        if reset == all {
            break;
        }
    }

    RESULT {
        name: "D11P2".to_string(),
        result: round,
        time: Duration::from_micros(0),
        percentage: 0f32,
    }
}
