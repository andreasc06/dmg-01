
use core::panic;

use colored::Colorize;

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
        let exit_op: bool = true;
        loop {
            let current_instruction = self.fetch_n8(); 
            let pc_of_opcode = self.pc - 1;

            if (pc_of_opcode == 0x55) {
                self.debug_state(pc_of_opcode, current_instruction, true);
            }

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

    pub fn push_stack16(&mut self, value: u16){
        let high: u8 = (value >> 8) as u8;
        let low: u8 = (value & 0xFF) as u8;

        self.sp -= 1;
        self.bus.write(self.sp, high);
        self.sp -= 1;
        self.bus.write(self.sp, low);
    }
    pub fn pop_stack16(&mut self) -> u16{
        let low = self.bus.read(self.sp) as u16;
        self.sp += 1;
        let high= self.bus.read(self.sp) as u16;
        self.sp += 1;

        (high << 8) | low

    }
    pub fn set_sp(&mut self, value: u16){
        self.sp = value;
    }
    pub fn get_sp(& self) -> u16 {
        return self.sp
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

    

    pub fn debug_state(&self, pc: u16, opcode: u8, exit: bool) {
    
        println!("{}", format!(
                    "\nVRAM BLOCK 0:\n{}",
                    (0x8000..=0x8800)
                        .map(|addr| format!("{:02X}", self.bus.read(addr as u16)))
                        .collect::<Vec<String>>()
                        .chunks(16) // 16 bytes per line
                        .map(|line| line.join(" "))
                        .collect::<Vec<String>>()
                        .join("\n")
                ).blue());
        println!("{}", format!(
                    "\nSTACK\n{}",
                    (0xFF80..=0xFFFE)
                        .map(|addr| {
                            let value = self.bus.read(addr as u16);
                            if addr == self.sp {
                                format!("{}", format!("{:02X}", value).red())  // Highlight value at cpu.sp in red
                            } else {
                                format!("{:02X}", value)  // Regular formatting for other values
                            }
                        })
                        .collect::<Vec<String>>()
                        .chunks(64)  // Printing 16 bytes per line
                        .map(|line| line.join(" "))
                        .collect::<Vec<String>>()
                        .join("\n")
                ).yellow());  // This blue only applies to the full output string, not individual bytes
    
        println!("{}",
                    format!("\nRegisters: A: 0x{:02X}, B: 0x{:02X}, C: 0x{:02X}, D: 0x{:02X}, E: 0x{:02X}, H: 0x{:02X}, L: 0x{:02X}, SP: 0x{:04X}, PC: 0x{:04X}",
                    self.register.get_8(&Register::A),
                    self.register.get_8(&Register::B),
                    self.register.get_8(&Register::C),
                    self.register.get_8(&Register::D),
                    self.register.get_8(&Register::E),
                    self.register.get_8(&Register::H),
                    self.register.get_8(&Register::L),
                    self.sp, 
                    pc).bold()
                );
            println!("{}",
                    format!("\nFlags: Z: {}, N: {}, H: {}, C: {}\n",
                    self.register.get_flag(&Flag::Z),
                    self.register.get_flag(&Flag::N),
                    self.register.get_flag(&Flag::H),
                    self.register.get_flag(&Flag::C)).bold()
                );
    
        if exit {
            panic!("{}", format!("Unknown opcode: 0x{:02X} at 0x{:02X}", opcode, pc).bold().red()) // Handle unknown opcode
        } else {
            println!("{}", format!("Unknown opcode: 0x{:02X} at 0x{:02X}", opcode, pc).bold().red())
        }
        
            
    }


}
    



