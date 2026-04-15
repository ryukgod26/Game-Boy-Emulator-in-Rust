pub struct MemoryBus{
    memory: [u8; 0xFFFF],
    gpu: GPU{},
}


impl MemoryBus{
    fn read_byte(&self,address: u16) ->u8{
        //self.memory[address as usize]
        let address = address as usize;
        match address{
            VRAM_BEGIN...VRAM_END{
                self.gpu.read_vram(address - VRAM_BEGIN)
            }
            _ => panic!("Yet to add Support for other areas of memory.")
        }
    }

    fn write_byte(&self, address: u16, value: u8) {
        let address = address as usize;
        match address{
            VRAM_BEGIN...VRAM_END {
                self.gpu.write_vram(address - VRAM_BEGIN, value)
            }
            _ => panic!("Yet to add Support for other areas of memory.")
        }
    }

}
