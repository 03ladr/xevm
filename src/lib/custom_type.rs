use ethers::types::U256;

#[derive(Debug, Clone, Copy)]
// Big endian u256 type
pub struct U256BE([u8; 32]);
impl U256BE {
    // Return self as slice
    pub fn as_slice(self) -> [u8; 32] {
        self.0
    }

    // Convert self to usize
    pub fn to_usize(self) -> usize {
        let ret: [u8; 8] = self.0[24..=31].try_into().unwrap();
        usize::from_be_bytes(ret)
    }

    // Convert self to u160
    pub fn to_u160(self) -> U160 {
        U160(self.0[12..].try_into().unwrap())
    }

    // Convert self to u256
    pub fn to_u256(self) -> U256 {
        U256::from_big_endian(&self.0)
    }

    // NOT bitwise operator: !self
    pub fn not(self) -> Self {
        let mut ret = [0u8; 32];
        self.0
            .into_iter()
            .enumerate()
            .for_each(|(idx, x)| ret[idx] = !x);
        U256BE(ret)
    }

    // AND bitwise operator: self & value
    pub fn and(self, value: U256BE) -> Self {
        let mut ret = [0u8; 32];
        self.0.into_iter().enumerate().for_each(|(idx, x)| ret[idx] = x & value.0[idx]);
        U256BE(ret)
    }

    // OR bitwise operator: self | value
    pub fn or(self, value: U256BE) -> Self {
        let mut ret = [0u8; 32];
        self.0.into_iter().enumerate().for_each(|(idx, x)| ret[idx] = x | value.0[idx]);
        U256BE(ret)
    }

    // XOR bitwise operator: self ^ value
    pub fn xor(self, value: U256BE) -> Self {
        let mut ret = [0u8; 32];
        self.0.into_iter().enumerate().for_each(|(idx, x)| ret[idx] = x ^ value.0[idx]);
        U256BE(ret)
    }

    // Equivalence operator: self == value
    pub fn eq(self, value: U256BE) -> Self {
        if self.0 == value.0 { U256BE::from_u8(1) }
        else { U256BE::zero() }
    }

    // Initialize U256BE from slice
    pub fn from_slice(slice: &[u8]) -> Self {
        let mut ret: [u8; 32] = [0; 32];
        ret[32 - slice.len()..].clone_from_slice(slice);
        U256BE(ret)
    }

    // Initialize U256BE from u8
    pub fn from_u8(value: u8) -> Self {
        let mut ret: [u8; 32] = [0; 32];
        ret[31] = value;
        U256BE(ret)
    }

    // Initialize U256BE from usize
    pub fn from_usize(value: usize) -> Self {
        let mut ret: [u8; 32] = [0; 32];
        ret[24..=31].clone_from_slice(&value.to_be_bytes());
        U256BE(ret)
    }

    // Initialize U256BE from Ethers U256
    pub fn from_u256(value: U256) -> Self {
        let mut ret: [u8; 32] = [0; 32];
        value.to_big_endian(&mut ret);
        U256BE(ret)
    }

    // Return self as zero
    pub fn zero() -> Self {
        U256BE([0; 32])
    }
}

#[derive(Clone, Copy)]
// Big endian u160 type
pub struct U160([u8; 20]);
impl U160 {
    // Convert self to u256be
    pub fn to_u256be(self) -> U256BE {
        U256BE::from_slice(&self.0)
    }
}
