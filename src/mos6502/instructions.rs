use std::ops::Add;

use crate::cpu_memory::CpuMemory;

use super::Operand;

impl super::Mos6502 {
    pub fn adc(&mut self, memory: &mut CpuMemory, operand: Operand) {
        let operand = operand.read(memory);
        if !self.decimal_mode {
            let res = operand.overflowing_add(self.accumulator);
            let carry_in_res = res.0.overflowing_add(self.carry as u8);
            self.carry = res.1 | carry_in_res.1;
            self.accumulator = carry_in_res.0;
        } else {
            let a_low = self.accumulator & 0b00001111;
            let a_high = (self.accumulator & 0b11110000) >> 4;
            let op_low = operand & 0b00001111;
            let op_high = (operand & 0b11110000) >> 4;

            let res_lower = a_low + op_low + self.carry as u8;
            let carry = res_lower > 9;
            let res_lower = res_lower % 10;

            let res_upper = a_high + op_high + carry as u8;
            self.carry = res_upper > 9;
            let res_upper = res_upper % 10;

            self.accumulator = (res_upper << 4) | res_lower;
        }
    }

    pub fn and(&mut self, memory: &mut CpuMemory, operand: Operand) {
        let operand = operand.read(memory);
        self.accumulator = self.accumulator & operand;
    }

    pub fn asl(&mut self) {
        self.accumulator <<= 1;
    }
}
