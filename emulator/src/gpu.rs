use atd;

use super::{VRAM_BEGIN,VRAM_SIZE};

const NUMBER_OF_OBJECTS: usize = 40;

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Copy,Clone,Debug,PartialEq)]
pub enum Color{
    White = 255,
    LightGray = 192,
    DarkGray = 96,
    Black = 0,
}

impl std::convert::From<u8> for Color{
    fn from(n: u8) -> Self{
        match n {
            0 => Color::White,
            1 => Color::LightGray,
            2 => Color::DarkGray,
            3 => Color::Black,
            _ => panic!("Cannot Convert {} to Color", n)
        }
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Copy,Clone,Debug,PartialEq)]
pub struct BackgroundColors(Color, Color, Color, Color);

impl BackgroundColors{
    fn new() -> Self{
        BackgroundColors (
            Color::White,
            Color::LightGray,
            Color::DarkGray,
            Color::Black,
            )
    }
}

impl std::convert::From<u8> for BackgroundColors {
    fn from(value: u8) -> Self {
        BackgroundColor (
            (value & 0b11).into(),
            ((value >> 2) & 0b11).into(),
            ((value >> 4) & 0b11).into(),
            (value >> 6).into()
            )
    }
}

pub struct GPU{
    vram: [u8; VRAM_SIZE],
    tile_set: [Tile; 384],
}

type TileRow = [TilePixelValue;8];
type Tile = [TileRow;8];

#[derive(Copy,Clone)]
pub enum TilePixelValue{
    Zero,
    One,
    Two,
    Three,
}

fn empty_tile() -> Tile{
    [[Default::default(); 8]; 8]
}


impl Default for TilePixelValue{
    fn default() -> Self {
        TilePixelValue::Zero
    }
}

impl GPU{

    pub fn new() -> Self{
        GPU { vram: [0;VRAM_SIZE], tile_set: [Tile::new();384] }
    }

    fn write_vram(&mut self, index: usize,value: u8){
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

