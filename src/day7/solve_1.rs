// use regex::Regex;
// use std::collections::HashMap;
// use std::fs;

// const FILEPATH: &str = "src/day7/input_test.txt";

// // Description of filesystem structure
// struct Dir {
//     name: String,
//     files: HashMap<String, u64>, // size of the files contained in this directory
//     dirs: Vec<String>,           // size of the file
// }

// impl Dir {
//     fn add_file(&mut self, file_name: &str, file_size: u64) {
//         self.files.insert(String::from(file_name), file_size);
//     }
//     fn add_dir(&mut self, dir_name: &str) {
//         self.dirs.push(String::from(dir_name));
//     }
//     fn get_size(&self, dir_list: &HashMap<String, Dir>) -> u64 {
//         // to get the size, return the sum of all the files and then look for the dir catalogue
//         // (And let's pray that the problem doesn't contain any circular references...)
//         let mut sum_files: u64 = 0;
//         for (_file, size) in &self.files {
//             sum_files += size;
//         }
//         let mut sum_dirs: u64 = 0;
//         for dir_name in &self.dirs {
//             let sub_dir = dir_list.get(dir_name.as_str());
//             match sub_dir {
//                 Some(sub_dir) => {
//                     sum_dirs += sub_dir.get_size(dir_list);
//                 }
//                 None => {}
//             }
//         }
//         return sum_files + sum_dirs;
//     }
// }

// #[test]
// fn test_fs() {
//     let mut root_dir = Dir {
//         name: String::from("/"),
//         files: HashMap::new(),
//         dirs: HashMap::new(),
//     };

//     root_dir.add_file("first", 254);
//     assert!(root_dir.files.contains_key("first"));
//     assert_eq!(root_dir.get_size(), 254);
//     root_dir.add_file("second", 200);
//     assert_eq!(root_dir.get_size(), 454);
//     root_dir.add_dir("first_d");
//     let d = root_dir.get_dir("first_d");
//     d.add_file("file_3", 150);
//     d.add_file("second_d", 210);
//     assert_eq!(d.get_size(), 360);
//     assert_eq!(root_dir.get_size(), 454 + 360);
// }

// // ------------ LINE PARSING ---------------

// #[derive(Debug, PartialEq)]
// enum CommandLine {
//     ChangeDir(String),
//     Ls,
//     File { name: String, size: u64 },
//     Directory(String),
// }

// fn parse_line(line_input: &str) -> Option<CommandLine> {
//     // Yes I know - the regex are compiled each time the function is called
//     // but I didn't manage to make global variables out of those.
//     let re_cd: Regex = Regex::new(r"^\$ cd (.+)$").unwrap();
//     let re_ls: Regex = Regex::new(r"^\$ ls").unwrap();
//     let re_dir: Regex = Regex::new(r"^dir (\w+)").unwrap();
//     let re_file: Regex = Regex::new(r"^(\d+) (.+)$").unwrap();

//     if re_cd.is_match(line_input) {
//         let matches = re_cd.captures(line_input).unwrap();
//         let directory_name = matches.get(1).map_or("", |x| x.as_str());
//         return Some(CommandLine::ChangeDir(String::from(directory_name)));
//     }
//     if re_ls.is_match(line_input) {
//         return Some(CommandLine::Ls);
//     }
//     if re_dir.is_match(line_input) {
//         let matches = re_dir.captures(line_input).unwrap();
//         let directory_name = matches.get(1).map_or("", |x| x.as_str());
//         return Some(CommandLine::Directory(String::from(directory_name)));
//     }
//     if re_file.is_match(line_input) {
//         let matches = re_file.captures(line_input).unwrap();
//         let file_size = matches[1].parse::<u64>().unwrap();
//         return Some(CommandLine::File {
//             name: (String::from(&matches[2])),
//             size: (file_size),
//         });
//     }
//     println!("No matches!");
//     return None;
// }

// #[test]
// fn test_regex() {
//     let cd_string = "$ cd fcqv";
//     let ls_string = "$ ls";
//     let dir_string = "dir jljrdvw";
//     let file1_string = "247592 zldbq";
//     let file2_string = "96717 wdqqqv.pcr";

//     assert_eq!(parse_line(ls_string), Some(CommandLine::Ls));
//     assert_eq!(
//         parse_line(cd_string),
//         Some(CommandLine::ChangeDir(String::from("fcqv")))
//     );
//     assert_eq!(
//         parse_line(dir_string),
//         Some(CommandLine::Directory(String::from("jljrdvw")))
//     );
//     assert_eq!(
//         parse_line(file1_string),
//         Some(CommandLine::File {
//             name: (String::from("zldbq")),
//             size: (247592)
//         })
//     );
//     assert_eq!(
//         parse_line(file2_string),
//         Some(CommandLine::File {
//             name: (String::from("wdqqqv.pcr")),
//             size: (96717)
//         })
//     );
// }

pub fn solve() {
    // // In a first step we will build the directory, then we will browse through it to find the
    // let mut current_dir_name = String::from("/");
    // let mut all_dirs: HashMap<String, Dir> = HashMap::new();

    // // Setup of line parsing -> regular expressions

    // let contents = fs::read_to_string(FILEPATH).expect("Should have been able to read the file");
    // let line_array: Vec<&str> = contents.split("\n").collect();
    // for line in line_array {
    //     println!("Parsing line {}", line);
    //     let command = parse_line(&line);
    //     // match command {
    //     //     Some(CommandLine::Directory(dir_name)) => {
    //     //         let current_dir: &Dir = all_dirs
    //     //             .get(&current_dir_name)
    //     //             .expect("Cannot find directory!");
    //     //         let mut new_dir_list = current_dir.dirs.clone();
    //     //         new_dir_list.push(dir_name.clone());
    //     //         let new_dir = Dir {
    //     //             name: String::from(current_dir_name),
    //     //             files: current_dir.files.clone(),
    //     //             dirs: new_dir_list,
    //     //         };
    //     //         // all_dirs.insert(current_dir_name.clone(), new_dir);
    //     //     }
    //     //     Some(CommandLine::File { name, size }) => {
    //     //         let current_dir = all_dirs
    //     //             .get(&current_dir_name)
    //     //             .expect("Cannot find directory! (add file)");
    //     //         let mut new_file_list = current_dir.files.clone();
    //     //         new_file_list.insert(name, size);
    //     //         let new_dir = Dir {
    //     //             name: String::from(current_dir_name),
    //     //             files: new_file_list,
    //     //             dirs: current_dir.dirs.clone(),
    //     //         };
    //     //         // all_dirs.insert(current_dir_name.clone(), new_dir);
    //     //     }
    //     //     Some(CommandLine::ChangeDir(dir_name)) => {
    //     //         if dir_name != ".." {
    //     //             // We insert the directory in the reference table
    //     //             let new_dir = Dir {
    //     //                 name: String::from(dir_name),
    //     //                 files: HashMap::new(),
    //     //                 dirs: Vec::new(),
    //     //             };

    //     //             // current_dir_name = String::from(&dir_name);
    //     //             all_dirs.insert(String::from(dir_name), new_dir);
    //     //         }
    //     //     }
    //     //     Some(CommandLine::Ls) => {}
    //     //     None => panic!("Don't manage to parse the line {}", line),
    //     // }
    // }

    // Now (theoretically) we have built our file system
    // We can answer the question
}
