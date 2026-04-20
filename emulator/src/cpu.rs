use super::{Instruction,JumpTest,ArithmeticTarget,Registers,MemoryBus,StackTarget,LoadByteTarget,LoadType,LoadByteSource,JumpTarget,IncDecTarget,LoadWordTarget,Indirect};

pub struct CPU{
    pub registers: Registers,
    pc: u16,
    sp: u16,
    pub bus: MemoryBus,
    is_halted: bool,
}

macro_rules! manipulate_8bit_register{

    ($self: ident : $getter: ident => $func: ident) => {
        {
            let value = $self.registers.$getter;
            $self.$func(value)
        }
    };

    ($self: ident : ($register: ident @ bit_position ) => $func: ident => $setter: ident) => {
        {
            let result = manipulate_8bit_register!($self: ($register @ bit_position) => $func);
            $self.registers.$setter = result;
        }
    };

    ($self: ident : $reg: ident => $func: ident => $_flag_reg: ident) => {
        {
            let val = $self.registers.$reg;
            let result = $func(val);
            $self.registers.$reg = result;
            $self.pc.wrapping_add(1)
        }
    };

    ($self: ident : ($register: ident @ $bit_position: ident) => $func: ident) =>{
        {
            let value = $self.registers.$register;
            $self.$func(value,$bit_position)
        }
    }

}

macro_rules! manipulate_16bit_register{
        ($self: ident : $get_func: ident => $func: ident => $set_func: ident) => {
        {
            let val = $self.registers.$get_func();
            let result = $self.$func(val);
            $self.registers.$set_func(result);
        }
    };
}

macro_rules! arithmetic_instruction{
    ($register: ident, $self: ident.$func: ident) =>{
        {
            match $register{
                ArithmeticTarget::A => manipulate_8bit_register!($self : a => $work),
                ArithmeticTarget::B => manipulate_8bit_register!($self : b => $func),
                ArithmeticTarget::C => manipulate_8bit_register!($self : c => $func),
                ArithmeticTarget::D => manipulate_8bit_register!($self : d => $func),
                ArithmeticTarget::E => manipulate_8bit_register!($self : e => $func),
                ArithmeticTarget::H => manipulate_8bit_register!($self : h => $func),
                ArithmeticTarget::L => manipulate_8bit_register!($self : l => $func),
                ArithmeticTarget::D8 => {
                    let value = $self.read_next_byte();
                    $self.$func(value);
                }
                ArithmeticTarget::HLI => {
                    let value = $self.bus.read_byte($self.registers.get_hl());
                    $self.$func(value);
                }
            };
            match $register{
                ArithmeticTarget::D8 => ($self.pc.wrapping_add(2),8),
                ArithmeticTarget::HLI => ($self.pc.wrapping_add(1),8),
                _ => ($self.pc.wrapping_add(1),4)

            }
        }
    };

    ($register: ident, $self: ident.$func: ident => a) => {
       match $register{
           ArithmeticTarget::A => manipulate_8bit_register!($self : a => $func => a),
           ArithmeticTarget::B => manipulate_8bit_register!($self : b => $func => a),
           ArithmeticTarget::C => manipulate_8bit_register!($self : c => $func => a),
           ArithmeticTarget::D => manipulate_8bit_register!($self : d => $func => a),
           ArithmeticTarget::E => manipulate_8bit_register!($self : e => $func => a),
           ArithmeticTarget::H => manipulate_8bit_register!($self : h => $func => a),
           ArithmeticTarget::L => manipulate_8bit_register!($self : l => $func => a),
           ArithmeticTarget::D8 => {
               let val = $self.read_next_byte();
               let result = $self.$func(val);
               $self.registers.a = result;
           }
           ArithmeticTarget::HLI => {
               let val = $self.bus.read_byte($self.registers.get_hl());
               let result = $self.$func(val);
               $self.registers.a = result;
           }
       };
       match $register{
           ArithmeticTarget::D8 => ($self.pc.wrapping_add(2),8),
           ArithmeticTarget::HLI => ($self.pc.wrapping_add(1),8),
           _ => ($self.pc.wrapping_add(1),4)
       }
    };
}


impl CPU {
    pub fn new() -> Self{
        CPU{
            registers: Registers::new(),
            pc: 0x0,
            sp: 0x00,
            bus: MemoryBus::new(),
            is_halted: false
        }
    }

    fn inc_8bit(&mut self,value: u8) -> u8{
        let new_value = value.wrapping_add(1);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = value & 0xF == 0xF;
        new_value
    }

    fn inc_16bit(&mut self,value: u16) -> u16{
        value.wrapping_add(1)
    }

    fn execute(&mut self,instruction: Instruction) -> (u16,u8){
        // if self.is_halted{
        //     return self.pc
        // }
        match instruction{

            Instruction::INC(target) =>{
                match target{
                    IncDecTarget::A => manipulate_8bit_register!(self: a => inc_8bit => a),
                    IncDecTarget::B => manipulate_8bit_register!(self: b => inc_8bit => b),
                    IncDecTarget::C => manipulate_8bit_register!(self: c => inc_8bit => c),
                    IncDecTarget::D => manipulate_8bit_register!(self: d => inc_8bit => d),
                    IncDecTarget::E => manipulate_8bit_register!(self: e => inc_8bit => e),
                    IncDecTarget::H => manipulate_8bit_register!(self: h => inc_8bit => h),
                    IncDecTarget::L => manipulate_8bit_register!(self: l => inc_8bit => l),
                    // IncDecTarget::AF => manipulate_16bit_register!(self: get_af => inc_16bit => set_af),
                    IncDecTarget::BC => {manipulate_16bit_register!(self: get_bc => inc_16bit => set_bc)}
                    IncDecTarget::HL => {manipulate_16bit_register!(self: get_hl => inc_16bit => set_hl)}
                    IncDecTarget::DE => {manipulate_16bit_register!(self: get_de => inc_16bit => set_de)}
                    IncDecTarget::SP => {
                        let amount = self.sp;
                        let result = self.inc_16bit(amount);
                        self.sp = result;
                    }
                    IncDecTarget::HLI => {
                        let hl = self.registers.get_hl();
                        let amount = self.bus.read_byte(hl);
                        let result = self.inc_8bit(amount);
                        self.bus.write_byte(hl,result);
                    }
                };
                let cycles = match target {
                    IncDecTarget::BC | IncDecTarget::DE | IncDecTarget::HL | IncDecTarget::SP=> 8,
                    IncDecTarget::HLI => 12,
                    _ => 4,
                };
                (self.pc.wrapping_add(1),cycles)
            }

            Instruction::JP(target) => {
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
                arithmetic_instruction!( target, self.add_without_carry => a)
            },

            Instruction::LD(load_type) => {
                match load_type{
                LoadType::Byte(target,source) => {
                    let source_val = match source {
                        LoadByteSource::A => self.registers.a,
                        LoadByteSource::B => self.registers.b,
                        LoadByteSource::C => self.registers.c,
                        LoadByteSource::D => self.registers.d,
                        LoadByteSource::E => self.registers.e,
                        LoadByteSource::H => self.registers.h,
                        LoadByteSource::L => self.registers.l,
                        LoadByteSource::D8 => self.read_next_byte(),
                        LoadByteSource::HLI => self.bus.read_byte(self.registers.get_hl()),
                        _ => {panic!("Other Sources Not Implemented!!!")}
                    };
                
                    match target{
                        LoadByteTarget::A => self.registers.a = source_val,
                        LoadByteTarget::B => self.registers.b = source_val,
                        LoadByteTarget::C => self.registers.c = source_val,
                        LoadByteTarget::D => self.registers.d = source_val,
                        LoadByteTarget::E => self.registers.e = source_val,
                        LoadByteTarget::H => self.registers.h = source_val,
                        LoadByteTarget::L => self.registers.l = source_val,
                        LoadByteTarget::HLI => self.bus.write_byte(self.registers.get_hl(),source_val),
                        _ => {panic!("Other Targets Not Implemented")}
                    };

                    match source{
                        LoadByteSource::D8 => (self.pc.wrapping_add(2),8),
                        LoadByteSource::HLI => (self.pc.wrapping_add(1),8),
                        _ => (self.pc.wrapping_add(1),4),
                    }
                }

                LoadType::Word(target) => {
                    let word = self.read_next_word();
                    match target{
                        LoadWordTarget::BC => self.registers.set_bc(word),
                        LoadWordTarget::DE => self.registers.set_de(word),
                        LoadWordTarget::HL => self.registers.set_hl(word),
                        LoadWordTarget::SP => self.sp = word,
                    };
                    (self.pc.wrapping_add(3),12)
                }

                LoadType::AFromIndirect(source) => {
                    self.registers.a = match source{
                        Indirect::BCIndirect => self.bus.read_byte(self.registers.get_bc()),
                        Indirect::DEIndirect => self.bus.read_byte(self.registers.get_de()),
                        Indirect::HLIndirectMinus => {
                            let hl = self.registers.get_hl();
                            self.registers.set_hl(hl.wrapping_sub(1));
                            self.bus.read_byte(hl)
                        }
                        Indirect::HLIndirectPlus => {
                            let hl = self.registers.get_hl();
                            self.registers.set_hl(hl.wrapping_add(1));
                            self.bus.read_byte(hl)
                        }
                        Indirect::WordIndirect => self.bus.read_byte(self.read_next_word()),
                        Indirect::LastByteIndirect => self.bus.read_byte(0xFF00 + self.registers.c as u16)
                    };

                    match source{
                        Indirect::WordIndirect => (self.pc.wrapping_add(3), 16),
                        _ => (self.pc.wrapping_add(1),8)
                    }
                }
                
                LoadType::IndirectFromA(target) => {
                    let a = self.registers.a;
                    match target {
                        Indirect::BCIndirect => self.bus.write_byte(self.registers.get_bc(), a),
                        Indirect::DEIndirect => self.bus.write_byte(self.registers.get_de(), a),
                        Indirect::HLIndirectMinus => {
                            let hl = self.registers.get_hl();
                            self.registers.set_hl(hl.wrapping_sub(1));
                            self.bus.write_byte(hl,a);
                        }
                        Indirect::HLIndirectPlus => {
                            let hl = self.registers.get_hl();
                            self.registers.set_hl(hl.wrapping_add(1));
                            self.bus.write_byte(hl, a);
                        }
                        Indirect::WordIndirect => self.bus.write_byte(self.read_next_word(), a),
                        Indirect::LastByteIndirect => self.bus.write_byte(0xFF00 + (self.registers.c as u16), a),
                    };

                    match target{
                        Indirect::WordIndirect => (self.pc.wrapping_add(3),16),
                        _ => (self.pc.wrapping_add(1),8)
                    }
                }

                LoadType::AFromByteAddress => {
                    let offset = self.read_next_byte() as u16;
                    self.registers.a = self.bus.read_byte(0xFF00 + offset);
                    (self.pc.wrapping_add(2),12)
                }

                LoadType::ByteAddressFromA =>{
                    let offset = self.read_next_byte() as u16;
                    self.bus.write_byte(0xFF00 + offset, self.registers.a);
                    (self.pc.wrapping_add(2),12)
                }

                
                _ => {panic!("Other Load Types not Implemented Yet")}
                }
            },

            Instruction::PUSH(target) => {
                let value = match target{
                    StackTarget::AF => self.registers.get_af(),
                    StackTarget::BC => self.registers.get_bc(),
                    StackTarget::DE => self.registers.get_de(),
                    StackTarget::HL => self.registers.get_hl(),
                    _ => {panic!("Other Targets not Supported Yet!!!")}
                };
                self.push(value);
                (self.pc.wrapping_add(1), 16)
            },

            Instruction::POP(target) => {
                let result = self.pop();
                match target{
                    StackTarget::AF => self.registers.set_af(result),
                    StackTarget::BC => self.registers.set_bc(result),
                    StackTarget::DE => self.registers.set_de(result),
                    StackTarget::HL => self.registers.set_hl(result),
                    _ => {panic!("Yet to Add Support for more Instruction in StackTarget")},
                };
                (self.pc.wrapping_add(1), 12)
            },

            Instruction::CALL(function) => {
                let jump_condition = match function{
                    JumpTarget::NotZero => !self.registers.f.zero,
                    _=> {panic!("Yet to Add more Conditions")},
                };
                self.call(jump_condition)
            },

            Instruction::RET(function) => {
                let jump_condition = match function {
                    JumpTarget::NotZero => !self.registers.f.zero,
                    JumpTarget::Zero => self.registers.f.zero,
                    JumpTarget::NotCarry => !self.registers.f.carry,
                    JumpTarget::Carry => self.registers.f.carry,
                    JumpTarget::Always => true,
                    _=>{panic!("Yet to add more Conditions")}
                };
                
                let next_pc = self.return_(jump_condition);

                let cycles = if jump_condition && function == JumpTarget::Always{
                    16
                } else if jump_condition {
                    20
                } else {
                    8
                };
                (next_pc,cycles)
            },

            Instruction::NOP => {
                (self.pc.wrapping_add(1), 4)
            },

            Instruction::Halt => {
                self.is_halted = true;
                (self.pc.wrapping_add(1),4)
            },

            _ => {panic!("Support for more Instructions not Added Yet.")}
        };
        
    }

    fn call(&mut self,should_jump: bool) -> (u16,u8){
        let next_pc = self.pc.wrapping_add(3);
        if should_jump{
            self.push(next_pc);
            (self.read_next_word(),24)
        }else{
            (next_pc,12)
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

    fn pop(&mut self) -> u16{
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
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
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

    fn jump(&self,condition:bool) -> (u16,u8) {
        if condition {
            // let least_significant_byte = self.bus.read_byte(self.pc +1) as u16;
            // let most_significant_byte = self.bus.read_byte(self.pc + 2) as u16;
            // (most_significant_byte<<8) | least_significant_byte
            (self.read_next_word(),16)
        } else{
            // self.pc.wrapping_add(3)
            (self.pc.wrapping_add(3),12)
        }
    }

    fn read_next_word(&self) -> u16{
        ((self.bus.read_byte(self.pc + 2) as u16) << 8) | (self.bus.read_byte(self.pc+1) as u16)
    }

    fn read_next_byte(&self) -> u8{
        (self.bus.read_byte(self.pc + 1) as u8)
    }
}
