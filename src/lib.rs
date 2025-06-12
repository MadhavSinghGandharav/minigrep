mod horspool;

use std::env;
use std::fs;
use horspool::Horsepool;

pub struct Config{
    pattern : String,
    file_path : String,
    flags : Flags
}

struct Flags{
    ignore_case : bool,
    line_number : bool,
    word_count : bool
}

struct Content<'a>{
    line : &'a str,
    line_number : u32
}

impl Flags{
    fn new() -> Self{
        Self { ignore_case: false, line_number: false, word_count: false }
    }
}

impl Config {
    pub fn build(mut args: env::Args) -> Result<Config,&'static str>{
        args.next() ; // to skip the first argument
        
        let pattern = match args.next(){
            Some(pattern) => pattern,
            None => return Err("USAGE minigrep PATTERN FILE -FLAGS")
        };


        let file_path = match args.next(){
            Some(file_path) => file_path,
            None => return Err("FILE PATH NOT GIVEN")
        };

        // iteration over the flags
        let mut flags = Flags::new();

        for flag in args{
            match flag.as_str(){
                "-i" => flags.ignore_case = true,
                "-n" => flags.line_number = true,
                "-c" => flags.word_count = true,
                _ => return Err("unknown flag specified")
            }
        }

        Ok(Config{
            pattern,
            file_path,
            flags
        })

    }
}

// Function to read the file

pub fn run(config: Config) -> Result<(), &'static str> {
    let file = match fs::read_to_string(&config.file_path) {
        Ok(file) => file,
        Err(_) => return Err("Unable to open file"),
    };

    let result = search(&config.pattern, &file, &config.flags);
    if config.flags.word_count {
        println!("Total matches found: {}",result.len());
        return Ok(());
    }
    for line in &result{
        if config.flags.line_number {
            println!("{} {}", line.line_number, line.line.trim());
        } else {
            println!("{}", line.line.trim())
        }
    }
    Ok(())
}

fn search<'b>(query: &str, content: &'b str,flags: &Flags) -> Vec<Content<'b>> {

    let mut line_number: u32 = 0;
    let horspool = Horsepool::build(query, flags.ignore_case);
    let mut result :Vec<Content> = Vec::new(); 

    for line in content.lines(){
        if horspool.search(&line){
            result.push(Content{
                line,
                line_number
            })
        }

        line_number += 1;
    }

    result
   }
