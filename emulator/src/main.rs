mod cpu;
mod instruction;
mod memory_bus;
mod registers;
mod flags_register;
mod gpu;
mod joypad;
mod timer;
mod utils;
mod interrupt_flags;
use std::io::Read;

pub use cpu::CPU;
pub use instruction::*;
pub use memory_bus::*;
pub use registers::Registers;
pub use flags_register::FlagsRegister;
pub use gpu::GPU;

const ENLARGEMENT_FACTOR: usize = 1;
const WINDOW_DIMENSIONS: [usize; 2] = [(160 * ENLARGEMENT_FACTOR), (144 * ENLARGEMENT_FACTOR)];
const ONE_SECOND_IN_MICROS: usize = 1000000000;
const ONE_SECOND_IN_CYCLES: usize = 4190000;
const ONE_FRAME_IN_CYCLES: usize = 70224;
const NUMBER_OF_PIXELS: usize = 23040;


fn main() {
    println!("Hello, world!");
}

fn buffer_from_file(path: &str) -> Vec<u8>{
    let mut file = std::fs::File::open(path).expect("File not Found");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Could not read file");
    buffer
}