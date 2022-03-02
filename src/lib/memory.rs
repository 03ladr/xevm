use super::custom_type::U256BE;
use super::statuscode::StatusCode;

pub struct Memory {
    // Vector of unsigned 8-bit integers to represent EVM memory
    storage: Vec<u8>,
}
impl Memory {
    // Initialize memory with length 4096
    pub fn init() -> Self {
        Memory {
            storage: Vec::with_capacity(4096),
        }
    }

    // Load 32-byte word from memory at offset
    pub fn load(&mut self, offset: usize) -> Result<Vec<u8>, StatusCode> {
        let len_original = self.storage.len();
        if offset + 31 >= len_original {
            self.storage.resize((offset + len_original | 31) + 1, 0);
        };
        let ret = self.storage[offset..offset + 32].to_vec();
        self.storage.truncate(len_original);
        Ok(ret)
    }

    // Load bytes from memory within range
    pub fn load_range(&mut self, offset: usize, length: usize) -> Result<Vec<u8>, StatusCode> {
        let len_original = self.storage.len();
        if offset + length >= len_original {
            self.storage.resize((offset + length + len_original | 31) + 1, 0);
        };
        let ret = self.storage[offset..offset + length].to_vec();
        self.storage.truncate(len_original);
        Ok(ret)
    }

    // Store 32-byte word in memory at offset
    pub fn store(&mut self, offset: usize, value: U256BE) -> Result<(), StatusCode> {
        if offset >= self.storage.len() { self.storage.resize(offset + 32, 0); };
        self.storage[offset..offset + 32].clone_from_slice(&value.as_slice());
        if self.storage.len() % 32 != 0 {
            self.storage.resize((self.storage.len() | 31) + 1, 0);
        };
        Ok(())
    }

    // Return reference to memory vector
    pub fn load_full(&mut self) -> &Vec<u8> {
        &self.storage
    }

    // Return length of memory
    pub fn len(&mut self) -> usize {
        self.storage.len()
    }
}
