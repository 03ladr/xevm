use ethers::types::U256;
use eyre::{eyre, Result};

#[derive(Debug)]
pub struct Stack {
    pub storage: Vec<U256>,
}
impl Stack {
    pub fn init() -> Self {
        Stack { storage: Vec::with_capacity(1024) }
    }

    pub fn pop(&mut self) -> Result<U256> {
        match self.storage.pop() {
            Some(n) => {
                Ok(n)
            },
            None => Err(eyre!("Stack Underflow"))
        }
    }

    pub fn push(&mut self, value: U256) -> Result<()> {
        if self.storage.len() > 1024 {
            return Err(eyre!("Stack Overflow"));
            // self.storage.pop();
        };
        self.storage.push(value);
        Ok(())
    }
}
