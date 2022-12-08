use std::env;
use std::fs;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

fn display_usage() {
    println!("usage:");
    println!(" 'cargo run -- run x' will run the code corresponding to day x");
    println!(" 'cargo run -- add x' will add folder and code corresponding to day x");
}

fn add_folder(n: u16) {
    // Create directory
    let dirpath = format!("src/day{}", n);
    // TEST -> If the folder already exists, then panic (we don't want to overwrite already written code!)
    if std::path::Path::new(&dirpath).exists() {
        panic!("The path already exists - exiting...");
    }

    fs::create_dir(&dirpath).expect("Didn't manage to create the folder!");

    // Copy all the files from the template directory into the proper path
    let all_template_files =
        fs::read_dir("day_template").expect("Didn't manage to read the template file!");
    for template_file in all_template_files {
        let template_file_path = template_file.unwrap().path();
        println!("Copying file {}", template_file_path.display());
        let mut destination_file = std::path::PathBuf::from(&dirpath);
        destination_file.push(&template_file_path.file_name().unwrap());
        fs::copy(&template_file_path, destination_file).expect("Error in file copying!");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        display_usage();
        return;
    }

    let feature = &args[1];
    let series = &args[2];

    let series: u16 = series
        .trim()
        .parse()
        .expect("Should input the day series as number!");

    match feature.as_str() {
        "help" => {
            display_usage();
        }
        "run" => {
            println!(" running the suite {}", series);
            match series {
                1 => day1::solve(),
                2 => {
                    day2::solve_1::solve();
                    println!("Solving question 2 -=-=-=-==-=-=-=-=-=-=-=-=-=-=-=-");
                    day2::solve_2::solve();
                }
                3 => {
                    day3::solve_1::solve();
                    println!("Solving question 2 -=-=-=-==-=-=-=-=-=-=-=-=-=-=-=-");
                    day3::solve_2::solve();
                }
                4 => {
                    day4::solve_1::solve();
                    println!("Solving question 2 -=-=-=-==-=-=-=-=-=-=-=-=-=-=-=-");
                    day4::solve_2::solve();
                }
                5 => {
                    day5::solve_1::solve();
                    println!("Solving question 2 -=-=-=-==-=-=-=-=-=-=-=-=-=-=-=-");
                    day5::solve_2::solve();
                }
                6 => {
                    day6::solve_1::solve();
                    println!("Solving question 2 -=-=-=-==-=-=-=-=-=-=-=-=-=-=-=-");
                    day6::solve_2::solve();
                }
                7 => {
                    day7::solve_1::solve();
                }
                _ => println!("Module not found!"),
            }
        }
        "add" => add_folder(series),
        _ => {
            println!("usage: ");
            println!(" 'cargo run -- run x' will run the code corresponding to day x");
            println!(" 'cargo run -- add x' will add folder and code corresponding to day x");
        }
    }
}
