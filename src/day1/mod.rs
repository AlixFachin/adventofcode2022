use std::fs;

const FILEPATH: &str = "src/day1/input.txt";

pub fn solve() {
    let contents = fs::read_to_string(FILEPATH).expect("Should have been able to read the file");

    let elf_array: Vec<&str> = contents.split("\n\n").collect();
    let mut elf_total_calories: Vec<i32> = [].to_vec();

    for elf in elf_array {
        // elf contains a set of strings with this elf
        let snacks_list: Vec<&str> = elf.split('\n').collect();
        let mut elf_calories_total: i32 = 0;
        for snack in snacks_list {
            let current_calories = snack.parse::<i32>().expect("Woops. Failed to parse");
            elf_calories_total = elf_calories_total + current_calories;
        }
        elf_total_calories.push(elf_calories_total);
    }

    elf_total_calories.sort();
    elf_total_calories.reverse();

    for elf in &elf_total_calories {
        println!("Elf Calories: {}", elf);
    }

    println!("Answer to first questions is: {}", elf_total_calories[0]);
    println!(
        "Answer to second question is {}",
        (elf_total_calories[0] + elf_total_calories[1] + elf_total_calories[2])
    );
}
