extern crate byteorder;

use std::fs::File;
use std::io::Read;

mod cpu;
mod instructions;
use cpu::CPU;

fn print(cpu: &mut CPU, addr: u16) {
    let mut new = cpu.clone();
    new.stack = vec![];
    new.pc = 1518;
    new.registers[0] = addr;

    new.run();
}

pub fn command(cpu: &mut CPU, string: &String) -> bool {
    match string.trim() {
        "hack teleporter" => {
            // Magic value, determined using "teleport.c"
            let value = 25734;

            cpu.registers[7] = value;
            cpu.memory[6027] = 1;
            cpu.memory[6028] = 32769;
            cpu.memory[6029] = 32775;
            cpu.memory[6030] = 1;
            cpu.memory[6031] = 32768;
            cpu.memory[6032] = 6;
            cpu.memory[6033] = 21;

            true
        },
        _ => false
    }
}

fn main() {
    let mut cpu = CPU::new();
    cpu.load_from_file(&mut File::open("challenge.bin").unwrap());

    println!(" - Dumping bytecode...");
    cpu.dis(&mut File::create("dis.txt").unwrap());

    let mut buf = String::new();
    File::open("script.txt").unwrap().read_to_string(&mut buf).unwrap();
    cpu.input_buffer = buf.replace("\r", "");

    println!(" - Running...");
    cpu.run();

    println!(" - Program finished!")
}
