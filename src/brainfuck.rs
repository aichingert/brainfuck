use std::io::Read;
use crate::OpCode;

pub struct Brainfuck {
    buffer: Vec<u8>,
    pointer: usize
}

pub enum Instruction {
    Increment,
    Decrement,
    PointerInc,
    PointerDec,
    Write,
    Read,
    Loop(Vec<Instruction>)
}

impl Brainfuck {
    pub fn new() -> Self {
        Self {
            buffer: vec![0; 30000],
            pointer: 15000
        }
    }

    pub fn run(&mut self, instructions: &Vec<Instruction>) {
        for instr in instructions.iter() {
            match instr {
                Instruction::Increment => self.buffer[self.pointer] += 1,
                Instruction::Decrement => self.buffer[self.pointer] -= 1,
                Instruction::PointerInc => self.pointer += 1,
                Instruction::PointerDec => self.pointer -= 1,
                Instruction::Write => print!("{}", self.buffer[self.pointer] as char),
                Instruction::Read => {
                    let mut buf: [u8; 1] = [0; 1];
                    std::io::stdin().read_exact(&mut buf).expect("unable to read stdin");
                    self.buffer[self.pointer] = buf[0];
                },
                Instruction::Loop(nested) => {
                    while self.buffer[self.pointer] != 0 {
                        self.run(nested)
                    }
                },
            }
        }
    }
}

pub fn parse(opcode: &Vec<OpCode>) -> Vec<Instruction> {
    let mut program: Vec<Instruction> = Vec::new();
    let mut loop_stack: usize = 0;
    let mut loop_start: usize = 0;

    for (i, op) in opcode.iter().enumerate() {
        if loop_stack == 0 {
            let instruction = match op {
                OpCode::PointerInc => Some(Instruction::PointerInc),
                OpCode::PointerDec => Some(Instruction::PointerDec),
                OpCode::Increment => Some(Instruction::Increment),
                OpCode::Decrement => Some(Instruction::Decrement),
                OpCode::Write => Some(Instruction::Write),
                OpCode::Read => Some(Instruction::Read),

                OpCode::LoopBegin => {
                    loop_start = i;
                    loop_stack += 1;
                    None
                },

                OpCode::LoopEnd => panic!("loop ending at #{} has no beginning", i),
                OpCode::None => panic!("still none values in opcodes")
            };

            match instruction {
                Some(instr) => program.push(instr),
                None => ()
            }
        } else {
            match op {
                OpCode::LoopBegin => {
                    loop_stack += 1;
                },
                OpCode::LoopEnd => {
                    loop_stack -= 1;

                    if loop_stack == 0 {
                        program.push(Instruction::Loop(parse(&opcode[loop_start + 1..i].to_vec())));
                    }
                }
                _ => {}
            }
        }
    }

    if loop_stack != 0 {
        panic!("loop at #{}, has no matching end!", loop_start);
    }

    program
}