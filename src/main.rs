use clap::Parser;
use colored::Colorize;
use ignore::{DirEntry, WalkBuilder};
use std::fs::canonicalize;
use std::{collections::HashMap, path::Path};

fn sort_key(dirent: &DirEntry) -> (String, bool, String) {
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

const EMPTY: &str = " \u{a0}\u{a0} ";
const ELBOW: &str = "└── ";
const TJOIN: &str = "├── ";
const OVERA: &str = "│\u{a0}\u{a0} ";

fn tre(path: String, max_depth: usize) -> String {
    let mut files = WalkBuilder::new(path.clone())
        .hidden(false)
        .max_depth(Some(max_depth))
        .build()
        .filter_map(|x| x.ok())
        .filter(|x| !x.path().starts_with("./.git"))
        .collect::<Vec<_>>();

    files.sort_unstable_by_key(sort_key);

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

/// A basic tree cli tool
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// File path for tree to run from, defaults to .
    #[arg(default_value = ".")]
    path: String,
    #[arg(short = 'd', long = "depth", default_value = "5")]
    max_depth: usize,
}

fn main() {
    let args = Args::parse();
    println!("{}", tre(args.path, args.max_depth));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let expected: Vec<&str> = "tests/case1
├── .gitignore
├── file_0.txt
├── file_1.txt
├── file_2.txt
└── dir_0_0
    ├── file_0.txt
    ├── dir_1_0
    │   ├── dir_2_0
    │   │   ├── file_0.txt
    │   │   └── file_1.txt
    │   └── dir_2_1
    └── dir_1_1
        ├── file_0.txt
        ├── file_1.txt
        └── dir_2_0
            ├── file_0.txt
            ├── dir_3_0
            │   └── file_0.txt
            └── dir_3_1"
            .lines()
            .collect();

        let actual = tre("tests/case1".into(), 5);
        let actual: Vec<&str> = actual.lines().collect();
        actual
            .iter()
            .zip(expected.clone())
            .rev()
            .for_each(|(&act, exp)| assert_eq!(act, exp));
        assert_eq!(actual.len(), expected.len());
    }
}
