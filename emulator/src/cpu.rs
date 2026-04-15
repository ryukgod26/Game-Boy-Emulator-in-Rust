use super::{Instruction,JumpTest,ArithmeticTarget,Registers,MemoryBus,StackTarget,LoadByteTarget,LoadType,LoadByteSource,JumpTarget};

pub struct CPU{
    pub registers: Registers,
    pub pc: u16,
    pub sp: u16,
    pub bus: MemoryBus,
    pub is_halted: bool,
}

impl CPU {
    fn execute(&mut self,instruction: Instruction) -> u16{
        if self.is_halted{
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
                _ => (self.pc.wrapping_add(1))
                }
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
                self.pc.wrapping_add(1)
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
            },

            Instruction::RET(function) => {
                let jump_condition = match function {
                    JumpTarget::NotZero => !self.registers.f.zero,
                    _=>{panic!("Yet to add more Conditions")}
                };
                self.return_(jump_condition)
            },

            Instruction::NOP => {
                self.pc.wrapping_add(1)
            },

            Instruction::Halt => {
                self.is_halted = true;
                self.pc.wrapping_add(1)
            },

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

    fn jump(&self,condition:bool) ->u16 {
        if condition {
            let least_significant_byte = self.bus.read_byte(self.pc +1) as u16;
            let most_significant_byte = self.bus.read_byte(self.pc + 2) as u16;
            (most_significant_byte<<8) | least_significant_byte
        } else{
            self.pc.wrapping_add(3)
        }
    }

    fn read_next_word(&self) -> u16{
        ((self.bus.read_byte(self.pc + 2) as u16) << 8) | (self.bus.read_byte(self.pc+1) as u16)
    }

    fn read_next_byte(&self) -> u8{
        (self.bus.read_byte(self.pc + 1) as u8)
    }
}