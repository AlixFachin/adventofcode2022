use std::fs;

const FILEPATH: &str = "src/day8/input.txt";

fn is_visible_up(trees: &Vec<Vec<i16>>, row: usize, col: usize) -> bool {
    let mut i = row;
    let pivot = trees[row][col];
    while i > 0 {
        i = i - 1;
        if trees[i][col] >= pivot {
            // one tree is blocking the view -> we exit
            return false;
        }
    }
    return true;
}

fn is_visible_down(trees: &Vec<Vec<i16>>, row: usize, col: usize) -> bool {
    let mut i = row + 1;
    let pivot = trees[row][col];
    while i < trees.len() {
        if trees[i][col] >= pivot {
            // one tree is blocking the view -> we exit
            println!("Tree is blocking the view {},{}", i, col);
            return false;
        }
        i = i + 1;
    }
    return true;
}

fn is_visible_left(trees: &Vec<Vec<i16>>, row: usize, col: usize) -> bool {
    let mut i = col;
    let pivot = trees[row][col];
    while i > 0 {
        i = i - 1;
        if trees[row][i] >= pivot {
            // one tree is blocking the view -> we exit
            return false;
        }
    }
    return true;
}

fn is_visible_right(trees: &Vec<Vec<i16>>, row: usize, col: usize) -> bool {
    let mut i = col + 1;
    let pivot = trees[row][col];
    while i < trees.len() {
        if trees[row][i] >= pivot {
            // one tree is blocking the view -> we exit
            return false;
        }
        i = i + 1;
    }
    return true;
}

pub fn is_visible(trees: &Vec<Vec<i16>>, row: usize, col: usize) -> bool {
    // One tree is visible if it is on the edges *OR* if it is visible from one direction

    let n = trees.len();
    if row == 0 || col == 0 || row == n - 1 || col == n - 1 {
        return true;
    }

    // Not on the edge -> need to look at the four directions
    return is_visible_up(&trees, row, col)
        || is_visible_down(&trees, row, col)
        || is_visible_left(&trees, row, col)
        || is_visible_right(&trees, row, col);
}

#[test]
fn test_is_visible() {
    let test_trees = vec![
        vec![1, 1, 7, 4],
        vec![1, 2, 3, 2],
        vec![7, 2, 3, 7],
        vec![1, 1, 7, 4],
    ];

    // All the edge values are visible
    assert!(is_visible(&test_trees, 0, 0));
    assert!(is_visible(&test_trees, 0, 1));
    assert!(is_visible(&test_trees, 0, 2));
    assert!(is_visible(&test_trees, 0, 3));
    assert!(is_visible(&test_trees, 1, 0));
    assert!(is_visible(&test_trees, 1, 3));
    assert!(is_visible(&test_trees, 2, 0));
    assert!(is_visible(&test_trees, 2, 3));
    assert!(is_visible(&test_trees, 3, 0));
    assert!(is_visible(&test_trees, 3, 1));
    assert!(is_visible(&test_trees, 3, 2));
    assert!(is_visible(&test_trees, 3, 3));

    // Center Values are visible if one
    assert!(is_visible(&test_trees, 1, 1));
    assert!(is_visible_up(&test_trees, 1, 1));
    assert!(!is_visible_up(&test_trees, 2, 1));
    assert!(!is_visible_up(&test_trees, 2, 2));
    assert!(!is_visible_down(&test_trees, 2, 2));
    assert!(!is_visible_right(&test_trees, 2, 2));
    assert!(!is_visible_left(&test_trees, 2, 2));

    println!("flf");
    assert!(is_visible_down(&test_trees, 2, 1));

    assert!(!is_visible(&test_trees, 2, 2));
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
    let mut nr_visible: i64 = 0;
    let n = (&trees).len();
    for row in 0..n {
        for col in 0..n {
            if is_visible(&trees, row, col) {
                nr_visible = nr_visible + 1;
            }
        }
    }

    println!("Solution of question 1 is {}", nr_visible);
}
