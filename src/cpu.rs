use crate::bus::Bus;

pub enum RegEnum {
    A, B, C, D, E, H, L
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
}

impl CPU {

    pub fn new() -> Self {
        Self {register: Registers::new(), pc: 0, sp: 0, bus: Bus::new()}
    }

    pub fn verify(&self) -> bool{
        return true;
    }

    pub fn boot(&mut self){
        loop {
            let current_instruction = self.bus.read(self.pc);
        
            self.pc += 0x1;

            match current_instruction {
                0x31 => self.ld_sp_n16(),
                0xAF => self.xor_a(),
                0x21 => self.ld_hl_n16(),
                0x32 => self.ld_hld_a(),
                0xCB => { 
                    let cb_instruction = self.bus.read(self.pc);
                    self.pc += 1;
                    match cb_instruction {
                        0x7C => self.bit_7_h(),
                        _ => break,
                    }
                },

            
                0x20 => self.jr_nz_i8(),
                _ => break
            }

            println!();
            println!("Current Instruction: {:#x}", current_instruction);
            println!("PC: {:X} SP: {:X}", self.pc, self.sp);
            println!("A: {:X} F: {:X} B: {:X} C: {:X} D: {:X} E: {:X} H: {:X} L: {:X}", self.register.a, self.register.f, self.register.b, self.register.c, self.register.d, self.register.e, self.register.h, self.register.l);
            println!();


        }
    }

    pub fn get_registers(&self, reg: &RegEnum) -> u8{
        match reg {
            RegEnum::A => self.register.a,
            RegEnum::B => self.register.b,
            RegEnum::C => self.register.c,
            RegEnum::D => self.register.d,
            RegEnum::E => self.register.e,
            RegEnum::H => self.register.h,
            RegEnum::L => self.register.l
        }     
    }

    pub fn set_register(&mut self, reg: &RegEnum, value: u8){
        match reg {
            RegEnum::A => self.register.a = value,
            RegEnum::B => self.register.b = value,
            RegEnum::C => self.register.c = value,
            RegEnum::D => self.register.d = value,
            RegEnum::E => self.register.e = value,
            RegEnum::H => self.register.h = value,
            RegEnum::L => self.register.l = value,
        }
    }

    fn ld_sp_n16(&mut self){
        let low_bytes: u8 = self.bus.read(self.pc);
        self.pc += 1;
        let high_bytes: u8 = self.bus.read(self.pc);
        self.pc += 1;

        let n16: u16 = (high_bytes as u16) << 8 | low_bytes as u16;

        self.sp = n16;
    }
    fn xor_a(&mut self) {
        self.register.a ^= self.register.a;
        self.register.f = 1 << 7; // 
    }
    
    fn ld_hl_n16(&mut self){
        let low_bytes: u8 = self.bus.read(self.pc);
        self.pc += 1;
        let high_bytes: u8 = self.bus.read(self.pc);
        self.pc += 1;

        self.register.h = high_bytes;
        self.register.l = low_bytes;
    }
    fn ld_hld_a(&mut self){
        self.bus.write(self.register.get_hl(),self.register.a);
        let new_hl: u16 = self.register.get_hl() - 1;
        self.register.set_hl(new_hl);
    }
    fn bit_7_h(&mut self) {
        let bit: u8 = (self.register.h >> 7) & 1; 
        if bit == 1 {
            self.register.f &= !(1 << 7); 
        } else if bit == 0 {
            self.register.f |= 1 << 7; 
        }

    }
    
    fn jr_nz_i8(&mut self) {
        let offset = self.bus.read(self.pc) as i8;
        self.pc += 1;
        if (self.register.f & (1 << 7)) == 0 { 
            self.pc = self.pc.wrapping_add(offset as i16 as u16); 
        }
    }

    

}
