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

    memory.load_program(&input_bytes, 0)?;

    loop {
        let opcode = cpu.fetch_byte(&memory)?;
        let instruction = decode(opcode, &mut cpu, &memory)?;
        let keep_running = cpu.execute(instruction, &mut memory)?;

        if !keep_running {
            break;
        }
    }

    println!("Program has finished. Registers state:\n{:#?}", cpu.registers);

    Ok(())
}
