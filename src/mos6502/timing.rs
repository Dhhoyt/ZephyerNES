use super::{addressingmodes::AddressingMode, instructions::Instruction};

fn get_timing(mode: AddressingMode, instruction: Instruction, crossed_page: bool) -> usize {
    match mode {
        AddressingMode::Implied => match instruction {
            Instruction::BRK => 7,
            Instruction::RTI | Instruction::RTS => 6,
            _ => 2,
        },
        AddressingMode::Immediate => 2,
        AddressingMode::ZeroPage => {
            if instruction.rwr() {
                5
            } else {
                3
            }
        }
        AddressingMode::ZeroPageX => {
            if instruction.rwr() {
                6
            } else {
                4
            }
        }
        AddressingMode::ZeroPageY => 4,
        AddressingMode::Absolute => match instruction {
            Instruction::JMP => 3,
            Instruction::JSR => 6,
            _ => {
                if instruction.rwr() {
                    6
                } else {
                    {
                        4
                    }
                }
            }
        },
        AddressingMode::AbsoluteX => {
            if instruction == Instruction::STA {
                5
            } else if instruction.rwr() {
                7
            } else if crossed_page {
                5
            } else {
                4
            }
        }
        AddressingMode::AbsoluteY => {
            if instruction == Instruction::STA {
                5
            } else if crossed_page {
                5
            } else {
                4
            }
        }
        AddressingMode::AbsoluteIndirect => 5,
        AddressingMode::ZeroPageIndexedIndirectX => 6,
        AddressingMode::ZeroPageIndirectIndexedY => {
            if instruction == Instruction::STA {
                6
            } else if crossed_page {
                6
            } else {
                5
            }
        }
        AddressingMode::Relative => {
            if crossed_page {
                3
            } else {
                2
            }
        }
    }
}

impl Instruction {
    fn rwr(&self) -> bool {
        match self {
            Self::ASL | Self::DEC | Self::INC | Self::LSR | Self::ROL | Self::ROR => true,
            _ => false,
        }
    }
}
