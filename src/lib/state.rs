use super::custom_type::{U256BE, U160};

// Block Object
pub struct Block {
    timestamp: U256BE,
    blocknumber: usize,
    basegas: usize,
    difficulty: usize,
    mixhash: U256BE,
    parenthash: U256BE,
    transactions: U256BE,
    stateroot: U256BE,
    nonce: usize
}
impl Block {
    // Initialize block with arbitrary values for testing
    pub fn default() -> Self {
        Block {
            timestamp: U256BE::zero(),
            blocknumber: 0,
            basegas: 21000,
            difficulty: 0,
            mixhash: U256BE::zero(),
            parenthash: U256BE::zero(),
            transactions: U256BE::zero(),
            stateroot: U256BE::zero(),
            nonce: 0
        }
    }
}

// Transaction Object
pub struct Transaction {
    recipient: U160,
    signature: U256BE,
    value: usize,
    data: U256BE,
    gaslimit: usize,
    maxprioritygas: usize,
    maxgas: usize
}

// Log Object
pub struct Log {
    address: U160,
    blockhash: U256BE,
    blocknumber: usize,
    data: Vec<u8>,
    logindex: usize,
    topics: Vec<U256BE>,
    txhash: U256BE,
    txindex: usize,
}

// Account Object
pub struct Account {
    nonce: usize,
    balance: usize,
    codehash: U256BE,
    storageroot: U256BE
}
