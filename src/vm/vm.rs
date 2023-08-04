use thiserror::Error;
use super::ir;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Unit,
}

#[derive(Debug, PartialEq, Eq, Error)]
pub enum VMError {
    #[error("stack underflow")]
    StackUnderflow,
}

struct VM {
    instructions: Vec<ir::Instruction>,
    data: Vec<Value>,
}

impl VM {
    fn pop_data(&mut self) -> Result<Value, VMError> {
        if let Some(value) = self.data.pop() {
            Ok(value)
        } else {
            Err(VMError::StackUnderflow)
        }
    }
}
