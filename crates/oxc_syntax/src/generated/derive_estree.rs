// Auto-generated code, DO NOT EDIT DIRECTLY!
// To edit this generated file you have to edit `tasks/ast_tools/src/derives/estree.rs`

#![allow(unused_imports, unused_mut, clippy::match_same_arms)]

use serde::{ser::SerializeMap, Serialize, Serializer};

use crate::operator::*;

impl Serialize for AssignmentOperator {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match *self {
            AssignmentOperator::Assign => {
                serializer.serialize_unit_variant("AssignmentOperator", 0u32, "=")
            }
            AssignmentOperator::Addition => {
                serializer.serialize_unit_variant("AssignmentOperator", 1u32, "+=")
            }
            AssignmentOperator::Subtraction => {
                serializer.serialize_unit_variant("AssignmentOperator", 2u32, "-=")
            }
            AssignmentOperator::Multiplication => {
                serializer.serialize_unit_variant("AssignmentOperator", 3u32, "*=")
            }
            AssignmentOperator::Division => {
                serializer.serialize_unit_variant("AssignmentOperator", 4u32, "/=")
            }
            AssignmentOperator::Remainder => {
                serializer.serialize_unit_variant("AssignmentOperator", 5u32, "%=")
            }
            AssignmentOperator::Exponential => {
                serializer.serialize_unit_variant("AssignmentOperator", 6u32, "**=")
            }
            AssignmentOperator::ShiftLeft => {
                serializer.serialize_unit_variant("AssignmentOperator", 7u32, "<<=")
            }
            AssignmentOperator::ShiftRight => {
                serializer.serialize_unit_variant("AssignmentOperator", 8u32, ">>=")
            }
            AssignmentOperator::ShiftRightZeroFill => {
                serializer.serialize_unit_variant("AssignmentOperator", 9u32, ">>>=")
            }
            AssignmentOperator::BitwiseOR => {
                serializer.serialize_unit_variant("AssignmentOperator", 10u32, "|=")
            }
            AssignmentOperator::BitwiseXOR => {
                serializer.serialize_unit_variant("AssignmentOperator", 11u32, "^=")
            }
            AssignmentOperator::BitwiseAnd => {
                serializer.serialize_unit_variant("AssignmentOperator", 12u32, "&=")
            }
            AssignmentOperator::LogicalOr => {
                serializer.serialize_unit_variant("AssignmentOperator", 13u32, "||=")
            }
            AssignmentOperator::LogicalAnd => {
                serializer.serialize_unit_variant("AssignmentOperator", 14u32, "&&=")
            }
            AssignmentOperator::LogicalNullish => {
                serializer.serialize_unit_variant("AssignmentOperator", 15u32, "??=")
            }
        }
    }
}

impl Serialize for GeneralBinaryOperator {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match *self {
            GeneralBinaryOperator::Equality => {
                serializer.serialize_unit_variant("GeneralBinaryOperator", 0u32, "==")
            }
            GeneralBinaryOperator::Inequality => {
                serializer.serialize_unit_variant("GeneralBinaryOperator", 1u32, "!=")
            }
            GeneralBinaryOperator::StrictEquality => {
                serializer.serialize_unit_variant("GeneralBinaryOperator", 2u32, "===")
            }
            GeneralBinaryOperator::StrictInequality => {
                serializer.serialize_unit_variant("GeneralBinaryOperator", 3u32, "!==")
            }
            GeneralBinaryOperator::LessThan => {
                serializer.serialize_unit_variant("GeneralBinaryOperator", 4u32, "<")
            }
            GeneralBinaryOperator::LessEqualThan => {
                serializer.serialize_unit_variant("GeneralBinaryOperator", 5u32, "<=")
            }
            GeneralBinaryOperator::GreaterThan => {
                serializer.serialize_unit_variant("GeneralBinaryOperator", 6u32, ">")
            }
            GeneralBinaryOperator::GreaterEqualThan => {
                serializer.serialize_unit_variant("GeneralBinaryOperator", 7u32, ">=")
            }
            GeneralBinaryOperator::Addition => {
                serializer.serialize_unit_variant("GeneralBinaryOperator", 8u32, "+")
            }
            GeneralBinaryOperator::Subtraction => {
                serializer.serialize_unit_variant("GeneralBinaryOperator", 9u32, "-")
            }
            GeneralBinaryOperator::Multiplication => {
                serializer.serialize_unit_variant("GeneralBinaryOperator", 10u32, "*")
            }
            GeneralBinaryOperator::Division => {
                serializer.serialize_unit_variant("GeneralBinaryOperator", 11u32, "/")
            }
            GeneralBinaryOperator::Remainder => {
                serializer.serialize_unit_variant("GeneralBinaryOperator", 12u32, "%")
            }
            GeneralBinaryOperator::Exponential => {
                serializer.serialize_unit_variant("GeneralBinaryOperator", 13u32, "**")
            }
            GeneralBinaryOperator::ShiftLeft => {
                serializer.serialize_unit_variant("GeneralBinaryOperator", 14u32, "<<")
            }
            GeneralBinaryOperator::ShiftRight => {
                serializer.serialize_unit_variant("GeneralBinaryOperator", 15u32, ">>")
            }
            GeneralBinaryOperator::ShiftRightZeroFill => {
                serializer.serialize_unit_variant("GeneralBinaryOperator", 16u32, ">>>")
            }
            GeneralBinaryOperator::BitwiseOR => {
                serializer.serialize_unit_variant("GeneralBinaryOperator", 17u32, "|")
            }
            GeneralBinaryOperator::BitwiseXOR => {
                serializer.serialize_unit_variant("GeneralBinaryOperator", 18u32, "^")
            }
            GeneralBinaryOperator::BitwiseAnd => {
                serializer.serialize_unit_variant("GeneralBinaryOperator", 19u32, "&")
            }
            GeneralBinaryOperator::In => {
                serializer.serialize_unit_variant("GeneralBinaryOperator", 20u32, "in")
            }
            GeneralBinaryOperator::Instanceof => {
                serializer.serialize_unit_variant("GeneralBinaryOperator", 21u32, "instanceof")
            }
        }
    }
}

impl Serialize for LogicalOperator {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match *self {
            LogicalOperator::Or => serializer.serialize_unit_variant("LogicalOperator", 0u32, "||"),
            LogicalOperator::And => {
                serializer.serialize_unit_variant("LogicalOperator", 1u32, "&&")
            }
            LogicalOperator::Coalesce => {
                serializer.serialize_unit_variant("LogicalOperator", 2u32, "??")
            }
        }
    }
}

impl Serialize for UnaryOperator {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match *self {
            UnaryOperator::UnaryPlus => {
                serializer.serialize_unit_variant("UnaryOperator", 0u32, "+")
            }
            UnaryOperator::UnaryNegation => {
                serializer.serialize_unit_variant("UnaryOperator", 1u32, "-")
            }
            UnaryOperator::LogicalNot => {
                serializer.serialize_unit_variant("UnaryOperator", 2u32, "!")
            }
            UnaryOperator::BitwiseNot => {
                serializer.serialize_unit_variant("UnaryOperator", 3u32, "~")
            }
            UnaryOperator::Typeof => {
                serializer.serialize_unit_variant("UnaryOperator", 4u32, "typeof")
            }
            UnaryOperator::Void => serializer.serialize_unit_variant("UnaryOperator", 5u32, "void"),
            UnaryOperator::Delete => {
                serializer.serialize_unit_variant("UnaryOperator", 6u32, "delete")
            }
        }
    }
}

impl Serialize for UpdateOperator {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match *self {
            UpdateOperator::Increment => {
                serializer.serialize_unit_variant("UpdateOperator", 0u32, "++")
            }
            UpdateOperator::Decrement => {
                serializer.serialize_unit_variant("UpdateOperator", 1u32, "--")
            }
        }
    }
}
