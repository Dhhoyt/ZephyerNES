mod addressingmodes;
mod instructions;
mod timing;

use addressingmodes::{AddressingMode, ADDRESSING_MODES};
use instructions::{Instruction, INSTRUCTIONS};

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
        match instruction {
            Instruction::ADC =>

        }
        0
    }

    fn get_address(&mut self, memory: &CpuMemory, mode: AddressingMode) -> Option<(u16, bool)> {
        match mode {
            AddressingMode::Absolute | AddressingMode::AbsoluteIndirect => {
                let low = memory.read(self.program_counter + 1);
                let high = memory.read(self.program_counter + 2);
                let address = u16::from_be_bytes([high, low]);
                Some((address, false))
            }
            AddressingMode::AbsoluteX => {
                let low = memory.read(self.program_counter + 1);
                let high = memory.read(self.program_counter + 2);
                let address = u16::from_be_bytes([high, low]) + self.index_x as u16;
                let page_crossed = self.program_counter & 0xFF00 != address & 0xFF00;
                Some((address, page_crossed))
            }
            AddressingMode::AbsoluteY => {
                let low = memory.read(self.program_counter + 1);
                let high = memory.read(self.program_counter + 2);
                let address = u16::from_be_bytes([high, low]) + self.index_y as u16;
                let page_crossed = self.program_counter & 0xFF00 != address & 0xFF00;
                Some((address, page_crossed))
            }
            AddressingMode::ZeroPage => {
                let address = memory.read(self.program_counter + 1);
                Some((address as u16, false))
            }
            AddressingMode::ZeroPageIndexedIndirectX => {
                let address: u8 = memory
                    .read(self.program_counter + 1)
                    .wrapping_add(self.index_x);
                let low = memory.read(address as u16);
                let high = memory.read(address.wrapping_add(1) as u16);
                let address = u16::from_be_bytes([high, low]);
                Some((address, false))
            }
            AddressingMode::ZeroPageX => {
                let address: u8 = memory
                    .read(self.program_counter + 1)
                    .wrapping_add(self.index_x);
                Some(((address as u16), false))
            }
            AddressingMode::ZeroPageY => {
                let address: u8 = memory
                    .read(self.program_counter + 1)
                    .wrapping_add(self.index_y);
                Some(((address as u16), false))
            }
            AddressingMode::ZeroPageIndirectIndexedY => {
                let address: u8 = memory.read(self.program_counter);
                let low = memory.read(address as u16);
                let high = memory.read(address.wrapping_add(1) as u16);
                let address = u16::from_be_bytes([high, low]).wrapping_add(self.index_y as u16);
                Some((address, false))
            }
            _ => todo!("immediate, implied"),
        }
    }
}
