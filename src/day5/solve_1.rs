use regex::Regex;
use std::fs;

const FILEPATH: &str = "src/day5/input.txt";

const TEST_MODE: bool = false;

pub fn solve() {
    let contents = fs::read_to_string(FILEPATH).expect("Should have been able to read the file");
    // Ok so parsing the initial setting would be too complicated. It seems that
    let stacks_test: Vec<&str> = vec!["ZN", "MCD", "P"];
    let stacks_prod: Vec<&str> = vec![
        "DMSZRFWN", "WPQGS", "WRVQFNJC", "FZPCGDL", "TPS", "HDFWRL", "ZNDC", "WNRFVSJQ", "RMSGZWV",
    ];
    let stacks_init = if TEST_MODE { stacks_test } else { stacks_prod };

    let mut stacks: Vec<String> = vec![];
    for stack_description in stacks_init {
        let mut s: String = String::new();
        s.push_str(stack_description);
        stacks.push(s);
    }

    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)\n").unwrap();

    for crate_move in re.captures_iter(&contents) {
        let nr_crates = crate_move[1].parse::<usize>().unwrap();
        let from_stack = crate_move[2].parse::<usize>().unwrap() - 1;
        let to_stack = crate_move[3].parse::<usize>().unwrap() - 1;
        println!(
            "Moving {} from {} to {}",
            &nr_crates, &from_stack, &to_stack
        );
        for i in 1..=nr_crates {
            // The lines below seems like a strange dance to allow mut variables
            let mut x = stacks[from_stack].clone();
            let c = x.pop().expect("woops");
            let mut d = stacks[to_stack].clone();
            d.push(c);
            stacks[to_stack] = d;
            stacks[from_stack] = x;
        }
        for stack_after in &stacks {
            println!("  {}", &stack_after);
        }
    }
    let mut final_string: String = String::new();
    for stack_final in &stacks {
        if stack_final.len() > 0 {
            // To get the last character
            let mut s: String = String::new();
            s.push_str(stack_final);
            let c = s.pop().expect("the stack should not be empty");
            final_string.push(c);
        }
    }
    println!("Solution is {}", final_string);
}
