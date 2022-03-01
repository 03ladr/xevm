use super::custom_type::U256BE;
use super::statuscode::StatusCode;

pub struct Stack {
    storage: Vec<U256BE>,
}
impl Stack {
    pub fn init() -> Self {
        Stack {
            storage: Vec::with_capacity(1024),
        }
    }

    pub fn pop(&mut self) -> Result<U256BE, StatusCode> {
        match self.storage.pop() {
            Some(n) => Ok(n),
            None => Err(StatusCode::StackUnderflow),
        }
    }

    pub fn len(&self) -> usize {
        self.storage.len()
    }

    pub fn push(&mut self, value: U256BE) -> Result<(), StatusCode> {
        if self.storage.len() > 1024 {
            return Err(StatusCode::StackOverflow);
        };
        self.storage.push(value);
        Ok(())
    }

    pub fn push_to(&mut self, idx: usize, value: U256BE) -> Result<(), StatusCode> {
        if self.storage.len() > 1024 {
            return Err(StatusCode::StackOverflow);
        } else if idx >= self.storage.len() {
            return Err(StatusCode::ArgOutOfRange);
        };
        self.storage.push(value);
        Ok(())
    }

    pub fn peek(&self, idx: usize) -> Result<U256BE, StatusCode> {
        if idx >= self.storage.len() {
            return Err(StatusCode::ArgOutOfRange);
        };
        Ok(self.storage[idx])
    }

    pub fn peek_full(&mut self) -> &Vec<U256BE> {
        &self.storage
    }
}
