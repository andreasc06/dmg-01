
use core::panic;

use crate::{bus::Bus, registers::{Registers,Register,Flag}, instructions::execute_instruction};

pub struct CPU {
    pub register: Registers,
    pc: u16,
    sp: u16,
    pub bus: Bus,
    
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

            execute_instruction(self, current_instruction);

            println!("Opcode: {:#x}, PC: {:#x}, Loop: {}", current_instruction, self.pc, loops);
            loops += 1;
        }
    }

    pub fn fetch_n8(&mut self) -> u8{
        let n8: u8 = self.bus.read(self.pc);
        self.pc = self.pc.wrapping_add(0x1);
        return n8;
    }
    pub fn push_stack8(&mut self, value: u8){

        self.sp -= 1;
        self.bus.write(self.sp, value);
 
    }
    pub fn push_stack16(&mut self, value: u16){
        let high: u8 = (value >> 8) as u8;
        let low: u8 = (value & 0xFF) as u8;

        self.sp -= 1;
        self.bus.write(self.sp, high);
        self.sp -= 1;
        self.bus.write(self.sp, low);
    }
    pub fn pop_stack(&mut self) -> u16{
        let low = self.bus.read(self.sp) as u16;
        self.sp += 1;
        let high= self.bus.read(self.sp) as u16;
        self.sp += 1;

        (high << 8) | low

    }
    pub fn set_sp(&mut self, value: u16){
        self.sp = value;
    }
    pub fn set_pc(&mut self, value: u16){
        self.pc = value;
    }
    pub fn get_pc(& self) -> u16 {
        return self.pc
    }
    pub fn offset_pc(&mut self, offset: i8){
        self.pc = self.pc.wrapping_add(offset as u16);
    }


}
