use std::time::Duration;

use crate::util::{read_file, RESULT};

pub fn day3part1() -> RESULT {
    let mut eht = Vec::new();
    let res: Vec<String> = read_file("day3")
        .lines()
        .map(|f| f.to_string())
        .collect();

    for _ in 0..(2 * res[0].chars().count()) {
        eht.push(0);
    }

    res.iter().for_each(|x| {
        let mut current_value = 0;
        for c in x.chars() {
            match c {
                '0' => {
                    eht[2 * current_value as usize] += 1;
                }
                _ => {
                    eht[2 * current_value as usize + 1] += 1;
                }
            }
            current_value += 1
        }
    });
    let mut hight = 0;
    let mut low = 0;
    eht.reverse();
    for i in 0..res[0].chars().count() {
        if eht[i * 2] >= eht[i * 2 + 1] {
            hight += 2i32.pow((i as i32).try_into().unwrap())
        } else {
            low += 2i32.pow((i as i32).try_into().unwrap())
        }
    }
    RESULT {
        name: "D3P1".to_string(),
        result: (low * hight) as i64,
        time: Duration::from_micros(0),
        percentage: 0f32,
    }
}

#[derive(Clone, Debug)]
struct NodeId {
    index: usize,
}

#[derive(Clone, Debug)]
struct NODE {
    count: i32,
    left: Option<NodeId>,
    right: Option<NodeId>,
    index: NodeId,
}

impl NODE {
    pub fn get_index(&self) -> usize {
        self.index.index
    }

    pub fn increment(&mut self) {
        self.count += 1;
    }
}

struct ARENA {
    pub nodes: Vec<NODE>,
}

impl ARENA {
    pub fn get_left(&self, node: NODE) -> Option<NODE> {
        match node.left {
            None => Option::None,
            Some(e) => Option::Some(self.nodes[e.index].clone()),
        }
    }

    pub fn get_right(&self, node: NODE) -> Option<NODE> {
        match node.right {
            None => Option::None,
            Some(e) => Option::Some(self.nodes[e.index].clone()),
        }
    }

    pub fn insert_left(&mut self, parent: NODE, child: NODE) -> NODE {
        let index = NodeId {
            index: self.nodes.len(),
        };
        let n_child = NODE {
            count: child.count,
            left: None,
            right: None,
            index: index.clone(),
        };
        self.nodes.push(n_child);
        self.nodes[parent.get_index()].left = Option::Some(index);
        self.nodes[parent.get_index()].clone()
    }

    pub fn insert_right(&mut self, parent: NODE, child: NODE) -> NODE {
        let index = NodeId {
            index: self.nodes.len(),
        };
        let n_child = NODE {
            count: child.count,
            left: None,
            right: None,
            index: index.clone(),
        };
        self.nodes.push(n_child);
        self.nodes[parent.get_index()].right = Option::Some(index);
        self.nodes[parent.get_index()].clone()
    }

    pub fn increment(&mut self, node: NODE) {
        self.nodes[node.get_index()].increment();
    }
}

pub fn day3part2() -> RESULT {
    let mut a = ARENA {
        nodes: vec![NODE {
            left: None,
            right: None,
            count: 0,
            index: NodeId { index: 0 },
        }],
    };

    read_file("day3")
        .lines()
        .map(|f| f.to_string())
        .for_each(|x| process_string(&mut a, x));
    let mut high = a.nodes[0].clone();
    let mut high_val: Vec<i32> = vec![];
    let mut low = a.nodes[0].clone();
    let mut low_val: Vec<i32> = vec![];

    while high.left.is_some() || high.right.is_some() {
        let (l_h, r_h): (Option<NODE>, Option<NODE>) =
            (a.get_left(high.clone()), a.get_right(high.clone()));
        let (l_l, r_l): (Option<NODE>, Option<NODE>) =
            (a.get_left(low.clone()), a.get_right(low.clone()));
        if l_h.is_none()
            || (r_h.is_some() && r_h.clone().unwrap().count >= l_h.clone().unwrap().count)
        {
            high = r_h.unwrap().clone();
            high_val.push(1);
        } else {
            high = l_h.unwrap();
            high_val.push(0);
        }

        if l_l.is_none()
            || (r_l.is_some() && r_l.clone().unwrap().count < l_l.clone().unwrap().count)
        {
            low = r_l.unwrap().clone();
            low_val.push(1);
        } else {
            low = l_l.unwrap().clone();
            low_val.push(0);
        }
    }
    high_val.reverse();
    low_val.reverse();
    let (mut h, mut l) = (0, 0);
    for i in 0..high_val.len() {
        if high_val[i] == 1 {
            h += 2i32.pow((i as i32).try_into().unwrap())
        }

        if low_val[i] == 1 {
            l += 2i32.pow((i as i32).try_into().unwrap())
        }
    }

    RESULT {
        name: "D3P2".to_string(),
        result: (l * h) as i64,
        time: Duration::from_micros(0),
        percentage: 0f32,
    }
}

fn process_string(a: &mut ARENA, x: String) {
    let mut curr_node = a.nodes[0].clone();
    for c in x.chars() {
        curr_node = match c {
            '0' => {
                let tmp = a.get_left(curr_node.clone());
                if tmp.is_none() {
                    curr_node = a.insert_left(
                        curr_node.clone(),
                        NODE {
                            count: 0,
                            left: None,
                            right: None,
                            index: NodeId { index: 0 },
                        },
                    )
                }
                a.get_left(curr_node.clone()).unwrap()
            }
            '1' => {
                let tmp = a.get_right(curr_node.clone());
                if tmp.is_none() {
                    curr_node = a.insert_right(
                        curr_node.clone(),
                        NODE {
                            count: 0,
                            left: None,
                            right: None,
                            index: NodeId { index: 0 },
                        },
                    )
                }
                a.get_right(curr_node.clone()).unwrap()
            }
            _ => panic!("OH FUCK OH SHIT"),
        };
        a.increment(curr_node.clone());
    }
}
