pub mod lib;
use eyre::Result;
use hex;
use lib::memory::Memory;
use lib::stack::Stack;
use lib::execution::ExecutionContext;

fn main() -> Result<()> {
    let stack = Stack::init();
    let memory = Memory::init();
    let mut code_input = String::new();
    std::io::stdin().read_line(&mut code_input);
    let mut executor = ExecutionContext::init(hex::decode(code_input.trim())?, stack, memory);
    executor.run()?;
    Ok(())
}
