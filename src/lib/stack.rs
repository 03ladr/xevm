use super::custom_type::U256BE;
use super::statuscode::StatusCode;

pub struct Stack {
    // Vector of big endian u256 words to represent EVM stack
    storage: Vec<U256BE>,
}
impl Stack {
    // Initialize stack with length 1024
    pub fn init() -> Self {
        Stack {
            storage: Vec::with_capacity(1024),
        }
    }

    // Push U256BE value onto stack
    pub fn push(&mut self, value: U256BE) -> Result<(), StatusCode> {
        if self.storage.len() > 1024 {
            return Err(StatusCode::StackOverflow);
        };
        self.storage.push(value);
        Ok(())
    }

    // Pop U256BE value off stack
    pub fn pop(&mut self) -> Result<U256BE, StatusCode> {
        match self.storage.pop() {
            Some(n) => Ok(n),
            None => Err(StatusCode::StackUnderflow),
        }
    }

    // Push U256BE value onto stack at index
    pub fn push_to(&mut self, idx: usize, value: U256BE) -> Result<(), StatusCode> {
        if self.storage.len() > 1024 {
            return Err(StatusCode::StackOverflow);
        } else if idx >= self.storage.len() {
            return Err(StatusCode::ArgOutOfRange);
        };
        self.storage.push(value);
        Ok(())
    }

    // Return U256BE value from stack at index
    pub fn peek(&self, idx: usize) -> Result<U256BE, StatusCode> {
        if idx >= self.storage.len() {
            return Err(StatusCode::ArgOutOfRange);
        };
        Ok(self.storage[idx])
    }

    // Return length of stack
    pub fn len(&self) -> usize {
        self.storage.len()
    }

    // Return reference to stack vector
    pub fn peek_full(&mut self) -> &Vec<U256BE> {
        &self.storage
    }
}
