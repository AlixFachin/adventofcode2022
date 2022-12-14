use regex::Regex;
use std::collections::HashMap;
use std::fs;

const FILEPATH: &str = "src/day7/input.txt";
// Description of filesystem structure
#[derive(Debug)]
struct Dir {
    files: HashMap<String, u64>,
    dirs: HashMap<String, usize>,
}
#[derive(Debug)]
struct FileSystem {
    dir_list: Vec<Dir>,
    root: usize,
    current_dir: usize,
}

impl FileSystem {
    fn init(&mut self) {
        self.dir_list = vec![Dir {
            files: HashMap::new(),
            dirs: HashMap::new(),
        }];
        self.root = 0;
        self.current_dir = 0;
    }

    fn mk_dir(&mut self, dir_name: &str) -> usize {
        let mut new_dir = Dir {
            files: HashMap::new(),
            dirs: HashMap::new(),
        };
        new_dir
            .dirs
            .insert(String::from(".."), self.current_dir.clone());
        // Adding the directory in the filesystem Dir array
        self.dir_list.push(new_dir);
        let dir_ref = self.dir_list.len() - 1;
        // Referencing the directory in the list of subfolders for the current directory
        self.dir_list[self.current_dir]
            .dirs
            .insert(String::from(dir_name), dir_ref.clone());

        return dir_ref;
    }

    fn add_file(&mut self, file_name: &str, file_size: u64) {
        // We add a file in the current directory
        self.dir_list[self.current_dir].add_file(file_name, file_size);
    }

    fn change_dir(&mut self, new_dir: &str) {
        if new_dir == "/" {
            self.current_dir = self.root.clone();
            return;
        }

        let current_dir = self.dir_list.get(self.current_dir).unwrap();
        let new_dir_ref = current_dir
            .dirs
            .get(new_dir)
            .expect("Cannot find sub-directory in change_dir!");
        self.current_dir = new_dir_ref.clone();
    }

    fn get_size(&self, dir_ref: usize) -> u64 {
        let mut sum_files: u64 = 0;
        for (_file, file_size) in &self.dir_list[dir_ref].files {
            sum_files += file_size;
        }

        let mut sum_dir_sizes: u64 = 0;
        for (subdir_name, subdir_ref) in &self.dir_list[dir_ref].dirs {
            if subdir_name != ".." {
                let subdir_size = self.get_size(*subdir_ref);
                sum_dir_sizes += subdir_size;
            }
        }

        return sum_dir_sizes + sum_files;
    }

    fn get_all_sizes(&self) -> Vec<u64> {
        let mut all_sizes_vec = Vec::new();
        for i in 0..self.dir_list.len() {
            all_sizes_vec.push(self.get_size(i));
        }
        return all_sizes_vec;
    }
}

impl Dir {
    fn add_file(&mut self, file_name: &str, file_size: u64) {
        self.files.insert(String::from(file_name), file_size);
    }
}

#[test]
fn test_fs() {
    let mut main_fs = FileSystem {
        root: 0,
        dir_list: Vec::new(),
        current_dir: 0,
    };

    main_fs.init();

    main_fs.add_file("first", 100);
    assert!(main_fs.dir_list[0].files.contains_key("first"));
    assert_eq!(main_fs.get_size(0), 100);

    main_fs.add_file("second", 150);
    assert_eq!(main_fs.dir_list[0].files.len(), 2);

    main_fs.mk_dir("first_dir");
    assert_eq!(main_fs.dir_list[0].dirs.len(), 1);

    // root_dir.add_file("second", 200);
    // assert_eq!(root_dir.get_size(&main_fs), 454);
    // let new_dir_ref = main_fs.mk_dir("first_d", root_dir_ref);
    // root_dir.add_dir_ref("first_d", new_dir_ref);

    // let d_ref = root_dir.get_dir_ref("first_d");
    // let d = main_fs.get_dir_mut(*d_ref);

    // d.add_file("file_3", 150);
    // d.add_file("second_d", 210);
    // assert_eq!(d.get_size(&main_fs), 360);
    // assert_eq!(root_dir.get_size(&main_fs), 454 + 360);
}

// ------------ LINE PARSING ---------------

#[derive(Debug, PartialEq)]
enum CommandLine {
    ChangeDir(String),
    Ls,
    File { name: String, size: u64 },
    Directory(String),
}

fn parse_line(line_input: &str) -> Option<CommandLine> {
    // Yes I know - the regex are compiled each time the function is called
    // but I didn't manage to make global variables out of those.
    let re_cd: Regex = Regex::new(r"^\$ cd (.+)$").unwrap();
    let re_ls: Regex = Regex::new(r"^\$ ls").unwrap();
    let re_dir: Regex = Regex::new(r"^dir (\w+)").unwrap();
    let re_file: Regex = Regex::new(r"^(\d+) (.+)$").unwrap();

    if re_cd.is_match(line_input) {
        let matches = re_cd.captures(line_input).unwrap();
        let directory_name = matches.get(1).map_or("", |x| x.as_str());
        return Some(CommandLine::ChangeDir(String::from(directory_name)));
    }
    if re_ls.is_match(line_input) {
        return Some(CommandLine::Ls);
    }
    if re_dir.is_match(line_input) {
        let matches = re_dir.captures(line_input).unwrap();
        let directory_name = matches.get(1).map_or("", |x| x.as_str());
        return Some(CommandLine::Directory(String::from(directory_name)));
    }
    if re_file.is_match(line_input) {
        let matches = re_file.captures(line_input).unwrap();
        let file_size = matches[1].parse::<u64>().unwrap();
        return Some(CommandLine::File {
            name: (String::from(&matches[2])),
            size: (file_size),
        });
    }
    println!("No matches!");
    return None;
}

#[test]
fn test_regex() {
    let cd_string = "$ cd fcqv";
    let ls_string = "$ ls";
    let dir_string = "dir jljrdvw";
    let file1_string = "247592 zldbq";
    let file2_string = "96717 wdqqqv.pcr";

    assert_eq!(parse_line(ls_string), Some(CommandLine::Ls));
    assert_eq!(
        parse_line(cd_string),
        Some(CommandLine::ChangeDir(String::from("fcqv")))
    );
    assert_eq!(
        parse_line(dir_string),
        Some(CommandLine::Directory(String::from("jljrdvw")))
    );
    assert_eq!(
        parse_line(file1_string),
        Some(CommandLine::File {
            name: (String::from("zldbq")),
            size: (247592)
        })
    );
    assert_eq!(
        parse_line(file2_string),
        Some(CommandLine::File {
            name: (String::from("wdqqqv.pcr")),
            size: (96717)
        })
    );
}

pub fn solve() {
    // In a first step we will build the directory, then we will browse through it to find the

    // Setup of line parsing -> regular expressions

    let contents = fs::read_to_string(FILEPATH).expect("Should have been able to read the file");
    let line_array: Vec<&str> = contents.split("\n").collect();

    let mut main_fs = FileSystem {
        root: 0,
        current_dir: 0,
        dir_list: Vec::new(),
    };

    main_fs.init();

    for line in line_array {
        println!("Parsing line {}", line);
        let command = parse_line(&line);
        match command {
            Some(CommandLine::Directory(dir_name)) => {
                main_fs.mk_dir(dir_name.as_str());
            }
            Some(CommandLine::File { name, size }) => {
                main_fs.add_file(name.as_str(), size);
            }
            Some(CommandLine::ChangeDir(dir_name)) => {
                println!("Changing dir to {}", dir_name);
                main_fs.change_dir(dir_name.as_str());
            }
            Some(CommandLine::Ls) => {}
            None => panic!("Don't manage to parse the line {}", line),
        }
    }

    println!("The file system is: {:?}", main_fs);

    // Now (theoretically) we have built our file system
    // We can answer the question

    let mut all_sizes = main_fs.get_all_sizes();
    all_sizes.sort();
    println!("All sizes = {:?}", all_sizes);

    let root_size = main_fs.get_size(0);
    let disk_size: u64 = 70_000_000;
    let update_size: u64 = 30_000_000;
    let size_to_delete = root_size - (disk_size - update_size);
    println!("Root size = {}, to del = {}", root_size, size_to_delete);

    for subfolder_size in all_sizes {
        if subfolder_size > size_to_delete {
            println!("The solution of question 1 is {}", subfolder_size);
        }
    }
}
