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
        if self.pc + idx >= self.code.len() {
            return Err(eyre!("Index Out Of Bounds"));
        };
        let value = self.code[self.pc + idx];
        Ok(value)
    }

    pub fn run(&mut self) -> Result<()> {
        while !self.stopped {
            let opcode: u8 = self.read_code(0)?;
            self.exec(opcode)?;
            println!("Stack: {:?}", self.stack.storage);
        }
        Ok(())
    }

    pub fn exec(&mut self, opcode: u8) -> Result<()> {
        macro_rules! arith_instructor {
            ( $op:tt ) => {
                let num2 = self.stack.pop()?;
                let num1 = self.stack.pop()?;
                self.stack.push(num1 $op num2)?;
            };
        };
        match opcode {
            STOP => {
                self.stop();
                println!("Execution Complete");
                Ok(())
            },
            PUSH1 => {
                let value = self.read_code(1)?;
                self.stack.push(U256::from(value))?;
                self.pc_increment(2);
                Ok(())
            },
            MUL => {
                arith_instructor!(*);
                self.pc_increment(1);
                Ok(())
            },
            ADD => {
                arith_instructor!(+);
                self.pc_increment(1);
                Ok(())
            },
            SUB => {
                arith_instructor!(-);
                self.pc_increment(1);
                Ok(())
            },
            DIV => {
                arith_instructor!(/);
                self.pc_increment(1);
                Ok(())
            },
            MOD => {
                arith_instructor!(%);
                self.pc_increment(1);
                Ok(())
            },
            EQ => {
                let num2 = self.stack.pop()?;
                let num1 = self.stack.pop()?;
                let mut eq: U256 = U256::from(0u8);
                if num1 == num2 {
                    eq = U256::from(1u8);
                };
                self.stack.push(eq)?;
                self.pc_increment(1);
                Ok(())
            },
            POP => {
                self.stack.pop()?;
                self.pc_increment(1);
                Ok(())
            }
            _ => Err(eyre!("Unknown Opcode: {}", opcode))
        }
    }
}
