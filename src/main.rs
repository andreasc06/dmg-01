
mod cpu;

fn main() {

    let mut cpu = cpu::CPU::new();

    if cpu.verify(){
        println!("CPU Initialized")
    }

    // fibonacci 
    cpu.set_register(&cpu::RegEnum::A, 0);
    cpu.set_register(&cpu::RegEnum::B, 1);
    cpu.set_register(&cpu::RegEnum::C, 0); 
    println!("{}", cpu.get_registers(&cpu::RegEnum::A));

    for i in 0..12 {
        cpu.add_rx_ty(&cpu::RegEnum::A, &cpu::RegEnum::B);
        cpu.ld_rx_ry(&cpu::RegEnum::C,&cpu::RegEnum::A);
        cpu.ld_rx_ry(&cpu::RegEnum::A,&cpu::RegEnum::B);
        cpu.ld_rx_ry(&cpu::RegEnum::B,&cpu::RegEnum::C);
        println!("{}", cpu.get_registers(&cpu::RegEnum::A));
    }

}
