use core::panic;
use colored::Colorize;

use crate::cpu::CPU;
use crate::registers::{Register,Flag};

pub fn execute_instruction(cpu: &mut CPU, opcode: u8) {

    let pc_of_ins = cpu.get_pc() - 1;

    match opcode {

        0x31 => ld_sp_n16(cpu, opcode), // LD SP n16
        0xAF => xor_a_r8(cpu, opcode), // XOR A, A
        0x21 => ld_r16_n16(cpu, opcode), // LD HL, n16
        0x32 => ld_hld_a(cpu, opcode), // LD (HL-), A
        0x20 => jr_cc_n16(cpu, opcode), // JR NZ, e8
        0x0E => ld_r8_n8(cpu, opcode), // LD C, n8
        0x3E => ld_r8_n8(cpu, opcode), // LD A, n8
        0xE2 => ldh_c_a(cpu, opcode), // LDH (C), A
        0x0C => inc_r8(cpu, opcode), // INC C
        0x77 => ld_hl_r8(cpu, opcode), // LD (HL), A
        0xE0 => ldh_n16_a(cpu, opcode), // LDH (a8), A
        0x11 => ld_r16_n16(cpu, opcode), // LD DE, n16
        0x1A => ld_a_r16(cpu, opcode), // LD A, (r16)
        0xCD => call(cpu, opcode), // CALL n16
        0x4F => ld_r8_r8(cpu, opcode), // LD C, A
        0x06 => ld_r8_n8(cpu, opcode), // LD B, n8
        0xC5 => push_r16(cpu, opcode), // PUSH BC
        0x17 => rla(cpu, opcode), // RLA
        0xC1 => pop_r16(cpu, opcode), // POP BC
        0x05 => dec_r8(cpu, opcode), // DEC B
        0x22 => ld_hli_a(cpu, opcode), // LD (HL+) A
        0x23 => inc_r16(cpu, opcode), // INC HL
        0xC9 => ret(cpu, opcode), // RET
        0x13 => inc_r16(cpu, opcode), // INC DE
        0x7B => ld_r8_r8(cpu, opcode), // LD A E
        0xFE => cp_a_n8(cpu, opcode), // CP A n8
        0xEA => ld_n16_a(cpu, opcode), // LD a16 A
        0x3D => dec_r8(cpu, opcode), // DEC A
        0x28 => jr_cc_n16(cpu, opcode), // JR Z, e8
        0x67 => ld_r8_r8(cpu, opcode), // LD H A
        0x57 => ld_r8_r8(cpu, opcode), // LD D A
        0x04 => inc_r8(cpu, opcode), // INC B
        0x1E => ld_r8_n8(cpu, opcode), // LD E n8
        0xF0 => ldh_a_n16(cpu, opcode), // LDH A a8
        0x0D => dec_r8(cpu, opcode), // DEC C
        0x2E => ld_r8_n8(cpu, opcode), // LD L n8
        0x18 => jr_n16(cpu, opcode), // JR n16


        
    

        0xCB => {

            let next = cpu.fetch_n8();

            match next {
                0x7C => bit_u3_r8(cpu, next), // BIT 7 H
                0x11 => rl_r8(cpu, next), // RL C
                _ => panic!("Unknown CB opcode: 0x{:02X} at 0x{:02X}", opcode, pc_of_ins), // Handle unknown opcode
            }


        }
        _ => {

            cpu.debug_state(pc_of_ins, opcode, true);
        }
    }
}
    

// Load instructions
fn ld_r8_r8(cpu: &mut CPU, opcode: u8){
    // Copy (aka Load) the value in register on the right into the register on the left.
    // Cycles: 1 -- Bytes: 1 -- Flags: None
    let src: u8 = opcode & 0b0000_0111;
    let des: u8 = (opcode >> 3) & 0b0000_0111;

    let src_register: Register = cpu.register.decode_register_8(src);
    let des_register: Register = cpu.register.decode_register_8(des);

    let value = cpu.register.get_8(&src_register);
    cpu.register.set_8(&des_register, value);
}
fn ld_r8_n8(cpu: &mut CPU, opcode: u8) {
    // Copy the value n8 into register r8.
    // Cycles: 2 -- Bytes: 2 -- Flags: None

    let n8 = cpu.fetch_n8();
    let des: u8 = (opcode >> 3) & 0b0000_0111;

    let des_register: Register = cpu.register.decode_register_8(des);

    cpu.register.set_8(&des_register,n8);

}
fn ld_r16_n16(cpu: &mut CPU, opcode: u8){
    // Copy the value n16 into register r16.
    // Cycles: 3 -- Bytes: 3 -- Flags: None

    let des: u8 = (opcode >> 3) & 0b0000_0111;

    let register = cpu.register.decode_register_16(des);

    let low_byte: u8 = cpu.fetch_n8();
    let high_byte: u8 = cpu.fetch_n8();

    let n16: u16 = (high_byte as u16) << 8 | low_byte as u16;

    cpu.register.set_16(&register, n16);
}
fn ld_hl_r8(cpu: &mut CPU, opcode: u8){
    // Copy the value in register r8 into the byte pointed to by HL.
    // Cycles: 3 -- Bytes: 2 -- Flags: None

    let src: u8 = (opcode ) & 0b0000_0111;

    let src_register: Register = cpu.register.decode_register_8(src);

    cpu.bus.write(cpu.register.get_16(&Register::HL), cpu.register.get_8(&src_register));
}
fn ld_hl_n8(cpu: &mut CPU, opcode: u8){
    // Copy the value n8 into the byte pointed to by HL.
    // Cycles: 3 -- Bytes: 2 -- Flags: None

    let n8: u8 = cpu.fetch_n8();

    cpu.bus.write(cpu.register.get_16(&Register::HL), n8);
}
fn ld_r8_hl(cpu: &mut CPU, opcode: u8){
    // Copy the value pointed to by HL into register r8.
    // Cycles: 2 -- Bytes: 1 -- Flags: None

    let des: u8 = (opcode >> 3) & 0b0000_0111;

    let des_register: Register = cpu.register.decode_register_8(des);

    let value: u8 = cpu.bus.read(cpu.register.get_16(&Register::HL));

    cpu.register.set_8(&des_register, value);
}
fn ld_r16_a(cpu: &mut CPU, opcode: u8){
    // Copy the value in register A into the byte pointed to by r16.
    // Cycles: 2 -- Bytes: 1 -- Flags: None
    let des: u8 = (opcode >> 3) & 0b0000_0111;

    let des_register= cpu.register.decode_register_16(des);

    let addr = cpu.register.get_16(&des_register);
    let data = cpu.register.get_8(&Register::A);
    cpu.bus.write(addr, data);
}
fn ld_n16_a(cpu: &mut CPU, opcode: u8){
    // Copy the value in register A into the byte at address n16.
    // Cycles: 4 -- Bytes: 3 -- Flags: None
    
    let low_byte: u8 = cpu.fetch_n8();
    let high_byte: u8 = cpu.fetch_n8();

    let addr: u16 = (high_byte as u16) << 8 | low_byte as u16;
    let data: u8 = cpu.register.get_8(&Register::A);

    cpu.bus.write(addr, data);
}
fn ldh_n16_a(cpu: &mut CPU, opcode: u8){
    // Copy the value in register A into the byte at address n16, provided the address is between $FF00 and $FFFF.
    // Cycles: 3 -- Bytes: 2 -- Flags: None

    let byte: u8 = cpu.fetch_n8();

    let addr = 0xFF00 + byte as u16;
    let data = cpu.register.get_8(&Register::A);
    
    cpu.bus.write(addr, data);
}
fn ldh_c_a(cpu: &mut CPU, opcode: u8){
    // Copy the value in register A into the byte at address $FF00+C.
    // Cycles: 2 -- Bytes: 1 -- Flags: None

    let addr: u16 = 0xFF00 + (cpu.register.get_8(&Register::C) as u16);
    let data = cpu.register.get_8(&Register::A);

    cpu.bus.write(addr, data);
}
fn ld_a_r16(cpu: &mut CPU, opcode: u8){
    // Copy the byte pointed to by r16 into register A.
    // Cycles: 2 -- Bytes: 1 -- Flags: None

    let src: u8 = (opcode >> 3) & 0b0000_0111;

    let src_register= cpu.register.decode_register_16(src);

    let value: u8 = cpu.bus.read(cpu.register.get_16(&src_register));
    cpu.register.set_8(&Register::A, value);
}
fn ld_a_n16(cpu: &mut CPU, opcode: u8){
    // Copy the byte at address n16 into register A.
    // Cycles: 4 -- Bytes: 3 -- Flags: None

    let high_byte: u8 = cpu.fetch_n8();
    let low_byte: u8 = cpu.fetch_n8();

    let addr: u16 = (high_byte as u16) << 8 | low_byte as u16;
    let value: u8 = cpu.bus.read(addr);

    cpu.register.set_8(&Register::A, value);
}
fn ldh_a_n16(cpu: &mut CPU, opcode: u8){
    // Copy the byte at address n16 into register A.
    // Cycles: 3 -- Bytes: 2 -- Flags: None

    let byte: u8 = cpu.fetch_n8();

    let value: u8 = cpu.bus.read(0xFF00 + (byte as u16));

    cpu.register.set_8(&Register::A, value);
}
fn ldh_a_c(cpu: &mut CPU, opcode: u8){
    // Copy the byte at address $FF00+C into register A.
    // Cycles: 2 -- Bytes: 1 -- Flags: None

    let value: u8 = cpu.bus.read(0xFF00 + (cpu.register.get_8(&Register::C) as u16));
    cpu.register.set_8(&Register::A, value);
}
fn ld_hli_a(cpu: &mut CPU, opcode: u8){
    // Copy the value in register A into the byte pointed by HL and increment HL afterwards.
    // Cycles: 2 -- Bytes: 1 -- Flags: None

    let hl: u16 = cpu.register.get_16(&Register::HL);
    let data: u8 = cpu.register.get_8(&Register::A);
    cpu.bus.write(hl, data);
    cpu.register.set_16(&Register::HL, hl + 1);
}
fn ld_hld_a(cpu: &mut CPU, opcode: u8){
    // Copy the value in register A into the byte pointed by HL and decrement HL afterwards.
    // Cycles: 2 -- Bytes: 1 -- Flags: None

    let hl: u16 = cpu.register.get_16(&Register::HL);
    let data: u8 = cpu.register.get_8(&Register::A);
    cpu.bus.write(hl, data);
    cpu.register.set_16(&Register::HL, hl - 1);
}
fn ld_a_hld(cpu: &mut CPU, opcode: u8){
    //Copy the byte pointed to by HL into register A, and decrement HL afterwards.
    // Cycles: 2 -- Bytes: 1 -- Flags: None

    let hl: u16 = cpu.register.get_16(&Register::HL);
    let value: u8 = cpu.bus.read(hl);

    cpu.register.set_8(&Register::A, value);
    cpu.register.set_16(&Register::HL, hl - 1);
}
fn ld_a_hdi(cpu: &mut CPU, opcode: u8){
    // Copy the byte pointed to by HL into register A, and increment HL afterwards.
    // Cycles: 2 -- Bytes: 1 -- Flags: None
    let hl: u16 = cpu.register.get_16(&Register::HL);
    let value: u8 = cpu.bus.read(hl);

    cpu.register.set_8(&Register::A, value);
    cpu.register.set_16(&Register::HL, hl + 1); 
}
fn ld_sp_n16(cpu: &mut CPU, opcode: u8){  
    // Copy the value n16 into register SP.
    // Cycles: 3 -- Bytes: 3 -- Flags: None
    let low_bytes: u8 = cpu.fetch_n8();
    let high_bytes: u8 = cpu.fetch_n8();

    let n16: u16 = (high_bytes as u16) << 8 | low_bytes as u16;

    cpu.set_sp(n16);
}
fn ld_n16_sp(cpu: &mut CPU, opcode: u8){
    // Copy SP & $FF at address n16 and SP >> 8 at address n16 + 1.
    // Cycles: 5 -- Bytes: 2 -- Flags: None
    todo!();
}
fn ld_hl_sp_add_e8(cpu: &mut CPU, opcode: u8){
    // Add the signed value e8 to SP and copy the result in HL.
    // Cycles: 3 -- Bytes: 2 -- Flags: H set if overflow from bit 3, C set if overflow from bit 7
    todo!();
}
fn ld_sp_hl(cpu: &mut CPU, opcode: u8){
    // Copy register HL into register SP.
    // Cycles: 2 -- Bytes: 1 -- Flags: None

    cpu.set_sp(cpu.register.get_16(&Register::HL));

}

// arthimetc
fn inc_r8(cpu: &mut CPU, opcode: u8){
    // Increment the value in register r8.
    // Cycles: 1 -- Bytes: 1 -- Flags: Z set if result is 0, N 0, H set if overflow from bit 3
    let r8 = (opcode >> 3) & 0b0000_0111;

    let r8_register: Register = cpu.register.decode_register_8(r8);

    let value: u8 = cpu.register.get_8(&r8_register).wrapping_add(1);
    cpu.register.set_8(&r8_register, value);

}
fn inc_r16(cpu: &mut CPU, opcode: u8){
    // Increment the value in register r16 by 1.
    // Cycles: 2 -- Bytes: 1: Flags: None
    let r16 = (opcode >> 3) & 0b0000_0111;

    let r16_register: Register = cpu.register.decode_register_16(r16);

    let value = cpu.register.get_16(&r16_register).wrapping_add(1);
    cpu.register.set_16(&r16_register, value);
}
fn dec_r8(cpu: &mut CPU, opcode: u8){
    // Decrement the value in register r8.
    // Cycles: 1, Bytes: 1, Flags: Z: set if result is 0, N 1, H set if borrow from bit 4
    let r8 = (opcode >> 3) & 0b0000_0111;

    let r8_register: Register = cpu.register.decode_register_8(r8);

    let value: u8 = cpu.register.get_8(&r8_register).wrapping_sub(1);
    cpu.register.set_8(&r8_register, value);
    
    cpu.register.set_flag(&Flag::Z, value == 0);


}
fn sub_r8(cpu: &mut CPU, opcode: u8){
    // Subtract the value in register r8 from the value in register A and store the result in register A.
    // Cycles: 1, Bytes: 1, Flags: Z: set if result is 0, N 1, H set if borrow from bit 4, C set if borrow (ie r8 > A)
    let r8: u8 = opcode & 0b0000_0111;

    let r8_register: Register = cpu.register.decode_register_8(r8);

    let result: u8 = cpu.register.get_8(&Register::A).wrapping_sub(cpu.register.get_8(&r8_register));
    cpu.register.set_8(&Register::A, result);

}
fn cp_a_r8(cpu: &mut CPU, opcode: u8){
    // ComPare the value in A with the value in r8.
    // Cycles: 1 -- Bytes: 1 -- Flags: Z if result is 0, N 1, H if borrow from bit 4, C if r8 > a
    let src = opcode & 0b0000_0111;
    let src_register = cpu.register.decode_register_8(src);

    let a = cpu.register.get_8(&Register::A);
    let r8 = cpu.register.get_8(&src_register);

    let result = a.wrapping_sub(r8);

    cpu.register.set_flag(&Flag::Z, result == 0);
    cpu.register.set_flag(&Flag::N, true);
    cpu.register.set_flag(&Flag::H, (a & 0x0F) < (r8 & 0x0F));
    cpu.register.set_flag(&Flag::C, a < r8);

}
fn cp_a_n8(cpu: &mut CPU, opcode: u8){

    let a = cpu.register.get_8(&Register::A);
    let n8 = cpu.fetch_n8();

    let result = a.wrapping_sub(n8);

    cpu.register.set_flag(&Flag::Z, result == 0);
    cpu.register.set_flag(&Flag::N, true);
    cpu.register.set_flag(&Flag::H, (a & 0x0F) < (n8 & 0x0F));
    cpu.register.set_flag(&Flag::C, a < n8);

}

// jump and subroutine 
fn jr_cc_n16(cpu: &mut CPU, opcode: u8){
    // Jump to the address PC + n16 if the condition specified by CC is met.
    // Cycles: 3 met else 2 -- Bytes: 2 -- Flags None

    let jump_offset: i8 = cpu.fetch_n8() as i8;

    let expected_output: bool = ((opcode >> 3) & 0b0000_0001) == 1; 
    let condition: u8 = (opcode >> 4) & 0b0000_0011; // condition to check

    let condition_index = match condition {
        0b10 => Flag::Z, // Zero flag
        0b11 => Flag::C, // Carry flag
        _=> panic!("Invalid condition"),
    };

    if cpu.register.get_flag(&condition_index) == expected_output { // if z = 0
            cpu.offset_pc(jump_offset);
        }
    }

fn jr_n16(cpu: &mut CPU, opcode: u8){
    // Relative Jump to address n6.
    // Cycles: 3 -- Bytes: 2 -- Flags: None

    let jump_offset: i8 = cpu.fetch_n8() as i8;

    cpu.offset_pc(jump_offset)


}
fn call(cpu: &mut CPU, opcode: u8){
    // Call address n16.
    // Cycles: 3 met else 2 -- Bytes: 3 -- Flags None
    let low_bytes: u8 = cpu.fetch_n8();
    let high_bytes: u8 = cpu.fetch_n8();

    let n16: u16 = (high_bytes as u16) << 8 | low_bytes as u16;

    cpu.push_stack16(cpu.get_pc());


    cpu.set_pc(n16);

}
fn ret(cpu: &mut CPU, opcode: u8) {
    // Return from subroutine.
    // Cycles: 4 -- Bytes: 1 -- Flags: None
    let value =  cpu.pop_stack16();
    cpu.set_pc(value);

}
fn ret_cc(cpu: &mut CPU, opcode: u8) {
    // Return from subroutine if condition cc is met.
    // Cycles 5 if met else 2 -- Bytes: 1 -- Flags: None

    let value = cpu.pop_stack16();

    let expected_output: bool = ((opcode >> 3) & 0b0000_0001) == 1; // expected output of the condition
    let condition: u8 = (opcode >> 4) & 0b0000_0011; // condition to check

    let condition_index = match condition {
        0b10 => Flag::Z, // Zero flag
        0b11 => Flag::C, // Carry flag
        _=> panic!("{:#b}", condition),
    };


    if cpu.register.get_flag(&condition_index) == expected_output {
        cpu.set_pc(value);
    }
}

// Stack manipulatiuon
fn push_r16(cpu: &mut CPU, opcode: u8) {
    // Push register r16 into the stack
    //Cycles: 4 -- Bytes: 1 -- Flags: None

    let r16_index = (opcode >> 3) & 0b0000_0111;

    let register= cpu.register.decode_register_16(r16_index);

    let value: u16 = cpu.register.get_16(&register);

    cpu.push_stack16(value);

}

// bitwise logic
fn xor_a_r8(cpu: &mut CPU, opcode: u8){
    // XOR the value in register r8 with the value in register A and store the result in register A
    // Cycles: 1, Bytes: 1, Flags: Z: set if result is 0, N 0, H 0, C 0
    let r8: u8 = opcode & 0b0000_0111;

    let r8_register: Register = cpu.register.decode_register_8(r8);

    let result: u8 = cpu.register.get_8(&Register::A) ^ cpu.register.get_8(&r8_register);

    cpu.register.set_8(&Register::A, result);
}

// but shift
fn bit_u3_r8(cpu: &mut CPU, opcode: u8){
    // Test the value in register r8 against the bit specified by u3.
    // Cycles: 2 -- Bytes: 2, Flags: Z if selected bit is 0, N 0, H, 1
    let u3: u8 = (opcode >> 3) & 0b0000_0111;
    let r8: u8 = opcode & 0b0000_0111;

    let r8_register: Register = cpu.register.decode_register_8(r8);

    let value: u8 = cpu.register.get_8(&r8_register);

    let result: u8 = value & (1 << u3);
    
    cpu.register.set_flag(&Flag::Z, result == 0);
}
fn rla(cpu: &mut CPU, opcode: u8) {
    // Rotate register A left, through the carry flag.
    // Cycles: 2 -- Bytes: 1 -- Flags: Z 0, N 0, H 0, C set according to result

    let mut value: u8 = cpu.register.get_8(&Register::A);

    let carry_flag: bool = cpu.register.get_flag(&Flag::C);
    let leaving_bit: u8 = value & (1 << 7);

    value = value << 1;
    cpu.register.set_flag(&Flag::C, leaving_bit != 0);
    value |= carry_flag as u8;
    cpu.register.set_8(&Register::A, value);

    cpu.register.set_flag(&Flag::Z, false);
    cpu.register.set_flag(&Flag::N, false);
    cpu.register.set_flag(&Flag::H, false);



}
fn rl_r8(cpu: &mut CPU, opcode: u8){
    // Rotate bits in register r8 left, through the carry flag.
    // Cycles: 2 -- Bytes: 2, Flags: Z set if result is 0, N 0, H 0, C Set according to result

    let r8_index = (opcode) & 0b0000_0111;
    let r8_register: Register = cpu.register.decode_register_8(r8_index);

    let mut value: u8 = cpu.register.get_8(&r8_register);

    let carry_flag: bool = cpu.register.get_flag(&Flag::C);
    let leaving_bit: u8 = value & (1 << 7);

    value = value << 1;
    cpu.register.set_flag(&Flag::C, leaving_bit != 0);

    value |= carry_flag as u8;

    cpu.register.set_8(&r8_register, value);
    cpu.register.set_flag(&Flag::Z, leaving_bit == 0);
    cpu.register.set_flag(&Flag::N, false);
    cpu.register.set_flag(&Flag::H, false);
}

// stack
fn pop_r16(cpu: &mut CPU, opcode: u8) {
    // Pop register r16 from the stack. This is roughly equivalent to the following imaginary instructions:
    // Cycles: 3 -- Bytes: 1 -- Flags: None

    let r16_index = (opcode >> 3) & 0b0000_0111;

    let register= cpu.register.decode_register_16(r16_index);

    let value = cpu.pop_stack16();

    cpu.register.set_16(&register, value);


}