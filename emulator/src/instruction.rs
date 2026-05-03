pub enum Instruction{
    Add(ArithmeticTarget),JP(JumpTest),LD(LoadType),PUSH(StackTarget),POP(StackTarget),CALL(JumpTarget),RET(JumpTarget),NOP,Halt,INC(IncDecTarget),
    RST(RSTLocation),ADDHL(ADDHLTarget),ADDSP,
}

pub enum ArithmeticTarget{
    A,B,C,D,E,H,L,HLI,
}

pub enum JumpTest{
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always,
}

pub enum LoadByteTarget{
    A,B,C,D,E,H,L,HLI
}

#[derive(Debug,Clone, Copy,PartialEq)]
pub enum ADDHLTarget{
    BC,
    DE,
    HL,
    SP,
}

pub enum LoadByteSource{
    A,B,C,D,E,H,L,D8,HLI
}

pub enum LoadType{
    Byte(LoadByteTarget,LoadByteSource), Word(LoadWordTarget), AFromIndirect(Indirect),IndirectFromA(Indirect),ByteAddressFromA,AFromByteAddress
}

pub enum Indirect{
    BCIndirect,DEIndirect,HLIndirectMinus,HLIndirectPlus,WordIndirect,LastByteIndirect
}

pub enum LoadWordTarget {
    BC,DE,HL,SP
}

pub enum IncDecTarget{
    A,B,C,D,E,H,L,BC,DE,SP,HL,HLI,
}

pub enum StackTarget{
    AF,DE,BC,HL
}

pub enum JumpTarget{
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always
}

#[derive(Debug,Clone, Copy,PartialEq)]
pub enum RSTLocation{
    X00,
    X08,
    X10,
    X18,
    X20,
    X28,
    X30,
    X38
}

impl RSTLocation {
    pub fn to_hex(&self) -> u16{
        match self {
            RSTLocation::X00 => 0x00,
            RSTLocation::X08 => 0x08,
            RSTLocation::X10 => 0x10,
            RSTLocation::X18 => 0x18,
            RSTLocation::X20 => 0x20,
            RSTLocation::X28 => 0x28,
            RSTLocation::X30 => 0x30,
            RSTLocation::X38 => 0x38
        }
    }
}

pub enum PrefixTarget{
    A,B,C,D,E,H,L
}

impl Instruction{
    pub fn from_byte(byte: u8,prefixed: bool) -> Option<Instruction>{
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
