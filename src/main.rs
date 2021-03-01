use std::{fs, io, path};

use anyhow::{Context, Result};
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt, Debug)]
struct Cli {
    /// The pattern to search for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: path::PathBuf,
}

#[derive(Debug)]
struct CustomError(String);

fn main() -> Result<()> {
    let args = Cli::from_args();

    let content = fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file {:?}", args.path))?;

    grrs::find_matches(&content, &args.pattern, &mut io::stdout());

    Ok(())
}
