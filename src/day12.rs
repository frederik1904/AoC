use std::{collections::HashMap, time::Duration};

use crate::util::{read_file, RESULT};

const END: &str = "end";
const START: &str = "start";

fn recourse(link: HashMap<&str, Vec<&str>>, visited: &mut Vec<&str>) -> i64 {
    let last = *visited.last().unwrap();
    let mut sum = 0;
    for n in link.get(last).unwrap() {
        if *n != START && (n.to_lowercase() != *n || !visited.contains(&n)) {
            if *n == END {
                sum += 1
            } else {
                let mut n_visited = visited.clone();
                let s = n.to_owned();
                n_visited.push(s);
                sum += recourse(link.clone(), &mut n_visited);
            }
        }
    }
    sum
}

pub fn day12part1() -> RESULT {
    let mut route: HashMap<&str, Vec<&str>> = HashMap::new();

    let strings = read_file("day12");

    strings.lines().for_each(|f| {
        let sp: Vec<&str> = f.split("-").collect();
        // DO ONE WAY
        let mut v = match route.get(sp[0]) {
            None => vec![],
            Some(s) => s.clone(),
        };
        if !v.contains(&sp[1]) {
            v.push(sp[1]);
            route.insert(sp[0], v);
        }

        // DO OTHER WAY
        let mut v = match route.get(sp[1]) {
            None => vec![],
            Some(s) => s.clone(),
        };
        if !v.contains(&sp[0]) {
            v.push(sp[0]);
            route.insert(sp[1], v);
        }
    });
    let mut res = 0;
    for s in route.get("start").unwrap().clone() {
        res += recourse(route.clone(), &mut vec![s]);
    }

    RESULT {
        name: "D12P1".to_string(),
        result: res,
        time: Duration::from_micros(0),
        percentage: 0f32,
    }
}

fn recourse_v2(link: HashMap<&str, Vec<&str>>, visited: &mut Vec<&str>, double_time: bool) -> i64 {
    let last = *visited.last().unwrap();
    let mut sum = 0;
    for n in link.get(last).unwrap() {
        let contained = visited.contains(&n) && n.to_lowercase() == *n;
        if *n != START && (n.to_lowercase() != *n || ((contained && !double_time) || !contained)) {
            if *n == END {
                sum += 1;
            } else {
                let mut n_visited = visited.clone();
                let s = n.to_owned();
                n_visited.push(s);
                sum += recourse_v2(link.clone(), &mut n_visited, contained || double_time);
            }
        }
    }
    sum
}

pub fn day12part2() -> RESULT {
    let mut route: HashMap<&str, Vec<&str>> = HashMap::new();

    let strings = read_file("day12");

    strings.lines().for_each(|f| {
        let sp: Vec<&str> = f.split("-").collect();
        // DO ONE WAY
        let mut v = match route.get(sp[0]) {
            None => vec![],
            Some(s) => s.clone(),
        };
        if !v.contains(&sp[1]) {
            v.push(sp[1]);
            route.insert(sp[0], v);
        }

        // DO OTHER WAY
        let mut v = match route.get(sp[1]) {
            None => vec![],
            Some(s) => s.clone(),
        };
        if !v.contains(&sp[0]) {
            v.push(sp[0]);
            route.insert(sp[1], v);
        }
    });
    let mut res = 0;
    for s in route.get("start").unwrap().clone() {
        res += recourse_v2(route.clone(), &mut vec![s], false);
    }
    RESULT {
        name: "D12P2".to_string(),
        result: res,
        time: Duration::from_micros(0),
        percentage: 0f32,
    }
}
