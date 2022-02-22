pub mod lib;
use hex;
use lib::memory::Memory;
use lib::statuscode::StatusCode;
use lib::stack::Stack;
use lib::execution::ExecutionContext;

fn main() -> Result<(), StatusCode> {
    println!("Enter code to execute below:");
    let stack = Stack::init();
    let memory = Memory::init();
    let mut code_input = String::new();
    std::io::stdin().read_line(&mut code_input).unwrap();
    let mut executor = ExecutionContext::init(hex::decode(code_input.trim()).unwrap(), stack, memory);
    executor.run()
}
