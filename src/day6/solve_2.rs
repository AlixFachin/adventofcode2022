use std::fs;

const FILEPATH: &str = "src/day6/input.txt";
const MARKER_LENGTH: usize = 14;

// return true if the string has a duplicated character
fn has_doubles(src: &str) -> bool {
    // the easy way is just to check if the char
    let mut chars_array: Vec<char> = src.chars().collect();
    chars_array.sort();
    for i in 0..chars_array.len() - 1 {
        if chars_array[i] == chars_array[i + 1] {
            return true;
        }
    }
    return false;
}

pub fn solve() {
    let contents = fs::read_to_string(FILEPATH).expect("Should have been able to read the file");

    // Now we browse the string until we get
    let mut i: usize = 0;
    let mut found = false;
    while !found && i < contents.len() - MARKER_LENGTH {
        let temp_slice = &contents[i..i + MARKER_LENGTH];

        found = !has_doubles(&temp_slice);
        i += 1;
    }
    println!(
        "The answer question 1 is {}, string is {}",
        i + MARKER_LENGTH - 1,
        &contents[i - 1..i - 1 + MARKER_LENGTH],
    );
}
