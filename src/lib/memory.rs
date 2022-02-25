use super::statuscode::StatusCode;
use ethers::types::U256;

pub struct Memory {
    storage: Vec<u8>
}
impl Memory {
    pub fn init() -> Self {
        Memory { storage: Vec::with_capacity(4096) }
    }

    pub fn load(&mut self, offset: usize) -> Result<Vec<u8>, StatusCode> {
        if offset + 31 >= self.storage.len() {
            Err(StatusCode::InvalidMemoryAccess)
        } else {
            Ok(self.storage[offset..offset+32].to_vec())
        }
    }

    pub fn load_range(&mut self, offset: usize, length: usize) -> Result<Vec<u8>, StatusCode> {
        if offset + length - 1 >= self.storage.len() {
            Err(StatusCode::InvalidMemoryAccess)
        } else {
            Ok(self.storage[offset..offset+length].to_vec())
        }
    }

    pub fn store(&mut self, offset: usize, value: U256) -> Result<(), StatusCode> {
        if offset >= self.storage.len() {
            self.storage.resize(offset+32, 0);
        };
        value.to_big_endian(&mut self.storage[offset..offset+32]);
        if self.storage.len() % 32 != 0 { self.storage.resize((self.storage.len()|31)+1, 0);};
        Ok(())
    }


    pub fn load_full(&mut self) -> &Vec<u8> {
        &self.storage
    }

    pub fn len(&mut self) -> usize {
        self.storage.len()
    }
}
