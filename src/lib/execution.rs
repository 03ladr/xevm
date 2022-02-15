use ethers::types::U256;
use eyre::{eyre, Result};
use super::opcode::*;
use super::memory::Memory;
use super::stack::Stack;

pub struct ExecutionContext {
    code: Vec<u8>,
    stack: Stack,
    memory: Memory,
    pc: usize,
    stopped: bool
}
impl ExecutionContext {
    pub fn init(code: Vec<u8>, stack: Stack, memory: Memory) -> Self {
        ExecutionContext {
            code: code,
            stack: stack,
            memory: memory,
            pc: 0,
            stopped: false
        }
    }

    pub fn stop(&mut self) -> () {
        self.stopped = true
    }

    pub fn pc_increment(&mut self, idx: usize) -> () {
        self.pc = self.pc + idx
    }

    pub fn read_code(&mut self, idx: usize) -> Result<u8> {
        if self.pc + idx - 1 >= self.code.len() {
            return Err(eyre!("Index Out Of Bounds"));
        };
        let value = self.code[self.pc + idx - 1];
        self.pc_increment(1);
        Ok(value)
    }

    pub fn run(&mut self) -> Result<()> {
        while !self.stopped {
            let opcode: u8 = self.read_code(1)?;
            _ = self.exec(opcode)?;
            println!("Stack: {:?}", self.stack.storage);
        }
        Ok(())
    }

    pub fn exec(&mut self, opcode: u8) -> Result<()> {
        match opcode {
            STOP => {
                self.stop();
                println!("Execution Complete");
                Ok(())
            },
            PUSH1 => {
                let value = self.read_code(1)?;
                self.stack.push(U256::from(value))?;
                Ok(())
            },
            MUL => {
                let mul1 = self.stack.pop()?;
                let mul2 = self.stack.pop()?;
                self.stack.push(mul1 * mul2)?;
                Ok(())
            },
            ADD => {
                let mul1 = self.stack.pop()?;
                let mul2 = self.stack.pop()?;

                self.stack.push(mul1 + mul2)?;
                Ok(())
            },
            POP => {
                self.stack.pop()?;
                Ok(())
            }
            _ => Err(eyre!("Unknown Opcode"))
        }
    }
}
