use super::ast;

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

}
