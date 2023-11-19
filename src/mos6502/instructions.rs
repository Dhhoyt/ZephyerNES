use crate::cpu_memory::CpuMemory;

impl super::Mos6502 {
    pub fn adc(&mut self, memory: &mut CpuMemory) {
        if self.decimal_mode {
            return;
        }
    }
}