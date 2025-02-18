// Auto-generated code, DO NOT EDIT DIRECTLY!
// To edit this generated file you have to edit `tasks/ast_tools/src/derives/clone_in.rs`

#![allow(clippy::default_trait_access)]

use oxc_allocator::{Allocator, CloneIn};

use crate::number::*;

use crate::operator::*;

impl<'alloc> CloneIn<'alloc> for NumberBase {
    type Cloned = NumberBase;
    fn clone_in(&self, _: &'alloc Allocator) -> Self::Cloned {
        match self {
            Self::Float => NumberBase::Float,
            Self::Decimal => NumberBase::Decimal,
            Self::Binary => NumberBase::Binary,
            Self::Octal => NumberBase::Octal,
            Self::Hex => NumberBase::Hex,
        }
    }
}

impl<'alloc> CloneIn<'alloc> for BigintBase {
    type Cloned = BigintBase;
    fn clone_in(&self, _: &'alloc Allocator) -> Self::Cloned {
        match self {
            Self::Decimal => BigintBase::Decimal,
            Self::Binary => BigintBase::Binary,
            Self::Octal => BigintBase::Octal,
            Self::Hex => BigintBase::Hex,
        }
    }
}

impl<'alloc> CloneIn<'alloc> for AssignmentOperator {
    type Cloned = AssignmentOperator;
    fn clone_in(&self, _: &'alloc Allocator) -> Self::Cloned {
        match self {
            Self::Assign => AssignmentOperator::Assign,
            Self::Addition => AssignmentOperator::Addition,
            Self::Subtraction => AssignmentOperator::Subtraction,
            Self::Multiplication => AssignmentOperator::Multiplication,
            Self::Division => AssignmentOperator::Division,
            Self::Remainder => AssignmentOperator::Remainder,
            Self::Exponential => AssignmentOperator::Exponential,
            Self::ShiftLeft => AssignmentOperator::ShiftLeft,
            Self::ShiftRight => AssignmentOperator::ShiftRight,
            Self::ShiftRightZeroFill => AssignmentOperator::ShiftRightZeroFill,
            Self::BitwiseOR => AssignmentOperator::BitwiseOR,
            Self::BitwiseXOR => AssignmentOperator::BitwiseXOR,
            Self::BitwiseAnd => AssignmentOperator::BitwiseAnd,
            Self::LogicalOr => AssignmentOperator::LogicalOr,
            Self::LogicalAnd => AssignmentOperator::LogicalAnd,
            Self::LogicalNullish => AssignmentOperator::LogicalNullish,
        }
    }
}

impl<'alloc> CloneIn<'alloc> for GeneralBinaryOperator {
    type Cloned = GeneralBinaryOperator;
    fn clone_in(&self, _: &'alloc Allocator) -> Self::Cloned {
        match self {
            Self::Equality => GeneralBinaryOperator::Equality,
            Self::Inequality => GeneralBinaryOperator::Inequality,
            Self::StrictEquality => GeneralBinaryOperator::StrictEquality,
            Self::StrictInequality => GeneralBinaryOperator::StrictInequality,
            Self::LessThan => GeneralBinaryOperator::LessThan,
            Self::LessEqualThan => GeneralBinaryOperator::LessEqualThan,
            Self::GreaterThan => GeneralBinaryOperator::GreaterThan,
            Self::GreaterEqualThan => GeneralBinaryOperator::GreaterEqualThan,
            Self::Addition => GeneralBinaryOperator::Addition,
            Self::Subtraction => GeneralBinaryOperator::Subtraction,
            Self::Multiplication => GeneralBinaryOperator::Multiplication,
            Self::Division => GeneralBinaryOperator::Division,
            Self::Remainder => GeneralBinaryOperator::Remainder,
            Self::Exponential => GeneralBinaryOperator::Exponential,
            Self::ShiftLeft => GeneralBinaryOperator::ShiftLeft,
            Self::ShiftRight => GeneralBinaryOperator::ShiftRight,
            Self::ShiftRightZeroFill => GeneralBinaryOperator::ShiftRightZeroFill,
            Self::BitwiseOR => GeneralBinaryOperator::BitwiseOR,
            Self::BitwiseXOR => GeneralBinaryOperator::BitwiseXOR,
            Self::BitwiseAnd => GeneralBinaryOperator::BitwiseAnd,
            Self::In => GeneralBinaryOperator::In,
            Self::Instanceof => GeneralBinaryOperator::Instanceof,
        }
    }
}

impl<'alloc> CloneIn<'alloc> for LogicalOperator {
    type Cloned = LogicalOperator;
    fn clone_in(&self, _: &'alloc Allocator) -> Self::Cloned {
        match self {
            Self::Or => LogicalOperator::Or,
            Self::And => LogicalOperator::And,
            Self::Coalesce => LogicalOperator::Coalesce,
        }
    }
}

impl<'alloc> CloneIn<'alloc> for UnaryOperator {
    type Cloned = UnaryOperator;
    fn clone_in(&self, _: &'alloc Allocator) -> Self::Cloned {
        match self {
            Self::UnaryPlus => UnaryOperator::UnaryPlus,
            Self::UnaryNegation => UnaryOperator::UnaryNegation,
            Self::LogicalNot => UnaryOperator::LogicalNot,
            Self::BitwiseNot => UnaryOperator::BitwiseNot,
            Self::Typeof => UnaryOperator::Typeof,
            Self::Void => UnaryOperator::Void,
            Self::Delete => UnaryOperator::Delete,
        }
    }
}

impl<'alloc> CloneIn<'alloc> for UpdateOperator {
    type Cloned = UpdateOperator;
    fn clone_in(&self, _: &'alloc Allocator) -> Self::Cloned {
        match self {
            Self::Increment => UpdateOperator::Increment,
            Self::Decrement => UpdateOperator::Decrement,
        }
    }
}
