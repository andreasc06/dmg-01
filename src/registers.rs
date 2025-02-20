#[derive(Debug)]
pub enum Register {
    A, B, C, D, E, H, L, F,
    BC, DE, HL,
}
pub enum Flag {
    Z, N, H, C
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

impl Registers {
    pub fn new() -> Self {
        Self { a: 0, f: 0, b: 0, c: 0, d: 0, e: 0, h: 0, l: 0}
    }

    pub fn get_8(&self, reg: &Register) -> u8 {
        match reg {
            Register::A => self.a,
            Register::F => self.f,
            Register::B => self.b,
            Register::C => self.c,
            Register::D => self.d,
            Register::E => self.e,
            Register::H => self.h,
            Register::L => self.l,
            _ => panic!("{:?}", reg)
        }
    }
    pub fn set_8(&mut self, reg: &Register, value: u8) {
        match reg {
            Register::A => self.a = value,
            Register::F => self.f = value,
            Register::B => self.b = value,
            Register::C => self.c = value,
            Register::D => self.d = value,
            Register::E => self.e = value,
            Register::H => self.h = value,
            Register::L => self.l = value,
            _ => panic!("{:?}", reg)
        }

    }

    pub fn get_16(&self, reg: &Register) -> u16{
        match reg {
            Register::BC => (self.b as u16) << 8 | self.c as u16,
            Register::DE => (self.d as u16) << 8 | self.e as u16,
            Register::HL => (self.h as u16) << 8 | self.l as u16,
            _ => panic!("16-bit register can't be read")
        }
    }
    pub fn set_16(&mut self, reg: &Register, value: u16){
        match reg {
            Register::BC =>{
                self.b = (value >> 8) as u8;
                self.c = (value & 0xFF) as u8;
            },
            Register::DE =>{
                self.d = (value >> 8) as u8;
                self.e = (value & 0xFF) as u8;
            },
            Register::HL =>{
                self.h = (value >> 8) as u8;
                self.l = (value & 0xFF) as u8;
            },
            _ => panic!("16-bit register can't be read")
        }
    }
    pub fn set_flag(&mut self, flag: &Flag, value: bool) {
        let bit = match flag {
            Flag::C => 4,
            Flag::H => 5,
            Flag::N => 6,
            Flag::Z => 7,
        };
    
        if value {
            self.f |= 1 << bit; 
        } else {
            self.f &= !(1 << bit); 
        }
    }

    pub fn get_flag(&self, flag: &Flag) -> bool{
        let bit = match flag {
            Flag::C => 4,
            Flag::H => 5,
            Flag::N => 6,
            Flag::Z => 7,
        };
        (self.f & (1 << bit)) != 0
    }

    
    pub fn decode_register_8(&self, index: u8) -> Register{
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
    pub fn decode_register_16(&self, index: u8) -> Register{
        match index {
            0b000 => Register::BC,
            0b001 => Register::BC,
            0b010 => Register::DE,
            0b011 => Register::DE,
            0b100 => Register::HL,
            0b101 => Register::HL,
            _ => todo!() 

        }
    }


}
