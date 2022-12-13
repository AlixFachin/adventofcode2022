use regex::Regex;
use std::fs;

const FILEPATH: &str = "src/day11/input.txt";

pub fn solve() {
    let contents = fs::read_to_string(FILEPATH).expect("Should have been able to read the file");

    let line_array: Vec<&str> = contents.split("\n").collect();

    //     let re_cd: Regex = Regex::new(r"^\$ cd (.+)$").unwrap();
    //     if re_cd.is_match(line_input) {
    //         let matches = re_cd.captures(line_input).unwrap();
    //         let directory_name = matches.get(1).map_or("", |x| x.as_str());
    //         return Some(CommandLine::ChangeDir(String::from(directory_name)));
    //     }
}
