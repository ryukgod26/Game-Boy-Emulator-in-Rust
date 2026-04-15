pub enum Instruction{
    Add(ArithmeticTarget),Jp(JumpTest),LD(LoadType,LoadSource),PUSH(StackTarget),POP(StackTarget),CALL(JumpTarget),RET(JumpTarget),NOP,Halt,INC(IncDecTarget)
}

impl Instruction{
    fn from_byte(byte: u8,prefixed: bool) -> Option<Instruction>{
        if prefixed{
            Self::from_prefixed_byte(byte)
        }
        else{    
            Self::from_not_prefixed_byte(byte)
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
            0x06 => Some(Instruction::LD(LoadType::Byte(Target::B),LoadSource::NextByte)),
            0x0E => Some(Instruction::LD(LoadType::Byte(Target::C),LoadSource::NextByte)),
            0x02 => Some(Instruction::INC(IncDecTarget::BC)),
            0x13 => Some(Instruction::INC(IncDecTarget::DE)),
            _ => None,
        }
    }
}