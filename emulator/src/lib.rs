#[cfg(feature = "serialize")]
#[macro_use]
extern crate serde_derive;

pub mod gpu;
pub mod cpu;
pub mod interrupt_flags;
pub mod joypad;
pub mod memory_bus;
pub mod utils;
pub mod timer;
pub mod instruction;
pub mod registers;
pub mod flags_register;

pub use joypad::Joypad;
pub use cpu::CPU;
pub use flags_register::FlagsRegister;
pub use registers::Registers;
pub use memory_bus::MemoryBus;
pub use instruction::*;