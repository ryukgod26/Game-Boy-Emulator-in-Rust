use crate::utils::bit;

#[cfg_attr(feature="serialize",derive(Serialize))]
#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub enum Column{
    Zero,
    One,
}

#[cfg_attr(feature="serialize",derive(Serialize))]
#[derive(Copy,Clone,Debug)]
pub struct Joypad{
    pub column: Column,
    pub start: bool,
    pub select:bool,
    pub a: bool,
    pub b: bool,
    pub up: bool,
    pub down: bool,
    pub right: bool,
    pub left: bool,
}

impl Joypad{
    pub fn new() -> Self{
        Joypad{
            column: Column::Zero,
            start: false,
            select: false,
            a: false,
            b: false,
            up: false,
            down: false,
            left: false,
            right: false,
        }
    }

    pub fn to_byte(&self) -> u8{
        let column_bit = if self.column == Column::Zero{
            1 << 5
        }else{
            1<<4
        };
        let bit_4 = bit(!(((self.down && self.reading_column_0()) || (self.start && self.reading_column_1())))) << 3;
        let bit_3 = bit(!((self.up && self.reading_column_0()) || (self.select && self.reading_column_1()))) << 2;
        let bit_2 = bit(!((self.left && self.reading_column_0()) || (self.b && self.reading_column_1()))) << 1;
        let bit_1 = bit(!((self.right && self.reading_column_0()) || (self.a && self.reading_column_1())));

        let row_bits = bit_4 | bit_3 | bit_2 | bit_1;
        column_bit | row_bits
    }
}