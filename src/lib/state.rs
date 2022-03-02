use super::custom_type::{U256BE, U160};

// Block Object
pub struct Block {
    pub blocknumber: usize,
    pub basegas: usize,
    pub gaslimit: usize,
    pub difficulty: usize,
    pub nonce: usize,
    pub coinbase: U160,
    pub timestamp: U256BE,
    pub mixhash: U256BE,
    pub parenthash: U256BE,
    pub transactions: U256BE,
    pub stateroot: U256BE,
}
impl Block {
    // Initialize block with arbitrary values for testing
    pub fn default() -> Self {
        Block {
            blocknumber: 1,
            basegas: 21000,
            gaslimit: 8000000,
            difficulty: 6,
            timestamp: U256BE::from_u8(8),
            nonce: 7,
            coinbase: U256BE::from_u8(4).to_u160(),
            mixhash: U256BE::from_u8(2),
            parenthash: U256BE::from_u8(3),
            transactions: U256BE::from_u8(9),
            stateroot: U256BE::from_u8(5)
        }
    }
}

// // Transaction Object
// pub struct Transaction {
//     recipient: U160,
//     signature: U256BE,
//     value: usize,
//     data: U256BE,
//     gaslimit: usize,
//     maxprioritygas: usize,
//     maxgas: usize
// }
//
// // Log Object
// pub struct Log {
//     address: U160,
//     blockhash: U256BE,
//     blocknumber: usize,
//     data: Vec<u8>,
//     logindex: usize,
//     topics: Vec<U256BE>,
//     txhash: U256BE,
//     txindex: usize,
// }
//
// // Account Object
// pub struct Account {
//     nonce: usize,
//     balance: usize,
//     codehash: U256BE,
//     storageroot: U256BE
// }
