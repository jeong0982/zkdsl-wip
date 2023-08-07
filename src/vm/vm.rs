use thiserror::Error;
use crate::ir::Instruction;

use super::ir;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Value {
    Unit,
}

#[derive(Debug, PartialEq, Eq, Error)]
pub enum VMError {
    #[error("stack underflow")]
    StackUnderflow,
}

pub struct VM {
    // instruction pointer
    pub ip: Value,
    pub instructions: Vec<ir::Instruction>,
    pub data: Vec<Value>,
}

impl VM {
    pub fn pop_data(&mut self) -> Result<Value, VMError> {
        if let Some(value) = self.data.pop() {
            Ok(value)
        } else {
            Err(VMError::StackUnderflow)
        }
    }

    pub fn push_data(&mut self, value: Value) {
        // TODO: Stack depth and stack overflow (maybe), it can be vary
        self.data.push(value);
    }

    fn execute_nop(&mut self) -> Value {
       Value::Unit
    }

    fn execute_instruction(&mut self, instruction: &Instruction) -> Result<ExecStep, VMError> {
        let result = match instruction {
            Instruction::Nop => self.execute_nop(),
            _ => todo!()
        };
        Ok(ExecStep { ip: self.ip, op: instruction.clone() })
    }

    pub fn execute(&mut self) -> Result<ExecTrace, VMError> {
        let mut log = vec![];
        let instructions = self.instructions.clone();
        for instr in instructions.iter() {
            let result = self.execute_instruction(instr)?;
            log.push(result);
        }
        Ok(ExecTrace { log })
    }

    pub fn evaluate(&mut self) {
        todo!()
    }
}

pub struct State {
    pub ip: Value,

}

#[derive(Clone, Debug)]
pub struct ExecTrace {
    pub log: Vec<ExecStep>,
}

#[derive(Clone, Debug)]
pub struct ExecStep {
    pub ip: Value,
    pub op: ir::Instruction,
}
