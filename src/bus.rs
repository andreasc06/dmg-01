
//  0x0000–0x3FFF  | 16KB   | ROM Bank 0 
//  0x4000–0x7FFF  | 16KB   | ROM Bank 1
//  0x8000–0x9FFF  | 8KB    | VRAM
//  0xA000–0xBFFF  | 8KB    | External RAM
//  0xC000–0xCFFF  | 4KB    | Work RAM (WRAM Bank 0)
//  0xD000–0xDFFF  | 4KB    | Work RAM (WRAM Bank 1)
//  0xE000–0xFDFF  | 8KB    | Echo RAM (prohibited, mirror of C000-DDFF)
//  0xFE00–0xFE9F  | 160B   | OAM 
//  0xFEA0–0xFEFF  | 96B    | Unused (prohibited)
//  0xFF00–0xFF7F  | 128B   | I/O Registers
//  0xFF80–0xFFFE  | 127B   | High RAM (HRAM)
//  0xFFFF         | 1B     | Interrupt Enable Register (IE)

pub struct Bus {

    pub boot_rom: [u8; 0xFF + 1],

    pub cartridge: [u8; 0x7FFF + 1],
    pub vram: [u8; 0x1FFF + 1],
    pub wram: [u8; 0x1FFF + 1],
    pub oam: [u8; 0x10 + 1],
    pub io: [u8; 0x80 + 1],
    pub hram: [u8; 0x7F + 1],
    pub ie: u8,
}

impl Bus {
    pub fn new() -> Self {
        Self {
            boot_rom: [0; 0xFF + 1],
            cartridge: [0; 0x7FFF + 1],
            vram: [0; 0x1FFF + 1],
            wram: [0; 0x1FFF + 1],
            oam: [0; 0x10 + 1],
            io: [0; 0x80 + 1],
            hram: [0; 0x7F + 1],
            ie: 0,
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        if addr < 0x8000 { // Cartridge
            if addr < 0x100 && self.io[0xFF50 - 0xFF00] == 0 {
                return self.boot_rom[addr as usize];
            } else {
                return self.cartridge[addr as usize];
            }
        } else if addr < 0xA000 { // VRAM
            return self.vram[(addr - 0x8000) as usize];
        } else if addr < 0xC000 { // External RAM
            todo!();
        } else if addr < 0xDFFF { // WRAM
            return self.wram[(addr - 0xC000) as usize];
        } else if addr < 0xFDFF { // Echo RAM
            todo!();
        } else if addr < 0xFF00 { // OAM
            return self.oam[(addr - 0xFE00) as usize];
        } else if addr < 0xFF7F { // I/O Registers
            return self.io[(addr - 0xFF00) as usize];
        } else if addr < 0xFFFF { // HRAM
            return self.hram[(addr - 0xFF80) as usize];
        } else { // Interrupt Enable Register
            return self.ie;
        }
        
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        if addr < 0x8000 { // Cartridge
            self.cartridge[addr as usize] = data;
        } else if addr < 0xA000 { // VRAM
            self.vram[(addr - 0x8000) as usize] = data;
        } else if addr < 0xC000 { // External RAM
            todo!();
        } else if addr < 0xDFFF { // WRAM
            self.wram[(addr - 0xC000) as usize] = data;
        } else if addr < 0xFDFF { // Echo RAM
            todo!();
        } else if addr < 0xFF00 { // OAM
            self.oam[(addr - 0xFE00) as usize] = data;
        } else if addr < 0xFF7F { // I/O Registers
            self.io[(addr - 0xFF00) as usize] = data;
        } else if addr < 0xFFFF { // HRAM
            self.hram[(addr - 0xFF80) as usize] = data;
        } else { // Interrupt Enable Register
            self.ie = data;
        }
    }
}