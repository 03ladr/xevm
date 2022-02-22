use ethers::types::U256;
use super::statuscode::StatusCode;

#[derive(Debug)]
pub struct Stack {
    pub storage: Vec<U256>,
}
impl Stack {
    pub fn init() -> Self {
        Stack { storage: Vec::with_capacity(1024) }
    }

    pub fn pop(&mut self) -> Result<U256, StatusCode> {
        match self.storage.pop() {
            Some(n) => {
                Ok(n)
            },
            None => Err(StatusCode::StackUnderflow)
        }
    }

    pub fn push(&mut self, value: U256) -> Result<(), StatusCode> {
        if self.storage.len() > 1024 {
            return Err(StatusCode::StackUnderflow)
        };
        self.storage.push(value);
        Ok(())
    }
}
