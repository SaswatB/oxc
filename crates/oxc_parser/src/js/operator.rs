use oxc_syntax::{
    operator::{
        AssignmentOperator, GeneralBinaryOperator, LogicalOperator, UnaryOperator, UpdateOperator,
    },
    precedence::Precedence,
};

use crate::lexer::Kind;

pub fn kind_to_precedence(kind: Kind, is_typescript: bool) -> Option<Precedence> {
    match kind {
        Kind::Question2 => Some(Precedence::NullishCoalescing),
        Kind::Pipe2 => Some(Precedence::LogicalOr),
        Kind::Amp2 => Some(Precedence::LogicalAnd),
        Kind::Pipe => Some(Precedence::BitwiseOr),
        Kind::Caret => Some(Precedence::BitwiseXor),
        Kind::Amp => Some(Precedence::BitwiseAnd),
        Kind::Eq2 | Kind::Eq3 | Kind::Neq | Kind::Neq2 => Some(Precedence::Equals),
        Kind::LAngle | Kind::RAngle | Kind::LtEq | Kind::GtEq | Kind::Instanceof | Kind::In => {
            Some(Precedence::Compare)
        }
        Kind::ShiftLeft | Kind::ShiftRight | Kind::ShiftRight3 => Some(Precedence::Shift),
        Kind::Plus | Kind::Minus => Some(Precedence::Add),
        Kind::Star | Kind::Slash | Kind::Percent => Some(Precedence::Multiply),
        Kind::Star2 => Some(Precedence::Exponentiation),
        Kind::As | Kind::Satisfies if is_typescript => Some(Precedence::Compare),
        _ => None,
    }
}

pub fn map_general_binary_operator(kind: Kind) -> GeneralBinaryOperator {
    match kind {
        Kind::Eq2 => GeneralBinaryOperator::Equality,
        Kind::Neq => GeneralBinaryOperator::Inequality,
        Kind::Eq3 => GeneralBinaryOperator::StrictEquality,
        Kind::Neq2 => GeneralBinaryOperator::StrictInequality,
        Kind::LAngle => GeneralBinaryOperator::LessThan,
        Kind::LtEq => GeneralBinaryOperator::LessEqualThan,
        Kind::RAngle => GeneralBinaryOperator::GreaterThan,
        Kind::GtEq => GeneralBinaryOperator::GreaterEqualThan,
        Kind::ShiftLeft => GeneralBinaryOperator::ShiftLeft,
        Kind::ShiftRight => GeneralBinaryOperator::ShiftRight,
        Kind::ShiftRight3 => GeneralBinaryOperator::ShiftRightZeroFill,
        Kind::Plus => GeneralBinaryOperator::Addition,
        Kind::Minus => GeneralBinaryOperator::Subtraction,
        Kind::Star => GeneralBinaryOperator::Multiplication,
        Kind::Slash => GeneralBinaryOperator::Division,
        Kind::Percent => GeneralBinaryOperator::Remainder,
        Kind::Pipe => GeneralBinaryOperator::BitwiseOR,
        Kind::Caret => GeneralBinaryOperator::BitwiseXOR,
        Kind::Amp => GeneralBinaryOperator::BitwiseAnd,
        Kind::In => GeneralBinaryOperator::In,
        Kind::Instanceof => GeneralBinaryOperator::Instanceof,
        Kind::Star2 => GeneralBinaryOperator::Exponential,
        _ => unreachable!("Binary Operator: {kind:?}"),
    }
}

pub fn map_unary_operator(kind: Kind) -> UnaryOperator {
    match kind {
        Kind::Minus => UnaryOperator::UnaryNegation,
        Kind::Plus => UnaryOperator::UnaryPlus,
        Kind::Bang => UnaryOperator::LogicalNot,
        Kind::Tilde => UnaryOperator::BitwiseNot,
        Kind::Typeof => UnaryOperator::Typeof,
        Kind::Void => UnaryOperator::Void,
        Kind::Delete => UnaryOperator::Delete,
        _ => unreachable!("Unary Operator: {kind:?}"),
    }
}

pub fn map_logical_operator(kind: Kind) -> LogicalOperator {
    match kind {
        Kind::Pipe2 => LogicalOperator::Or,
        Kind::Amp2 => LogicalOperator::And,
        Kind::Question2 => LogicalOperator::Coalesce,
        _ => unreachable!("Logical Operator: {kind:?}"),
    }
}

pub fn map_update_operator(kind: Kind) -> UpdateOperator {
    match kind {
        Kind::Plus2 => UpdateOperator::Increment,
        Kind::Minus2 => UpdateOperator::Decrement,
        _ => unreachable!("Update Operator: {kind:?}"),
    }
}

pub fn map_assignment_operator(kind: Kind) -> AssignmentOperator {
    match kind {
        Kind::Eq => AssignmentOperator::Assign,
        Kind::PlusEq => AssignmentOperator::Addition,
        Kind::MinusEq => AssignmentOperator::Subtraction,
        Kind::StarEq => AssignmentOperator::Multiplication,
        Kind::SlashEq => AssignmentOperator::Division,
        Kind::PercentEq => AssignmentOperator::Remainder,
        Kind::ShiftLeftEq => AssignmentOperator::ShiftLeft,
        Kind::ShiftRightEq => AssignmentOperator::ShiftRight,
        Kind::ShiftRight3Eq => AssignmentOperator::ShiftRightZeroFill,
        Kind::PipeEq => AssignmentOperator::BitwiseOR,
        Kind::CaretEq => AssignmentOperator::BitwiseXOR,
        Kind::AmpEq => AssignmentOperator::BitwiseAnd,
        Kind::Amp2Eq => AssignmentOperator::LogicalAnd,
        Kind::Pipe2Eq => AssignmentOperator::LogicalOr,
        Kind::Question2Eq => AssignmentOperator::LogicalNullish,
        Kind::Star2Eq => AssignmentOperator::Exponential,
        _ => unreachable!("Update Operator: {kind:?}"),
    }
}
