use ethnum::{u256, i256};

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
    pub fn to_u256(self) -> u256 {
        u256::from_be_bytes(self.0)
    }

    // Convert self to i256
    pub fn to_i256(self) -> i256 {
        i256::from_be_bytes(self.0)
    }

    // Convert self to u32
    pub fn to_u32(self) -> u32 {
        let ret: [u8; 4] = self.0[28..=31].try_into().unwrap();
        u32::from_be_bytes(ret)
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
        self.0
            .into_iter()
            .enumerate()
            .for_each(|(idx, x)| ret[idx] = x & value.0[idx]);
        U256BE(ret)
    }

    // OR bitwise operator: self | value
    pub fn or(self, value: U256BE) -> Self {
        let mut ret = [0u8; 32];
        self.0
            .into_iter()
            .enumerate()
            .for_each(|(idx, x)| ret[idx] = x | value.0[idx]);
        U256BE(ret)
    }

    // XOR bitwise operator: self ^ value
    pub fn xor(self, value: U256BE) -> Self {
        let mut ret = [0u8; 32];
        self.0
            .into_iter()
            .enumerate()
            .for_each(|(idx, x)| ret[idx] = x ^ value.0[idx]);
        U256BE(ret)
    }

    // GT operator: self > value
    pub fn gt(self, value: U256BE) -> Self {
        if self.0 > value.0 { U256BE::from_u8(1) }
        else { U256BE([0;32]) }
    }

    // LT operator: self < value
    pub fn lt(self, value: U256BE) -> Self {
        if self.0 < value.0 { U256BE::from_u8(1) }
        else { U256BE([0;32]) }
    }

    // SGT operator: int256<self> > int256<value>
    pub fn sgt(self, value: U256BE) -> Self {
        if i256::from_be_bytes(self.0) > i256::from_be_bytes(value.0) {
            U256BE::from_u8(1)
        } else { U256BE([0;32]) }
    }

    // SLT operator: int256<self> < <int256<value>
    pub fn slt(self, value: U256BE) -> Self {
        if i256::from_be_bytes(self.0) < i256::from_be_bytes(value.0) {
            U256BE::from_u8(1)
        } else { U256BE([0;32]) }
    }

    // SHL operator: int256<self> << int256<value>
    pub fn shl(self, value: U256BE) -> Self {
        U256BE::from_u256(u256::from_be_bytes(self.0) << u256::from_be_bytes(value.0))
    } // Todo: Must overflow

    // SHR operator: int256<self> << int256<value>
    pub fn shr(self, value: U256BE) -> Self {
        U256BE::from_u256(u256::from_be_bytes(self.0) >> u256::from_be_bytes(value.0))
    } // Todo: Must overflow

    // SAR operator: int256<self> >> int256<value>
    pub fn sar(self, value: U256BE) -> Self {
        U256BE::from_i256(i256::from_be_bytes(self.0) >> i256::from_be_bytes(value.0))
    } // Todo:  Must overflow

    // Equivalence operator: self == value
    pub fn eq(self, value: U256BE) -> bool {
        if self.0 == value.0 { true }
        else { false }
    }

    // Equivalence operator: self == value, return U256BE 1/0
    pub fn uint_eq(self, value: U256BE) -> Self {
        if self.0 == value.0 { U256BE::from_u8(1) }
        else { U256BE::zero() }
    }

    // Returns whether self is equal to [0;32]
    pub fn is_zero(self) -> bool {
        if self.0 == [0; 32] { true }
        else { false }
    }

    // Returns whether self is equal to [0;32], return U256BE 1/0
    pub fn uint_is_zero(self) -> Self {
        if self.0 == [0; 32] { U256BE::from_u8(1) }
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

    // Initialize U256BE from u256
    pub fn from_u256(value: u256) -> Self {
        let mut ret: [u8; 32] = [0; 32];
        ret[..].clone_from_slice(&value.to_be_bytes());
        U256BE(ret)
    }

    // Initialize U256BE from i256
    pub fn from_i256(value: i256) -> Self {
        let mut ret: [u8; 32] = [0; 32];
        ret[..].clone_from_slice(&value.to_be_bytes());
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
    // Convert self to U256BE
    pub fn to_u256_be(self) -> U256BE {
        U256BE::from_slice(&self.0)
    }
}
