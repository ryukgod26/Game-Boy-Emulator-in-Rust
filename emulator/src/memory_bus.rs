use crate::{
    gpu::{BackgroundAndWindowDataSelect,InterruptRequest,ObjectSize,TileMap,GPU},
};

pub const BOOT_ROM_START: usize = 0x00;
pub const BOOT_ROM_END: usize = 0xFF;
pub const BOOT_ROM_SIZEi: usize = BOOT_ROM_END - BOOT_ROM_START + 1;

pub const ROM_BANK_0_START: usize = 0x0000;
pub const ROM_BANK_0_END: usize = 0x3FFF;
pub const ROM_BANK_0_SIZE: usize = ROM_BANK_0_END - ROM_BANK_0_START + 1;

pub const ROM_BANK_N_START: usize = 0x4000;
pub const ROM_BANK_N_END: usize = 0x7FFF;
pub const ROM_BANK_N_SIZE: usize = ROM_BANK_N_END - ROM_BANK_N_START + 1;

pub const EXTERNAL_RAM_START: usize = 0xA000;
pub const EXTERNAL_RAM_END: usize = 0xBFFF;
pub const EXTERNAL_RAM_SIZE: usize = EXTERNAL_RAM_END - EXTERNAL_RAM_START + 1;

pub const WORKING_RAM_START: usize = 0xC000;
pub const WORKING_RAM_END: usize = 0xDFFF;
pub const WORKING_RAM_SIZE: usize = WORKING_RAM_END - WORKING_RAM_START + 1;

pub const ECHO_RAM_START: usize = 0xE000;
pub const ECHO_RAM_END: usize = 0xFDFF;
pub const ECHO_RAM_SIZE: usize = ECHO_RAM_END - ECHO_RAM_START + 1;

pub const OAM_START: usize = 0xFE00;
pub const OAM_END: usize = 0xFE9F;
pub const OAM_SIZE: usize = OAM_END - OAM_START + 1;

pub const UNUSED_START: usize = 0xFEA0;
pub const UNUSED_END: usize = 0xFEFF;
pub const UNUSED_SIZE: usize = UNUSED_END - UNUSED_START + 1;

pub const IO_REGISTERS_START: usize = 0xFF00;
pub const IO_REGISTERS_END: usize = 0xFF7F;
pub const IO_REGISTERS_SIZE: usize = IO_REGISTERS_END - IO_REGISTERS_START + 1;

pub const ZERO_PAGE_START: usize = 0xFF80;
pub const ZERO_PAGE_END: usize = 0xFFFE;
pub const ZERO_PAGE_SIZE: usize = ZERO_PAGE_END - ZERO_PAGE_START + 1;

pub const INTERRUPT_ENABLE_REGISTER: usize = 0xFFFF;

pub const VBLANK_VECTOR: u16 = 0x40;
pub const LCDSTAT_VECTOR: u16 = 0x48;
pub const TIMER_VECTOR: u16 = 0x50;

pub const VRAM_BEGIN: usize = 0x8000;
pub const VRAM_END: usize = 0x9FFF;
pub const VRAM_SIZE: usize = VRAM_END - VRAM_BEGIN + 1;


#[cfg_attr(feature="serialize",derive(Serialize))]
pub struct MemoryBus{
    #[cfg_attr(feature="serialize", serde(skip_serializing))]
    boot_rom: Option<[u8; BOOT_ROM_SIZE]>,
    #[cfg_attr(feature="serialize", serde(skip_serializing))]
    rom_bank_0: [u8; ROM_BANK_0_SIZE],
    #[cfg_attr(feature="serialize", serde(skip_serializing))]
    rom_bank_n: [u8; ROM_BANK_N_SIZE],
    #[cfg_attr(feature="serialize", serde(skip_serializing))]
    external_ram: [u8; EXTERNAL_RAM_SIZE],
    #[cfg_attr(feature="serialize", serde(skip_serializing))]
    working_ram: [u8; WORKING_RAM_SIZE],
    #[cfg_attr(feature="serialize", serde(skip_serializing))]
    zero_page: [u8; ZERO_PAGE_SIZE],

    pub gpu: GPU,
    pub interrupt_enable: InterruptFlags,
    pub interrupt_flag: InterruptFlags,
    timer: Timer,
    divider: Timer,
    pub joypad: Joypad,
}


impl MemoryBus{

    pub fn new(boot_rom_buffer: Option<Vec<u8>>, game_rom: Vec<u8>) -> Self{
        let boot_rom = boot_rom_buffer.map(|boot_rom_buffer| {
            if boot_rom_buffer.len() != BOOT_ROM_SIZE {
                panic!("Supplied Boot rom in wrong size. {} bytes should be {} bytes",boot_rom_buffer.len(),BOOT_ROM_SIZE);
            }
            let mut boot_rom = [0; BOOT_ROM_SIZE];
            boot_rom.copy_from_slice(&boot_rom_buffer);
            boot_rom
        });
        let mut rom_bank_0 = [0; ROM_BANK_0_SIZE];
        for i in 0..ROM_BANK_0_SIZE{
            rom_bank_0[i] = game_rom[i];
        }
        let mut rom_bank_n = [0; ROM_BANK_N_SIZE];
        for i in 0..ROM_BANK_N_SIZE{
            rom_bank_n[i] = gane_rom[ROM_BANK_0_SIZE + i];
        }
        let mut divider = Timer::new(Frequency::F16384);
        divider.on = true;

        MemoryBus{
            boot_rom,
            rom_bank_0,
            rom_bank_n,
            external_ram: [0; EXTERNAL_RAM_SIZE],
            working_ram: [0; WORKING_RAM_SIZE],
            zero_page: [0; ZERO_PAGE_SIZE],
            gpu: GPU::new(),
            interrupt_enable: InterruptFlags::new(),
            interrupt_flag: InterruptFlags::new(),
            timer: Timer::new(Frequency::F4096),
            divider,
            joypad: Joypad::new(),
        }
    }

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
