use std::fs::File;
use std::io::{self, BufRead};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> io::Result<()> {
    let args = Cli::parse();
    let file = File::open(&args.path).expect("could not open file");
    
    match file {
        Ok(file) => {
            let reader = io::BufReader::new(file);

            for line in reader.lines() {
                match line {
                    Ok(content) => {
                        if content.contains(&args.pattern) {
                            println!("{}", content);
                        }
                    }
                    Err(error) => println!("Error reading line: {}", error),
                }
            }
        }
        Err(error) => {
            println!("Oh noes: {}", error);
        }
    }
   
}
