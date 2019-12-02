use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub struct IntCodeParser {
    rom: Vec<u32>,
    memory: Vec<u32>,
    ip: usize,
}

impl IntCodeParser {
    pub fn new() -> IntCodeParser {
        IntCodeParser {
            rom: Vec::new(),
            memory: Vec::new(),
            ip: 0_usize,
        }
    }
    pub fn load_program<'a>(&mut self, reader: &'a mut dyn BufRead) -> &mut Self {
        for line in reader.lines() {
            for byte in line.unwrap().split(",") {
                self.rom.push(byte.parse::<u32>().unwrap());
            }
        }
        self.memory = self.rom.clone();
        self
    }

    pub fn set_program_code<'a>(&mut self, pos: usize, program: &'a [u32]) -> &mut Self {
        for i in 0..program.len() {
            self.write_pos(pos + i, program[i]);
        }
        self
    }

    pub fn clear_memory(&mut self) -> &mut Self {
        self.memory = self.rom.clone();
        self
    }

    pub fn run(&mut self) -> &mut Self {
        while let Some(op) = self.next() {
            match op {
                Operation::Add(op1, op2, dst) => {
                    let operand1 = self.read_pos(op1 as usize);
                    let operand2 = self.read_pos(op2 as usize);
                    println!(
                        "Adding {} and {} and writing to {}",
                        operand1.unwrap(),
                        operand2.unwrap(),
                        dst
                    );
                    self.write_pos(dst as usize, operand1.unwrap() + operand2.unwrap());
                }
                Operation::Mul(op1, op2, dst) => {
                    let operand1 = self.read_pos(op1 as usize);
                    let operand2 = self.read_pos(op2 as usize);
                    println!(
                        "Multiplying {} and {} and writing to {}",
                        operand1.unwrap(),
                        operand2.unwrap(),
                        dst
                    );
                    self.write_pos(dst as usize, operand1.unwrap() * operand2.unwrap());
                }
                Operation::Terminate => {
                    println!("Graceful exit");
                    return self;
                }
                Operation::Invalid => {
                    println!("Invalid op code");
                    return self;
                }
            }
        }
        self.ip = 0;
        self
    }
}

impl IntCodeParser {
    pub fn read_pos(&self, pos: usize) -> Option<u32> {
        if self.memory.len() <= pos {
            None
        } else {
            Some(self.memory[pos])
        }
    }
    fn write_pos(&mut self, pos: usize, val: u32) -> Result<(), io::Error> {
        if self.memory.len() <= pos {
            Err(io::Error::new(
                io::ErrorKind::Other,
                "attempt to write to address out of range",
            ))
        } else {
            self.memory[pos] = val;
            Ok(())
        }
    }
}

impl Iterator for IntCodeParser {
    type Item = Operation;
    fn next(&mut self) -> Option<Operation> {
        self.ip += 4;
        match OpCode::from(self.memory[self.ip]) {
            OpCode::Add => Some(Operation::Add(
                self.memory[self.ip + 1],
                self.memory[self.ip + 2],
                self.memory[self.ip + 3],
            )),
            OpCode::Mul => Some(Operation::Mul(
                self.memory[self.ip + 1],
                self.memory[self.ip + 2],
                self.memory[self.ip + 3],
            )),
            OpCode::Term => None,
            OpCode::Invalid => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum OpCode {
    Add,
    Mul,
    Term,
    Invalid,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operation {
    Add(u32, u32, u32),
    Mul(u32, u32, u32),
    Terminate,
    Invalid,
}

impl From<u32> for OpCode {
    fn from(val: u32) -> OpCode {
        match val {
            1 => OpCode::Add,
            2 => OpCode::Mul,
            99 => OpCode::Term,
            _ => OpCode::Invalid,
        }
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file = File::open(args[1].clone())?;
    let mut reader = BufReader::new(file);
    let mut cpu = IntCodeParser::new();
    cpu.load_program(&mut reader)
        .set_program_code(1, &vec![12, 02])
        .run();
    for i in 0..100 {
        for j in 0..100 {
            if cpu
                .clear_memory()
                .set_program_code(1, &vec![i, j])
                .run()
                .read_pos(0)
                .unwrap()
                == 19690720
            {
                println!("{}", 100 * i + j);
                return Ok(());
            }
        }
    }

    Ok(())
}
