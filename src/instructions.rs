use crate::{Cpu,byte};


// increment x register
pub fn inx(cpu: &mut Cpu) {
    cpu.x.increment();
    cpu.flags.update_zero_and_negative(cpu.x.get());
}

// increment y register
pub fn iny(cpu: &mut Cpu) {
    cpu.y.increment();
    cpu.flags.update_zero_and_negative(cpu.y.get());
}

// increment memory
pub fn inc(cpu: &mut Cpu, addr: u16) {
    let new_value = cpu.memory.read(addr).wrapping_add(1);
    cpu.memory.write(addr, new_value);
    cpu.flags.update_zero_and_negative(new_value);
}

// decrement memory
pub fn dec(cpu: &mut Cpu, addr: u16) {
    let new_value = cpu.memory.read(addr).wrapping_sub(1);
    cpu.memory.write(addr, new_value);
    cpu.flags.update_zero_and_negative(new_value);
}

// decrement x register
pub fn dex(cpu: &mut Cpu) {
    cpu.x.decrement();
    cpu.flags.update_zero_and_negative(cpu.x.get());
}

// decrement y register
pub fn dey(cpu: &mut Cpu) {
    cpu.y.decrement();
    cpu.flags.update_zero_and_negative(cpu.y.get());
}

// add with carry
pub fn adc(cpu: &mut Cpu, val: u8) {
    let old_value = cpu.a.get();
    let result = old_value as u32 + val as u32 + cpu.flags.c as u32;
    let new_value = (result & 0xFF) as u8;
    cpu.a.set(new_value);
    cpu.flags.update_zero_and_negative(new_value);

    // C and V flags are set in case of unsigned and signed overflow respectively
    cpu.flags.c = byte::overflows(result);
    cpu.flags.v = (byte::is_positive(old_value) &&
                    byte::is_positive(val) &&
                    byte::is_negative(new_value)) ||
                    (byte::is_negative(old_value) &&
                    byte::is_negative(val) &&
                    byte::is_positive(new_value));
}

// subtract with carry
pub fn sbc(cpu: &mut Cpu, val: u8) {
    adc(cpu,!val);
}

// arithmetic shift left
pub fn asl(cpu: &mut Cpu, addr: u16) {
    let old_value = cpu.memory.read(addr);
    cpu.flags.c = (old_value >> 7) & 1 != 0;
    let new_value = old_value << 1;
    cpu.flags.update_zero_and_negative(new_value);
    cpu.memory.write(addr,new_value);
}

// arithmetic shift left (accumulator)
pub fn asl_a(cpu: &mut Cpu) {
    let old_value = cpu.a.get();
    cpu.flags.c = (old_value >> 7) & 1 != 0;
    let new_value = old_value << 1;
    cpu.flags.update_zero_and_negative(new_value);
    cpu.a.set(new_value);
}

// logical shift right
pub fn lsr(cpu: &mut Cpu, addr: u16) {
    let old_value = cpu.memory.read(addr);
    cpu.flags.c = old_value & 1 != 0;
    let new_value = old_value >> 1;
    cpu.flags.update_zero_and_negative(new_value);
    cpu.memory.write(addr,new_value);
}

// logical shift right (accumulator)
pub fn lsr_a(cpu: &mut Cpu) {
    let old_value = cpu.a.get();
    cpu.flags.c = old_value & 1 != 0;
    let new_value = old_value >> 1;
    cpu.flags.update_zero_and_negative(new_value);
    cpu.a.set(new_value);
}

// rotate left
pub fn rol(cpu: &mut Cpu, addr: u16) {
    let old_value = cpu.memory.read(addr);
    let bit7 = old_value >> 7 & 1 != 0;
    let shifted_value = old_value << 1;
    let new_value = byte::set_bit(shifted_value,0,cpu.flags.c as u8);
    cpu.memory.write(addr,new_value);
    cpu.flags.c = bit7;
    cpu.flags.update_zero_and_negative(new_value);
}

// rotate left (accumulator)
pub fn rol_a(cpu: &mut Cpu) {
    let old_value = cpu.a.get();
    let bit7 = old_value >> 7 & 1 != 0;
    let shifted_value = old_value << 1;
    let new_value = byte::set_bit(shifted_value,0,cpu.flags.c as u8);
    cpu.a.set(new_value);
    cpu.flags.c = bit7;
    cpu.flags.update_zero_and_negative(new_value);
}

// rotate right
pub fn ror(cpu: &mut Cpu, addr: u16) {
    let old_value = cpu.memory.read(addr);
    let bit0 = old_value & 1 != 0;
    let shifted_value = old_value >> 1;
    let new_value = byte::set_bit(shifted_value,7,cpu.flags.c as u8);
    cpu.memory.write(addr,new_value);
    cpu.flags.c = bit0;
    cpu.flags.update_zero_and_negative(new_value);
}

// rotate right (accumulator)
pub fn ror_a(cpu: &mut Cpu) {
    let old_value = cpu.a.get();
    let bit0 = old_value & 1 != 0;
    let shifted_value = old_value >> 1;
    let new_value = byte::set_bit(shifted_value,7,cpu.flags.c as u8);
    cpu.a.set(new_value);
    cpu.flags.c = bit0;
    cpu.flags.update_zero_and_negative(new_value);
}
