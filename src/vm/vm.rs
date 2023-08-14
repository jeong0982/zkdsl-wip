use crate::{
    ast::ast,
    ir::{Block, BlockExit, Instruction, Program},
};
use num_bigint::BigInt;
use num_traits::cast::ToPrimitive;
use thiserror::Error;

use super::ir;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Unit,
    Num { value: BigInt },
}

impl Value {
    pub fn get_num(self) -> Result<BigInt, VMError> {
        match self {
            Value::Unit => Err(VMError::VoidValue),
            Value::Num { value } => Ok(value),
        }
    }

    pub fn is_true(self) -> Result<bool, VMError> {
        todo!();
    }
}

#[derive(Debug, PartialEq, Eq, Error)]
pub enum VMError {
    #[error("stack underflow")]
    StackUnderflow,
    #[error("Unit to Num")]
    VoidValue,
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

    fn execute_nop(&mut self) {
        ()
    }

    fn execute_swap(&mut self) -> Result<(), VMError> {
        let a = self.pop_data()?;
        let b = self.pop_data()?;
        self.push_data(a);
        self.push_data(b);
        Ok(())
    }

    fn execute_dup(&mut self) -> Result<(), VMError> {
        let data = self.pop_data()?;
        self.push_data(data.clone());
        self.push_data(data);
        Ok(())
    }

    fn execute_binop(&mut self, op: &ast::BinaryOperator) -> Result<Value, VMError> {
        let lhs = self.pop_data()?.get_num()?;
        let rhs = self.pop_data()?.get_num()?;

        let result = match op {
            ast::BinaryOperator::Plus => Value::Num { value: lhs + rhs },
            ast::BinaryOperator::Multiply => Value::Num { value: lhs * rhs },
            _ => Value::Unit,
        };
        Ok(result)
    }

    fn execute_unop(&mut self, op: &ast::UnaryOperator) -> Result<Value, VMError> {
        let result = match op {
            ast::UnaryOperator::Minus => Value::Unit,
            _ => Value::Unit,
        };
        Ok(result)
    }

    fn execute_instruction(&mut self, instruction: &Instruction) -> Result<ExecStep, VMError> {
        match instruction {
            Instruction::Nop => {
                self.execute_nop();
            }
            Instruction::BinOp { op } => {
                let result = self.execute_binop(op)?;
                self.push_data(result);
            }
            Instruction::UnaryOp { op } => {
                let result = self.execute_unop(op)?;
                self.push_data(result);
            }
            Instruction::Swap => {
                self.execute_swap()?;
            }
            Instruction::Dup => {
                self.execute_dup()?;
            }
            _ => todo!(),
        };
        self.ip += 1;
        Ok(ExecStep {
            ip: self.ip,
            op: StepFunction::from(instruction.clone()),
        })
    }

    fn execute_blockexit(&mut self, exit: &BlockExit) -> Result<ExecStep, VMError> {
        match exit {
            BlockExit::Jump => {
                self.ip = self
                    .pop_data()?
                    .get_num()?
                    .to_usize()
                    .ok_or(VMError::VoidValue)?;
            }
            BlockExit::ConditionalJump => {
                let condition = self.pop_data()?.is_true()?;
                let ip_then = self.pop_data()?;
                let ip_else = self.pop_data()?;
                if condition {
                    self.ip = ip_then.get_num()?.to_usize().ok_or(VMError::VoidValue)?;
                } else {
                    self.ip = ip_else.get_num()?.to_usize().ok_or(VMError::VoidValue)?;
                }
            }
            BlockExit::Return => {
                
            }
            _ => todo!(),
        };
        Ok(ExecStep {
            ip: self.ip,
            op: StepFunction::from(*exit),
        })
    }

    fn execute_block(&mut self, block: &Block) -> Result<ExecTrace, VMError> {
        let mut log = vec![];
        for instr in block.instructions.iter() {
            let result = self.execute_instruction(instr)?;
            log.push(result);
        }
        let exit = self.execute_blockexit(&block.exit)?;
        log.push(exit);
        Ok(ExecTrace { log })
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

impl Default for ExecTrace {
    fn default() -> Self {
        Self::new()
    }
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
    pub op: StepFunction,
}

#[derive(Clone, Debug)]
pub enum StepFunction {
    Instruction { instr: ir::Instruction },
    Exit { exit: ir::BlockExit },
}

impl From<ir::Instruction> for StepFunction {
    fn from(instr: ir::Instruction) -> Self {
        StepFunction::Instruction { instr }
    }
}

impl From<ir::BlockExit> for StepFunction {
    fn from(exit: ir::BlockExit) -> Self {
        StepFunction::Exit { exit }
    }
}
