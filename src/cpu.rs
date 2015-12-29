extern crate byteorder;

use std::fs::{File};
use std::io::{Write};
use byteorder::{LittleEndian, ReadBytesExt};

use instructions::Instruction;

#[derive(Clone)]
pub struct CPU {
    pub memory: Vec<u16>,
    pub registers: Vec<u16>,
    pub stack: Vec<u16>,
    pub pc: u16,
    pub running: bool,
    pub input_buffer: String,
    pub saved: Option<Box<CPU>>
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            memory: vec![0; 32768],
            registers: vec![0; 8],
            stack: Vec::new(),
            pc: 0,
            running: true,
            input_buffer: String::new(),
            saved: None
        }
    }

    pub fn load_from_file(&mut self, file: &mut File) {
        let mut i = 0;
        while let Ok(b) = file.read_u16::<LittleEndian>() {
            self.memory[i] = b;
            i += 1;
        }
    }

    pub fn execute_next(&mut self) {
        let opcode_val = self.memory[self.pc as usize];
        let opcode = Instruction::from_opcode(opcode_val).expect(&format!("Invalid opcode {}", opcode_val));
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

    pub fn command(&mut self, string: &String) -> bool {
        match string.trim() {
            "hack teleporter" => {
                // Magic value, determined using "teleport.c"
                let value = 25734;

                self.registers[7] = value;
                self.memory[6027] = 1;
                self.memory[6028] = 32769;
                self.memory[6029] = 32775;
                self.memory[6030] = 1;
                self.memory[6031] = 32768;
                self.memory[6032] = 6;
                self.memory[6033] = 21;

                true
            },
            "save" => {
                self.saved = Some(Box::new(self.clone()));
                true
            },
            "load" => {
                let s = self.saved.take().unwrap();
                self.pc = s.pc;
                self.memory = s.memory.clone();
                self.registers = s.registers.clone();
                self.stack = s.stack.clone();

                self.saved = None;
                true
            }
            _ => false
        }
    }

    pub fn dis(&mut self, file: &mut File) {
        let mut pc = 0;
        while pc < 32768 {
            let opcode_val = self.memory[pc as usize];
            match Instruction::from_opcode(opcode_val) {
                Some(opcode) => {
                    let arity = opcode.arity();

                    let args = self.memory[pc as usize + 1..pc as usize + 1 + arity as usize].to_vec();

                    let args_str = args.iter()
                    .map(|x| {
                        match *x {
                            0...32767 => format!("{}", x),
                            32768...32775 => format!("#{}", ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'][*x as usize - 32768]),
                            _ => format!("")
                        }
                    }).collect::<Vec<_>>().join(&", ".to_string());
                    let val = &format!("[{}] {:?}: {}\r\n", pc, opcode, args_str).into_bytes();
                    file.write_all(val).unwrap();

                    pc += arity + 1;
                },
                None => {
                    pc += 1;
                }
            }
        }
        file.sync_all().unwrap();
    }

    pub fn run(&mut self) {
        while self.running {
            self.execute_next();
        }
    }
}
