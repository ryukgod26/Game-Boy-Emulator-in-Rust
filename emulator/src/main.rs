mod cpu;
mod instruction;
mod memory_bus;
mod registers;
mod flags_register;
mod gpu;
pub use cpu::CPU;
pub use instruction::*;
pub use memory_bus::*;
pub use registers::Registers;
pub use flags_register::FlagsRegister;
pub use gpu::GPU;


static is_halted: bool = false;



fn main() {
    println!("Hello, world!");
}
