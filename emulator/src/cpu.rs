use std::result;

use crate::ADDHLTarget;

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

macro_rules! prefix_instruction{
    ($register: ident,$self: ident.$work: ident => reg) => {
        {
        match $register {
            PrefixTarget::A => manipulate_8bit_register!($self: a => $work => a),
            PrefixTarget::B => manipulate_8bit_register!($self: b => $work => b),
            PrefixTarget::C => manipulate_8bit_register!($self: c => $work => c),
            PrefixTarget::D => manipulate_8bit_register!($self: d => $work => d),
            PrefixTarget::E => manipulate_8bit_register!($self: e => $work => e),
            PrefixTarget::H => manipulate_8bit_register!($self: h => $work => h),
            PrefixTarget::L => manipulate_8bit_register!($self: l => $work => l),
            PrefixTarget::HLI => {
                let hl = $self.registers.get_hl();
                let value = $self.bus.read_byte(hl);
                let result = $self.$work(value);
                $self.bus.write_byte(hl,result)
            }
        }
        let cycles = match $register{
            PrefixTarget::HLI => 16,
            _                 => 8,
        };
        ($self.pc.wrapping_add(2),cycles)
    }
    };

    ($register: ident, ($self: ident.$work: ident @ bit_position: ident) => reg) => {
        {
            match $register{
                PrefixTarget::A => manipulate_8bit_register!($self: (a @ $bit_position) => $work => a),
                PrefixTarget::B => manipulate_8bit_register!($self: (b @ $bit_position) => $work => b),
                PrefixTarget::C => manipulate_8bit_register!($self: (c @ $bit_position) => $work => c),
                PrefixTarget::D => manipulaye_8bit_register!($self: (d @ $bit_position) => $work => d),
                PrefixTarget::E => manipulate_8bit_register!($self: (e @ $bit_position) => $work => e),
                PrefixTarget::H => manipulate_8bit_register!($self: (h @ $bit_position) => $work => h),
                PrefixTarget::L => manipulate_8bit_register!($self: (l @ $bit_position) => $work => l),
                PrefixTarget::HLI => {
                    let hl = $self.registers.get_hl();
                    let value = $self.bus.read_byte(hl);
                    let result = $self.$work(value, $bit_position);
                    $self.bus.write_byte(hl,result);
                }
            }

            let cycles = match $register{
                PrefixTarget::HLI => 16,
                _ => 8
            };
            ($self.pc.wrapping_add(2),cycles)
        }
    };

    ($register: ident, $self: ident.$work: ident @ $bit_position: ident) => {
        {
            match $register{
                PrefixTarget::A => manipulate_8bit_register!($self: (a @ $bit_position) => $work),
                PrefixTarget::B => manipulate_8bit_register!($self: (b @ $bit_position) => $work),
                PrefixTarget::C => manipulate_8bit_register!($self: (c @ $bit_position) => $work),
                PrefixTarget::D => manipulate_8bit_register!($self: (d @ $bit_position) => $work),
                PrefixTarget::E => manipulate_8bit_register!($self: (e @ $bit_position) => $work),
                PrefixTarget::H => manipulate_8bit_register!($self: (h @ $bit_position) => $work),
                PrefixTarget::L => manipulate_8bit_register!($self: (l @ $bit_position) => $work),
                PrefixTarget::HLI => {
                    let value = $self.bus.read_byte($self.registers.get_hl());
                    $self.$work(value,$bit_position);
                }
            }
            let cycles = match $register{
                PrefixTarget::HLI => 16,
                _ => 8
            };
            ($self.pc.wrapping_add(2),cycles)
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
            }

            Instruction::JR(target) => {
                let jump_condition = match target{
                    JumpTest::NotZero => !self.registers.f.zero,
                    JumpTest::Zero => self.registers.f.zero,
                    JumpTest::NotCarry => !self.registers.f.carry,
                    JumpTest::Carry => self.registers.f.carry,
                    JumpTest::Always => true,
                };
                self.jump_relative(jump_condition)
            }

            Instruction::JPI =>{
                (self.registers.get_hl(),4)
            }

            Instruction::Add(target) => {
                arithmetic_instruction!( target, self.add_without_carry => a)
            }

            Instruction::ADDHL(target){
                let value = match target{
                    ADDHLTarget::BC => self.registers.get_bc(),
                    ADDHLTarget::DE => self.registers.get_de(),
                    ADDHLTarget::HL => self.registers.get_hl(),
                    ADDHLTarget::SP => self.sp,
                };
                let result = self.add_hl(value);
                self.registers.set_hl(result);
                (self.pc.wrapping_add(1),8)
            }

            Instruction::ADC(register) => {
                arithmetic_instruction!(register, self.add_with_carry => a)
            }

            Instruction::SBC(register) => {
                arithmetic_instructiom!(register, self.sub_with_carry => a)
            }

            Instruction::AND(register) => {
                arithmetic_instruction!(register, self.and => a)
            }

            Instruction::OR(register) => {
                arithmetic_instruction!(register, self.or => a)
            }

            Instruction::XOR(register) => {
                arithmetic_instruction!(register, self.xor => a)
            }

            Instruction::CP(registet) => {
                arithmetic_instruction!(register, self.compare)
            }

            Instruction::RETI => {
                self.interupts_enabled = true;
                (self.pop(),16)
            }

            Instruction::RST(loc) => {
                self.rst();
                (loc.to_hex(),24)
            }

            Instruction::ADDSP => {
                // First We are casting the next bit as signed 8 and then signed 16 and then
                // unsigned 16
                let value = self.read_next_byte() as i8 as i16 as u16;
                let result = self.sp.wrapping_add(value);

                //Half and Carry are Computed at nibble and byte level instead of byte and word level
                let half_carry_mask = 0xF;
                self.registers.f.carry = (self.sp & half_carry_mask) + (value & half_carry_mask ) > half_carry_mask;
                let carry mask = 0xff;
                self.registers.f.carry = (self.sp & carry_mask) + (value & carry_mask) > carry_mask;
                self.registers.f.zero = false;
                self.registers.f.subtract = false;

                self.sp = result;
                (self.pc.wrapping_add(2),16)
            }

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
            }

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
            }

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
            }

            Instruction::CCF => {
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                self.regsiters.f.carry = !self.registers.f.carry;
                (self.pc.wrapping_add(1),4)
            }

            Instruction::SCF => {
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                self.registers.f.carry = true;
                (self.pc.wrapping_add(1),4)
            }

            Instruction::RRA => {
                manipulate_8bit_register!(self : a => rotate_right_through_carry_retain_zero => a);
                (self.pc.wrapping_add(1),4)
            }

            Instruction::RLA => {
                manipulate_8bit_register!(self: a => rotate_left_through_carry_retain_zero => a);
                (self.pc.wrapping_add(1),4)
            }

            Instruction::RRCA => {
                manipulate_8bit_register!(self: a => rotate_right_retain_zero => a);
                (self.pc.wrapping_add(1),4)
            }

            Instruction::RLCA =>{
                manipulate_8bit_rehister!(self: a => rotate_left_retain_zero => a);
                (self.pc.wrapping_add(1),4)
            }

            Instruction::CPL => {
                manipulate_8bit_register!(self: a => complement => a);
                (self.pc.wrapping_add(1),4)
            }

            Instruction::DAA => {
                manipulate_8bit_register!(self: a => decimal_adjust => a);
                (self.pc.wrapping_add(1),4)
            }

            Instruction::BIT(register,bit_position) => {
                prefix_instruction!(register, self.bit_test @ bit_position)
            }

            Instruction::RES(register, bit_position) => {
                prefix_instruction!(register, (self.reset_bit @ bit_position) => reg)
            }

            Instruction::SET(register,bit_position) => {
                prefix_instruction!(register, (self.set_bit @ bit_position) => reg)
            }

            Instruction::SRL(register) => {
                prefix_instruction!(register, self.shift_right_logical => reg)
            }

            Instruction::RR(register) => {
                prefix_instruction!(register, self.rotate_right_through_carry_set_zero => reg)
            }

            Instruction::RL(register) => {
                prefix_instruction!(register,self.rotate_left_through_carry_set_zero => reg)
            }

            Instruction::RRC(register) => {
                prefix_instruction!(register,self.rotate_right_set_zero => reg)
            }

            Instruction::RLC(register) => {
                prefix_instruction!(register, self.rotate_left_set_zero => reg)
            }

            Instruction::SRA(register) => {
                prefix_instruction!(register, self.shift_right_arithmetic => reg)
            }

            Instruction::SLA(register) => {
                prefix_instruction!(register, self.shift_left_arithmetic => reg)
            }

            Instruction::SWAP(register) => {
                prefix_instruction!(register, self.swap_nibbles => reg)
            }

            Instruction::CALL(function) => {
                let jump_condition = match function {
                    JumpTarget::NotZero => !self.registers.f.zero,
                    JumpTarget::Zero => self.registers.f.zero,
                    JumpTarget::NotCarry => !self.registers.f.carry,
                    JumpTarget::Carry => self.registers.f.carry,
                    JumpTarget::Always => true
                };
                self.call(jump_condition)
            }

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
            }

            Instruction::NOP => {
                (self.pc.wrapping_add(1), 4)
            }

            Instruction::Halt => {
                self.is_halted = true;
                (self.pc.wrapping_add(1),4)
            }

            Instruction::DI => {
                self.interupts_enabled = false;
                (self.pc.wrapping_add(1),4)
            }

            Instruction::EI => {
                self.interupts_enabled = true;
                (self.pc.wrapping_add(1),4)
            }

            Instruction::RST(loc) => {
                self.rst();
                (loc.to_hex(),24)
            }

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

    #[inline(always)]
    fn add_hl(&mut self, value: u16) -> u16{
        let hl = self.registers.get_hl();
        let (result,carry) = hl.overflowing_add(value);
        self.registers.f.carry = carry;
        self.registers.f.subtract = false;
        // Half carry tests if we flow over the 11h bit which means does adding the two numbers together cause the 11th bit to flip
        let mask = 0b111_1111_1111;
        self.registers.f.half_carry = (value & mask) + (hl & mask) > mask;

        result
    }

    #[inline(always)]
    fn rotate_right_through_carry_retain_zero(&mut self,value: u8) -> u8{
        self.rotate_right_through_carry(value,false)
    }

    #[inline(always)]
    fn rotate_right_through_carry_set_zero(&mut self,value: u8) -> u8{
        self.rotate_right_through_carry(value,true)
    }

    #[inline(always)]
    fn rotate_right_through_carry(&mut self,value: u8, set_zero: bool) -> u8{
        let carry_bit = if self.registers.f.carry {1} else {0} << 7;
        let new_value = carry_bit | (value >> 1);
        self.registers.f.zero = set_zero && new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = value & 0b1 == 0b1;

        new_value
    }

    #[inline(always)]
    fn rotate_right_set_zero(&mut self, value: u8) -> u8{
        self.rotate_right(value,true)
    }

    #[inline(always)]
    fn rotate_right_retain_zero(&mut self, value: u8) -> u8{
        self.rotate_right(value,false)
    }

    #[inline(always)]
    fn rotate_left_through_carry_set_zero(&mut self, value: u8) -> u8{
        self.rotate_left_through_carry(value,true)
    }

    #[inline(always)]
    fn rotate_left_through_carry_retain_zero(&mut self, value: u8) -> u8{
        self.rotate_left_through_carry(value,false)
    }

    #[inline(always)]
    fn rotate_left_through_carry(&mut self, value: u8, set_zero: bool) -> u8 {
        let carry_bit = if self.registers.f.carry {1} else {0};
        let new_value = (value << 1) | carry_bit;
        self.registers.f.zero = set_zero && new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self registers.f.carry = (value & 0x80) == 0x80;

        new_value
    }

    #[inline(always)]
    fn rotate_left_set_zero(&mut self,value: u8) -> u8{
        self.rotate_left(value,true)
    }

    #[inline(always)]
    fn rotate_left_retain_zero(&mut self, value: u8) -> u8{
        self.rotate_left(value,false)
    }

    #[inline(always)]
    fn rotate_left(&mut self, value: u8, set_zero: bool) -> u8{
        let carry = (value & 0x80) >> 7;
        let new_value = value.rotate_left(1) | carry;
        self.registers.f.zero = set_zero && new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = carry == 0x01;

        new_value
    }

    #[inline(always)]
    fn rotate_right(&mut self,value: u8, set_zero: bool) -> u8{
        let new_value = value.rotate_right(1);
        self.registers.f.zero = set_zeto && new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carty = false;
        self.registers.f.carry = value & 0b1 == 0b1;

        new_value

    }

    #[inline(always)]
    fn shift_right_arithmetic(&mut self, value: u8) -> u8 {
        let msb = value 0x80;
        let new_value = msb | (value >> 1);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = value & 0b1 == 0b1;

        new_value
    }

    #[inline(always)]
    fn shift_left_arithmetic(&mut self, value: u8) -> u8{
        let new_value = value << 1;
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = value & 0x80 == 0x80;

        new_value
    }

    #[inline(always)]
    fn swap_nibbles(&mut self, value: u8) -> u8 {
        let new_value = ((value & 0xf) << 4) | ((value & 0xf0) >> 4); 
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carty = false;
        self.registers.f.carry = false;

        new_value
    }

    #[inline(always)]
    fn decimal_adjust(&mut self,value: u8) -> u8{
        let flags = self.register.f;
        let mut carry = false;

        let result = if !flags.subtract{
            let mut result = value;
            if flags.carry || value > 0x99 {
                carry = true;
                result = result.wrapping_add(0x60);
            }

            if flags.half_carry || value & 0x0F > 0x09{
                result = result.wrapping_add(0x06);
            }
            
            result
        } else if flags.carry {
            carry = true;
            let add = if flags.half_carry {0x9A} else {0xA0};
            value.wrapping_add(add)
        } else if flags.half_carry {
            value.wrapping_add(0xFA)
        } else{
            value
        };

        self.registers.f.zero = result == 0;
        self.registers.f.carry = carry;
        self.registers.f.half_carry = false;

        result
    }

    #[inline(always)]
    fn complement(&mut self, value: u8) -> u8 {
        let new_value = !value;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = true;
        new_value
    }

    #[inline(always)]
    fn bit_test(&mut self, value: u8, bit_position: BitPosition){
        let bit_position: u8 = bit_position.into();
        let result = (value >> bit_position) & 0b1;
        self.registers.f.zero = result == 0;
        self.regsiters.f.subtract = false;
        self.registers.f.half_carry = true;
    }

    #[inline(always)]
    fn reset_bit(&mut self, value: u8, bit_position: BitPosition) -> u8{
        let bit_position: u8 = bit_position.into();
        value & !(1 << bit_position)
    }

    #[inline(always)]
    fn sey_bit(&mut self, value: u8, bit_position: BitPosition) -> u8{
        let bit_position: u8 = bit_position.into();
        value | (1 << bit_position)
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


    #[inline(always)]
    fn rst(&mut self){
        self.push(self.pc.wrapping_add(1));
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
    
    #[inline(always)]
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

    #[inline(always)]
    fn jump_relative(&self, should_jump: bool) -> (u16,u8) {
        let next_step = self.pc.wrapping_add(2);
        if should_jump {
            let offset = self.read_next_byte() as i8;
            let pc = next_step.wrapping_add(offset.abs() as u16);
            (pc,16)
            }else{
                (next_step,12)
        }
        }
    }

    fn read_next_word(&self) -> u16{
        ((self.bus.read_byte(self.pc + 2) as u16) << 8) | (self.bus.read_byte(self.pc+1) as u16)
    }

    fn read_next_byte(&self) -> u8{
        self.bus.read_byte(self.pc + 1) as u8
    }
}
