extern crate byteorder;

use std::fs::{File};
use byteorder::{LittleEndian, ReadBytesExt};
use super::instructions::Instruction;
use super::instructions::Instruction::*;

#[derive(Debug)]
pub struct CPU {
    memory: Vec<u16>,
    pub registers: Vec<u16>,
    stack: Vec<u16>,
    pub pc: u16,
    pub running: bool
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            memory: vec![0; 32768],
            registers: vec![0; 8],
            stack: Vec::new(),
            pc: 0,
            running: true
        }
    }

    pub fn load_from_file(&mut self, file: &mut File) {
        let mut i = 0;
        while let Ok(b) = file.read_u16::<LittleEndian>() {
            self.memory[i] = b;
            i += 1;
        }
    }

    fn execute_next(&mut self) {
        let opcode_val = self.memory[self.pc as usize];
        let opcode = Instruction::from_opcode(opcode_val);
        let arity = opcode.arity();

        let args = self.memory[self.pc as usize + 1..self.pc as usize + 1 + arity as usize].to_vec();

        opcode.execute(self, &args);

        self.pc += arity + 1;
    }

    pub fn get_value(&self, value: u16) -> u16 {
        match value {
            0...32767 => value,
            32768...32775 => self.registers[value as usize - 32768],
            _ => panic!(format!("Invalid literal value {}", value))
        }
    }

    pub fn run(&mut self) {
        while self.running {
            self.execute_next();
        }
    }
}
