mod horspool;

use horspool::{Horsepool, Match};
use std::env;
use std::fs;

pub struct Config {
    pattern: String,
    file_path: String,
    flags: Flags,
}

struct Flags {
    ignore_case: bool,
    line_number: bool,
    word_count: bool,
}

struct Content {
    line: String,
    line_number: u32,
}

impl Flags {
    fn new() -> Self {
        Self {
            ignore_case: false,
            line_number: false,
            word_count: false,
        }
    }
}

impl Config {
    pub fn build(mut args: env::Args) -> Result<Config, &'static str> {
        args.next(); // to skip the first argument

        let pattern = match args.next() {
            Some(pattern) => pattern,
            None => return Err("USAGE minigrep PATTERN FILE -FLAGS"),
        };

        let file_path = match args.next() {
            Some(file_path) => file_path,
            None => return Err("FILE PATH NOT GIVEN"),
        };

        // iteration over the flags
        let mut flags = Flags::new();

        for flag in args {
            match flag.as_str() {
                "-i" => flags.ignore_case = true,
                "-n" => flags.line_number = true,
                "-c" => flags.word_count = true,
                _ => return Err("unknown flag specified"),
            }
        }

        Ok(Config {
            pattern,
            file_path,
            flags,
        })
    }
}

// Function to read the FIle

pub fn run(config: Config) -> Result<(), &'static str> {
    let file = match fs::read_to_string(&config.file_path) {
        Ok(file) => file,
        Err(_) => return Err("Unable to open file"),
    };

    let result = search(&config.pattern, &file, &config.flags);
    if config.flags.word_count {
        println!("Total matches found: {}", result.len());
        return Ok(());
    }

    for content in result {
        if config.flags.line_number {
            println!("{} {}", content.line_number, content.line.trim());
        } else {
            println!("{}", content.line.trim())
        }
    }
    Ok(())
}

fn search(query: &str, content: &str, flags: &Flags) -> Vec<Content> {
    let mut line_number: u32 = 0;
    let horspool = Horsepool::build(query, flags.ignore_case);
    let mut result: Vec<Content> = Vec::new();

    for line in content.lines() {
        let matches = horspool.search(line);
        if !matches.is_empty() {
            let line = highlight_line(line, matches);
            result.push(Content {
                line: line,
                line_number,
            })
        }
        line_number += 1;
    }
    result
}

fn highlight_line(line: &str, matches: Vec<Match>) -> String {
    let mut result = String::new();
    let mut last_end = 0;

    for match_info in matches {
        // Add text before the match
        result.push_str(&line[last_end..match_info.start]);

        // Add highlighted match (using ANSI color codes)
        result.push_str("\x1b[1;31m"); // Bold red
        result.push_str(&line[match_info.start..match_info.end]);
        result.push_str("\x1b[0m"); // Reset color

        last_end = match_info.end;
    }

    // Add remaining text after the last match
    result.push_str(&line[last_end..]);

    result
}
