extern crate byteorder;

use std::fs::File;
use std::io::Read;
use byteorder::{LittleEndian, ReadBytesExt};

pub mod cpu;
pub mod instructions;

fn main() {
    let mut cpu = cpu::CPU::new();
    cpu.load_from_file(&mut File::open("challenge.bin").unwrap());
    cpu.run();
}
