#![allow(unused)]

use clap::Parser;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    let file = File::open(&args.path).expect("Could not read file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(str) => {
                if str.contains(&args.pattern) {
                    println!("{}", str);
                }
            },
            Err(e) => println!("Error reading file: {}", e), 
        }
    }
}
