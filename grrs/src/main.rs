use std::fs::File;
use std::io::{self, BufRead};
use clap::Parser;
use std::io::{self, Write};
use idicatif::ProgressBar;

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
            let stdout = io::stdout();
            let mut handle = stdout.lock();

            // Count total lines for progress bar
            let total_lines = reader.lines().count() as u64;
            let pb = ProgressBar::new(total_lines); // Initialize progress bar

            // Re-open file because `count()` consumed the iterator
            let file = File::open(&args.path).expect("Failed to reopen file");
            let reader = io::BufReader::new(file);

            for line in reader.lines() {
                match line {
                    Ok(content) => {
                        if content.contains(&args.pattern) {
                            writeln!(handle, "{}", content).expect("Failed to write to stdout");
                        }
                    }
                    Err(error) => {
                        writeln!(handle, "Error reading line: {}", error).expect("Failed to write error to stdout");
                    }
                }
                pb.inc(1); // Update progress bar
            }

            pb.finish_with_message("Search complete!");
        }
        Err(error) => {
            eprintln!("Oh noes: {}", error);
        }
    }
   
}
