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
pub use cpu::CPU;
pub use instruction::*;
pub use memory_bus::*;
pub use registers::Registers;
pub use flags_register::FlagsRegister;
pub use gpu::GPU;

const ENLARGEMENT_FACTOR: usize = 1;
const WINDOW_DIMENSIONS: [usize; 2] = [(160 * ENLARGEMENT_FACTOR), (144 * ENLARGEMENT_FACTOR)];



fn main() {
    println!("Hello, world!");
}
