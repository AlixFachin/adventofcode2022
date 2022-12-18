use std::{collections::VecDeque, fs};

const FILEPATH: &str = "src/day12/input.txt";
const END_CODE: i32 = 'z' as i32 - 'a' as i32;

#[derive(Debug)]
struct GameData {
    start: (isize, isize),
    end: (isize, isize),
    nr_rows: usize,
    nr_cols: usize,
    height_matrix: Vec<Vec<i32>>,
}

fn can_move(
    game_data: &GameData,
    current_row: usize,
    current_col: usize,
    to_row: isize,
    to_col: isize,
    visited_table: &Vec<Vec<bool>>,
) -> bool {
    if to_row < 0
        || to_row >= game_data.nr_rows as isize
        || to_col < 0
        || to_col >= game_data.nr_cols as isize
    {
        return false;
    }

    if visited_table[to_row as usize][to_col as usize] {
        return false;
    }

    return game_data.height_matrix[to_row as usize][to_col as usize]
        <= (game_data.height_matrix[current_row][current_col] + 1);
}

fn get_shortest_path(game_data: &GameData, visited_table: &mut Vec<Vec<bool>>) -> Option<i32> {
    let mut queue: VecDeque<_> = VecDeque::new();
    queue.push_back((game_data.start.0, game_data.start.1, 0));

    while let Some((row, col, len)) = queue.pop_front() {
        if (row, col) == game_data.end {
            return Some(len);
        }

        for (delta_row, delta_col) in vec![(1 as isize, 0), (0, 1), (-1, 0), (0, -1 as isize)] {
            let (next_row, next_col) = ((row + delta_row), (col + delta_col));
            if can_move(
                game_data,
                row as usize,
                col as usize,
                next_row,
                next_col,
                visited_table,
            ) {
                let next_row: usize = next_row as usize;
                let next_col: usize = next_col as usize;

                visited_table[next_row][next_col] = true;
                queue.push_back((next_row as isize, next_col as isize, len + 1));
            }
        }
    }

    return None;
}

#[test]
fn test_algo() {
    let test_game = GameData {
        start: (1, 1),
        end: (2, 2),
        nr_rows: 4,
        nr_cols: 4,
        height_matrix: vec![
            vec![1, 4, 3, 4],
            vec![1, 2, 3, 4],
            vec![2, 2, 2, 4],
            vec![1, 2, 3, 4],
        ],
    };

    let visited_table = vec![
        vec![false, false, false, false],
        vec![false, true, false, false],
        vec![true, false, true, false],
        vec![false, true, false, false],
    ];

    // Testing if we refuse moves which were already visited
    assert!(!can_move(&test_game, 2, 1, 2, 0, &visited_table));
    assert!(!can_move(&test_game, 2, 1, 2, 2, &visited_table));
    assert!(!can_move(&test_game, 2, 1, 1, 1, &visited_table));
    assert!(!can_move(&test_game, 2, 1, 3, 1, &visited_table));
    // Do we refuse moves which are on the edge
    assert!(!can_move(&test_game, 0, 0, 0, -1, &visited_table));
    assert!(!can_move(&test_game, 0, 0, -1, 0, &visited_table));
    assert!(!can_move(&test_game, 3, 0, 4, 0, &visited_table));
    assert!(!can_move(&test_game, 0, 3, 0, 4, &visited_table));
    assert!(!can_move(&test_game, 3, 3, 4, 3, &visited_table));
    assert!(!can_move(&test_game, 3, 3, 3, 4, &visited_table));
    // Do we refuse a move which is higher than the current stuff
    assert!(!can_move(&test_game, 1, 1, 0, 1, &visited_table));
    // Do we allow a move equal +1
    assert!(can_move(&test_game, 1, 1, 1, 2, &visited_table));
    // Do we allow an equal move
    assert!(can_move(&test_game, 1, 1, 2, 1, &visited_table));
    // Do we allow a move to lower height
    assert!(can_move(&test_game, 1, 1, 1, 0, &visited_table));
}

pub fn solve() {
    let contents = fs::read_to_string(FILEPATH).expect("Should have been able to read the file");

    let mut game_data = GameData {
        height_matrix: Vec::new(),
        start: (0, 0),
        end: (0, 0),
        nr_cols: 0,
        nr_rows: 0,
    };

    let line_array: Vec<&str> = contents.split("\n").collect();

    let mut row = 0;
    for line in line_array {
        let mut col = 0;
        let mut line_vec: Vec<i32> = Vec::new();
        for c in line.chars() {
            match c {
                'S' => {
                    game_data.start = (row, col);
                    line_vec.push(0);
                }
                'E' => {
                    game_data.end = (row, col);
                    line_vec.push(END_CODE);
                }
                _ => {
                    line_vec.push(c as i32 - ('a' as i32));
                }
            }
            col = col + 1;
        }
        game_data.height_matrix.push(line_vec);
        row = row + 1;
    }
    game_data.nr_rows = game_data.height_matrix.len();
    game_data.nr_cols = game_data.height_matrix[0].len();

    // Now we have the height matrix, the start position and the end position.
    // We have to figure out the _quickest_ way to get there
    // We can move only U,D,R,L on the grid, but we can only move up one level

    // Recursive algorithm:
    // We have the current point (X,Y), the number of steps so far
    // Need to keep an array of places already visited

    // Idea to optimize: eliminate cells which have only too high neighbours?

    let mut already_visited: Vec<Vec<bool>>;
    already_visited = Vec::new();
    for i in 0..game_data.nr_rows {
        let line_vec = &game_data.height_matrix[i];
        let mut bool_vec: Vec<bool> = Vec::new();
        for j in 0..line_vec.len() {
            if (i as isize, j as isize) == game_data.start {
                bool_vec.push(true);
            } else {
                bool_vec.push(false);
            }
        }
        already_visited.push(bool_vec);
    }

    // Here we go for the nasty loop
    let algo_result = get_shortest_path(&game_data, &mut already_visited);

    println!("Result of question 1 is {}", algo_result.unwrap());
}
