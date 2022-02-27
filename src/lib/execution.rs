use ethers::types::{U256, I256};
use sha3::{Digest, Keccak256};
use super::statuscode::StatusCode;
use super::opcode::*;
use super::memory::Memory;
use super::stack::Stack;

pub struct ExecutionContext {
    code: Vec<u8>,
    stack: Stack,
    memory: Memory,
    pc: usize,
    gas_limit: usize,
    stopped: bool,
    calldata: Vec<u8>,
    returndata: Vec<u8>
}
impl ExecutionContext {
    pub fn init(code: Vec<u8>, stack: Stack, memory: Memory, gas_limit: usize) -> Self {
        ExecutionContext {
            code: code,
            stack: stack,
            memory: memory,
            pc: 0,
            gas_limit: gas_limit,
            stopped: false,
            calldata: Vec::with_capacity(1024),
            returndata: Vec::with_capacity(1024)
        }
    }

    pub fn calldata_load(&mut self, offset: usize) -> Result<Vec<u8>, StatusCode> {
        let len_original = self.calldata.len();
        if len_original <= offset+31 { self.calldata.resize(offset + 32, 0); };
        let ret = self.calldata[offset..offset+32].to_vec();
        self.calldata.truncate(len_original);
        Ok(ret)
    }

    pub fn sub_gas(&mut self, by: usize) -> Result<(), StatusCode> {
        if by > self.gas_limit { return Err(StatusCode::OutOfGas); };
        self.gas_limit -= by;
        Ok(())
    }

    pub fn stop(&mut self) -> () {
        self.stopped = true
    }

    pub fn pc_jump(&mut self, dest: usize) -> Result<(), StatusCode> {
        if dest >= self.code.len() {
            return Err(StatusCode::BadJumpDest);
        };
        self.pc = dest;
        Ok(())
    }

    pub fn pc_increment(&mut self, idx: usize) -> () {
        self.pc = self.pc + idx
    }

    pub fn read_code(&mut self, idx: usize) -> Result<u8, StatusCode> {
        if self.pc + idx >= self.code.len() {
            return Err(StatusCode::Completion);
        };
        let value = self.code[self.pc + idx];
        Ok(value)
    }

    pub fn run(&mut self) -> Result<(), StatusCode> {
        while !self.stopped {
            let opcode: u8 = self.read_code(0)?;
            println!("Opcode: {} @ PC: {}", opcode, self.pc);
            match self.exec(opcode) {
                Err(e) => return Err(e),
                Ok(_) => ()
            };
            self.sub_gas(gas_fetch(opcode))?;
            println!("Stack: {:?}\nMemory Length: {}\nMemory: {:?}\nGas: {}", self.stack.peek_full(), self.memory.len(), self.memory.load_full(), self.gas_limit);
        }
        Ok(())
    }

    pub fn exec(&mut self, opcode: u8) -> Result<(), StatusCode> {
        macro_rules! pushn {
            ( $n:expr ) => {
                {
                    let slice = &self.code[self.pc + 1..=self.pc + $n];
                    let ret = U256::from_big_endian(&slice);
                    self.stack.push(ret)?;
                    self.pc_increment($n + 1);
                    Ok(())
                }
            }
        }
        macro_rules! dupn {
            ( $n:expr ) => {
                {
                    let ret = self.stack.peek(self.stack.len() - $n)?;
                    self.stack.push(ret)?;
                    self.pc_increment(1);
                    Ok(())
                }
            }
        }
        macro_rules! swapn {
            ( $n:expr ) => {
                {
                    let top = self.stack.peek(0)?;
                    let swp = self.stack.peek($n)?;
                    self.stack.push_to(0, swp)?;
                    self.stack.push_to($n, top)?;
                    self.pc_increment(1);
                    Ok(())
                }
            }
        }
        macro_rules! arith_eval {
            ( $op:tt ) => {
                {
                    let val1: U256 = self.stack.pop()?;
                    let val2: U256 = self.stack.pop()?;
                    let ret = val1.$op(val2);
                    if ret.1 { self.stop(); return Err(StatusCode::Revert); };
                    self.stack.push(ret.0)?;
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
                    self.stack.push(ret)?;
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
                    self.stack.push(ret)?;
                    self.pc_increment(1);
                    Ok(())
                }
            }
        }
        macro_rules! signed_term_eval {
            ( $op: tt ) => {
                {
                    let val1 = I256::try_from(self.stack.pop()?).unwrap();
                    let val2 = I256::try_from(self.stack.pop()?).unwrap();
                    let ret = val1 $op val2;
                    self.stack.push(U256::try_from(ret).unwrap())?;
                    self.pc_increment(1);
                    Ok(())
                }
            }
        }
        macro_rules! signed_bool_term_eval {
            ( $op:tt ) => {
                {
                    let val1 = I256::try_from(self.stack.pop()?).unwrap();
                    let val2 = I256::try_from(self.stack.pop()?).unwrap();
                    let mut ret = U256::zero();
                    let evaluation = val1 $op val2;
                    if evaluation { ret = U256::from(1u8) };
                    self.stack.push(ret)?;
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
                    if evaluation { ret = U256::from(1u8) };
                    self.stack.push(ret)?;
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
                    self.stack.push(ret)?;
                    self.pc_increment(1);
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
            POP => { self.stack.pop()?; self.pc_increment(1); Ok(()) },
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
            SWAP1 => swapn!(1),
            SWAP2 => swapn!(2),
            SWAP3 => swapn!(3),
            SWAP4 => swapn!(4),
            SWAP5 => swapn!(5),
            SWAP6 => swapn!(6),
            SWAP7 => swapn!(7),
            SWAP8 => swapn!(8),
            SWAP9 => swapn!(9),
            SWAP10 => swapn!(10),
            SWAP11 => swapn!(11),
            SWAP12 => swapn!(12),
            SWAP13 => swapn!(13),
            SWAP14 => swapn!(14),
            SWAP15 => swapn!(15),
            SWAP16 => swapn!(16),
            MUL => arith_eval!(overflowing_mul),
            ADD => arith_eval!(overflowing_add),
            SUB => arith_eval!(overflowing_sub),
            DIV => checked_arith_eval!(checked_div),
            EXP => arith_eval!(overflowing_pow),
            SDIV => signed_term_eval!(/),
            MOD => term_eval!(%),
            SMOD => signed_term_eval!(%),
            ADDMOD => polynomial_term_eval!(+, %),
            MULMOD => polynomial_term_eval!(*, %),
            EQ => bool_term_eval!(==),
            ISZERO => {
                let val = self.stack.pop()?;
                if val == U256::zero() { self.stack.push(U256::from(1u8))?; }
                else { self.stack.push(U256::zero())?; };
                self.pc_increment(1);
                Ok(())
            },
            AND => term_eval!(&),
            OR => term_eval!(|),
            XOR => term_eval!(^),
            NOT => { let val = self.stack.pop()?; self.stack.push(!val)?; self.pc_increment(1); Ok(()) },
            GT => bool_term_eval!(<),
            SGT => signed_bool_term_eval!(<),
            LT => bool_term_eval!(>),
            SLT => signed_bool_term_eval!(>),
            SHL => term_eval!(<<),
            SHR => term_eval!(>>),
            SAR => signed_term_eval!(>>),
            PC => { self.stack.push(U256::from(self.pc))?; self.pc_increment(1); Ok(()) },
            GAS => { self.stack.push(U256::from(self.gas_limit))?; self.pc_increment(1); Ok(()) }
            MLOAD => {
                let offset = self.stack.pop()?.as_usize();
                let loaded = self.memory.load(offset)?;
                self.stack.push(U256::from_big_endian(loaded.as_slice()))?;
                self.pc_increment(1);
                Ok(())
            },
            MSTORE => {
                let offset = self.stack.pop()?.as_usize();
                let value = self.stack.pop()?;
                self.memory.store(offset, value)?;
                self.pc_increment(1);
                Ok(())
            },
            MSTORE8 => {
                let offset = self.stack.pop()?.as_usize();
                let value = self.stack.pop()?;
                self.memory.store(offset, value & U256::from(0xFF))?;
                self.pc_increment(1);
                Ok(())
            },
            CALLDATALOAD => {
                let offset = self.stack.pop()?.as_usize();
                let loaded = self.calldata_load(offset)?;
                self.stack.push(U256::from_big_endian(loaded.as_slice()))?;
                self.pc_increment(1);
                Ok(())
            },
            CALLDATASIZE => {
                self.stack.push(U256::from(self.calldata.len()))?;
                self.pc_increment(1);
                Ok(())
            },
            JUMP => {
                let dest = self.stack.pop()?;
                self.pc_jump(dest.as_usize())
            },
            JUMPI => {
                let dest = self.stack.pop()?;
                let cond = self.stack.pop()?;
                if cond.is_zero() { self.pc_increment(1); Ok(()) }
                else { self.pc_jump(dest.as_usize()) }
            },
            SHA3 => {
                let offset = self.stack.pop()?.as_usize();
                let length = self.stack.pop()?.as_usize();
                let value = self.memory.load_range(offset, length)?;
                let mut hasher = Keccak256::default();
                hasher.update(value.as_slice());
                let ret = U256::from(hasher.finalize().to_vec().as_slice());
                self.stack.push(ret)?;
                self.pc_increment(1);
                Ok(())
            },
            RETURN => {
                let offset = self.stack.pop()?.as_usize();
                let length = self.stack.pop()?.as_usize();
                self.returndata = self.memory.load_range(offset, length)?;
                self.stop();
                println!("Return Data: {:?}", self.returndata);
                Err(StatusCode::Completion)
            },
            STOP => {
                self.stop();
                Err(StatusCode::Completion)
            },
            _ => Err(StatusCode::UndefinedInstruction)
        }
    }
}
