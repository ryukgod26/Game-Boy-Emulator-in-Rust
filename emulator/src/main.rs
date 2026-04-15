mod cpu;
mod instruction;
mod memory_bus;
mod registers;
mod flags_register;
mod gpu;
pub use cpu::CPU;
pub use instruction::*;
pub use memory_bus::MemoryBus;
pub use registers::Registers;
pub use flags_register::FlagsRegister;
pub use gpu::GPU;


static is_halted: bool = false;

const VRAM_BEGIN: usize = 0x8000;
const VRAM_END: usize = 0x9FFF;
const VRAM_SIZE: usize = VRAM_BEGIN - VRAM_END + 1;

type Tile = [[TilePixelValue; 8]; 8];




#[derive(Copy,Clone)]
enum TilePixelValue{
    Zero,
    One,
    Two,
    Three,
}

fn empty_tile() -> Tile{
    [[TilePixelValue; 8]; 8]
}


fn main() {
    println!("Hello, world!");
}
