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
        macro_rules! dupn {
            ( $idx:expr ) => {
                {
                    let val = self.stack.storage[self.stack.storage.len() - $idx];
                    self.stack.push(val);
                    self.pc_increment(1);
                    Ok(())
                }
            }
        }
        macro_rules! arith_eval {
            ( $op:tt ) => {
                {
                    let val1 = self.stack.pop()?;
                    let val2 = self.stack.pop()?;
                    let ret = val1.$op(val2).0;
                    self.stack.push(ret);
                    self.pc_increment(1);
                    Ok(())
                }
            }
        }
        macro_rules! checked_arith_eval {
            ( $op:tt ) => {
                {
                    let val1 = self.stack.pop()?;
                    let val2 = self.stack.pop()?;
                    let ret = val1.$op(val2).unwrap();
                    self.stack.push(ret);
                    self.pc_increment(1);
                    Ok(())
                }
            }
        }
        macro_rules! term_eval {
            ( $op:tt ) => {
                {
                    let val1 = self.stack.pop()?;
                    let val2 = self.stack.pop()?;
                    let ret = val1 $op val2;
                    self.stack.push(ret);
                    self.pc_increment(1);
                    Ok(())
                }
            }
        }
        macro_rules! bool_term_eval {
            ( $op:tt ) => {
                {
                    let val1 = self.stack.pop()?;
                    let val2 = self.stack.pop()?;
                    let mut ret = U256::zero();
                    let evaluation = val1 $op val2;
                    if evaluation == true { ret = U256::from(1u8) };
                    self.stack.push(ret);
                    self.pc_increment(1);
                    Ok(())
                }
            }
        }
        macro_rules! polynomial_term_eval {
            ( $op1:tt, $op2:tt ) => {
                {
                    let val1 = self.stack.pop()?;
                    let val2 = self.stack.pop()?;
                    let val3 = self.stack.pop()?;
                    let ret = (val1 $op1 val2) $op2 val3;
                    self.stack.push(ret);
                    self.pc_increment(1);
                    Ok(())
                }
            }
        }
        macro_rules! pushn {
            ( $n:expr ) => {
                {
                    let slice = &self.code[self.pc + 1..=self.pc + $n];
                    let ret = U256::from_big_endian(&slice);
                    self.stack.push(ret);
                    self.pc_increment($n + 1);
                    Ok(())
                }
            }
        }
        match opcode {
            PUSH1 => pushn!(1),
            PUSH2 => pushn!(2),
            PUSH3 => pushn!(3),
            PUSH4 => pushn!(4),
            PUSH5 => pushn!(5),
            PUSH6 => pushn!(6),
            PUSH7 => pushn!(7),
            PUSH8 => pushn!(8),
            PUSH9 => pushn!(9),
            PUSH10 => pushn!(10),
            PUSH11 => pushn!(11),
            PUSH12 => pushn!(12),
            PUSH13 => pushn!(13),
            PUSH14 => pushn!(14),
            PUSH15 => pushn!(15),
            PUSH16 => pushn!(16),
            POP => {
                self.stack.pop()?;
                self.pc_increment(1);
                Ok(())
            },
            DUP1 => dupn!(1),
            DUP2 => dupn!(2),
            DUP3 => dupn!(3),
            DUP4 => dupn!(4),
            DUP5 => dupn!(5),
            DUP6 => dupn!(6),
            DUP7 => dupn!(7),
            DUP8 => dupn!(8),
            DUP9 => dupn!(9),
            DUP10 => dupn!(10),
            DUP11 => dupn!(11),
            DUP12 => dupn!(12),
            DUP13 => dupn!(13),
            DUP14 => dupn!(14),
            DUP15 => dupn!(15),
            DUP16 => dupn!(16),
            // SWAP1 => swapn!(),
            // SWAP2 => swapn!(),
            // SWAP3 => swapn!(),
            // SWAP4 => swapn!(),
            // SWAP5 => swapn!(),
            // SWAP6 => swapn!(),
            // SWAP7 => swapn!(),
            // SWAP8 => swapn!(),
            // SWAP9 => swapn!(),
            // SWAP10 => swapn!(),
            // SWAP11 => swapn!(),
            // SWAP12 => swapn!(),
            // SWAP13 => swapn!(),
            // SWAP14 => swapn!(),
            // SWAP15 => swapn!(),
            // SWAP16 => swapn!(),
            MUL => arith_eval!(overflowing_mul),
            ADD => arith_eval!(overflowing_add),
            SUB => arith_eval!(overflowing_sub),
            DIV => checked_arith_eval!(checked_div),
            MOD => term_eval!(%),
            ADDMOD => polynomial_term_eval!(+, %),
            MULMOD => polynomial_term_eval!(*, %),
            EQ => bool_term_eval!(==),
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
            LT => bool_term_eval!(>),
            GT => bool_term_eval!(<),
            STOP => {
                self.stop();
                self.pc_increment(1);
                Ok(())
            },
            _ => Err(eyre!("Unknown Opcode: {}", opcode))
        }
    }
}
