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

#[cfg_attr(feature="serialize", derive(Serialize))]
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
    pub line_equals_line_check: bool,
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
        GPU { canvas_buffer: [0; SCREEN_HEIGHT * SCREEN_WIDTH * 4],
            tile_set: [empty_tile();384],
            object_data: [ObjectData::default(); NUMBER_OF_OBJECTS],
            vram: [0; VRAM_SIZE],
            oam: [0; OAM_SIZE],
            background_colors: BackgroundColors::new(),
            viewport_x_offset: 0,
            viewport_y_offset: 0,
            lcd_display_enabled: false,
            window_display_enabled: false,
            background_display_enabled: false,
            object_display_enabled: false,
            line_equals_line_check_interrupt_enabled: false,
            oam_interrupt_enabled: false,
            vblank_interrupt_enabled: false,
            hblank_interrupt_enabled: false,
            line_check: 0,
            line_equals_line_check: false,
            window_tile_map: TileMap::X9800,
            background_tile_map: TileMap::X9800,
            background_and_window_data_select: BackgroundAndWindowDataSelect::X8800,
            object_size: ObjectSize::OS8X8,
            obj_0_color_1: Color::LightGray,
            obj_0_color_2: Color::DarkGray,
            obj_0_color_3: Color::Black,
            obj_1_color_1: Color::LightGray,
            obj_1_color_2: Color::DarkGray,
            obj_1_color_3: Color::Black,
            window: Window { x: 0, y: 0 },
            line: 0,
            mode: Mode::HorizontalBlank,
            cycles: 0,
        }
    }

    pub fn write_oam(&mut self, index: usize, value: u8) {
        self.oam[index] = value;
        let object_index = index/4;

        if object_index > NUMBER_OF_OBJECTS{
            return;
        }

        let byte = index % 4;
        let mut data = self.object_data.get_mut(object_index).unwrap();
        match byte {
            0 => data.y = (value as i16) - 0x10,
            1 => data.x = (value as i16) - 0x8,
            2 => data.tile = value,
            _ => {
                data.palette = if (value & 0x10) != 0 {
                    ObjectPalette::One
                } else{
                    ObjectPalette::Zero
                };
                data.xflip = (value & 0x20) != 0;
                data.yflip = (value & 0x40) != 0;
                data.priority = (value & 0x80) == 0;
            }
        }
    }

    pub fn write_vram(&mut self, index: usize,value: u8){
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

    fn set_equal_lines_check(&mut self,request: &mut InterruptRequest){
        let line_equals_line_check = self.line == self.line_check;
        if line_equals_line_check && self.line_equals_line_check_interrupt_enabled{
            request.add(InterruptRequest::LCDStat);
        }
        self.line_equals_line_check = self.line_equals_line_check;
    }

    fn read_vram(&self,addr: usize) -> u8{
        self.vram[addr]
    }

    pub fn tile_set_as_buffer(&self,outline_lines: bool) -> Vec<u8>{
        let values_per_pixel = 4;
        let tile_width = 8;
        let tile_height = 8;
        let width_in_tiles = 24;
        let height_in_tiles = self.tile_set.len() / width_in_tiles;
        let row_width = tile_width * width_in_tiles * values_per_pixel;
        let mut data = vec![0; width_in_tiles * height_in_tiles * tile_height * tile_width * values_per_pixel];

        for(tile_index, tile) in self.tile_set.iter().enumerate(){
            let tile_row = tile_index / width_in_tiles;
            let tile_column = tile_index % width_in_tiles;
            let final_tile_row = tile_row == height_in_tiles -1;
            let final_tile_column = tile_column == width_in_tiles - 1;

            for(row_index, row) in tile.iter().enumerate(){
                let pixel_row_index = (tile_row * tile_height) + row_index;
                let start_of_canvas_row = pixel_row_index * row_width;
                let on_tile_row_border = pixel_row_index % 8 ==0 ;
                let start_of_column = tile_column * tile_width;
                let final_pixel_row = final_tile_row && row_index == 7;
                let mut index = start_of_canvas_row + (start_of_column * values_per_pixel);
                
                for(pixel_index,pixel) in row.iter().enumerate(){
                    let on_tile_column_border = pixel_index == 0;
                    let final_pixel_column = final_tile_column && pixel_index == 7;
                    if outline_lines && (on_tile_row_border || on_tile_column_border || final_pixel_row || final_pixel_column){
                        data[index] = 0;
                        data[index + 1] = 0;
                        data[index + 2] = 255;
                    } else{
                        let color = self.tile_value_to_background_color(pixel);
                        data[index] = color as u8;
                        data[index + 1] = color as u8;
                        data[index + 2] = color as u8;
                    }
                    data[index + 3] = 255;
                    index = index + values_per_pixel;
                }
            }
        }
        data
    }

    fn tile_value_to_background_color(&self,tile_value: &TilePixelValue) -> Color{
        match tile_value {
            TilePixelValue::Zero => self.background_colors.0,
            TilePixelValue::One => self.background_colors.1,
            TilePixelValue::Two => self.background_colors.2,
            TilePixelValue::Three => self.background_colors.3,
        }
    }

    fn background_1(&self) -> &[u8]{
        &self.vram[0x1800..0x1C00]
    }

}

