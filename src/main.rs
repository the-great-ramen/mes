use mes::{Cpu,CpuMemory,};

fn main() {
    let _memory: CpuMemory = CpuMemory::new();
    let _cpu = Cpu::new(_memory);
    println!("Hello, world!");
}
