use std::fs::File;
use std::io::{self, BufRead, Write};
use clap::Parser;
use indicatif::ProgressBar;

#[cfg(test)]
mod tests;

pub fn answer() -> i32 {
    42
}

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

// Function to make `find_matches` reusable and testable
fn find_matches(content: &str, pattern: &str) -> Vec<String> {
    let mut results = Vec::new();
    for line in content.lines() {
        if line.contains(pattern) {
            results.push(line.to_string());
        }
    }
    results
}

fn main() -> io::Result<()> {
    let args = Cli::parse();
    
    let file = File::open(&args.path)?;
    let reader = io::BufReader::new(&file);
    
    // Count total lines for progress bar
    let total_lines = reader.lines().count() as u64;
    let pb = ProgressBar::new(total_lines);

    // Re-open file because `.count()` consumed the iterator
    let file = File::open(&args.path)?;
    let reader = io::BufReader::new(file);

    let mut matches = Vec::new();

    // Iterate through the lines, collecting matches
    for line in reader.lines() {
        match line {
            Ok(content) => {
                let found_matches = find_matches(&content, &args.pattern);
                matches.extend(found_matches);
            }
            Err(error) => {
                eprintln!("Error reading line: {}", error);
            }
        }
        pb.inc(1); // Update progress bar
    }

    pb.finish_with_message("Processing complete."); // Finish with a message to show it's done
    io::stdout().flush()?; // Make sure to flush the output

    // Print matches after the progress bar is finished
    for m in matches {
        println!("{}", m);
    }

    Ok(())
}
