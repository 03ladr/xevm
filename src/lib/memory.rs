use ethers::types::U256;
use eyre::{eyre, Result};

#[derive(Debug)]
pub struct Memory {
    storage: Vec<U256>
}
impl Memory {
    pub fn init() -> Self {
        Memory { storage: vec![U256::zero(); 1024] }
    }

    pub fn store(&mut self, offset: usize, value: U256) -> Result<()> {
        if offset >= self.storage.len() {
            self.storage.resize(offset, U256::zero());
        };
        self.storage[offset - 1] = value;
        Ok(())
    }

    pub fn load(&mut self, offset: usize) -> Result<U256> {
        if offset >= self.storage.len() {
            Err(eyre!("Memory index out of bounds"))
        } else {
            Ok(self.storage[offset])
        }
    }
}
