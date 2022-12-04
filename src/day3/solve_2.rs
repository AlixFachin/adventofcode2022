use std::fs;

const FILEPATH: &str = "src/day3/input.txt";

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
    let mut group_index: usize = 0;

    while group_index * 3 + 2 < bags.len() {
        let (b1, b2, b3): (&str, &str, &str) = (
            bags[group_index * 3],
            bags[group_index * 3 + 1],
            bags[group_index * 3 + 2],
        );

        let mut found: bool = false;
        for c in b1.chars() {
            // The complexity is not great, but let's hope that rust is quick enough!
            if !found & b2.contains(c) & b3.contains(c) {
                found = true;
                println!("Yes -> the common item is {}", c);
                priority_sum += get_item_prority(&c);
            }
        }

        println!("Group {}: {},   {},  {}", group_index, b1, b2, b3);
        group_index += 1;
    }

    println!("Result of first question {}", priority_sum);
}
