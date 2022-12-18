use regex::Regex;
use std::fs;
use termion::{clear, color, cursor};

const FILEPATH: &str = "src/day14/input.txt";
const PUZZLE_WIDTH: usize = 800;
const PUZZLE_HEIGHT: usize = 200;
#[derive(Debug, Clone, PartialEq)]
enum Block {
    Rock,
    Empty,
    Rest,
}

fn display_map(
    cave_map: &Vec<Vec<Block>>,
    min_x: usize,
    max_x: usize,
    min_height: usize,
    max_height: usize,
) {
    print!("{}{}", clear::All, cursor::Goto(1, 2));
    for row in min_height..max_height.min(PUZZLE_HEIGHT - 1) {
        let display_slice = &cave_map[row][min_x..max_x.min(PUZZLE_WIDTH - 1)];
        for b in display_slice {
            match b {
                Block::Empty => print!("{}.", color::Fg(color::LightWhite)),
                Block::Rock => print!("{}X", color::Fg(color::White)),
                Block::Rest => print!("{}o", color::Fg(color::LightYellow)),
            }
        }
        print!("\n");
    }
}

fn draw_rock(cave_map: &mut Vec<Vec<Block>>, p1: (usize, usize), p2: (usize, usize)) {
    // we suppose only horizontal or vertical lines
    println!("Drawing a line btw {:?} and {:?}", p1, p2);
    if p1.0 == p2.0 {
        // horizontal line
        println!(
            "  Writing an horizontal line from {} to {} at {}",
            p1.1.min(p2.1),
            p1.1.max(p2.1),
            p1.0
        );
        for x in p1.1.min(p2.1)..=p1.1.max(p2.1) {
            cave_map[p1.0][x] = Block::Rock;
        }
    } else {
        println!(
            "  Writing a vertical line from {} to {} at {}",
            p1.0.min(p2.0),
            p1.0.max(p2.0),
            p1.1
        );
        for h in p1.0.min(p2.0)..=p1.0.max(p2.0) {
            cave_map[h][p1.1] = Block::Rock;
        }
    }
}

fn get_best_coords(new_point: &(usize, usize)) -> (usize, usize, usize, usize) {
    const DISPLAY_HALF_WIDTH: usize = 50;
    const DISPLAY_HALF_HEIGHT: usize = 50;
    const ROUNDING_WIDTH: usize = 10;
    const ROUNDING_HEIGHT: usize = 10;

    let ideal_height = new_point.0.max(DISPLAY_HALF_HEIGHT) - DISPLAY_HALF_HEIGHT;

    let min_h = (ideal_height.min(PUZZLE_HEIGHT - DISPLAY_HALF_HEIGHT).max(0) / ROUNDING_HEIGHT)
        * ROUNDING_HEIGHT;
    let max_h = min_h + DISPLAY_HALF_HEIGHT * 2;

    let ideal_x = new_point.1.max(DISPLAY_HALF_WIDTH) - DISPLAY_HALF_WIDTH;
    let min_x =
        (ideal_x.min(PUZZLE_WIDTH - DISPLAY_HALF_WIDTH).max(0) / ROUNDING_WIDTH) * ROUNDING_WIDTH;
    let max_x = min_x + DISPLAY_HALF_WIDTH * 2;

    return (min_x, max_x, min_h, max_h);
}

pub fn solve() {
    let contents = fs::read_to_string(FILEPATH).expect("Should have been able to read the file");

    let line_array: Vec<&str> = contents.split("\n").collect();

    let mut puzzle_map: Vec<Vec<Block>> = vec![vec![Block::Empty; PUZZLE_WIDTH]; PUZZLE_HEIGHT];

    let re_get_points: Regex = Regex::new(r"(?P<x>\d+),(?P<h>\d+)").unwrap();
    let mut max_h = 0;
    for line in line_array {
        let mut points_list: Vec<(usize, usize)> = Vec::new();

        for cap in re_get_points.captures_iter(line) {
            // We swap the dimensions here
            // By default the first index will be the height, so different than input!!!
            let h = cap["h"].parse::<usize>().unwrap();
            max_h = h.max(max_h);
            points_list.push((h, cap["x"].parse::<usize>().unwrap()));
        }

        for i in 0..points_list.len() - 1 {
            draw_rock(&mut puzzle_map, points_list[i], points_list[i + 1]);
        }
    }

    draw_rock(
        &mut puzzle_map,
        (max_h + 2, 0),
        (max_h + 2, PUZZLE_WIDTH - 1),
    );

    display_map(&puzzle_map, 430, 540, 0, 100);

    let mut turn_index: i64 = 0;
    let mut sand_full = false;
    while !sand_full {
        // One turn = one grain of sand
        let mut old_position = (0 as usize, 499 as usize);
        let mut new_position = (0 as usize, 500 as usize);
        while new_position != old_position {
            old_position = new_position.clone();
            // println!("Computing next position for {:?}", new_position);
            if new_position.0 < PUZZLE_HEIGHT - 1 {
                if puzzle_map[new_position.0 + 1][new_position.1] == Block::Empty {
                    // easiest case -> fall down
                    // println!(
                    //     "  Falling down until {:?}",
                    //     (new_position.0 + 1, new_position.1)
                    // );
                    new_position = (new_position.0 + 1, new_position.1);
                } else {
                    if puzzle_map[new_position.0 + 1][new_position.1 - 1] == Block::Empty {
                        // println!(
                        //     "  Moving down-left until {:?}",
                        //     (new_position.0 + 1, new_position.1 - 1)
                        // );
                        new_position = (new_position.0 + 1, new_position.1 - 1);
                    } else {
                        // println!(
                        //     "  Moving down-right until {:?}",
                        //     (new_position.0 + 1, new_position.1 + 1)
                        // );
                        if puzzle_map[new_position.0 + 1][new_position.1 + 1] == Block::Empty {
                            new_position = (new_position.0 + 1, new_position.1 + 1);
                        }
                    }
                }
            }
        }

        if new_position == (0, 500) {
            println!("The sand escapes at turn {}", turn_index);
            sand_full = true;
        } else {
            puzzle_map[new_position.0][new_position.1] = Block::Rest;

            if turn_index % 500 == 0 {
                let (min_x, max_x, min_h, max_h) = get_best_coords(&new_position);
                display_map(&puzzle_map, min_x, max_x, min_h, max_h);
            }
            // sleep(Duration::from_millis(10)).await;
            turn_index += 1;
        }
    }
}
