use super::ast::ast;
use num_bigint::BigInt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
    Nop,
    BinOp {
        op: ast::BinaryOperator,
        lhs: Operand,
        rhs: Operand,
        // type: Type,
    },
    UnaryOp {
        op: ast::UnaryOperator,
        operand: Operand,
        // type: Type,
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Operand {
    Constant(Constant),

}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Constant {
    Unit,
    Num {
        value: BigInt,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    pub instructions: Vec<Instruction>,
    pub exit: BlockExit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockExit {
    Jump {
        arg: JumpArg,
    },
    ConditionalJump {
        condition: Operand,
        arg_then: JumpArg,
        arg_else: JumpArg,
    },
    Return {
        value: Operand,
    },
    Unreachable,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JumpArg {
    pub args: Vec<Operand>,
}

