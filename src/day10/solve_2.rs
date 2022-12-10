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
    let mut string_to_display: String = String::new();
    for op in instruction_list {
        // Because the operation add takes place *AFTER* the two cycles,
        // we compute the signal strength before looking at the op
        let char_to_display;
        if ((i - 1) % 40) == x || ((i - 1) % 40) == x - 1 || ((i - 1) % 40) == x + 1 {
            char_to_display = '#';
        } else {
            char_to_display = '.';
        }
        println!(
            "Cycle {}, x={}, displaying {}, op = {:?}",
            &i, &x, &char_to_display, &op
        );
        string_to_display.push(char_to_display);

        if i % 40 == 0 {
            string_to_display.push('\n');
        }

        match &op {
            Operation::Add(d) => x = x + d,
            Operation::Noop => {}
        }

        i = i + 1;
    }
    println!("Solution for question 2 is \n{}", string_to_display);
}
