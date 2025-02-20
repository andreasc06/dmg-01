mod cpu;
mod bus;
mod instructions;
mod registers;

fn main() {

    let mut cpu = cpu::CPU::new();

    if cpu.verify(){
        println!("CPU Initialized")
    }

    let boot_rom = include_bytes!("../bin/dmg_boot.bin");
    let game_rom = include_bytes!("../bin/tetris.gb");
    // Loads bootrom from 0x000-0x100
    for (i, byte) in boot_rom.iter().enumerate() {
        //cpu.bus.write(i as u16, *byte);
        cpu.bus.boot_rom[i] = *byte;
    } 
    for (i, byte) in game_rom.iter().enumerate() {
        cpu.bus.cartridge[i] = *byte;
    } 

    cpu.boot()
}