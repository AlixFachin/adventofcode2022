use regex::Regex;
use std::fs;

const FILEPATH: &str = "src/day10/input.txt";
#[derive(Debug)]
enum Operation {
    Add(i32),
    Noop,
}

pub fn solve() {
    let contents = fs::read_to_string(FILEPATH).expect("Should have been able to read the file");
    let line_array: Vec<&str> = contents.split("\n").collect();

    let re_addx: Regex = Regex::new(r"^addx (-?\d+)$").unwrap();
    let re_noop: Regex = Regex::new(r"^noop$").unwrap();

    let mut instruction_list: Vec<Operation> = Vec::new();

    for line in line_array {
        if re_noop.is_match(line) {
            instruction_list.push(Operation::Noop);
            continue;
        }
        if re_addx.is_match(&line) {
            let captures = re_addx.captures(&line).unwrap();
            let operand = captures
                .get(1)
                .map(|x| x.as_str().parse::<i32>().unwrap())
                .unwrap();

            instruction_list.push(Operation::Noop); // because adding takes two cycles
            instruction_list.push(Operation::Add(operand));
        }
    }

    let mut x: i32 = 1;
    let mut i: i32 = 1;
    let mut signal_strength_sum = 0;
    for op in instruction_list {
        if i == 20 || i == 60 || i == 100 || i == 140 || i == 180 || i == 220 {
            signal_strength_sum = signal_strength_sum + i * x;
            println!(
                "Cycle {}, signal strength={}, sum={}",
                i,
                i * x,
                signal_strength_sum
            );
        }
        // Because the operation add takes place *AFTER* the two cycles,
        // we compute the signal strength before looking at the op
        match &op {
            Operation::Add(d) => x = x + d,
            Operation::Noop => {}
        }
        println!("Instruction: {:?}, x={}", &op, &x);

        i = i + 1;
    }
    println!("Solution for question 1 is {}", signal_strength_sum);
}
