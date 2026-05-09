use std::ptr::fn_addr_eq;

#[cfg_attr(feature="serialize",derive(Serialize))]
pub enum Frequency{
    F4096,
    F16384,
    F262144,
    F65536,
}

impl Frequency{
    fn cycles_per_tick(&self) -> usize{
        match self {
            Frequency::F4096 => 1024,
            Frequency::F16384 => 256,
            Frequency::F65536 => 64,
            Frequency::F262144 => 16,
        }
    }
}

#[cfg_attr(feature="serialize",derive(Serialize))]
pub struct Timer{
    pub frequency: Frequency,
    pub value: u8,
    pub modulo: u8,
    pub on: bool,
    cycles: usize,
}

impl Timer{
    pub fn new(frequency: Frequency) -> Self{
        Timer { 
            frequency,
            value: 0,
            modulo: 0,
            on: false,
            cycles: 0
        }
    }

    pub fn step(&mut self,cycles: u8) -> bool{
        if !self.on{
            return false;
        }

        self.cycles += cycles as usize;

        let cycles_per_tick = self.frequency.cycles_per_tick();
        let did_overflow = if self.cycles > cycles_per_tick{
            self.cycles = self.cycles % cycles_per_tick;
            let (new, did_overflow) = self.value.overflowing_add(1);
            self.value = new;
            did_overflow
        }else {
            false
        };
        if did_overflow{
            self.value = self.modulo;
        }
        did_overflow
    }
}