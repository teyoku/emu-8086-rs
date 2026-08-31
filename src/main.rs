pub mod cpu;
pub mod errors;
pub mod instructions;
pub mod memory;
pub mod registers;

use std::{error::Error, fs};

use clap::Parser;

use crate::{cpu::Cpu, instructions::decode, memory::Memory};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file_path: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let input_bytes = fs::read(&args.file_path)?;

    let mut memory = Memory::new();
    let mut cpu = Cpu::default();


    // load program into memory
    for (i, byte) in input_bytes.iter().enumerate() {
        memory.write_byte(0x0000 + i, *byte)?;
    }

    loop {
        let opcode = cpu.fetch_byte(&memory)?;
        let instruction = decode(opcode, &mut cpu, &memory)?;
        let should_stop = cpu.execute(instruction, &mut memory)?;

        if should_stop {
            break;
        }
    }

    Ok(())
}
