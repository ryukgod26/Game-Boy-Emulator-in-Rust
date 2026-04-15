mod cpu;
mod instruction;
mod memory_bus;
pub use cpu::CPU;
pub use instruction::Instruction;
pub use memory_bus::MemoryBus;

const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

static is_halted: bool = false;

const VRAM_BEGIN: usize = 0x8000;
const VRAM_END: usize = 0x9FFF;
const VRAM_SIZE: usize = VRAM_BEGIN - VRAM_END + 1;

type Tile = [[TilePixelValue; 8]; 8];

struct FlagsRegister{
    zero: bool,
    subtract: bool,
    half_carry: bool,
    carry: bool,
}

struct Registers{
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    g: u8,
    h: u8,
}




struct GPU{
    vram: [u8; VRAM_SIZE],
    tile_set: [Tile; 384],
}



enum ArithmeticTarget{
    A,B,C,D,E,H,L,
}

enum JumpTest{
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always,
}

enum LoadByteTarget{
    A,B,C,D,E,H,L,HLI
}

enum LoadByteSource{
    A,B,C,D,E,H,L,D8,HLI
}

enum LoadType{
    Byte(LoadByteTarget,LoadByteSource)
}

enum IncDecTarget{
    BC,DE
}

enum StackTarget{
    AF,DE,BC,HL
}

enum JumpTarget{
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always
}

enum PrefixTarget{
    A,B,C,D,E,H,L
}

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

impl GPU{
fn write_vram(&self, index: usize,value: u8){
    self.vram[index] = value;

    if index >= 0x1800 {return}
    
    let normalized_index = index & 0xFFFE;

    let byte1 = self.vram[normalized_index];
    let byte2 = self.vram[normalized_index + 1];

    let tile_index = index / 16;
    let row_index = (index % 16) /2;

    for pixel_index in 0..8{
        let mask = 1 << (7 - pixel_index);
        let lsb = byte1 & mask;
        let msb = byte2 & mask;

        let value = match (lsb != 0, msb != 0){
            (true,true) => TilePixelValue::Three,
            (false,true) => TilePixelValue::Two,
            (true,false) => TilePixelValue::One,
            (false,false) => TilePixelValue::Zero,
        };
        self.tile_set[tile_index][row_index][pixel_index] = value;
    }
}

fn read_vram(&self,addr: usize) -> u8{
    self.vram[addr]
}
}

impl Registers{
    fn get_bc(&self) -> u16{
        (self.b as u16) << 8 | (self.c as u16)
    }

    fn set_bc(&mut self, value: u16){
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }
}


impl std::convert::From<FlagsRegister> for u8{
    fn from(flag: FlagsRegister) -> u8{
        (if flag.zero {1} else {0}) << ZERO_FLAG_BYTE_POSITION |
        (if flag.subtract {1} else {0}) << SUBTRACT_FLAG_BYTE_POSITION |
        (if flag.half_carry {1} else {0}) << HALF_CARRY_FLAG_BYTE_POSITION |
        (if flag.carry {1} else {0}) << CARRY_FLAG_BYTE_POSITION

    }
}

impl std::convert::From<u8> for FlagsRegister{
    fn from(byte: u8) -> Self{
        let zero = ((byte >> ZERO_FLAG_BYTE_POSITION) & 0b1) != 0;
        let subtract = ((byte >> SUBTRACT_FLAG_BYTE_POSITION) & 0b1) != 0;
        let half_carry = ((byte >> HALF_CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;
        let carry = ((byte >> CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;

        FlagsRegister{
            zero,
            subtract,
            half_carry,
            carry
        }

    }
}


fn main() {
    println!("Hello, world!");
}
