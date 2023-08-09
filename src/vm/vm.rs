use thiserror::Error;
use num_bigint::BigInt;
use crate::{ir::{Instruction, Block, Program}, ast::ast};

use super::ir;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Unit,
    Num {
        value: BigInt,
    }
}

#[derive(Debug, PartialEq, Eq, Error)]
pub enum VMError {
    #[error("stack underflow")]
    StackUnderflow,
}

pub struct VM {
    // instruction pointer
    pub ip: usize,
    pub program: Program,
    pub data: Vec<Value>,
    pub address: Vec<Value>,
}

impl VM {
    fn pop_data(&mut self) -> Result<Value, VMError> {
        if let Some(value) = self.data.pop() {
            Ok(value)
        } else {
            Err(VMError::StackUnderflow)
        }
    }

    fn push_data(&mut self, value: Value) {
        // TODO: Stack depth and stack overflow (maybe), it can be vary
        self.data.push(value);
    }

    fn pop_address(&mut self) -> Result<Value, VMError> {
        if let Some(value) = self.address.pop() {
            Ok(value)
        } else {
            Err(VMError::StackUnderflow)
        }
    }

    fn push_address(&mut self, value: Value) {
        self.address.push(value);
    }

    fn execute_nop(&mut self) -> Value {
       Value::Unit
    }

    fn execute_binop(&mut self, op: &ast::BinaryOperator) -> Value {
        let result = match op {
            ast::BinaryOperator::Plus => {
                Value::Unit
            },
            ast::BinaryOperator::Multiply => {
                Value::Unit
            },
            _ => Value::Unit,
        };
        result
    }

    fn execute_unop(&mut self, op: &ast::UnaryOperator) -> Value {
        let result = match op {
            ast::UnaryOperator::Minus => {
                Value::Unit
            },
            _ => Value::Unit,
        };
        result
    }

    fn execute_instruction(&mut self, instruction: &Instruction) -> Result<ExecStep, VMError> {
        let result = match instruction {
            Instruction::Nop => self.execute_nop(),
            Instruction::BinOp { op } => self.execute_binop(op),
            Instruction::UnaryOp { op } => self.execute_unop(op),
            _ => todo!()
        };
        self.ip += 1;
        Ok(ExecStep { ip: self.ip, op: instruction.clone() })
    }

    fn execute_block(&mut self, block: &Block) -> Result<ExecTrace, VMError> {
        let mut log = vec![];
    }

    pub fn execute(&mut self) -> Result<ExecTrace, VMError> {
        let mut trace = ExecTrace::new();
        let program = self.program.clone();
        for block in program.blocks.iter() {
            let mut result = self.execute_block(block)?;
            trace.concat_trace(&mut result);
        }
        Ok(trace)
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

impl ExecTrace {
    pub fn new() -> ExecTrace {
        ExecTrace { log: vec![] }
    }

    /// log in `other` will be empty
    fn concat_trace(&mut self, other: &mut ExecTrace) {
        self.log.append(&mut other.log);
    }
}

#[derive(Clone, Debug)]
pub struct ExecStep {
    pub ip: usize,
    pub op: ir::Instruction,
}
