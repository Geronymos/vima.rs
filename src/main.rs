use std::env;
use std::fs;
use std::error::Error;
use strum::{EnumString, Display};
use std::str::FromStr;
use std::fs::File;
use std::io::prelude::*;
use std::convert::TryFrom;

#[derive(Debug)]
struct Instruction {
    opcode: OpCode,
    parameter: i32,
    instruction: i32,
    address: usize
}

impl Instruction {
    fn new(opcode: OpCode, parameter: i32) -> Instruction {
        Instruction {
            opcode, 
            parameter, 
            instruction: 0, 
            address: 0
        }
    }
}

// type u24 = [u8;3];
struct u24([u8;3]);

impl TryFrom<i32> for u24 {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value >= 0 && value <= 0xFFFFFF {
            Ok(u24([
                ((value >> 16) & 0xFF) as u8,
                ((value >> 8) & 0xFF) as u8,
                (value & 0xFF) as u8,
            ]))
        } else {
            Err("Value out of range for U24")
        }
    }
}

#[repr(i32)]
#[derive(Debug, PartialEq, EnumString, Display, Copy, Clone)]
enum OpCode {
    LDC =  0x00000,
    LDV =  0x10000,
    STV =  0x20000,
    ADD =  0x30000,
    AND =  0x40000,
    OR  =  0x50000,
    XOR =  0x60000,
    EQL =  0x70000,
    JMP =  0x80000,
    JMN =  0x90000,
    HALT=  0xF0000,
    NOT =  0xF1000,
    RAR =  0xF2000,
}

impl OpCode {
    fn has_parameter(self) -> bool {
        match self {
            OpCode::HALT | OpCode::NOT | OpCode::RAR => false,
            _ => true
        }
    }
}

trait ParseNumber {
    fn parse_number(&self) -> Result<i32, std::num::ParseIntError>;
}

impl ParseNumber for String {
    fn parse_number(&self) -> Result<i32, std::num::ParseIntError> {
        match &self[..self.len().min(2)] {
            "0x" => i32::from_str_radix(&self[2..], 16),
            "0b" => i32::from_str_radix(&self[2..], 2),
            _ => i32::from_str_radix(self, 10)
        }
    }
}

fn main() -> Result<(), Box<dyn Error>>  {
    if let (Some(file_path_asm), Some(file_path_obj)) = (env::args().nth(1), env::args().nth(2)) {
        let content = fs::read_to_string(file_path_asm)?;
        let mut tokens = content.split_whitespace();

        let mut instructions: Vec<u24> = Vec::new();

        while let Some(token) = tokens.next() {
            if let Ok(op) = OpCode::from_str(token) {
                let mut parameter = 0;
                if op.has_parameter() {
                    if let Ok(my_parameter) = tokens.next().unwrap().to_string().parse_number() {
                        parameter = my_parameter;
                    }
                }
                let instr = Instruction::new(op, parameter);
                instructions.push((op as i32 | parameter).try_into()?);
                println!("{instr:?}");
            }
        }

        let bin: &[u8] = &instructions.iter()
                  .flat_map(|array| array.0.iter().cloned())
                  .collect::<Vec<u8>>();

        let mut file_out = File::create(file_path_obj)?;
        let _ = file_out.write(bin);
    };
    Ok(())
}

