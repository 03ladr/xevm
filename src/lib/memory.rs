use super::statuscode::StatusCode;
use ethers::types::U256;

pub struct Memory {
    pub storage: Vec<u8>
}
impl Memory {
    pub fn init() -> Self {
        Memory { storage: Vec::with_capacity(4096) }
    }

    pub fn load(&mut self, offset: usize) -> Result<Vec<u8>, StatusCode> {
        let len_original = self.storage.len();
        if offset + 31 >= len_original {
            self.storage.resize((offset+len_original|31)+1, 0);
            // Err(StatusCode::InvalidMemoryAccess)
        };
        let ret = self.storage[offset..offset+32].to_vec();
        self.storage.truncate(len_original);
        Ok(ret)
    }

    pub fn load_range(&mut self, offset: usize, length: usize) -> Result<Vec<u8>, StatusCode> {
        let len_original = self.storage.len();
        if offset + length >= len_original {
            self.storage.resize((offset+length+len_original|31)+1, 0);
            // Err(StatusCode::InvalidMemoryAccess)
        };
        let ret = self.storage[offset..offset+length].to_vec();
        self.storage.truncate(len_original);
        Ok(ret)
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
