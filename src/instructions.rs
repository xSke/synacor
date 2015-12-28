use std::char;

use super::cpu::CPU;

pub enum Instruction {
    Halt = 0,
    Set = 1,
    Jmp = 6,
    Jt = 7,
    Jf = 8,
    Out = 19,
    Nop = 21
}

impl Instruction {
    pub fn opcode(self) -> u16 {
        self as u16
    }

    pub fn arity(&self) -> u16 {
        match *self {
            Instruction::Halt => 0,
            Instruction::Set => 2,
            Instruction::Jmp => 1,
            Instruction::Jt => 2,
            Instruction::Jf => 2,
            Instruction::Out => 1,
            Instruction::Nop => 0
        }
    }

    pub fn from_opcode(opcode: u16) -> Instruction {
        match opcode {
            0 => Instruction::Halt,
            1 => Instruction::Set,
            6 => Instruction::Jmp,
            7 => Instruction::Jt,
            8 => Instruction::Jf,
            19 => Instruction::Out,
            21 => Instruction::Nop,
            _ => {
                println!("Unknown opcode {}", opcode);
                Instruction::Nop
            },
        }
    }

    pub fn execute(&self, cpu: &mut CPU, arguments: &[u16]) {
        match *self {
            Instruction::Halt => {
                cpu.running = false;
            },
            Instruction::Set => {
                let mut register = arguments[0];
                if register >= 32768 {register -= 32768}

                cpu.registers[register as usize] = cpu.get_value(arguments[1]);
            },
            Instruction::Jmp => {
                let value = cpu.get_value(arguments[0]);

                // CPU will increment the PC with one + last instr's arity, so we counteract that
                // This instruction's arity is one, and you can add one and one together :)
                cpu.pc = value - 2;
            },
            Instruction::Jt => {
                let test = cpu.get_value(arguments[0]);
                let value = cpu.get_value(arguments[1]);

                if test != 0 {
                    // This one's arity is 2, so value is 3
                    cpu.pc = value - 3;
                }
            },
            Instruction::Jf => {
                let test = cpu.get_value(arguments[0]);
                let value = cpu.get_value(arguments[1]);

                if test == 0 {
                    // You know the drill
                    cpu.pc = value - 3;
                }
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
