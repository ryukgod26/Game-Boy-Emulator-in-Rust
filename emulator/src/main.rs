const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

static is_halted: bool = false;

const VRAM_BEGIN: usize = 0x8000;
const VRAM_END: usize = 0x9FFF;
const VRAM_SIZE: usize = VRAM_BEGIN - VRAM_END + 1;

type Tile = [[TilePixelValue; 8]; 8]

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

struct CPU{
    registers: Registers,
    pc: u16,
    sp: u16,
    bus: MemoryBus,
}

struct MemoryBus{
    memory: [u8; 0xFFFF]
}

struct GPU{
    vram: [u8; VRAM_SIZE],
    tile_set: [Tile, 384],
}

enum Instruction{
    Add(ArithmeticTarget),Jp(JumpTest),LD(LoadType),PUSH(StackTarget),POP(StackTarget),CALL(JumpTarget),RET(JumpTarget),NOP,Halt
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
    A,B,C,D,E,H,L,DB,HLI
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

impl CPU {
    fn execute(&mut self,instruction: Instruction) -> u16{
        if is_halted{
            return self.pc
        }
        match instruction{
            Instruction::Jp(target) => {
                let jump_condition = match target{
                    JumpTest::NotZero => !self.registers.f.zero,
                    JumpTest::Zero => self.registers.f.zero,
                    JumpTest::NotCarry => !self.registers.f.carry,
                    JumpTest::Carry => self.registers.f.carry,
                    JumpTest::Always => true,
                };
                self.jump(jump_condition)
            },

            Instruction::Add(target) => {
                match target{
                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                _ => {}
                }
            },

            Instruction::LD(Loadtype) => {
                match Loadtype{
                LoadType::Byte(target,source) => {
                    let source_val = match source {
                        LoadByteSource::A => self.registers.a,
                        LoadByteSource::D8 => self.read_next_byte(),
                        LoadByteSource::HLI => self.bus.read_byte(self.registers.get_hl()),
                        _ => {panic!("Other Sources Not Implemented!!!")}
                    };
                
                    match target{
                        LoadByteTarget::A => self.registers.a = source_val,
                        LoadByteTarget::HLI => self.bus.write_byte(self.registers.get_hl(),source_val),
                        _ => {panic!("Other Targets Not Implemented")}
                    };
                    match source{
                        LoadByteSource::D8 => self.pc.wrapping_add(2),
                        _ => self.pc.wrapping_add(1),
                    }
                }
                
                _ => {panic!("Other Load Types not Implemented Yet")}
                }
            },

            Instruction::PUSH(target) => {
                let value = match target{
                    StackTarget::BC => self.registers.get_bc(),
                    _ => {panic!("Other Targets not Supported Yet!!!")}
                };
                self.push(value);
                self.pc.wrapping_add(1);
            },

            Instruction::POP(target) => {
                let result = self.pop();
                match target{
                    StackTarget::BC => {
                        self.registers.set_bc(result)
                    }
                    _ => {panic!("Yet to Add Support for more Instruction in StackTarget")},
                };
                self.pc.wrapping_add(1)
            },

            Instruction::CALL(function) => {
                let jump_condition = match function{
                    JumpTarget::NotZero => !self.registers.f.zero,
                    _=> {panic!("Yet to Add more Conditions")},
                };
                self.call(jump_condition)
            }

            Instruction::RET(function) => {
                let jump_condition = match function {
                    JunpTarget::NotZero => !self.registers.f.zero,
                    _=>{panic!("Yet to add more Conditions")}
                };
                self.return_(jump_condition)
            }

            Inatruction::NOP => {
                self.pc = self.pc + 1;
            }

            Instruction::Halt => {
                is_halted = true;
            }

            _ => {panic!("Support for more Instructions not Added Yet.")}
        }
    }

    fn call(&mut self,should_jump: bool) -> u16{
        let next_pc = self.pc.wrapping_add(3);
        if should_jump{
            self.push(next_pc);
            self.read_next_word()
        }else{
            next_pc
        }
    }

    fn return_(&mut self,should_jump: bool) -> u16{
        if should_jump{
            self.pop()
        }else{
            self.pc.wrapping_add(1)
        }
    }

    fn push(&mut self,value: u16){
        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, ((value &0xFF00) >>8) as u8);

        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, (value & 0xFF) as u8);
        
    }

    fn pop(&mut self,val: u16) -> u16{
        let lsb = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);

        let msb = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);
        
        (msb << 8) | lsb
    }

    fn add(&mut self,value: u8) -> u8{
        let(new_value, is_overflow) = self.registers.a.overflowing_add(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registersf.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        self.registers.f.carry = is_overflow;
        new_value
    }

    fn step(&mut self){
        let mut instruction_byte = self.bus.read_byte(self.pc);
        let prefixed = instruction_byte == 0xCB;
        if prefixed{
            instruction_byte = self.bus.read_byte(self.pc + 1);
        }
        let next_pc = if let Some(instruction) = Instruction::from_byte(instruction_byte,prefixed){
            self.execute(instruction)
        } else{
            let description = format!("0x{}{:x}",if prefixed { "cb" } else {""},instruction_byte);
            panic!("Cannot find Instruction found fot: 0x{:x}",instruction_byte);
        };
        self.pc = next_pc;
    }

    fn jump(&self,condition:bool) ->u16 {
        if condition {
            let least_significant_byte = self.bus.read_byte(self.pc +1) as u16;
            let most_significant_byte = self.bus.read_byte(self.pc + 2) as u16;
            (most_significant_byte<<8) | least_significant_byte
        }
        else{
            self.pc.wrapping_add(3)
        }
    }
}

impl MemoryBus{
    fn read_byte(&self,address: u16) ->u8{
        self.memory[address as usize]
    }
}

impl Instruction{
    fn from_byte(byte: u8,prefixed: bool) -> Option<Instruction>{
        if prefixed{
            from_prefixed_byte(byte)
        }
        else{    
            from_not_prefixed_byte(byte)
        }
    }

    fn from_prefixed_byte(byte: u8)->Option<Instruction>{
        match byte{
            0x00 => Some(Instruction::RLC(PrefixTarget::B)),
            _ => None,
        }
    }

    fn from_not_prefixed_byte(byte:u8) -> Option<Instruction>{
        match byte{
            0x06 => Some(Instructon::LD(LoadType::Byte(Target::B),LoadSource::NextByte)),
            0x0E => Some(Instruction::LD(LoadType::Byte(Target::C),LoadSource::NextByte)),
            0x02 => Some(Instruction::INC(IncDecTarget::BC)),
            0x13 => Some(Instruction::INC(IncDecTarget::DE)),
            _ => None,
        }
    }
}

fn main() {
    println!("Hello, world!");
}
