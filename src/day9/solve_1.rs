use regex::Regex;
use std::{collections::HashMap, fs};

const FILEPATH: &str = "src/day9/input_test.txt";

// Problem has two phases:
//  a) Simulating the moves
//  b) recording the position of the head and the tail at each turn
//  c) counting the number of different positions for head and tail

struct Position {
    x: i32,
    y: i32,
}

struct RopeState {
    head: Position,
    tail: Position,
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
    fn apply_move(&mut self, current_move: Move) {
        // Moving the head
        match current_move {
            Move::Up => self.head.y = self.head.y + 1,
            Move::Down => self.head.y = self.head.y - 1,
            Move::Left => self.head.x = self.head.x - 1,
            Move::Right => self.head.x = self.head.x + 1,
        }
        // Now we have to move the tail
        let delta = Position {
            x: self.head.x - self.tail.x,
            y: self.head.y - self.tail.y,
        };

        let move_tail: Option<Position> = match delta {
            // case head on the left of tail
            Position { x: -2, y: -1 } => Some(Position { x: -1, y: -1 }),
            Position { x: -2, y: 0 } => Some(Position { x: -1, y: 0 }),
            Position { x: -2, y: 1 } => Some(Position { x: -1, y: 1 }),
            // Case head on top
            Position { x: -1, y: 2 } => Some(Position { x: -1, y: 1 }),
            Position { x: 0, y: 2 } => Some(Position { x: 0, y: 1 }),
            Position { x: 1, y: 2 } => Some(Position { x: 1, y: 1 }),
            // case head to the right
            Position { x: 2, y: -1 } => Some(Position { x: 1, y: -1 }),
            Position { x: 2, y: 0 } => Some(Position { x: 1, y: 0 }),
            Position { x: 2, y: 1 } => Some(Position { x: 1, y: 1 }),
            // case head at the bottom
            Position { x: -1, y: -2 } => Some(Position { x: -1, y: -1 }),
            Position { x: 0, y: -2 } => Some(Position { x: 0, y: -1 }),
            Position { x: 1, y: -2 } => Some(Position { x: 1, y: -1 }),
            _ => None,
        };
        // shadowing to unwrap the option
        let move_tail = move_tail.unwrap_or(Position { x: 0, y: 0 });

        self.tail.x = self.tail.x + move_tail.x;
        self.tail.y = self.tail.y + move_tail.y;

        println!(
            "Head pos: ({},{}) -- Tail Pos: ({},{})",
            &self.head.x, &self.head.y, &self.tail.x, &self.tail.y
        );
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
        head: Position { x: (0), y: (0) },
        tail: Position { x: (0), y: (0) },
    };

    // We will use a hash to record the tail positions
    let mut tail_positions: HashMap<String, u32> = HashMap::new();

    for current_move in move_list {
        rope_state.apply_move(current_move);
        // We just need to insert 1 if not present.
        // we don't care about the actual values, just the number of keys in the hash-map
        tail_positions.insert(format!("({},{})", rope_state.tail.x, rope_state.tail.y), 1);
    }

    println!("Solution of question 1 is {}", tail_positions.keys().len());
}
