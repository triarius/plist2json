mod json;

use clap::Parser;
use color_eyre::eyre::{Result, WrapErr};
use std::{fs::File, io::BufReader};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Path to plist file.
    #[arg(index = 1)]
    filename: String,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::parse();

    let file = File::open(&args.filename)
        .wrap_err_with(|| format!("failed to open file: {}", args.filename))?;
    let buffered_reader = BufReader::new(file);
    let value = plist::from_reader(buffered_reader)?;
    let json = json::convert(&value)?;
    println!("{json}");

    Ok(())
}
