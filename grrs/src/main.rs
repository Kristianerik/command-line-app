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
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}
