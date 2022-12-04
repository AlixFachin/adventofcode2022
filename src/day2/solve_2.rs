use std::fs;

const FILEPATH: &str = "src/day2/input.txt";

// Compared to question 1, we will standardize player move and elf move
// but we will add a player strategy

enum PlayerStrategy {
    L, // player needs to lose
    D, // player need to draw
    W, // player needs to win
}

enum Move {
    R, // Rock
    P, // Paper
    S, // Scissors
}

struct Round {
    my_move: Move,
    elf_move: Move,
}

impl Round {
    fn get_score(&self) -> i32 {
        let own_score = match &self.my_move {
            Move::R => 1,
            Move::P => 2,
            Move::S => 3,
        };

        let battle_score = match (&self.my_move, &self.elf_move) {
            (Move::R, Move::R) | (Move::P, Move::P) | (Move::S, Move::S) => 3,
            (Move::R, Move::P) | (Move::P, Move::S) | (Move::S, Move::R) => 0,
            (Move::R, Move::S) | (Move::P, Move::R) | (Move::S, Move::P) => 6,
        };

        return own_score + battle_score;
    }
}

fn get_round(play_round: &str) -> Round {
    let chars_array: Vec<char> = play_round.chars().collect();
    let my_letter = chars_array[2];
    let elf_letter = chars_array[0];
    let strategy = match my_letter {
        'X' => PlayerStrategy::L,
        'Y' => PlayerStrategy::D,
        'Z' => PlayerStrategy::W,
        _ => panic!("Cannot match the player move"),
    };
    let elf_play = match elf_letter {
        'A' => Move::R,
        'B' => Move::P,
        'C' => Move::S,
        _ => panic!("Cannot match the elf move"),
    };

    let my_play = match (&elf_play, &strategy) {
        (Move::R, PlayerStrategy::L) => Move::S,
        (Move::R, PlayerStrategy::D) => Move::R,
        (Move::R, PlayerStrategy::W) => Move::P,
        (Move::S, PlayerStrategy::L) => Move::P,
        (Move::S, PlayerStrategy::D) => Move::S,
        (Move::S, PlayerStrategy::W) => Move::R,
        (Move::P, PlayerStrategy::L) => Move::R,
        (Move::P, PlayerStrategy::D) => Move::P,
        (Move::P, PlayerStrategy::W) => Move::S,
    };

    return {
        Round {
            my_move: my_play,
            elf_move: elf_play,
        }
    };
}

pub fn solve() {
    let contents = fs::read_to_string(FILEPATH).expect("Should have been able to read the file");
    let plays: Vec<&str> = contents.split('\n').collect();
    let mut score = 0;
    for play in plays {
        let current_round = get_round(play);
        score = score + current_round.get_score();
    }
    println!("Total score is {}", score);
}
