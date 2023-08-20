pub struct CpuMemory {
    work_memory: [u8; 2048],
    ppu_ctrl: [u8; 8],
}

impl CpuMemory {
    #[inline]
    pub fn read(&self, address: u16) -> u8 {
        match address {
            // Work Memory & Mirrors
            0x0000..=0x1FFF => self.work_memory[(address % 2048) as usize],
            // PPU Ctrl Registers & Mirrors
            0x2000..=0x3FFF => self.ppu_ctrl[(address % 8) as usize],
            //APU and IO registers
            0x4000..=0x4017 => {
                todo!("APU and IO registers")
            }
            //Cpu Test Mode
            0x4018..=0x401F => {
                todo!("CPU test mode")
            }
            //Cartridge Read
            0x4020..=0xFFFF => {
                todo!("Cartridge read")
            }
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            // Work Memory & Mirrorsw
            0x0000..=0x1FFF => self.work_memory[(address % 2048) as usize] = value,
            // PPU Ctrl Registers & Mirrors
            0x2000..=0x3FFF => self.ppu_ctrl[(address % 8) as usize] = value,
            //APU and IO registers
            0x4000..=0x4017 => {
                todo!("APU and IO registers")
            }
            //Cpu Test Mode
            0x4018..=0x401F => {
                todo!("CPU test mode")
            }
            //Cartridge Write
            0x4020..=0xFFFF => {
                todo!("Cartridge read")
            }
        }
    }
}
