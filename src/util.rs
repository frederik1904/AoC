use std::{fs, time::Duration};

pub fn read_file(name: &str) -> String {
    let s = fs::read_to_string(format!("./inputs/{}.txt", name))
        .unwrap()
        .to_owned();
    s
}

pub struct RESULT {
    pub name: String,
    pub result: i64,
    pub time: Duration,
    pub percentage: f32,
}

impl ToString for RESULT {
    fn to_string(&self) -> String {
        format!(
            "{} finished after {}ns ({:.4}%) with result: {}",
            self.name,
            self.time.as_nanos(),
            self.percentage,
            self.result
        )
    }
}
