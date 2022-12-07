use crate::harness::input::RawInput;
use std::collections::HashMap;

pub fn solve_part1(input: RawInput) -> u32 {
    infer_filesystem(input)
        .get_directory_sizes()
        .into_iter()
        .filter(|&size| size <= 100000)
        .sum()
}

pub fn solve_part2(input: RawInput) -> u32 {
    let sizes = infer_filesystem(input).get_directory_sizes();
    let root_size = sizes[0];
    let space_to_free = root_size - 40000000;
    sizes
        .into_iter()
        .filter(|&size| size >= space_to_free)
        .min()
        .unwrap()
}

fn infer_filesystem(input: RawInput) -> FileSystem {
    let mut fs = FileSystem::new();
    let lines = input.per_line(|line| line.split_whitespace::<String>());
    for line in lines {
        match line[0].as_str() {
            "$" => {
                if &line[1] == "cd" {
                    fs.cd(&line[2]);
                }
            }
            "dir" => {
                fs.mkdir(line[1].clone());
            }
            _ => fs.add_file(line[0].parse().unwrap()),
        }
    }
    fs
}

#[derive(Debug)]
struct FileSystem {
    directories: Vec<Directory>,
    current_path: Vec<usize>,
}

#[derive(Debug, Default)]
struct Directory {
    total_file_size: u32,
    subdirectory_indices: HashMap<String, usize>,
}

impl FileSystem {
    fn new() -> Self {
        Self {
            directories: vec![Directory::default()],
            current_path: vec![0],
        }
    }

    fn cd(&mut self, s: &str) {
        match s {
            "/" => self.current_path = vec![0],
            ".." => {
                self.current_path.pop();
            }
            _ => self.current_path.push(self.cwd().subdirectory_indices[s]),
        }
    }

    fn cwd(&self) -> &Directory {
        &self.directories[*self.current_path.last().unwrap()]
    }

    fn cwd_mut(&mut self) -> &mut Directory {
        &mut self.directories[*self.current_path.last().unwrap()]
    }

    fn mkdir(&mut self, name: String) {
        let index = self.directories.len();
        self.cwd_mut().subdirectory_indices.insert(name, index);
        self.directories.push(Directory::default());
    }

    fn add_file(&mut self, size: u32) {
        self.cwd_mut().total_file_size += size;
    }

    fn get_directory_sizes(&self) -> Vec<u32> {
        let len = self.directories.len();
        let mut result = vec![0; len];
        for i in (0..len).rev() {
            let dir = &self.directories[i];
            let subdirectory_size: u32 = dir
                .subdirectory_indices
                .values()
                .map(|&index| result[index])
                .sum();
            result[i] = dir.total_file_size + subdirectory_size
        }
        result
    }
}
