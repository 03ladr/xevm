pub mod lib;
use eyre::Result;
use hex;
use lib::memory::Memory;
use lib::stack::Stack;
use lib::execution::ExecutionContext;

fn main() -> Result<()> {
    let stack = Stack::init();
    let memory = Memory::init();
    let code = vec![0x60, 0x4, 0x60, 0x3, 0x01, 0x00];
    let mut executor = ExecutionContext::init(code, stack, memory);
    executor.run()?;
    Ok(())
}
