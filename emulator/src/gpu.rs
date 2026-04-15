use super::{VRAM_BEGIN,VRAM_SIZE};
pub struct GPU{
    vram: [u8; VRAM_SIZE],
    tile_set: [Tile; 384],
}
impl GPU{
fn write_vram(&self, index: usize,value: u8){
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

