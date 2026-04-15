use super::GPU;

pub const VRAM_BEGIN: usize = 0x8000;
pub const VRAM_END: usize = 0x9FFF;
pub const VRAM_SIZE: usize = VRAM_END - VRAM_BEGIN + 1;


pub struct MemoryBus{
    memory: [u8; 0xFFFF],
    gpu: GPU,
}


impl MemoryBus{
    pub fn read_byte(&self,address: u16) ->u8{
        //self.memory[address as usize]
        let address = address as usize;
        match address{
            VRAM_BEGIN...VRAM_END{
                self.gpu.read_vram(address - VRAM_BEGIN)
            }
            _ => panic!("Yet to add Support for other areas of memory.")
        }
    }

    pub fn write_byte(&self, address: u16, value: u8) {
        let address = address as usize;
        match address{
            VRAM_BEGIN...VRAM_END {
                self.gpu.write_vram(address - VRAM_BEGIN, value)
            }
            _ => panic!("Yet to add Support for other areas of memory.")
        }
    }

}
