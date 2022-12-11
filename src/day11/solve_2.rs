use std::fs;

const FILEPATH: &str = "src/day11/input.txt";

pub fn solve() {
    let contents = fs::read_to_string(FILEPATH).expect("Should have been able to read the file");
    let line_array: Vec<&str> = contents.split("\n").collect();

    // let re_cd: Regex = Regex::new(r"^\$ cd (.+)$").unwrap();
    // if re_cd.is_match(line_input) {
    //     let matches = re_cd.captures(line_input).unwrap();
    //     let directory_name = matches.get(1).map_or("", |x| x.as_str());
    //     return Some(CommandLine::ChangeDir(String::from(directory_name)));
    // }
}

// Monkey Rules:

// ONE ROUND
//   Monkey 0 inspects all items (= one monkey's turn)
//      inspects: apply its own operation for the worry level
//      after inspection, before the test: < worry level is divided by 3 and rounded down >
//      tests if the worry level conforms to the tests
//      Throw item to other monkey according to Rules
//      When all the items are inspected, turn goes to next monkey

// When throw to other monkey, item goes to END of the list (FIFO)
// If monkey has no item, its turn ends

// Objective question 1:
// Count the number of times the monkey inspected an item!
