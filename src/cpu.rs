
use core::panic;

use crate::{bus::Bus, instructions::{bit_u3_r8, inc_r8, jr_cc_n16, ld_hl_r8, ld_hld_a, ld_n16_a, ld_r16_n16, ld_r8_n8, ld_sp_n16, ldh_c_a, ldh_n16_a, xor_a_r8}};

pub enum Register {
    A, B, C, D, E, H, L, F,
    BC, DE, HL
}

pub struct Registers {
    a: u8,
    f: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
}

pub struct CPU {
    pub register: Registers,
    pc: u16,
    sp: u16,
    pub bus: Bus,
    
}

impl Registers {
    pub fn new() -> Self {
        Self { a: 0, f: 0, b: 0, c: 0, d: 0, e: 0, h: 0, l: 0}
    }

    fn get_bc(&self) -> u16{
        return (self.b as u16) << 8 | self.c as u16;
    }
    fn get_de(&self) -> u16{
        return (self.d as u16) << 8 | self.e as u16;
    }
    fn get_hl(&self) -> u16{
        return (self.h as u16) << 8 | self.l as u16;
    }
    fn set_bc(&mut self, value: u16){
        self.b = (value >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }
    fn set_de(&mut self, value: u16){
        self.d = (value >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }
    fn set_hl(&mut self, value: u16){
        self.h = (value >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }
    pub fn set_flag(&mut self, bit: u8, value: bool){
        if value {
            self.f |= 1 << bit;
        } else {
            self.f &= !(1 << bit);
        }
    }
    pub fn get_flag(&self, bit: u8) -> u8{
        return self.f & (1 << bit);
    }

}

impl CPU {

    pub fn new() -> Self {
        Self {register: Registers::new(), pc: 0, sp: 0, bus: Bus::new()}
    }

    pub fn verify(&self) -> bool{
        return true;
    }

    pub fn boot(&mut self){
        let mut loops: i32 = 0;
        loop {
            let current_instruction = self.fetch_n8(); 

            match current_instruction {
                0x31 => ld_sp_n16(self, current_instruction),
                0xAF => xor_a_r8(self, current_instruction),
                0x21 => ld_r16_n16(self, current_instruction),
                0x32 => ld_hld_a(self, current_instruction),
                0xCB => {
                    let next_instruction = self.fetch_n8();
                    match next_instruction {
                        0x7c => bit_u3_r8(self, next_instruction),
                        _ => panic!()
                    }
                }
                0x20 => jr_cc_n16(self, current_instruction),
                0x0E => ld_r8_n8(self, current_instruction),
                0x3E => ld_r8_n8(self, current_instruction),
                0xE2 => ldh_c_a(self, current_instruction),
                0x0C => inc_r8(self, current_instruction),
                0x77 => ld_hl_r8(self, current_instruction),
                0xE0 => ldh_n16_a(self, current_instruction),
                0x11 => ld_r16_n16(self, current_instruction),

                _ =>  {           
                println!();
                println!("Current loop: {}", loops);
                println!("Current Instruction: {:#x}", current_instruction);
                println!("PC: {:X} SP: {:X}", self.pc, self.sp);
                println!("A: {:X} F: {:X} B: {:X} C: {:X} D: {:X} E: {:X} H: {:X} L: {:X}", self.register.a, self.register.f, self.register.b, self.register.c, self.register.d, self.register.e, self.register.h, self.register.l);
                println!("Flags in binary: {:08b}", self.register.f);
                panic!("Invalid instruction")
                
                }
            }

            loops += 1;
        }
    }

    pub fn fetch_n8(&mut self) -> u8{
        let n8: u8 = self.bus.read(self.pc);
        self.pc = self.pc.wrapping_add(0x1);
        return n8;
    }
    pub fn set_sp(&mut self, value: u16){
        self.sp = value;
    }
    pub fn set_pc(&mut self, value: u16){
        self.pc = value;
    }
    pub fn offset_pc(&mut self, offset: i8){
        self.pc = self.pc.wrapping_add(offset as u16);
    }

    pub fn get_r8(&self, reg: &Register) -> u8{
        match reg {
            Register::A => self.register.a,
            Register::B => self.register.b,
            Register::C => self.register.c,
            Register::D => self.register.d,
            Register::E => self.register.e,
            Register::H => self.register.h,
            Register::L => self.register.l,
            Register::F => self.register.f,
            _ => 0
        }     
    }
    pub fn set_r8(&mut self, reg: &Register, value: u8){
        match reg {
            Register::A => self.register.a = value,
            Register::B => self.register.b = value,
            Register::C => self.register.c = value,
            Register::D => self.register.d = value,
            Register::E => self.register.e = value,
            Register::H => self.register.h = value,
            Register::L => self.register.l = value,
            Register::F => self.register.f = value,
            _ => println!("Invalid register")
   
        }
    }
    pub fn get_r16(&self, reg: &Register) -> u16{
        match reg {
            Register::BC => self.register.get_bc(),
            Register::DE => self.register.get_de(),
            Register::HL => self.register.get_hl(),
            _ => 0

        }
    }
    pub fn set_r16(&mut self, reg: &Register, value: u16){
        match reg {
            Register::BC => self.register.set_bc(value),
            Register::DE => self.register.set_de(value),
            Register::HL => self.register.set_hl(value),
            _ => println!("Invalid register")

        }
    }

    pub fn decode_register(&self, index: u8) -> Register{
        match index {
            0b000 => Register::B,
            0b001 => Register::C,
            0b010 => Register::D,
            0b011 => Register::E,
            0b100 => Register::H,
            0b101 => Register::L,
            0b110 => Register::HL,
            0b111 => Register::A, 
            _ => todo!() 

        }
    }


}
