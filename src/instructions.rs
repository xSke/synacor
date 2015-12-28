use std::char;

use super::cpu::CPU;

pub enum Instruction {
    Halt,
    Out,
    Nop
}

impl Instruction {
    pub fn opcode(&self) -> u16 {
        match *self {
            Instruction::Halt => 0,
            Instruction::Out => 19,
            Instruction::Nop => 21
        }
    }

    pub fn arity(&self) -> u16 {
        match *self {
            Instruction::Halt => 0,
            Instruction::Out => 1,
            Instruction::Nop => 0
        }
    }

    pub fn from_opcode(opcode: u16) -> Instruction {
        match opcode {
            0 => Instruction::Halt,
            19 => Instruction::Out,
            21 => Instruction::Nop,
            //_ => panic!("Invalid opcode {}", opcode)
            _ => Instruction::Nop,
        }
    }

    pub fn execute(&self, cpu: &mut CPU, arguments: &[u16]) {
        match *self {
            Instruction::Halt => {
                cpu.running = false;
            },
            Instruction::Out => {
                let value = cpu.get_value(arguments[0]);
                let c = char::from_u32(value as u32).unwrap();
                print!("{}", c);
            },
            Instruction::Nop => {
            }
        }
    }
}
