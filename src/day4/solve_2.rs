use regex::Regex;
use std::fs;

const FILEPATH: &str = "src/day4/input.txt";

// Day 4 -> Analyzing interval intersections

struct Interval {
    begin: i16,
    end: i16,
}

struct ElfPair {
    elf1: Interval,
    elf2: Interval,
}

impl ElfPair {
    fn is_overlapping(&self) -> bool {
        let b1 = self.elf1.begin;
        let e1 = self.elf1.end;
        let b2 = self.elf2.begin;
        let e2 = self.elf2.end;

        if e1 < b2 || e2 < b1 {
            return false;
        }
        return true;
    }
}

pub fn solve() {
    let contents = fs::read_to_string(FILEPATH).expect("Should have been able to read the file");
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)\n").unwrap();
    let mut pair_vector: Vec<ElfPair> = Vec::new();
    println!("Capturing regex...");
    let mut nr_overlapping = 0;
    for cap in re.captures_iter(&contents) {
        pair_vector.push(ElfPair {
            elf1: Interval {
                begin: cap[1].parse::<i16>().unwrap(),
                end: cap[2].parse::<i16>().unwrap(),
            },
            elf2: Interval {
                begin: cap[3].parse::<i16>().unwrap(),
                end: cap[4].parse::<i16>().unwrap(),
            },
        });
    }
    for pair in &pair_vector {
        if pair.is_overlapping() {
            nr_overlapping += 1;
        }
    }
    println!("Nr of interval overlapping: {}", nr_overlapping);
}
