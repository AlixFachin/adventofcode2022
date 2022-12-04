use std::fs;

const FILEPATH: &str = "src/day3/input.txt";

fn split_string(original: &str) -> (&str, &str) {
    let half_length: usize = original.len() / 2;
    return (&original[0..half_length], &original[half_length..]);
}

fn get_item_prority(tgt: &char) -> u32 {
    return match &tgt {
        'a'..='z' => (*tgt as u32) - ('a' as u32) + 1,
        'A'..='Z' => (*tgt as u32) - ('A' as u32) + 27,
        _ => panic!("Cannot translate char!"),
    };
}

pub fn solve() {
    let contents = fs::read_to_string(FILEPATH).expect("Should have been able to read the file");
    let bags: Vec<&str> = contents.split('\n').collect();

    let mut priority_sum: u32 = 0;

    for bag in bags {
        // split the string
        let (s1, s2): (&str, &str) = split_string(&bag);
        let mut found: bool = false;
        // Check the occurences
        for c in s1.chars() {
            if !found & s2.contains(c) {
                println!("Yes - {} is inside {}", c, s2);
                found = true;
                priority_sum = priority_sum + get_item_prority(&c);
                println!("Current priority: {}", priority_sum);
                continue;
            }
        }
        println!(
            "the first bag is : {}, split in {} and {}, prioritySum={}",
            bag, s1, s2, priority_sum
        );
    }

    println!("Result of first question {}", priority_sum);

    // For each "bag",
    //  - separate the string in two substrings (two compartments)
    //  - check the item which is in both strings (for each char check if it is in string 2. Or maybe sort both strings and browse)
    //  - get the value of this item and add it to the total priority misplaced
}
