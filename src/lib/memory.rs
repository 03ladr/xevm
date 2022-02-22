use ethers::types::U256;
use super::statuscode::StatusCode;

#[derive(Debug)]
pub struct Memory {
    storage: Vec<U256>
}
impl Memory {
    pub fn init() -> Self {
        Memory { storage: Vec::with_capacity(4096) }
    }

    pub fn store(&mut self, offset: usize, value: U256) -> Result<(), StatusCode> {
        if offset >= self.storage.len() {
            self.storage.resize(offset + 1, U256::zero());
        };
        self.storage[offset] = value;
        Ok(())
    }

    pub fn load(&mut self, offset: usize) -> Result<U256, StatusCode> {
        if offset >= self.storage.len() {
            Err(StatusCode::InvalidMemoryAccess)
        } else {
            Ok(self.storage[offset])
        }
    }

    pub fn load_range(&mut self, offset: usize, length: usize) -> Result<Vec<U256>, StatusCode> {
        if offset >= self.storage.len() {
            Err(StatusCode::InvalidMemoryAccess)
        } else {
            Ok(self.storage[offset..offset + length].to_vec())
        }
    }

    pub fn load_full(&mut self) -> &Vec<U256> {
        &self.storage
    }
}
