use super::custom_type::U256BE;
use super::memory::Memory;
use super::opcode::*;
use super::stack::Stack;
use super::statuscode::StatusCode;
use super::state::Block;
use ethers::types::{I256, U256};
use sha3::{Digest, Keccak256};

// EVM Execution Context
pub struct ExecutionContext {
    code: Vec<u8>,
    stack: Stack,
    memory: Memory,
    pc: usize,
    gas_limit: usize,
    stopped: bool,
    calldata: Vec<u8>,
    returndata: Vec<u8>,
    block: Block
}
impl ExecutionContext {
    // Initialize execution context
    pub fn init(code: Vec<u8>, stack: Stack, memory: Memory, gas_limit: usize) -> Self {
        ExecutionContext {
            code: code,
            stack: stack,
            memory: memory,
            pc: 0,
            gas_limit: gas_limit,
            stopped: false,
            calldata: Vec::with_capacity(1024),
            returndata: Vec::with_capacity(1024),
            block: Block::default()
        }
    }

    // Load 32-byte word from calldata at offset
    pub fn calldata_load(&mut self, offset: usize) -> Result<Vec<u8>, StatusCode> {
        let len_original = self.calldata.len();
        if len_original <= offset + 31 { self.calldata.resize(offset + 32, 0); };
        let ret = self.calldata[offset..offset + 32].to_vec();
        self.calldata.truncate(len_original);
        Ok(ret)
    }

    // Deduct gas from limit
    pub fn sub_gas(&mut self, by: usize) -> Result<(), StatusCode> {
        if by > self.gas_limit {
            return Err(StatusCode::OutOfGas);
        };
        self.gas_limit -= by;
        Ok(())
    }

    // Halt execution
    pub fn stop(&mut self) -> () {
        self.stopped = true
    }

    // Set program counter to destination
    pub fn pc_jump(&mut self, dest: usize) -> Result<(), StatusCode> {
        if dest >= self.code.len() { return Err(StatusCode::BadJumpDest); };
        self.pc = dest;
        Ok(())
    }

    // Increment program counter by value
    pub fn pc_increment(&mut self, val: usize) -> () {
        self.pc = self.pc + val
    }

    // Push value onto stack then increment program counter by 1
    pub fn stack_step_push(&mut self, val: U256BE) -> Result<(), StatusCode> {
        self.stack.push(val)?;
        self.pc_increment(1);
        Ok(())
    }

    // Read code at (program counter + offset)
    pub fn read_code(&mut self, offset: usize) -> Result<u8, StatusCode> {
        if self.pc + offset >= self.code.len() { return Err(StatusCode::Completion); };
        let value = self.code[self.pc + offset];
        Ok(value)
    }

    // Begin code execution
    pub fn run(&mut self) -> Result<(), StatusCode> {
        while !self.stopped {
            let opcode: u8 = self.read_code(0)?;
            println!(
                "[ Opcode: {} | PC: {} | Gas: {} ]",
                opcode, self.pc, self.gas_limit
            );
            match self.exec(opcode) {
                Err(e) => return Err(e),
                Ok(_) => (),
            };
            self.sub_gas(gas_fetch(opcode))?;
            println!(
                "Stack: {:?}\nMemory: {:?}",
                self.stack.peek_full(),
                self.memory.load_full()
            );
        }
        Ok(())
    }

    // Execute opcode
    pub fn exec(&mut self, opcode: u8) -> Result<(), StatusCode> {
        // Push n values onto stack
        macro_rules! pushn {
            ( $n:expr ) => {{
                let slice = &self.code[self.pc + 1..=self.pc + $n];
                let ret = U256BE::from_slice(slice);
                self.stack.push(ret)?;
                self.pc_increment($n + 1);
                Ok(())
            }};
        }
        // Duplicate value onto stack at index (len-n)
        macro_rules! dupn {
            ( $n:expr ) => {{
                let ret = self.stack.peek(self.stack.len() - $n)?;
                self.stack_step_push(ret)
            }};
        }
        // Swap 1st and nth stack items
        macro_rules! swapn {
            ( $n:expr ) => {{
                let top = self.stack.peek(0)?;
                let swp = self.stack.peek($n)?;
                self.stack.push_to(0, swp)?;
                self.stack.push_to($n, top)?;
                self.pc_increment(1);
                Ok(())
            }};
        }
        // Evaluate: stack[0].$operator(stack[1])
        macro_rules! arith_eval {
            ( $op:tt ) => {{
                let val1 = self.stack.pop()?.to_u256();
                let val2 = self.stack.pop()?.to_u256();
                let ret = val1.$op(val2);
                if ret.1 { self.stop(); return Err(StatusCode::Revert); };
                self.stack_step_push(U256BE::from_u256(ret.0))
            }};
        }
        // Evaluate: stack[0] $operator stack[1]
        macro_rules! term_eval {
            ( $op:tt ) => {{
                let val1 = self.stack.pop()?.to_u256();
                let val2 = self.stack.pop()?.to_u256();
                let ret = val1 $op val2;
                self.stack_step_push(U256BE::from_u256(ret))
            }};
        }
        // Evaluate: I256(stack[0]) $operator I256(stack[1])
        macro_rules! signed_term_eval {
            ( $op: tt ) => {{
                let val1 = I256::try_from(self.stack.pop()?.to_u256()).unwrap();
                let val2 = I256::try_from(self.stack.pop()?.to_u256()).unwrap();
                let ret = val1 $op val2;
                self.stack_step_push(U256BE::from_u256(U256::try_from(ret).unwrap()))
            }};
        }
        // Evaluate: I256(stack[0]) $operator I256(stack[1])
        macro_rules! signed_bool_term_eval {
            ( $op:tt ) => {{
                let val1 = I256::try_from(self.stack.pop()?.to_u256()).unwrap();
                let val2 = I256::try_from(self.stack.pop()?.to_u256()).unwrap();
                let mut ret = U256BE::zero();
                let evaluation = val1 $op val2;
                if evaluation { ret = U256BE::from_u8(1) };
                self.stack_step_push(ret)
            }};
        }
        // Evaluate: stack[0] $operator stack[1]
        macro_rules! bool_term_eval {
            ( $op:tt ) => {{
                let val1 = self.stack.pop()?.to_u256();
                let val2 = self.stack.pop()?.to_u256();
                let mut ret = U256BE::zero();
                let evaluation = val1 $op val2;
                if evaluation { ret = U256BE::from_u8(1) };
                self.stack_step_push(ret)
            }};
        }
        // Evaluate: (stack[0] $operator1 stack[1]) $operator2 stack[2]
        macro_rules! polynomial_term_eval {
            ( $op1:tt, $op2:tt ) => {{
                let val1 = self.stack.pop()?.to_u256();
                let val2 = self.stack.pop()?.to_u256();
                let val3 = self.stack.pop()?.to_u256();
                let ret = (val1 $op1 val2) $op2 val3;
                self.stack_step_push(U256BE::from_u256(ret))
            }};
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
            DIV => {
                let val1 = self.stack.pop()?.to_u256();
                let val2 = self.stack.pop()?.to_u256();
                let ret = val1.checked_div(val2).unwrap();
                self.stack_step_push(U256BE::from_u256(ret))
            },
            EXP => arith_eval!(overflowing_pow),
            SDIV => signed_term_eval!(/),
            MOD => term_eval!(%),
            SMOD => signed_term_eval!(%),
            ADDMOD => polynomial_term_eval!(+, %),
            MULMOD => polynomial_term_eval!(*, %),
            GT => bool_term_eval!(<),
            SGT => signed_bool_term_eval!(<),
            LT => bool_term_eval!(>),
            SLT => signed_bool_term_eval!(>),
            SHL => term_eval!(<<),
            SHR => term_eval!(>>),
            SAR => signed_term_eval!(>>),
            EQ => {
                let val1 = self.stack.pop()?;
                let val2 = self.stack.pop()?;
                self.stack_step_push(val1.eq(val2))
            },
            ISZERO => {
                let val = self.stack.pop()?;
                self.stack_step_push(val.eq(U256BE::zero()))
            },
            AND => {
                let val1 = self.stack.pop()?;
                let val2 = self.stack.pop()?;
                self.stack_step_push(val1.and(val2))
            },
            OR => {
                let val1 = self.stack.pop()?;
                let val2 = self.stack.pop()?;
                self.stack_step_push(val1.or(val2))
            },
            XOR => {
                let val1 = self.stack.pop()?;
                let val2 = self.stack.pop()?;
                self.stack_step_push(val1.xor(val2))
            },
            NOT => {
                let val = self.stack.pop()?;
                self.stack_step_push(val.not())
            },
            PC => {
                self.stack_step_push(U256BE::from_usize(self.pc))
            }
            GAS => {
                self.stack_step_push(U256BE::from_usize(self.gas_limit))
            },
            MLOAD => {
                let offset = self.stack.pop()?.to_usize();
                let loaded = self.memory.load(offset)?;
                self.stack_step_push(U256BE::from_slice(loaded.as_slice()))
            },
            MSTORE => {
                let offset = self.stack.pop()?.to_usize();
                let value = self.stack.pop()?;
                self.memory.store(offset, value)?;
                self.pc_increment(1);
                Ok(())
            },
            MSTORE8 => {
                let offset = self.stack.pop()?.to_usize();
                let value = self.stack.pop()?;
                self.memory.store(offset, value.and(U256BE::from_u8(0xFFu8)))?;
                self.pc_increment(1);
                Ok(())
            },
            MSIZE => {
                let memlen = self.memory.len();
                self.stack_step_push(U256BE::from_usize(memlen))
            },
            CALLDATALOAD => {
                let offset = self.stack.pop()?.to_usize();
                let loaded = self.calldata_load(offset)?;
                self.stack_step_push(U256BE::from_slice(loaded.as_slice()))
            },
            CALLDATASIZE => {
                self.stack_step_push(U256BE::from_usize(self.calldata.len()))
            },
            JUMP => { let dest = self.stack.pop()?; self.pc_jump(dest.to_usize()) },
            JUMPI => {
                let dest = self.stack.pop()?;
                let cond = self.stack.pop()?.to_u256();
                if cond.is_zero() { self.pc_increment(1); Ok(()) }
                else { self.pc_jump(dest.to_usize()) }
            },
            SHA3 => {
                let offset = self.stack.pop()?.to_usize();
                let length = self.stack.pop()?.to_usize();
                let value = self.memory.load_range(offset, length)?;
                let mut hasher = Keccak256::default();
                hasher.update(value.as_slice());
                let ret = U256BE::from_slice(hasher.finalize().to_vec().as_slice());
                self.stack_step_push(ret)
            },
            COINBASE => self.stack_step_push(self.block.coinbase.to_u256be()),
            TIMESTAMP => self.stack_step_push(self.block.timestamp),
            NUMBER => self.stack_step_push(U256BE::from_usize(self.block.blocknumber)),
            DIFFICULTY => self.stack_step_push(U256BE::from_usize(self.block.difficulty)),
            BASEFEE => self.stack_step_push(U256BE::from_usize(self.block.basegas)),
            GASLIMIT => self.stack_step_push(U256BE::from_usize(self.block.gaslimit)),
            RETURN => {
                let offset = self.stack.pop()?.to_usize();
                let length = self.stack.pop()?.to_usize();
                self.returndata = self.memory.load_range(offset, length)?;
                self.stop();
                println!("Return Data: {:?}", self.returndata);
                Err(StatusCode::Completion)
            },
            STOP => {
                self.stop();
                Err(StatusCode::Completion)
            },
            _ => Err(StatusCode::UndefinedInstruction),
        }
    }
}
