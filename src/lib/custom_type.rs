use ethers::types::U256;

#[derive(Debug, Clone, Copy, PartialEq)]
// Big endian u256 type
pub struct U256BE([u8; 32]);
impl U256BE {
    pub fn as_slice(self) -> [u8; 32] {
        // Return self as slice
        self.0
    }

    pub fn as_usize(self) -> usize {
        // Return self as usize
        let ret: [u8; 8] = self.0[24..=31].try_into().unwrap();
        usize::from_be_bytes(ret)
    }

    pub fn to_u256(self) -> U256 {
        // Convert self to u256 then return
        U256::from_big_endian(&self.0)
    }

    pub fn not(self) -> Self {
        // NOT bitwise operation: !self
        let mut ret = [0u8; 32];
        self.0
            .into_iter()
            .enumerate()
            .for_each(|(idx, x)| ret[idx] = !x);
        U256BE(ret)
    }

    pub fn and(self, value: U256BE) -> Self {
        // AND bitwise operation: self & value
        let mut ret = [0u8; 32];
        self.0.into_iter().enumerate().for_each(|(idx, x)| ret[idx] = x & value.0[idx]);
        U256BE(ret)
    }

    pub fn or(self, value: U256BE) -> Self {
        // OR bitwise operation: self | value
        let mut ret = [0u8; 32];
        self.0.into_iter().enumerate().for_each(|(idx, x)| ret[idx] = x | value.0[idx]);
        U256BE(ret)
    }

    pub fn xor(self, value: U256BE) -> Self {
        // XOR bitwise operation: self ^ value
        let mut ret = [0u8; 32];
        self.0.into_iter().enumerate().for_each(|(idx, x)| ret[idx] = x ^ value.0[idx]);
        U256BE(ret)
    }

    pub fn from_slice(slice: &[u8]) -> Self {
        // Initialize U256BE from slice
        let mut ret: [u8; 32] = [0; 32];
        ret[32 - slice.len()..].clone_from_slice(slice);
        U256BE(ret)
    }

    pub fn from_u8(value: u8) -> Self {
        // Initialize U256BE from u8
        let mut ret: [u8; 32] = [0; 32];
        ret[31] = value;
        U256BE(ret)
    }

    pub fn from_usize(value: usize) -> Self {
        // Initialize U256BE from usize
        let mut ret: [u8; 32] = [0; 32];
        ret[24..=31].clone_from_slice(&value.to_be_bytes());
        U256BE(ret)
    }

    pub fn from_u256(value: U256) -> Self {
        // Initialize U256BE from Ethers U256
        let mut ret: [u8; 32] = [0; 32];
        value.to_big_endian(&mut ret);
        U256BE(ret)
    }

    pub fn zero() -> Self {
        // Return self as zero
        U256BE([0; 32])
    }
}
