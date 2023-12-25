mod addressingmodes;
mod instruction_table;
mod instructions;
mod timing;

use core::panic;

use addressingmodes::{AddressingMode, ADDRESSING_MODES};
use instruction_table::{Instruction, INSTRUCTIONS};

use crate::cpu_memory::CpuMemory;

struct Mos6502 {
    program_counter: u16,
    accumulator: u8,
    index_x: u8,
    index_y: u8,
    stack_pointer: u8,
    carry: bool,
    zero: bool,
    interrupt_enable: bool,
    decimal_mode: bool,
    interrupt: bool,
    overflow: bool,
    sign: bool,
}

impl Mos6502 {
    // Returns the number of cycles this instruction took
    fn run_instruction(&mut self, memory: &mut CpuMemory) -> usize {
        let opcode = memory.read(self.program_counter);
        let addressing_mode: addressingmodes::AddressingMode = ADDRESSING_MODES[opcode as usize];
        let instruction = INSTRUCTIONS[opcode as usize];
        let (operand, crossed_page) = self.get_operand(memory, addressing_mode);
        match instruction {
            Instruction::ADC => self.adc(memory, operand),
            _ => todo!(),
        }
        0
    }

    fn get_operand(&self, memory: &CpuMemory, mode: AddressingMode) -> (Operand, bool) {
        match mode {
            AddressingMode::Absolute | AddressingMode::AbsoluteIndirect => {
                let low = memory.read(self.program_counter + 1);
                let high = memory.read(self.program_counter + 2);
                let address = u16::from_be_bytes([high, low]);
                (Operand::Address(address), false)
            }
            AddressingMode::AbsoluteX => {
                let low = memory.read(self.program_counter + 1);
                let high = memory.read(self.program_counter + 2);
                let address = u16::from_be_bytes([high, low]) + self.index_x as u16;
                let page_crossed = self.program_counter & 0xFF00 != address & 0xFF00;
                (Operand::Address(address), page_crossed)
            }
            AddressingMode::AbsoluteY => {
                let low = memory.read(self.program_counter + 1);
                let high = memory.read(self.program_counter + 2);
                let address = u16::from_be_bytes([high, low]) + self.index_y as u16;
                let page_crossed = self.program_counter & 0xFF00 != address & 0xFF00;
                (Operand::Address(address), page_crossed)
            }
            AddressingMode::ZeroPage => {
                let address = memory.read(self.program_counter + 1);
                (Operand::Address(address as u16), false)
            }
            AddressingMode::ZeroPageIndexedIndirectX => {
                let address: u8 = memory
                    .read(self.program_counter + 1)
                    .wrapping_add(self.index_x);
                let low = memory.read(address as u16);
                let high = memory.read(address.wrapping_add(1) as u16);
                let address = u16::from_be_bytes([high, low]);
                (Operand::Address(address), false)
            }
            AddressingMode::ZeroPageX => {
                let address: u8 = memory
                    .read(self.program_counter + 1)
                    .wrapping_add(self.index_x);
                (Operand::Address(address as u16), false)
            }
            AddressingMode::ZeroPageY => {
                let address: u8 = memory
                    .read(self.program_counter + 1)
                    .wrapping_add(self.index_y);
                (Operand::Address(address as u16), false)
            }
            AddressingMode::ZeroPageIndirectIndexedY => {
                let address: u8 = memory.read(self.program_counter);
                let low = memory.read(address as u16);
                let high = memory.read(address.wrapping_add(1) as u16);
                let address = u16::from_be_bytes([high, low]).wrapping_add(self.index_y as u16);
                (Operand::Address(address), false)
            }
            AddressingMode::Immediate => {
                let immediate: u8 = memory.read(self.program_counter + 1);
                (Operand::Immediate(immediate), false)
            }
            AddressingMode::Relative => {
                let low = memory.read(self.program_counter + 1);
                let high = memory.read(self.program_counter + 2);
                let offset = u16::from_be_bytes([high, low]);
                (Operand::Offset(offset), false)
            }
            AddressingMode::Implied => (Operand::Implied, false),
        }
    }

    fn move_program_counter(&mut self, mode: AddressingMode) -> usize {
        let step = match mode {
            AddressingMode::Absolute
            | AddressingMode::AbsoluteIndirect
            | AddressingMode::AbsoluteX
            | AddressingMode::AbsoluteY => 3,
            AddressingMode::ZeroPage
            | AddressingMode::ZeroPageX
            | AddressingMode::ZeroPageY
            | AddressingMode::ZeroPageIndirectIndexedY
            | AddressingMode::ZeroPageIndexedIndirectX
            | AddressingMode::Immediate
            | AddressingMode::Relative => 2,
            AddressingMode::Implied => 1,
        };
        let prev_counter = self.program_counter;
        self.program_counter = self.program_counter.wrapping_add(step);
        (self.program_counter >> 8 != prev_counter >> 8) as usize
    }
}

enum Operand {
    Address(u16),
    Immediate(u8),
    Offset(u16),
    Implied,
}

impl Operand {
    pub fn read(&self, memory: &CpuMemory) -> u8 {
        match self {
            Operand::Address(v) => memory.read(*v),
            Operand::Immediate(v) => *v,
            _ => panic!("Tried to read the value of a non-operand value"),
        }
    }
}
