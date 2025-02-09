use crate::cpu::{Register, CPU};

struct Instructions {
    pub opcode: u8,
    pub name: &'static str,
    pub execute: fn(&mut CPU, u8)
}


pub fn ld_r8_r8(cpu: &mut CPU, opcode: u8){
    // Copy (aka Load) the value in register on the right into the register on the left.
    let src: u8 = opcode & 0b0000_0111;
    let des: u8 = (opcode >> 3) & 0b0000_0111;

    let src_register: Register = cpu.decode_register(src);
    let des_register: Register = cpu.decode_register(des);

    cpu.set_r8(&des_register, cpu.get_r8(&src_register));
}

pub fn ld_r8_n8(cpu: &mut CPU, opcode: u8) {
    // Copy the value n8 into register r8.

    let n8 = cpu.fetch_n8();
    let register_byte: u8 = (opcode >> 3) & 0b0000_0111;

    let register: Register = cpu.decode_register(register_byte);

    cpu.set_r8(&register,n8);

}

pub fn ld_r16_n16(cpu: &mut CPU, opcode: u8){
    // Copy the value n16 into register r16.

    let des: u8 = (opcode >> 3) & 0b0000_0111;

    let register = match cpu.decode_register(des) {
        Register::B => Register::BC,
        Register::D => Register::DE,
        Register::H => Register::HL,
        _ => panic!(),
    };

    let low_bytes: u8 = cpu.fetch_n8();
    let high_bytes: u8 = cpu.fetch_n8();

    let n16: u16 = (high_bytes as u16) << 8 | low_bytes as u16;

    cpu.set_r16(&register, n16);
}

pub fn ld_hl_r8(cpu: &mut CPU, opcode: u8){
    // Copy the value in register r8 into the byte pointed to by HL.

    let r8: u8 = (opcode >> 3) & 0b0000_0111;

    let r8_register: Register = cpu.decode_register(r8);

    cpu.bus.write(cpu.get_r16(&Register::HL), cpu.get_r8(&r8_register));
}

pub fn ld_hl_n8(cpu: &mut CPU, opcode: u8){
    // Copy the value n8 into the byte pointed to by HL.
    let n8: u8 = cpu.fetch_n8();

    cpu.bus.write(cpu.get_r16(&Register::HL), n8);
}

pub fn ld_r8_hl(cpu: &mut CPU, opcode: u8){
    // Copy the value pointed to by HL into register r8.

    let r8: u8 = (opcode >> 3) & 0b0000_0111;

    let r8_register: Register = cpu.decode_register(r8);

    let value: u8 = cpu.bus.read(cpu.get_r16(&Register::HL));

    cpu.set_r8(&r8_register, value);
}

pub fn ld_r16_a(cpu: &mut CPU, opcode: u8){
    // Copy the value in register A into the byte pointed to by r16.
    let r16: u8 = (opcode >> 3) & 0b0000_0111;

    let register = match cpu.decode_register(r16) {
        Register::B => Register::BC,
        Register::D => Register::DE,
        Register::H => Register::HL,
        _ => panic!(),
    };

    cpu.bus.write(cpu.get_r16(&register), cpu.get_r8(&Register::A));
}

pub fn ld_n16_a(cpu: &mut CPU, opcode: u8){
    // Copy the value in register A into the byte at address n16.
    
    let low_bytes: u8 = cpu.fetch_n8();
    let high_bytes: u8 = cpu.fetch_n8();

    let n16: u16 = (high_bytes as u16) << 8 | low_bytes as u16;

    cpu.bus.write(n16, cpu.get_r8(&Register::A));
}

pub fn ldh_n16_a(cpu: &mut CPU, opcode: u8){
    // Copy the value in register A into the byte at address n16, provided the address is between $FF00 and $FFFF.
    let byte: u8 = cpu.fetch_n8();
    
    cpu.bus.write(0xFF00 + byte as u16, cpu.get_r8(&Register::A));  
}

pub fn ldh_c_a(cpu: &mut CPU, opcode: u8){
    // Copy the value in register A into the byte at address $FF00+C.

    let adr: u16 = 0xFF00 + (cpu.get_r8(&Register::C) as u16);

    cpu.bus.write(adr, cpu.get_r8(&Register::A));
}

pub fn ld_a_r16(cpu: &mut CPU, opcode: u8){
    // Copy the byte pointed to by r16 into register A.

    let r16: u8 = (opcode >> 3) & 0b0000_0111;

    let register = match cpu.decode_register(r16) {
        Register::B => Register::BC,
        Register::D => Register::DE,
        Register::H => Register::HL,
        _ => panic!(),
    };

    let value: u8 = cpu.bus.read(cpu.get_r16(&register));
    cpu.set_r8(&Register::A, value);
}

pub fn ld_a_n16(cpu: &mut CPU, opcode: u8){
    // Copy the byte at address n16 into register A.

    let byte: u8 = cpu.fetch_n8();

    let value: u8 = cpu.bus.read(0xFF00 + (byte as u16));

    cpu.set_r8(&Register::A, value);
}

pub fn ldh_a_c(cpu: &mut CPU, opcode: u8){
    // Copy the byte at address $FF00+C into register A.

    let value: u8 = cpu.bus.read(0xFF00 +( cpu.get_r8(&Register::C) as u16));
    cpu.set_r8(&Register::A, value);
}

pub fn ld_hli_a(cpu: &mut CPU, opcode: u8){
    // Copy the value in register A into the byte pointed by HL and increment HL afterwards.
    let hl: u16 = cpu.get_r16(&Register::HL);
    let data: u8 = cpu.get_r8(&Register::A);
    cpu.bus.write(hl, data);
    cpu.set_r16(&Register::HL, hl + 1);
}

pub fn ld_hld_a(cpu: &mut CPU, opcode: u8){
    // Copy the value in register A into the byte pointed by HL and decrement HL afterwards.
    let hl: u16 = cpu.get_r16(&Register::HL);
    let data: u8 = cpu.get_r8(&Register::A);
    cpu.bus.write(hl, data);
    cpu.set_r16(&Register::HL, hl - 1);
}

pub fn ld_a_hld(cpu: &mut CPU, opcode: u8){
    //Copy the byte pointed to by HL into register A, and decrement HL afterwards.
    let hl: u16 = cpu.get_r16(&Register::HL);
    let value: u8 = cpu.bus.read(hl);

    cpu.set_r8(&Register::A, value);
    cpu.set_r16(&Register::HL, hl - 1);
}

pub fn ld_a_hdi(cpu: &mut CPU, opcode: u8){
    // Copy the byte pointed to by HL into register A, and increment HL afterwards.
    let hl: u16 = cpu.get_r16(&Register::HL);
    let value: u8 = cpu.bus.read(hl);

    cpu.set_r8(&Register::A, value);
    cpu.set_r16(&Register::HL, hl + 1); 
}

pub fn ld_sp_n16(cpu: &mut CPU, opcode: u8){  
    // Copy the value n16 into register SP.
    let low_bytes: u8 = cpu.fetch_n8();
    let high_bytes: u8 = cpu.fetch_n8();

    let n16: u16 = (high_bytes as u16) << 8 | low_bytes as u16;

    cpu.set_sp(n16);
}

pub fn ld_n16_sp(cpu: &mut CPU, opcode: u8){
    // Copy SP & $FF at address n16 and SP >> 8 at address n16 + 1.
    todo!();
}
pub fn ld_hl_sp_add_e8(cpu: &mut CPU, opcode: u8){
    // Add the signed value e8 to SP and copy the result in HL.
    todo!();
}
pub fn ld_sp_hl(cpu: &mut CPU, opcode: u8){
    // Copy register HL into register SP.
    cpu.set_sp(cpu.get_r16(&Register::HL));

}
