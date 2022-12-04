use std::fs;

const FILEPATH: &str = "src/day2/input.txt";

enum PlayerMove {
    X,
    Y,
    Z,
}

enum ElfMove {
    A,
    B,
    C,
}

struct Round {
    my_move: PlayerMove,
    elf_move: ElfMove,
}

impl Round {
    fn get_score(&self) -> i32 {
        let own_score = match &self.my_move {
            PlayerMove::X => 1,
            PlayerMove::Y => 2,
            PlayerMove::Z => 3,
        };

        let battle_score = match (&self.my_move, &self.elf_move) {
            (PlayerMove::X, ElfMove::A)
            | (PlayerMove::Y, ElfMove::B)
            | (PlayerMove::Z, ElfMove::C) => 3,
            (PlayerMove::X, ElfMove::B)
            | (PlayerMove::Y, ElfMove::C)
            | (PlayerMove::Z, ElfMove::A) => 0,
            (PlayerMove::X, ElfMove::C)
            | (PlayerMove::Y, ElfMove::A)
            | (PlayerMove::Z, ElfMove::B) => 6,
        };

        return own_score + battle_score;
    }
}

fn get_round(play_round: &str) -> Round {
    println!("Trying to parse the string {}", &play_round);
    let chars_array: Vec<char> = play_round.chars().collect();
    let my_letter = chars_array[2];
    let elf_letter = chars_array[0];
    let my_play = match my_letter {
        'X' => PlayerMove::X,
        'Y' => PlayerMove::Y,
        'Z' => PlayerMove::Z,
        _ => panic!("Cannot match the player move"),
    };
    let elf_play = match elf_letter {
        'A' => ElfMove::A,
        'B' => ElfMove::B,
        'C' => ElfMove::C,
        _ => panic!("Cannot match the elf move"),
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
