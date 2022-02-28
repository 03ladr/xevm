use ethers::types::{U256, I256};

#[derive(Debug, Clone, Copy)]
pub struct U256BE([u8; 32]);
impl U256BE {
    pub fn as_slice(&self) -> [u8; 32] {
        self.0
    }

    pub fn as_usize(&self) -> usize {
        let last: [u8; 8] = self.0[24..=31].try_into().unwrap();
        usize::from_be_bytes(last)
    }

    pub fn to_U256(&self) -> U256 {
        U256::from_big_endian(&self.0)
    }

    pub fn from_slice(slice: &[u8]) -> Self {
        let mut array: [u8; 32] = [0; 32];
        array[32-slice.len()..].clone_from_slice(slice);
        U256BE(array)
    }

    pub fn from_u8(value: u8) -> Self {
        let mut array: [u8; 32] = [0; 32];
        array[31] = value;
        U256BE(array)
    }

    pub fn from_usize(value: usize) -> Self {
        let mut array: [u8; 32] = [0; 32];
        array[23..31].clone_from_slice(&value.to_be_bytes());
        U256BE(array)
    }

    pub fn from_U256(value: U256) -> Self {
        let mut array: [u8; 32] = [0; 32];
        value.to_big_endian(&mut array);
        U256BE(array)
    }

    pub fn zero() -> Self {
        U256BE([0; 32])
    }
}
