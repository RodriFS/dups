use std::env;
use std::process;

mod lib;
use lib::{collect_files, find_dups};

struct Config {
    directory: String,
    recursive: bool,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let mut recursive = false;
        for arg in args {
            if arg == "-r" || arg == "--recursive" {
                recursive = true;
            }
        }

        let directory = args[args.len() -1].clone();

        Ok(Config { directory, recursive })
    }
}



fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let mut files = collect_files(&config.directory, config.recursive)
        .expect("Cant read directory");
    let dups = find_dups(&mut files)
        .expect("Cant find dups");

    for dup in dups {
        println!("Duplicate: {} {}", dup.path, dup.length);
        for d in dup.duplications {
            println!("Duplicate: {} {}", d, dup.length)
        }
        println!("")
    }
    
}