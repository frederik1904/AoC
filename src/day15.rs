use std::{collections::LinkedList, time::Duration};

use crate::util::{read_file, RESULT};

pub fn day15part1() -> RESULT {
    let mut matrix: Vec<Vec<u8>> = vec![];
    read_file("day15")
        .lines()
        .for_each(|f| matrix.push(f.chars().map(|s| s.to_string().parse().unwrap()).collect()));
    let mut cost_matrix = vec![vec![i32::MAX; matrix[0].len()]; matrix.len()];
    cost_matrix[0][0] = 0;
    let mut stack = LinkedList::from([(0, 0)]);
    while stack.len() > 0 {
        let (x, y) = stack.pop_back().unwrap();
        let cur_val = cost_matrix[y][x];
        if x > 0 {
            if (cur_val + matrix[y][x - 1] as i32) < cost_matrix[y][x - 1] {
                cost_matrix[y][x - 1] = cur_val + matrix[y][x - 1] as i32;
                stack.push_front((x - 1, y));
            }
        }
        if x < matrix[y].len() - 1 {
            if (cur_val + matrix[y][x + 1] as i32) < cost_matrix[y][x + 1] {
                cost_matrix[y][x + 1] = cur_val + matrix[y][x + 1] as i32;
                stack.push_front((x + 1, y));
            }
        }
        if y > 0 {
            if (cur_val + matrix[y - 1][x] as i32) < cost_matrix[y - 1][x] {
                cost_matrix[y - 1][x] = cur_val + matrix[y - 1][x] as i32;
                stack.push_front((x, y - 1));
            }
        }
        if y < matrix.len() - 1 {
            if (cur_val + matrix[y + 1][x] as i32) < cost_matrix[y + 1][x] {
                cost_matrix[y + 1][x] = cur_val + matrix[y + 1][x] as i32;
                stack.push_front((x, y + 1));
            }
        }
    }

    RESULT {
        name: "D15P1".to_string(),
        result: *cost_matrix.iter().last().unwrap().iter().last().unwrap() as i64,
        time: Duration::from_micros(0),
        percentage: 0f32,
    }
}

pub fn day15part2() -> RESULT {
    let mut matrix: Vec<Vec<u8>> = vec![];
    read_file("day15")
        .lines()
        .for_each(|f| matrix.push(f.chars().map(|s| s.to_string().parse().unwrap()).collect()));

    
    let (w, h) = (matrix.len() * 5, matrix.len() * 5);
    let mut r_matrix: Vec<Vec<u8>> = vec![vec![0; w]; h];
    for y in 0..h {
        for x in 0..w {
            let val = if x < matrix.len() && y < matrix.len() {
                matrix[x][y]
            } else {
                let top = if y > matrix.len() - 1 {r_matrix[y - matrix.len()][x]} else {0};
                let left = if x > matrix.len() - 1 {r_matrix[y][x - matrix.len()]} else {0};
                (if top > left {top} else {left} + 1)
            };
            r_matrix[y][x] = (val as i8 % 10).max(1) as u8;
        }
    }
    
    matrix = r_matrix;
    let mut cost_matrix = vec![vec![i32::MAX; matrix[0].len()]; matrix.len()];
    cost_matrix[0][0] = 0;
    let mut stack = LinkedList::from([(0, 0)]);
    while stack.len() > 0 {
        let (x, y) = stack.pop_back().unwrap();
        let cur_val = cost_matrix[y][x];
        if x > 0 {
            if (cur_val + matrix[y][x - 1] as i32) < cost_matrix[y][x - 1] {
                cost_matrix[y][x - 1] = cur_val + matrix[y][x - 1] as i32;
                stack.push_front((x - 1, y));
            }
        }
        if x < matrix[y].len() - 1 {
            if (cur_val + matrix[y][x + 1] as i32) < cost_matrix[y][x + 1] {
                cost_matrix[y][x + 1] = cur_val + matrix[y][x + 1] as i32;
                stack.push_front((x + 1, y));
            }
        }
        if y > 0 {
            if (cur_val + matrix[y - 1][x] as i32) < cost_matrix[y - 1][x] {
                cost_matrix[y - 1][x] = cur_val + matrix[y - 1][x] as i32;
                stack.push_front((x, y - 1));
            }
        }
        if y < matrix.len() - 1 {
            if (cur_val + matrix[y + 1][x] as i32) < cost_matrix[y + 1][x] {
                cost_matrix[y + 1][x] = cur_val + matrix[y + 1][x] as i32;
                stack.push_front((x, y + 1));
            }
        }
    }
    RESULT {
        name: "D15P2".to_string(),
        result: *cost_matrix.iter().last().unwrap().iter().last().unwrap() as i64,
        time: Duration::from_micros(0),
        percentage: 0f32,
    }
}
