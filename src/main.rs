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
    let input_bytes = fs::read_to_string(&args.file_path)?.into_bytes();

    println!("Bytes: {input_bytes:?}");

    Ok(())
}
