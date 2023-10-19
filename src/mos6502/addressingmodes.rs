/* http://www.emulator101.com/6502-addressing-modes.html
* The second byte is the least-sig byte of the address.
* The thrid byte is the most-sig byte of the address.
*
* Absolute: The next two bytes of the instruction form the memory address.
*
* Indirect: The data you are retreiving is a pointer.
* In cases like the jmp instruction, the PC is set to the pointer,
* in other cases the opperand could be the memory at that pointer.
* For example, if memory at address $1000 was 52 3a and JMP  ($1000) was
* executed, the value of PC would be 3a52
*
* Zero Page: Refers to the first 256 bytes of memory. This exists
* because the program counter is split into two 8-bit registers.
* The high register will determine which block of 256 bytes to address.
* Each of these blocks are called pages.
* So only activating the low register will cause the first 256 bytes to be
* addressed. Because the high register isn't activated, zero page address will
* wrap when values are added to it.
*/

#[derive(Clone, Copy)]
pub enum AddressingMode {
    Absolute,                 // Second and third byte form the address of the opperand.
    AbsoluteX, // Second and third bytes of instruction form an address to which the X register is added. The sum is the address of the operand
    AbsoluteY, // Second and third bytes of instruction form an address to which the Y register is added. The sum is the address of the operand
    AbsoluteIndirect, // Second and third bytes of instruction form an address to a pointer. Program Counter is set to this pointer
    Immediate,        // Operand is the second byte
    Implied,          // A single bye instruction, no operands
    Relative, // An offset in the second byte of the instruction is added to the program counter if the branch statement is true.
    ZeroPage, // Addresses the zero page with the second byte of the instruction. Essentially the Absolute addressing equivalent.
    ZeroPageIndexedIndirectX, // The X register is added to the second byte to form an address of a pointer. Operand is the data at this pointer
    ZeroPageX, // The X Register is added to the second byte to form the address of the operand.
    ZeroPageY, // The Y Register is added to the second byte to form the address of the operand.
    ZeroPageIndirectIndexedY, // Retreives an address from the zero page. The Y address is then added to this address. The resulting address is a pointer to the operand.
}

pub const ADDRESSING_MODES: [AddressingMode; 256] = {
    let mut res = [AddressingMode::Absolute; 256];
    // For loops not allowed in a const expression as of rustc 1.71.1
    let mut i = 0;
    while i < 256 {
        match i & 0b00000011 {
            0b01 => res[i] = group_one(i as u8),
            0b10 => res[i] = group_two(i as u8),
            0b00 => res[i] = group_three(i as u8),
            0b11 => res[i] = group_four(i as u8),
            _ => (),
        };
        i += 1;
    }
    res
};

// Next four functions have been constructed from the info in this website https://llx.com/Neil/a2/opcodes.html
const fn group_one(opcode: u8) -> AddressingMode {
    match (opcode & 0b00011100) >> 2 {
        0b000 => AddressingMode::ZeroPageIndexedIndirectX,
        0b001 => AddressingMode::ZeroPage,
        0b010 => AddressingMode::Immediate,
        0b011 => AddressingMode::Absolute,
        0b100 => AddressingMode::ZeroPageIndirectIndexedY,
        0b101 => AddressingMode::ZeroPageX,
        0b110 => AddressingMode::AbsoluteY,
        0b111 => AddressingMode::AbsoluteX,
        _ => panic!(),
    }
}

const fn group_two(opcode: u8) -> AddressingMode {
    match (opcode & 0b00011100) >> 2 {
        0b000 => AddressingMode::Immediate,
        0b001 => AddressingMode::ZeroPage,
        0b010 => AddressingMode::Implied,
        0b011 => AddressingMode::Absolute,
        0b100 => AddressingMode::Implied, // Every instruction here is JAM
        0b101 => {
            if opcode == 0x96 || opcode == 0xB6 {
                AddressingMode::ZeroPageY
            } else {
                AddressingMode::ZeroPageX
            }
        }
        0b110 => AddressingMode::Implied,
        0b111 => {
            if opcode == 0x9E || opcode == 0xBE {
                AddressingMode::AbsoluteY
            } else {
                AddressingMode::AbsoluteX
            }
        }
        _ => panic!(),
    }
}

const fn group_three(opcode: u8) -> AddressingMode {
    match (opcode & 0b00011100) >> 2 {
        0b000 => match opcode {
            0x00 | 0x40 | 0x60 => AddressingMode::Implied,
            0x20 => AddressingMode::Absolute,
            0x80 | 0xA0 | 0xC0 | 0xE0 => AddressingMode::Immediate,
            _ => panic!(),
        },
        0b001 => AddressingMode::ZeroPage,
        0b010 => AddressingMode::Implied,
        0b011 => {
            if opcode == 0x6C {
                AddressingMode::AbsoluteIndirect
            } else {
                AddressingMode::Absolute
            }
        }
        0b100 => AddressingMode::Relative,
        0b101 => AddressingMode::ZeroPageX,
        0b110 => AddressingMode::Implied,
        0b111 => AddressingMode::AbsoluteX,
        _ => panic!(),
    }
}

const fn group_four(opcode: u8) -> AddressingMode {
    match (opcode & 0b00011100) >> 2 {
        0b000 => AddressingMode::ZeroPageIndexedIndirectX,
        0b001 => AddressingMode::ZeroPage,
        0b010 => AddressingMode::Immediate,
        0b011 => AddressingMode::Absolute,
        0b100 => AddressingMode::ZeroPageIndirectIndexedY,
        0b101 => {
            if opcode == 0x97 || opcode == 0xB7 {
                AddressingMode::ZeroPageY
            } else {
                AddressingMode::ZeroPageX
            }
        }
        0b110 => AddressingMode::AbsoluteY,
        0b111 => {
            if opcode == 0x9F || opcode == 0xBF {
                AddressingMode::AbsoluteY
            } else {
                AddressingMode::AbsoluteX
            }
        }
        _ => panic!(),
    }
}
