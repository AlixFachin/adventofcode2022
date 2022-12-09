use std::fs;

const FILEPATH: &str = "src/day8/input.txt";

fn count_up(trees: &Vec<Vec<i16>>, row: usize, col: usize) -> u64 {
    let mut i = row;
    let mut count: u64 = 0;
    let pivot = trees[row][col];
    while i > 0 {
        i = i - 1;
        count += 1;
        if trees[i][col] >= pivot {
            // one tree is blocking the view -> we exit
            return count;
        }
    }
    return count;
}

fn count_down(trees: &Vec<Vec<i16>>, row: usize, col: usize) -> u64 {
    let mut i = row + 1;
    let mut count: u64 = 0;
    let pivot = trees[row][col];
    while i < trees.len() {
        count += 1;
        if trees[i][col] >= pivot {
            // one tree is blocking the view -> we exit
            return count;
        }
        i = i + 1;
    }
    return count;
}

fn count_left(trees: &Vec<Vec<i16>>, row: usize, col: usize) -> u64 {
    let mut i = col;
    let mut count: u64 = 0;
    let pivot = trees[row][col];
    while i > 0 {
        i = i - 1;
        count += 1;
        if trees[row][i] >= pivot {
            // one tree is blocking the view -> we exit
            return count;
        }
    }
    return count;
}

fn count_right(trees: &Vec<Vec<i16>>, row: usize, col: usize) -> u64 {
    let mut i = col + 1;
    let mut count: u64 = 0;
    let pivot = trees[row][col];
    while i < trees.len() {
        count += 1;
        if trees[row][i] >= pivot {
            // one tree is blocking the view -> we exit
            return count;
        }
        i = i + 1;
    }
    return count;
}

pub fn get_count(trees: &Vec<Vec<i16>>, row: usize, col: usize) -> u64 {
    return count_up(&trees, row, col)
        * count_down(&trees, row, col)
        * count_left(&trees, row, col)
        * count_right(&trees, row, col);
}

#[test]
fn test_count() {
    let test_trees = vec![
        vec![1, 1, 7, 4],
        vec![1, 2, 3, 2],
        vec![7, 2, 3, 7],
        vec![1, 1, 7, 4],
    ];

    assert_eq!(count_up(&test_trees, 0, 0), 0);
    assert_eq!(count_up(&test_trees, 2, 2), 1);
    assert_eq!(count_up(&test_trees, 2, 3), 2);

    assert_eq!(count_down(&test_trees, 2, 3), 1);
    assert_eq!(count_down(&test_trees, 3, 1), 0);
    assert_eq!(count_down(&test_trees, 0, 2), 3);

    assert_eq!(count_right(&test_trees, 2, 3), 0);
    assert_eq!(count_right(&test_trees, 2, 1), 1);
    assert_eq!(count_right(&test_trees, 2, 0), 3);
    assert_eq!(count_right(&test_trees, 3, 0), 1);

    assert_eq!(count_left(&test_trees, 2, 2), 2);
    assert_eq!(count_left(&test_trees, 2, 0), 0);
    assert_eq!(count_left(&test_trees, 2, 3), 3);
}

pub fn solve() {
    let contents = fs::read_to_string(FILEPATH).expect("Should have been able to read the file");

    let mut trees: Vec<Vec<i16>> = Vec::new();

    // Step 1 -> filling the matrix of trees
    let tree_lines: Vec<&str> = contents.split("\n").collect();
    for line in tree_lines {
        let mut row: Vec<i16> = Vec::new();
        let tree_chars: Vec<char> = line.chars().collect();
        for tree in tree_chars {
            row.push(String::from(tree).parse::<i16>().unwrap());
        }
        trees.push(row);
    }

    println!("This is the table after parsing: \n {:?}", trees);
    // Step 2 -> Figuring out which tree is visible or not
    let mut max_score: u64 = 0;
    let n = (&trees).len();
    for row in 0..n {
        for col in 0..n {
            let current_score = get_count(&trees, row, col);
            if current_score > max_score {
                max_score = current_score;
            }
        }
    }

    println!("Solution of question 1 is {}", max_score);
}
