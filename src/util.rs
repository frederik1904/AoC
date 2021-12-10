use std::fs;

pub fn read_file(name: &str) -> String {
    let s= fs::read_to_string(format!("./inputs/{}.txt", name)).unwrap().to_owned();
    s
}