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
fn find_matches(content: &str, pattern: &str, mut writer: impl Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line).expect("failed to write to output");
        }
    }
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
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    for line in reader.lines() {
        match line {
            Ok(content) => {
                find_matches(&content, &args.pattern, &mut handle);
            }
            Err(error) => {
                writeln!(handle, "Error reading line: {}", error)?;
            }
        }
        pb.inc(1); // Update progress bar
    }

    pb.finish_with_message("Search complete!");
    Ok(())
}
   
