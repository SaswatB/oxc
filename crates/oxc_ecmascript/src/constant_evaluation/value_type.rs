use oxc_ast::ast::{Expression, GeneralBinaryExpression};
use oxc_syntax::operator::{GeneralBinaryOperator, UnaryOperator};

/// JavaScript Language Type
///
/// <https://tc39.es/ecma262/#sec-ecmascript-language-types>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueType {
    Undefined, // a.k.a `Void` in closure compiler
    Null,
    Number,
    BigInt,
    String,
    Boolean,
    Object,
    Undetermined,
}

impl ValueType {
    pub fn is_undefined(self) -> bool {
        self == Self::Undefined
    }

    pub fn is_null(self) -> bool {
        self == Self::Null
    }

    pub fn is_string(self) -> bool {
        self == Self::String
    }

    pub fn is_number(self) -> bool {
        self == Self::Number
    }

    pub fn is_bigint(self) -> bool {
        self == Self::BigInt
    }

    pub fn is_boolean(self) -> bool {
        self == Self::Boolean
    }

    pub fn is_object(self) -> bool {
        self == Self::Object
    }

    pub fn is_undetermined(self) -> bool {
        self == Self::Undetermined
    }
}

/// `get_known_value_type`
///
/// Evaluate  and attempt to determine which primitive value type it could resolve to.
/// Without proper type information some assumptions had to be made for operations that could
/// result in a BigInt or a Number. If there is not enough information available to determine one
/// or the other then we assume Number in order to maintain historical behavior of the compiler and
/// avoid breaking projects that relied on this behavior.
impl<'a> From<&Expression<'a>> for ValueType {
    fn from(expr: &Expression<'a>) -> Self {
        // TODO: complete this
        match expr {
            Expression::BigIntLiteral(_) => Self::BigInt,
            Expression::BooleanLiteral(_) => Self::Boolean,
            Expression::NullLiteral(_) => Self::Null,
            Expression::NumericLiteral(_) => Self::Number,
            Expression::StringLiteral(_) | Expression::TemplateExpression(_) => Self::String,
            Expression::ObjectExpression(_)
            | Expression::ArrayExpression(_)
            | Expression::RegExpLiteral(_)
            | Expression::FunctionExpression(_) => Self::Object,
            Expression::Identifier(ident) => match ident.name.as_str() {
                "undefined" => Self::Undefined,
                "NaN" | "Infinity" => Self::Number,
                _ => Self::Undetermined,
            },
            Expression::UnaryExpression(unary_expr) => match unary_expr.operator {
                UnaryOperator::Void => Self::Undefined,
                UnaryOperator::UnaryNegation => {
                    let argument_ty = Self::from(&unary_expr.argument);
                    if argument_ty == Self::BigInt {
                        return Self::BigInt;
                    }
                    Self::Number
                }
                UnaryOperator::UnaryPlus => Self::Number,
                UnaryOperator::LogicalNot | UnaryOperator::Delete => Self::Boolean,
                UnaryOperator::Typeof => Self::String,
                UnaryOperator::BitwiseNot => Self::Undetermined,
            },
            Expression::GeneralBinaryExpression(e) => Self::from(&**e),
            Expression::SequenceExpression(e) => {
                e.expressions.last().map_or(ValueType::Undetermined, Self::from)
            }
            Expression::AssignmentExpression(e) => Self::from(&e.right),
            _ => Self::Undetermined,
        }
    }
}

impl<'a> From<&GeneralBinaryExpression<'a>> for ValueType {
    fn from(e: &GeneralBinaryExpression<'a>) -> Self {
        match e.operator {
            GeneralBinaryOperator::Addition => {
                let left = Self::from(&e.left);
                let right = Self::from(&e.right);
                if left == Self::Boolean
                    && matches!(right, Self::Undefined | Self::Null | Self::Number)
                {
                    return Self::Number;
                }
                if left == Self::String || right == Self::String {
                    return Self::String;
                }
                // There are some pretty weird cases for object types:
                //   {} + [] === "0"
                //   [] + {} === "[object Object]"
                if left == Self::Object || right == Self::Object {
                    return Self::Undetermined;
                }
                Self::Undetermined
            }
            GeneralBinaryOperator::Subtraction
            | GeneralBinaryOperator::Multiplication
            | GeneralBinaryOperator::Division
            | GeneralBinaryOperator::Remainder
            | GeneralBinaryOperator::ShiftLeft
            | GeneralBinaryOperator::BitwiseOR
            | GeneralBinaryOperator::ShiftRight
            | GeneralBinaryOperator::BitwiseXOR
            | GeneralBinaryOperator::BitwiseAnd
            | GeneralBinaryOperator::Exponential
            | GeneralBinaryOperator::ShiftRightZeroFill => Self::Number,
            GeneralBinaryOperator::Instanceof
            | GeneralBinaryOperator::In
            | GeneralBinaryOperator::Equality
            | GeneralBinaryOperator::Inequality
            | GeneralBinaryOperator::StrictEquality
            | GeneralBinaryOperator::StrictInequality
            | GeneralBinaryOperator::LessThan
            | GeneralBinaryOperator::LessEqualThan
            | GeneralBinaryOperator::GreaterThan
            | GeneralBinaryOperator::GreaterEqualThan => Self::Boolean,
        }
    }
}
