use std::{io::{BufRead, BufReader}, fs::File};

use clap::Parser;
use anyhow::{Context, Result, Ok};

// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
#[derive(Debug)]
struct Cli {
    // The pattern to look for
    pattern: String,
    // The path to the file to read. Use clap to parse.
    // PathBuf is like a String but for file system paths that work cross-platform.
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let pattern = &args.pattern;
    let path = args.path.to_str().unwrap();
    let file = File::open(path)
        .with_context(|| format!("could not read file `{}`", path))?;
    let reader = BufReader::new(file);
    let content = reader.lines();

    grrs::find_matches(content, pattern, &mut std::io::stdout());

    Ok(())
}


