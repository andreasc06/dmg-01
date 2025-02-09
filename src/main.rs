
mod cpu;
mod bus;
mod instructions;

fn main() {

    let mut cpu = cpu::CPU::new();

    if cpu.verify(){
        println!("CPU Initialized")
    }

    let rom = include_bytes!("../bin/dmg_boot.bin");
    
    // Loads bootrom from 0x000-0x100
    for (i, byte) in rom.iter().enumerate() {
        cpu.bus.write(i as u16, *byte);
    } 

    cpu.boot()
}
