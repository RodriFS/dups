use std::env;
use std::process;

mod lib;
use lib::{collect_files, find_dups, print_help};

struct Config {
    directory: String,
    recursive: bool,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            println!("Error: not enough arguments\n");
            print_help();
            std::process::exit(0);
        }

        let mut recursive = false;
        for arg in args {
            if arg == "-r" || arg == "--recursive" {
                recursive = true;
            }
            if arg == "-h" || arg == "--help" {
                print_help();
                std::process::exit(0);
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

    let mut files = collect_files(&config.directory, config.recursive).unwrap_or_else(|err| {
        eprintln!("Problem parsing directory: {}", err);
        process::exit(1);
    });

    let dups = find_dups(&mut files).unwrap_or_else(|err| {
        eprintln!("Problem parsing files: {}", err);
        process::exit(1);
    });

    for dup in dups {
        println!("Duplicate: {} {}", dup.path, dup.length);
        for d in dup.duplications {
            println!("Duplicate: {} {}", d, dup.length)
        }
        println!("")
    }
    
}