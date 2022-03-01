// Stack
pub const PUSH1: u8 = 0x60;
pub const PUSH2: u8 = 0x61;
pub const PUSH3: u8 = 0x62;
pub const PUSH4: u8 = 0x63;
pub const PUSH5: u8 = 0x64;
pub const PUSH6: u8 = 0x65;
pub const PUSH7: u8 = 0x66;
pub const PUSH8: u8 = 0x67;
pub const PUSH9: u8 = 0x68;
pub const PUSH10: u8 = 0x69;
pub const PUSH11: u8 = 0x6A;
pub const PUSH12: u8 = 0x6B;
pub const PUSH13: u8 = 0x6C;
pub const PUSH14: u8 = 0x6D;
pub const PUSH15: u8 = 0x6E;
pub const PUSH16: u8 = 0x6F;
pub const POP: u8 = 0x50;
pub const DUP1: u8 = 0x80;
pub const DUP2: u8 = 0x81;
pub const DUP3: u8 = 0x82;
pub const DUP4: u8 = 0x83;
pub const DUP5: u8 = 0x84;
pub const DUP6: u8 = 0x85;
pub const DUP7: u8 = 0x86;
pub const DUP8: u8 = 0x87;
pub const DUP9: u8 = 0x88;
pub const DUP10: u8 = 0x89;
pub const DUP11: u8 = 0x8A;
pub const DUP12: u8 = 0x8B;
pub const DUP13: u8 = 0x8C;
pub const DUP14: u8 = 0x8D;
pub const DUP15: u8 = 0x8E;
pub const DUP16: u8 = 0x8F;
pub const SWAP1: u8 = 0x90;
pub const SWAP2: u8 = 0x91;
pub const SWAP3: u8 = 0x92;
pub const SWAP4: u8 = 0x93;
pub const SWAP5: u8 = 0x94;
pub const SWAP6: u8 = 0x95;
pub const SWAP7: u8 = 0x96;
pub const SWAP8: u8 = 0x97;
pub const SWAP9: u8 = 0x98;
pub const SWAP10: u8 = 0x99;
pub const SWAP11: u8 = 0x9A;
pub const SWAP12: u8 = 0x9B;
pub const SWAP13: u8 = 0x9C;
pub const SWAP14: u8 = 0x9D;
pub const SWAP15: u8 = 0x9E;
pub const SWAP16: u8 = 0x9F;
pub const MUL: u8 = 0x02;
pub const ADD: u8 = 0x01;
pub const SUB: u8 = 0x03;
pub const DIV: u8 = 0x04;
pub const EXP: u8 = 0x0A;
pub const SDIV: u8 = 0x05;
pub const MOD: u8 = 0x06;
pub const SMOD: u8 = 0x07;
pub const ADDMOD: u8 = 0x08;
pub const MULMOD: u8 = 0x09;
pub const EQ: u8 = 0x14;
pub const ISZERO: u8 = 0x15;
pub const AND: u8 = 0x16;
pub const OR: u8 = 0x17;
pub const XOR: u8 = 0x18;
pub const NOT: u8 = 0x19;
pub const GT: u8 = 0x11;
pub const LT: u8 = 0x10;
pub const SLT: u8 = 0x12;
pub const SGT: u8 = 0x13;
pub const SHL: u8 = 0x1B;
pub const SHR: u8 = 0x1C;
pub const SAR: u8 = 0x1D;
// Memory
pub const MLOAD: u8 = 0x51;
pub const MSTORE: u8 = 0x52;
pub const MSTORE8: u8 = 0x53;
pub const MSIZE: u8 = 0x59;
// Execution Context
pub const JUMP: u8 = 0x56;
pub const JUMPI: u8 = 0x57;
// Calldata
pub const CALLDATALOAD: u8 = 0x35;
pub const CALLDATASIZE: u8 = 0x36;
pub const CALLDATACOPY: u8 = 0x37;
// Other
pub const RETURN: u8 = 0xF3;
pub const STOP: u8 = 0x00;
pub const PC: u8 = 0x58;
pub const GAS: u8 = 0x5A;
pub const SHA3: u8 = 0x20;

// Gas fetcher
pub fn gas_fetch(key: u8) -> usize {
    match key {
        POP | PC | GAS | CALLDATASIZE | MSIZE => 2,
        PUSH1 | PUSH2 | PUSH3 | PUSH4 | PUSH5 | PUSH6 | PUSH7 | PUSH8 | PUSH9 | PUSH10 | PUSH11
        | PUSH12 | PUSH13 | PUSH14 | PUSH15 | PUSH16 | DUP1 | DUP2 | DUP3 | DUP4 | DUP5 | DUP6
        | DUP7 | DUP8 | DUP9 | DUP10 | DUP11 | DUP12 | DUP13 | DUP14 | DUP15 | DUP16 | SWAP1
        | SWAP2 | SWAP3 | SWAP4 | SWAP5 | SWAP6 | SWAP7 | SWAP8 | SWAP9 | SWAP10 | SWAP11 | XOR
        | SWAP12 | SWAP13 | SWAP14 | SWAP15 | SWAP16 | ADD | SUB | EQ | ISZERO | AND | OR
        | NOT | GT | LT | SGT | SLT | SHL | SHR | SAR | CALLDATALOAD => 3,
        MUL | DIV | SDIV | MOD | SMOD => 5,
        JUMP | ADDMOD | MULMOD => 8,
        JUMPI => 10,
        _ => 0,
    }
}
