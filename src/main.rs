pub mod lib;
use hex;
use lib::execution::ExecutionContext;
use lib::memory::Memory;
use lib::stack::Stack;
use lib::statuscode::StatusCode;
use std::env;

// cargo run {gas_limit} {code}
fn main() -> Result<(), StatusCode> {
    let args: Vec<String> = env::args().collect();
    let stack = Stack::init();
    let memory = Memory::init();
    let mut executor = ExecutionContext::init(
        hex::decode(&args[2]).unwrap(),
        stack,
        memory,
        str::parse::<usize>(&args[1]).unwrap(),
    );
    executor.run()
}
