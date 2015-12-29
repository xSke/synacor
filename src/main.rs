extern crate byteorder;

use std::fs::File;
use std::io::Read;

mod cpu;
mod instructions;
use cpu::CPU;

fn main() {
    let mut cpu = CPU::new();
    cpu.load_from_file(&mut File::open("challenge.bin").unwrap());

    println!(" - Dumping bytecode...");
    cpu.dis(&mut File::create("disassembly.txt").unwrap());

    let sf = File::open("script.txt");
    if let Ok(mut file) = sf {
        let mut buf = String::new();
        file.read_to_string(&mut buf).unwrap();
        cpu.input_buffer = buf.replace("\r", "");
    }

    let af = File::open("arch-spec");
    if let Ok(mut file) = af {
        let mut buf = String::new();
        file.read_to_string(&mut buf).unwrap();

        let needle = "Here's a code for the challenge website: ";
        let pos = buf.find(needle).unwrap();
        println!("Spec code is {}", &buf[pos + needle.len()..pos + needle.len() + 12]);
    }

    println!(" - Running...");
    cpu.run();

    println!(" - Program finished!");
}
