use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;
use crate::input_reader::read_input;
use crate::p7::Output::{ChangeTo, ChangeUp, Directory, File, List};

pub fn solve_p7() {
    let input = read_input(7);

    let mut state_machine = StateMachine::new();
    state_machine.process_outputs(input.lines().flat_map(Output::from_str));
    let sum_dirs = state_machine.sum_of_dirs_with_at_most_100k();

    println!("Solution 1: {sum_dirs}");

    let size_deletable = state_machine.size_of_deletable_directory();

    println!("Solution 2: {size_deletable}");
}

struct StateMachine {
    directory_stack: Vec<String>,
    directory_contents: HashMap<String, DirectoryContent>,
}

impl StateMachine {
    fn new() -> Self {
        StateMachine {
            directory_stack: Vec::new(),
            directory_contents: HashMap::new(),
        }
    }

    fn process_outputs(&mut self, outputs: impl IntoIterator<Item=Output>) {
        outputs.into_iter().for_each(|o| self.process(o))
    }

    fn process(&mut self, output: Output) {
        match output {
            ChangeTo(dir) => self.directory_stack.push(dir),
            ChangeUp => { self.directory_stack.pop(); }
            Directory(dir) => self.add_directory_to_current(dir),
            File(len) => self.add_file_size_to_current(len),
            List => {}
        }
    }

    fn add_directory_to_current(&mut self, dir: String) {
        let path = self.create_current_path() + dir.as_str();
        let content = self.get_current_dir_content();
        content.add_sub_directory(path)
    }

    fn add_file_size_to_current(&mut self, len: usize) {
        let content = self.get_current_dir_content();
        content.add_file_size(len)
    }

    fn get_current_dir_content(&mut self) -> &mut DirectoryContent {
        let current = self.create_current_path();

        match self.directory_contents.entry(current.clone()) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => v.insert(DirectoryContent::new())
        }
    }

    fn create_current_path(&self) -> String {
        self.directory_stack.iter().map(|s| s.as_str()).collect()
    }

    fn sum_of_dirs_with_at_most_100k(&self) -> usize {
        self.directory_contents
            .keys()
            .map(|dir| self.size_of_dir(dir))
            .filter(|size| *size <= 100000)
            .sum()
    }

    fn size_of_deletable_directory(&self) -> usize {
        let free = 70000000 - self.size_of_dir("/");
        let required_free_space = 30000000 - free;

        self.directory_contents
            .keys()
            .map(|dir| self.size_of_dir(dir))
            .filter(|size| *size >= required_free_space)
            .min()
            .unwrap_or(0)
    }

    fn size_of_dir(&self, dir: &str) -> usize {
        let contents = self.directory_contents.get(dir).expect("should be set");
        contents.files_size + contents.sub_directories
            .iter()
            .map(|dir| self.size_of_dir(dir))
            .sum::<usize>()
    }
}

#[derive(Debug)]
struct DirectoryContent {
    files_size: usize,
    sub_directories: HashSet<String>,
}

impl DirectoryContent {
    fn new() -> Self {
        DirectoryContent {
            files_size: 0,
            sub_directories: HashSet::new(),
        }
    }

    fn add_sub_directory(&mut self, dir: String) {
        self.sub_directories.insert(dir);
    }

    fn add_file_size(&mut self, len: usize) {
        self.files_size += len
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Output {
    ChangeTo(String),
    ChangeUp,
    List,
    Directory(String),
    File(usize),
}

impl Output {
    fn from_str(s: &str) -> Option<Output> {
        let parts = s.split(" ").collect::<Vec<_>>();

        match parts[0] {
            "$" => match parts[1] {
                "cd" => match parts[2] {
                    ".." => Some(ChangeUp),
                    s => Some(ChangeTo(s.to_string()))
                },
                "ls" => Some(List),
                _ => None
            },
            "dir" => Some(Directory(parts[1].to_string())),
            num if num.parse::<usize>().is_ok() => Some(File(num.parse::<usize>().unwrap())),
            _ => None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::p7::{Output, StateMachine};
    use crate::p7::Output::*;

    #[test]
    fn output_from_str_works() {
        let input_expected = [
            ("$ cd jmdf", Some(ChangeTo("jmdf".to_string()))),
            ("$ cd ..", Some(ChangeUp)),
            ("$ ls", None),
            ("dir jmdf", Some(Directory("jmdf".to_string()))),
            ("177917 pvlvsfjw.qvw", Some(File(177917)))
        ];

        for (input, expected) in input_expected {
            assert_eq!(Output::from_str(input), expected)
        }
    }

    #[test]
    fn examples_work() {
        let example = "
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

        let mut state_machine = StateMachine::new();
        state_machine.process_outputs(example.lines().flat_map(Output::from_str));
        assert_eq!(state_machine.sum_of_dirs_with_at_most_100k(), 95437);
        assert_eq!(state_machine.size_of_deletable_directory(), 24933642)
    }
}