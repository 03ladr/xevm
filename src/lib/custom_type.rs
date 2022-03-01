use ethers::types::U256;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct U256BE([u8; 32]);
impl U256BE {
    pub fn as_slice(self) -> [u8; 32] {
        self.0
    }

    pub fn as_usize(self) -> usize {
        let ret: [u8; 8] = self.0[24..=31].try_into().unwrap();
        usize::from_be_bytes(ret)
    }

    pub fn to_u256(self) -> U256 {
        U256::from_big_endian(&self.0)
    }

    pub fn not(self) -> Self {
        let mut ret = [0u8; 32];
        self.0
            .into_iter()
            .enumerate()
            .for_each(|(idx, x)| ret[idx] = !x);
        U256BE(ret)
    }

    pub fn and(self, value: U256BE) -> Self {
        let mut ret = [0u8; 32];
        self.0.into_iter().enumerate().for_each(|(idx, x)| ret[idx] = x & value.0[idx]);
        U256BE(ret)
    }

    pub fn or(self, value: U256BE) -> Self {
        let mut ret = [0u8; 32];
        self.0.into_iter().enumerate().for_each(|(idx, x)| ret[idx] = x | value.0[idx]);
        U256BE(ret)
    }

    pub fn xor(self, value: U256BE) -> Self {
        let mut ret = [0u8; 32];
        self.0.into_iter().enumerate().for_each(|(idx, x)| ret[idx] = x ^ value.0[idx]);
        U256BE(ret)
    }


    pub fn from_slice(slice: &[u8]) -> Self {
        let mut ret: [u8; 32] = [0; 32];
        ret[32 - slice.len()..].clone_from_slice(slice);
        U256BE(ret)
    }

    pub fn from_u8(value: u8) -> Self {
        let mut ret: [u8; 32] = [0; 32];
        ret[31] = value;
        U256BE(ret)
    }

    pub fn from_usize(value: usize) -> Self {
        let mut ret: [u8; 32] = [0; 32];
        ret[24..=31].clone_from_slice(&value.to_be_bytes());
        U256BE(ret)
    }

    pub fn from_u256(value: U256) -> Self {
        let mut ret: [u8; 32] = [0; 32];
        value.to_big_endian(&mut ret);
        U256BE(ret)
    }

    pub fn zero() -> Self {
        U256BE([0; 32])
    }
}
