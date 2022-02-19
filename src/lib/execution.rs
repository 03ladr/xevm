use ethers::types::U256;
use eyre::{eyre, Result};
use super::opcode::*;
use super::memory::Memory;
use super::stack::Stack;

#[derive(Debug)]
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
            return Ok(00);
        };
        let value = self.code[self.pc + idx];
        Ok(value)
    }

    pub fn run(&mut self) -> Result<()> {
        while !self.stopped {
            let opcode: u8 = self.read_code(0)?;
            self.exec(opcode)?;
            println!("Opcode: {} @ PC: {}\nStack: {:?}", opcode, self.pc, self.stack.storage);
        }
        Ok(())
    }

    pub fn exec(&mut self, opcode: u8) -> Result<()> {
        match opcode {
            STOP => {
                self.stop();
                self.pc_increment(1);
                Ok(())
            },
            PUSH1 => {
                let value = self.read_code(1)?;
                self.stack.push(U256::from(value))?;
                self.pc_increment(2);
                Ok(())
            },
            MUL => {
                arith_instructor!(self, *, 1);
                Ok(())
            },
            ADD => {
                arith_instructor!(self, +, 1);
                Ok(())
            },
            SUB => {
                arith_instructor!(self, -, 1);
                Ok(())
            },
            DIV => {
                arith_instructor!(self, /, 1);
                Ok(())
            },
            MOD => {
                arith_instructor!(self, %, 1);
                Ok(())
            },
            EQ => {
                bool_arith_instructor!(self, ==, 1);
                Ok(())
            },
            ADDMOD => {
                polynomial_arith_instructor!(self, +, %, 1);
                Ok(())
            },
            MULMOD => {
                polynomial_arith_instructor!(self, *, %, 1);
                Ok(())
            },
            ISZERO => {
                let num = self.stack.pop()?;
                if num == U256::zero() {
                    self.stack.push(U256::from(1u8))?;
                } else {
                    self.stack.push(U256::zero())?;
                };
                self.pc_increment(1);
                Ok(())
            },
            LT => {
                bool_arith_instructor!(self, >, 1);
                Ok(())
            },
            GT => {
                bool_arith_instructor!(self, <, 1);
                Ok(())
            },
            DUP1 => {
                dupn!(self, 1);
                Ok(())
            },
            DUP2 => {
                dupn!(self, 2);
                Ok(())
            },
            DUP3 => {
                dupn!(self, 3);
                Ok(())
            },
            DUP4 => {
                dupn!(self, 4);
                Ok(())
            },
            DUP5 => {
                dupn!(self, 5);
                Ok(())
            },
            DUP6 => {
                dupn!(self, 6);
                Ok(())
            },
            DUP7 => {
                dupn!(self, 7);
                Ok(())
            },
            DUP8 => {
                dupn!(self, 8);
                Ok(())
            },
            DUP9 => {
                dupn!(self, 10);
                Ok(())
            },
            DUP11 => {
                dupn!(self, 11);
                Ok(())
            },
            DUP12 => {
                dupn!(self, 12);
                Ok(())
            },
            DUP13 => {
                dupn!(self, 13);
                Ok(())
            },
            DUP14 => {
                dupn!(self, 14);
                Ok(())
            },
            DUP15 => {
                dupn!(self, 15);
                Ok(())
            },
            DUP16 => {
                dupn!(self, 16);
                Ok(())
            },
            // SWAP1 => {
            //     swapn!();
            //     Ok(())
            // },
            // SWAP2 => {
            //     swapn!();
            //     Ok(())
            // },
            // SWAP3 => {
            //     swapn!();
            //     Ok(())
            // },
            // SWAP4 => {
            //     swapn!();
            //     Ok(())
            // },
            // SWAP5 => {
            //     swapn!();
            //     Ok(())
            // },
            // SWAP6 => {
            //     swapn!();
            //     Ok(())
            // },
            // SWAP7 => {
            //     swapn!();
            //     Ok(())
            // },
            // SWAP8 => {
            //     swapn!();
            //     Ok(())
            // },
            // SWAP9 => {
            //     swapn!();
            //     Ok(())
            // },
            // SWAP10 => {
            //     swapn!();
            //     Ok(())
            // },
            // SWAP11 => {
            //     swapn!();
            //     Ok(())
            // },
            // SWAP12 => {
            //     swapn!();
            //     Ok(())
            // },
            // SWAP13 => {
            //     swapn!();
            //     Ok(())
            // },
            // SWAP14 => {
            //     swapn!();
            //     Ok(())
            // },
            // SWAP15 => {
            //     swapn!();
            //     Ok(())
            // },
            // SWAP16 => {
            //     swapn!();
            //     Ok(())
            // },
            POP => {
                self.stack.pop()?;
                self.pc_increment(1);
                Ok(())
            },
            _ => Err(eyre!("Unknown Opcode: {}", opcode))
        }
    }
}
