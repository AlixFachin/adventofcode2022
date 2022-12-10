use regex::Regex;
use std::fmt;
use std::{collections::HashMap, fs};

const FILEPATH: &str = "src/day9/input.txt";

// Problem has two phases:
//  a) Simulating the moves
//  b) recording the position of the head and the tail at each turn
//  c) counting the number of different positions for head and tail

#[derive(Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
}
impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{} {}]", self.x, self.y)
    }
}

struct RopeState {
    len: usize,
    knots: Vec<Position>,
}

// Choosing one move of the head
#[derive(Debug)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl Move {
    fn clone(&self) -> Move {
        return match self {
            Move::Down => Move::Down,
            Move::Up => Move::Up,
            Move::Left => Move::Left,
            Move::Right => Move::Right,
        };
    }
}

impl RopeState {
    fn init(&mut self) {
        for _i in 0..self.len {
            self.knots.push(Position { x: 0, y: 0 });
        }
    }

    fn get_tail_position(&self) -> &Position {
        // I don't mind the program panicking if the index is out of bounds
        return &(self.knots[self.len - 1]);
    }

    fn apply_move(&mut self, current_move: Move) {
        // Moving the head first, and then looping through the knots to propagate the move

        let mut current_knot = self.knots[0].clone();
        match current_move {
            Move::Up => current_knot.y = current_knot.y + 1,
            Move::Down => current_knot.y = current_knot.y - 1,
            Move::Left => current_knot.x = current_knot.x - 1,
            Move::Right => current_knot.x = current_knot.x + 1,
        }
        self.knots[0] = current_knot;

        for i in 1..self.len {
            // We compute the delta between the previous knot and the current knot
            let previous_knot: &Position = &self.knots[i - 1];
            current_knot = self.knots[i].clone();
            // Now we have to look at the difference
            let delta = Position {
                x: previous_knot.x - current_knot.x,
                y: previous_knot.y - current_knot.y,
            };
            let move_tail: Option<Position> = match delta {
                // Compared to question 1, we indeed have two more possible moves
                // case head on the left of tail
                Position { x: -2, y: -2 } => Some(Position { x: -1, y: -1 }),
                Position { x: -2, y: -1 } => Some(Position { x: -1, y: -1 }),
                Position { x: -2, y: 0 } => Some(Position { x: -1, y: 0 }),
                Position { x: -2, y: 1 } => Some(Position { x: -1, y: 1 }),
                Position { x: -2, y: 2 } => Some(Position { x: -1, y: 1 }),
                // Case head on top
                Position { x: -1, y: 2 } => Some(Position { x: -1, y: 1 }),
                Position { x: 0, y: 2 } => Some(Position { x: 0, y: 1 }),
                Position { x: 1, y: 2 } => Some(Position { x: 1, y: 1 }),
                // case head to the right
                Position { x: 2, y: -2 } => Some(Position { x: 1, y: -1 }),
                Position { x: 2, y: -1 } => Some(Position { x: 1, y: -1 }),
                Position { x: 2, y: 0 } => Some(Position { x: 1, y: 0 }),
                Position { x: 2, y: 1 } => Some(Position { x: 1, y: 1 }),
                Position { x: 2, y: 2 } => Some(Position { x: 1, y: 1 }),
                // case head at the bottom
                Position { x: -1, y: -2 } => Some(Position { x: -1, y: -1 }),
                Position { x: 0, y: -2 } => Some(Position { x: 0, y: -1 }),
                Position { x: 1, y: -2 } => Some(Position { x: 1, y: -1 }),
                _ => None,
            };
            // shadowing to unwrap the option
            let move_tail = move_tail.unwrap_or(Position { x: 0, y: 0 });

            current_knot.x = current_knot.x + move_tail.x;
            current_knot.y = current_knot.y + move_tail.y;
            self.knots[i] = current_knot;
        }
    }
}

pub fn solve() {
    let contents = fs::read_to_string(FILEPATH).expect("Should have been able to read the file");
    let line_vec = contents.split("\n");
    let mut move_list: Vec<Move> = Vec::new();
    let move_re = Regex::new(r"^([RULD]) (\d+)$").unwrap();
    for line in line_vec {
        let matches = move_re.captures(line).unwrap();
        let n: usize = matches[2].parse::<usize>().unwrap();
        let current_move = matches.get(1).map_or("", |x| x.as_str());
        let current_move_enum = match current_move {
            "U" => Move::Up,
            "D" => Move::Down,
            "L" => Move::Left,
            "R" => Move::Right,
            _ => panic!("Unexpected move character"),
        };
        for _i in 0..n {
            move_list.push(current_move_enum.clone());
        }
    }
    println!("The list of moves is: {:?}", &move_list);

    let mut rope_state = RopeState {
        len: 10,
        knots: Vec::new(),
    };
    rope_state.init();

    // We will use a hash to record the tail positions
    let mut tail_positions: HashMap<String, u32> = HashMap::new();

    for current_move in move_list {
        rope_state.apply_move(current_move);
        println!("\nThe rope is currently: {:?}", rope_state.knots);
        // We just need to insert 1 if not present.
        // we don't care about the actual values, just the number of keys in the hash-map
        let tail = rope_state.get_tail_position();
        tail_positions.insert(format!("({},{})", tail.x, tail.y), 1);
    }

    println!("Solution of question 2 is {}", tail_positions.keys().len());
}
