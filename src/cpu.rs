pub enum RegEnum {
    A, B, C, D, E, H, L
}

pub struct Registers {
    a: u8,
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
}

impl Registers {
    pub fn new() -> Self {
        Self { a: 0, b: 0, c: 0, d: 0, e: 0, h: 0, l: 0}
    }
}

impl CPU {

    pub fn new() -> Self {
        Self {register: Registers::new(), pc: 0, sp: 0}
    }

    pub fn verify(&self) -> bool{
        return true;
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


    // Instructions
    pub fn ld_rx_ry(&mut self, des: &RegEnum, src: &RegEnum){
        let value: u8 = self.get_registers(&src);
        self.set_register(des, value);
    }

    pub fn add_rx_ty(&mut self, rx: &RegEnum, ry: &RegEnum){
        let value: u8 = self.get_registers(&rx) + self.get_registers(&ry);
        self.set_register(&rx, value);
    }

    pub fn sub_rx_ty(&mut self, rx: &RegEnum, ry: &RegEnum){
        let value: u8 = self.get_registers(&rx) - self.get_registers(&ry);
        self.set_register(&rx, value);
    }
    
    pub fn inc_rx(&mut self, reg: &RegEnum){
        let value: u8 = self.get_registers(&reg) + 1;
        self.set_register(&reg, value);
    }

    pub fn dec_rx(&mut self, reg: &RegEnum){
        let value: u8 = self.get_registers(&reg) - 1;
        self.set_register(&reg, value);
    }
    
}
