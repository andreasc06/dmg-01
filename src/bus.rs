
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

    memory: [u8; 0xFFFF],
}

impl Bus {
    pub fn new() -> Self {
        Self {memory: [0; 0xFFFF] }
    }

    pub fn read(&self, addr: u16) -> u8 {
        return self.memory[addr as usize];
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }
}