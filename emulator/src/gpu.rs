use std;

use super::{VRAM_BEGIN,VRAM_SIZE,OAM_SIZE};

const NUMBER_OF_OBJECTS: usize = 40;
const SCREEN_WIDTH: usize = 160;
const SCREEN_HEIGHT: usize = 144;

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Copy,Clone,Debug,PartialEq)]
pub enum BackgroundAndWindowDataSelect{
    X8000,
    X8800,
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Copy,Clone,Debug,PartialEq)]
pub enum Color{
    White = 255,
    LightGray = 192,
    DarkGray = 96,
    Black = 0,
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Copy,Clone,Debug,PartialEq)]
pub enum TileMap{
    X9800,
    X9C00,
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Copy,Clone,Debug,PartialEq)]
pub enum ObjectSize{
    OS8X8,
    OS8X16,
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Copy,Clone,Debug,PartialEq)]
pub enum Mode{
    HorizontalBlank,
    VerticalBlank,
    OAMAccess,
    VRAMAccess
}

impl std::convert::From<Mode> for u8{
    fn from(value: Mode) -> u8{
        match value{
            Mode::HorizontalBlank => 0,
            Mode::VerticalBlank => 1,
            Mode::OAMAccess => 2,
            Mode::VRAMAccess => 3,
        }
    }
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

#[derive(PartialEq, Eq)]
pub enum InterruptRequest{
    None,
    VBlank,
    LCDStat,
    Both,
}

impl InterruptRequest{
    fn add(&mut self, other: InterruptRequest){
        match self{
            InterruptRequest::None => *self = other,
            InterruptRequest::VBlank => {
                if other == InterruptRequest::LCDStat{
                    *self = InterruptRequest::Both
                }
            }
            InterruptRequest::LCDStat => {
                if other == InterruptRequest::VBlank{
                    *self = InterruptRequest::Both
                }
            }
            _ => {}
        };
    }
}

#[derive(Clone, Copy,PartialEq,Debug)]
pub struct ObjectData{
    x: i16,
    y: i16,
    tile: u8,
    palette: ObjectPalette,
    xflip: bool,
    yflip: bool,
    priority: bool,
}

impl Default for ObjectData{
    fn default() -> Self {
        ObjectData { 
            x: -16,
            y: -8,
            tile: Default::default(), 
            palette: Default::default(),
            xflip: Default::default(), 
            yflip: Default::default(), 
            priority: Default::default()
         }
    }
}

#[derive(Clone, Copy,Debug,PartialEq)]
enum ObjectPalette{
    Zero,
    One
}

impl Default for ObjectPalette{
    fn default() -> Self {
        ObjectPalette::Zero
    }
}

#[cfg_attr(feature="serialize", derive(Serialize))]
pub struct Window{
    pub x: u8,
    pub y: u8,
}

pub struct GPU{
    #[cfg_attr(feature="serialize", serde(skip_serializing))]
    pub canvas_buffer: [u8; SCREEN_HEIGHT * SCREEN_WIDTH * 4],
    #[cfg_attr(feature="serialize", serde(skip_serializing))]
    pub tile_set: [Tile; 384],
    #[cfg_attr(feature="serialize", serde(skip_serializing))]
    pub object_data: [ObjectData; NUMBER_OF_OBJECTS],
    #[cfg_attr(feature="serialize", serde(skip_serializing))]
    pub vram: [u8; VRAM_SIZE],
    #[cfg_attr(feature="serialize", serde(skip_serializing))]
    pub oam: [u8; OAM_SIZE],
    
    pub background_colors: BackgroundColors,
    pub viewport_x_offset: u8,
    pub viewport_y_offset: u8,
    pub lcd_display_enabled: bool,
    pub window_display_enabled: bool,
    pub background_display_enabled: bool,
    pub object_display_enabled: bool,
    pub line_equals_line_check_interrupt_enabled: bool,
    pub oam_interrupt_enabled: bool,
    pub vblank_interrupt_enabled: bool,
    pub hblank_interrupt_enabled: bool,
    pub line_check: u8,
    pub line_equals_line_check: u8,
    pub window_tile_map: TileMap,
    pub background_tile_map: TileMap,
    pub background_and_window_data_select: BackgroundAndWindowDataSelect,
    pub object_size: ObjectSize,
    pub obj_0_color_1: Color,
    pub obj_0_color_2: Color,
    pub obj_0_color_3: Color,
    pub obj_1_color_1: Color,
    pub obj_1_color_2: Color,
    pub obj_1_color_3: Color,
    pub window: Window,
    pub line: u8,
    pub mode: Mode,
    cycles: u16,
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

const SCREEN_WIDTH: usize = 160;
const SCREEN_HEIGHT: usize = 144;

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

