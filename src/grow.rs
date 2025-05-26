use colored::Colorize;
use ignore::{DirEntry, WalkBuilder};
use std::ffi::OsStr;
use std::fs::canonicalize;
use std::{collections::HashMap, path::Path};

fn sort_key(dirent: &DirEntry) -> (String, bool, String) {
    // Create a tuple on which to sort the files in the tree
    // First sort on the parent directory
    // Second put files before directories
    // Third sort files or directories alphabetically

    let dirent = dirent.clone();
    let isdir = dirent.file_type().unwrap().is_dir();
    let filename = dirent.file_name().display().to_string();

    let parent = if isdir {
        dirent.path().to_string_lossy().to_string()
    } else {
        dirent
            .path()
            .parent()
            .unwrap()
            .to_string_lossy()
            .to_string()
    };

    (parent, !isdir, filename)
}

fn get_files(path: String, max_depth: usize) -> Vec<DirEntry> {
    // Get the relevant files and sort them in the correct order

    let mut files = WalkBuilder::new(path.clone())
        .hidden(false)
        .max_depth(Some(max_depth))
        .build()
        .filter_map(|x| x.ok())
        .filter(|x| !x.path().starts_with("./.git") & (x.file_name() != OsStr::new(".gitkeep")))
        .collect::<Vec<_>>();

    files.sort_unstable_by_key(sort_key);

    files
}

// Spacers
const EMPTY: &str = " \u{a0}\u{a0} ";
const ELBOW: &str = "└── ";
const TJOIN: &str = "├── ";
const OVERA: &str = "│\u{a0}\u{a0} ";

pub fn tre(path: String, max_depth: usize) -> String {
    // Build the tree to be printed out

    let files = get_files(path.clone(), max_depth);

    let mut parent_ends = HashMap::new();
    for (line, file) in files.iter().enumerate().rev() {
        let parent = file.path().parent().unwrap(); //.to_string_lossy().to_string();
        let _ = parent_ends.entry(parent).or_insert(line);
    }

    let mut output: Vec<String> = Vec::new();
    let mut parents: Vec<&Path> = Vec::new();

    for file in files.iter().rev() {
        let mut last = false;

        if file.path().is_dir() {
            parents.retain(|&p| p != file.path());
        }

        let parent_levels: Vec<_> = parents
            .iter()
            .filter_map(|&p| p.strip_prefix(Path::new(&path)).ok())
            .map(|p| p.components().count() + 1)
            .collect();

        let mut line = String::new();

        for level in 1..file.depth() {
            if parent_levels.contains(&level) {
                line.push_str(OVERA);
            } else {
                line.push_str(EMPTY);
            }
        }

        if !parents.contains(&file.path().parent().expect("can't find parent dir")) {
            parents.push(file.path().parent().expect("can't find parent dir"));
            last = true;
        }

        if file.path() == Path::new(&path) {
            if file.path() == Path::new(".") {
                let resolved_name = canonicalize(file.file_name()).expect("can't canonicalize '.'");
                line.push_str(&format!(
                    "{}",
                    resolved_name
                        .file_name()
                        .expect("can't get filename of '.'")
                        .to_string_lossy()
                        .blue()
                ));
            } else {
                line.push_str(&format!("{}", file.path().to_string_lossy().blue()));
            }
        } else {
            if last {
                line.push_str(ELBOW);
            } else {
                line.push_str(TJOIN);
            }
            let name = file.file_name().to_string_lossy();
            if file.path().is_dir() {
                line.push_str(&format!("{}", name.blue()));
            } else {
                line.push_str(&name);
            }
        }

        line.push('\n');
        output.push(line);
    }

    output.into_iter().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let expected: Vec<&str> = "\u{1b}[34mtests/case1\u{1b}[0m
├── .gitignore
├── file_0.txt
├── file_1.txt
├── file_2.txt
└── \u{1b}[34mdir_0_0\u{1b}[0m
    ├── file_0.txt
    ├── \u{1b}[34mdir_1_0\u{1b}[0m
    │   ├── \u{1b}[34mdir_2_0\u{1b}[0m
    │   │   ├── file_0.txt
    │   │   └── file_1.txt
    │   └── \u{1b}[34mdir_2_1\u{1b}[0m
    └── \u{1b}[34mdir_1_1\u{1b}[0m
        ├── file_0.txt
        ├── file_1.txt
        └── \u{1b}[34mdir_2_0\u{1b}[0m
            ├── file_0.txt
            ├── \u{1b}[34mdir_3_0\u{1b}[0m
            │   └── file_0.txt
            └── \u{1b}[34mdir_3_1\u{1b}[0m"
            .lines()
            .collect();

        let actual = tre("tests/case1".into(), 7);
        let actual: Vec<&str> = actual.lines().collect();
        actual
            .iter()
            .zip(expected.clone())
            .rev()
            .for_each(|(&act, exp)| assert_eq!(act, exp));
        assert_eq!(actual.len(), expected.len());
    }
}
