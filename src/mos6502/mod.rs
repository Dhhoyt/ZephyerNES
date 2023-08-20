mod addressingmodes;
mod instructions;

use addressingmodes::{AddressingMode, ADDRESSING_MODES};
use instructions::INSTRUCTIONS;

use crate::cpu_memory::CpuMemory;

struct Mos6502 {
    program_counter: u16,
    accumulator: u8,
    index_x: u8,
    index_y: u8,
    stack_pointer: u8,
}

impl Mos6502 {
    // Returns the number of cycles this instruction took
    fn run_instruction(&mut self, memory: &mut CpuMemory) -> usize {
        let opcode = memory.read(self.program_counter);
        let addressing_mode: addressingmodes::AddressingMode = ADDRESSING_MODES[opcode as usize];
        let instruction = INSTRUCTIONS[opcode as usize];
        0
    }

    fn get_address(&mut self, memory: &CpuMemory, mode: AddressingMode) -> Option<u16> {
        match mode {
            AddressingMode::Absolute => {
                let low = memory.read(self.program_counter + 1);
                let high = memory.read(self.program_counter + 2);
                let address = u16::from_be_bytes([high, low]);
                return Some(address);
            },
            _ => panic!(),
        }
    }
}
