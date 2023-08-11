use super::ast::ast;
use num_bigint::BigInt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
    Nop,
    BinOp {
        op: ast::BinaryOperator,
        // type: Type,
    },
    UnaryOp {
        op: ast::UnaryOperator,
        // type: Type,
    },
    Swap,
    Dup,
    // If there is a memory,
    // Load,
    // Store,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Operand {
    Constant(Constant),
    StackValue(usize),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Constant {
    Unit,
    Num { value: BigInt },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    pub instructions: Vec<Instruction>,
    pub exit: BlockExit,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum BlockExit {
    Jump,
    ConditionalJump,
    Return,
    Unreachable,
}

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct JumpArg {
//     pub args: Vec<Operand>,
// }

// TODO: if function declaration is added, Program should have decls
#[derive(Debug, Clone)]
pub struct Program {
    /// Blocks
    /// Can be changed to BTreeMap<BlockId, Block>
    pub blocks: Vec<Block>,
}
