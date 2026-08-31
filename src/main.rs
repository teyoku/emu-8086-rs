pub mod instructions;
pub mod cpu;
pub mod registers;
pub mod memory;
use std::{error::Error, fs};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file_path: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let input_bytes = fs::read(&args.file_path)?;

    println!("Bytes: {input_bytes:?}");

    Ok(())
}
