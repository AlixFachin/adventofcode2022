use std::fs;

const FILEPATH: &str = "src/day2/input.txt";

pub fn solve() {
    let contents = fs::read_to_string(FILEPATH).expect("Should have been able to read the file");
}
