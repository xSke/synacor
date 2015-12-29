use std::char;
use std::io::{self, BufRead};
use std::num::Wrapping;

use super::cpu::CPU;

#[derive(Debug)]
pub enum Instruction {
    Halt = 0,
    Set = 1,
    Push = 2,
    Pop = 3,
    Eq = 4,
    Gt = 5,
    Jmp = 6,
    Jt = 7,
    Jf = 8,
    Add = 9,
    Mult = 10,
    Mod = 11,
    And = 12,
    Or = 13,
    Not = 14,
    Rmem = 15,
    Wmem = 16,
    Call = 17,
    Ret = 18,
    Out = 19,
    In = 20,
    Nop = 21
}

impl Instruction {
    pub fn arity(&self) -> u16 {
        match *self {
            Instruction::Halt => 0,
            Instruction::Set => 2,
            Instruction::Push => 1,
            Instruction::Pop => 1,
            Instruction::Eq => 3,
            Instruction::Gt => 3,
            Instruction::Jmp => 1,
            Instruction::Jt => 2,
            Instruction::Jf => 2,
            Instruction::Add => 3,
            Instruction::Mult => 3,
            Instruction::Mod => 3,
            Instruction::And => 3,
            Instruction::Or => 3,
            Instruction::Not => 2,
            Instruction::Rmem => 2,
            Instruction::Wmem => 2,
            Instruction::Call => 1,
            Instruction::Ret => 0,
            Instruction::Out => 1,
            Instruction::In => 1,
            Instruction::Nop => 0
        }
    }

    pub fn from_opcode(opcode: u16) -> Option<Instruction> {
        match opcode {
            0 => Some(Instruction::Halt),
            1 => Some(Instruction::Set),
            2 => Some(Instruction::Push),
            3 => Some(Instruction::Pop),
            4 => Some(Instruction::Eq),
            5 => Some(Instruction::Gt),
            6 => Some(Instruction::Jmp),
            7 => Some(Instruction::Jt),
            8 => Some(Instruction::Jf),
            9 => Some(Instruction::Add),
            10 => Some(Instruction::Mult),
            11 => Some(Instruction::Mod),
            12 => Some(Instruction::And),
            13 => Some(Instruction::Or),
            14 => Some(Instruction::Not),
            15 => Some(Instruction::Rmem),
            16 => Some(Instruction::Wmem),
            17 => Some(Instruction::Call),
            18 => Some(Instruction::Ret),
            19 => Some(Instruction::Out),
            20 => Some(Instruction::In),
            21 => Some(Instruction::Nop),
            _ => None
        }
    }

    pub fn execute(&self, cpu: &mut CPU, arguments: &[u16]) {
        match *self {
            Instruction::Halt => {
                cpu.running = false;
            },
            Instruction::Set => {
                cpu.registers[(arguments[0] % 32768) as usize] = cpu.get_value(arguments[1]);
            },
            Instruction::Push => {
                let value = cpu.get_value(arguments[0]);
                cpu.stack.push(value);
            },
            Instruction::Pop => {
                match cpu.stack.pop() {
                    Some(v) => cpu.registers[(arguments[0] % 32768) as usize] = v,
                    None => panic!("Attempted to pop from empty stack")
                }
            }
            Instruction::Eq => {
                let a = cpu.get_value(arguments[1]);
                let b = cpu.get_value(arguments[2]);

                cpu.registers[(arguments[0] % 32768) as usize] = if a == b {1} else {0}
            },
            Instruction::Gt => {
                let a = cpu.get_value(arguments[1]);
                let b = cpu.get_value(arguments[2]);

                cpu.registers[(arguments[0] % 32768) as usize] = if a > b {1} else {0}
            }
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
            Instruction::Add => {
                let a = cpu.get_value(arguments[1]);
                let b = cpu.get_value(arguments[2]);

                cpu.registers[(arguments[0] % 32768) as usize] = (Wrapping(a) + Wrapping(b)).0 % 32768;
            },
            Instruction::Mult => {
                let a = cpu.get_value(arguments[1]);
                let b = cpu.get_value(arguments[2]);

                cpu.registers[(arguments[0] % 32768) as usize] = (Wrapping(a) * Wrapping(b)).0 % 32768;
            },
            Instruction::Mod => {
                let a = cpu.get_value(arguments[1]);
                let b = cpu.get_value(arguments[2]);

                cpu.registers[(arguments[0] % 32768) as usize] = a % b;
            },
            Instruction::And => {
                let a = cpu.get_value(arguments[1]);
                let b = cpu.get_value(arguments[2]);

                cpu.registers[(arguments[0] % 32768) as usize] = (Wrapping(a) & Wrapping(b)).0 % 32768;
            },
            Instruction::Or => {
                let a = cpu.get_value(arguments[1]);
                let b = cpu.get_value(arguments[2]);

                cpu.registers[(arguments[0] % 32768) as usize] = (Wrapping(a) | Wrapping(b)).0 % 32768;
            },
            Instruction::Not => {
                let b = cpu.get_value(arguments[1]);

                cpu.registers[(arguments[0] % 32768) as usize] = (!Wrapping(b)).0 % 32768;
            },
            Instruction::Rmem  => {
                cpu.registers[(arguments[0] % 32768) as usize] = cpu.memory[cpu.get_value(arguments[1]) as usize];
            },
            Instruction::Wmem  => {
                let addr = cpu.get_value(arguments[0]) as usize;
                cpu.memory[addr] = cpu.get_value(arguments[1]);
            },
            Instruction::Call => {
                // Arity and all...
                let ret = cpu.pc + 2;
                cpu.stack.push(ret);

                cpu.pc = cpu.get_value(arguments[0]) - 2;
            },
            Instruction::Ret => {
                // Yeah yeah
                cpu.pc = match cpu.stack.pop() {
                    Some(v) => v - 1,
                    None => {
                        cpu.running = false;
                        0
                    }
                }
            },
            Instruction::Out => {
                let value = cpu.get_value(arguments[0]);
                let c = char::from_u32(value as u32).unwrap();
                print!("{}", c);
            },
            Instruction::In => {
                loop {
                    if cpu.input_buffer.len() == 0 {
                        let stdin = io::stdin();
                        stdin.lock().read_line(&mut cpu.input_buffer).unwrap();
                        cpu.input_buffer = cpu.input_buffer.replace("\r", "");
                    }

                    let string = {
                        let next_line = cpu.input_buffer.split("\n").next().unwrap();
                        next_line.to_string()
                    };

                    if super::command(cpu, &string) {
                        cpu.input_buffer = cpu.input_buffer[string.len()..].to_string();
                    } else {
                        break;
                    }
                }

                let mut next = cpu.input_buffer.remove(0);

                while next as u8 == 13 {
                    next = cpu.input_buffer.remove(0);
                }

                cpu.registers[(arguments[0] % 32768) as usize] = next as u16;
            },
            Instruction::Nop => {
            }
        }
    }
}
