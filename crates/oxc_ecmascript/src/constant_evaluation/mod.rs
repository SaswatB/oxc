use std::{borrow::Cow, cmp::Ordering};

use num_bigint::BigInt;
use num_traits::{ToPrimitive, Zero};

use oxc_ast::ast::*;

use crate::{side_effects::MayHaveSideEffects, ToBigInt, ToBoolean, ToInt32, ToJsString, ToNumber};

mod is_literal_value;
mod value;
mod value_type;
pub use is_literal_value::IsLiteralValue;
pub use value::ConstantValue;
pub use value_type::ValueType;

pub trait ConstantEvaluation<'a>: MayHaveSideEffects {
    fn resolve_binding(&self, ident: &IdentifierReference<'a>) -> Option<ConstantValue<'a>> {
        match ident.name.as_str() {
            "undefined" if self.is_global_reference(ident) => Some(ConstantValue::Undefined),
            "NaN" if self.is_global_reference(ident) => Some(ConstantValue::Number(f64::NAN)),
            "Infinity" if self.is_global_reference(ident) => {
                Some(ConstantValue::Number(f64::INFINITY))
            }
            _ => None,
        }
    }

    fn get_side_free_number_value(&self, expr: &Expression<'a>) -> Option<f64> {
        let value = self.eval_to_number(expr);
        // Calculating the number value, if any, is likely to be faster than calculating side effects,
        // and there are only a very few cases where we can compute a number value, but there could
        // also be side effects. e.g. `void doSomething()` has value NaN, regardless of the behavior
        // of `doSomething()`
        if value.is_some() && self.expression_may_have_side_efffects(expr) {
            None
        } else {
            value
        }
    }

    fn get_side_free_string_value(&self, expr: &Expression<'a>) -> Option<Cow<'a, str>> {
        let value = expr.to_js_string();
        if value.is_some() && !self.expression_may_have_side_efffects(expr) {
            return value;
        }
        None
    }

    fn get_side_free_boolean_value(&self, expr: &Expression<'a>) -> Option<bool> {
        let value = self.get_boolean_value(expr);
        if value.is_some() && !self.expression_may_have_side_efffects(expr) {
            return value;
        }
        None
    }

    fn get_side_free_bigint_value(&self, expr: &Expression<'a>) -> Option<BigInt> {
        let value = expr.to_big_int();
        if value.is_some() && self.expression_may_have_side_efffects(expr) {
            None
        } else {
            value
        }
    }

    fn get_boolean_value(&self, expr: &Expression<'a>) -> Option<bool> {
        match expr {
            Expression::Identifier(ident) => match ident.name.as_str() {
                "undefined" | "NaN" if self.is_global_reference(ident) => Some(false),
                "Infinity" if self.is_global_reference(ident) => Some(true),
                _ => None,
            },
            Expression::LogicalExpression(logical_expr) => {
                match logical_expr.operator {
                    // true && true -> true
                    // true && false -> false
                    // a && true -> None
                    LogicalOperator::And => {
                        let left = self.get_boolean_value(&logical_expr.left);
                        let right = self.get_boolean_value(&logical_expr.right);
                        match (left, right) {
                            (Some(true), Some(true)) => Some(true),
                            (Some(false), _) | (_, Some(false)) => Some(false),
                            (None, _) | (_, None) => None,
                        }
                    }
                    // true || false -> true
                    // false || false -> false
                    // a || b -> None
                    LogicalOperator::Or => {
                        let left = self.get_boolean_value(&logical_expr.left);
                        let right = self.get_boolean_value(&logical_expr.right);
                        match (left, right) {
                            (Some(true), _) | (_, Some(true)) => Some(true),
                            (Some(false), Some(false)) => Some(false),
                            (None, _) | (_, None) => None,
                        }
                    }
                    LogicalOperator::Coalesce => None,
                }
            }
            Expression::SequenceExpression(sequence_expr) => {
                // For sequence expression, the value is the value of the RHS.
                sequence_expr.expressions.last().and_then(|e| self.get_boolean_value(e))
            }
            Expression::UnaryExpression(unary_expr) => {
                match unary_expr.operator {
                    UnaryOperator::Void => Some(false),
                    UnaryOperator::BitwiseNot
                    | UnaryOperator::UnaryPlus
                    | UnaryOperator::UnaryNegation => {
                        // `~0 -> true` `+1 -> true` `+0 -> false` `-0 -> false`
                        self.eval_to_number(expr).map(|value| !value.is_zero())
                    }
                    UnaryOperator::LogicalNot => {
                        // !true -> false
                        self.get_boolean_value(&unary_expr.argument).map(|b| !b)
                    }
                    _ => None,
                }
            }
            expr => expr.to_boolean(),
        }
    }

    fn eval_to_number(&self, expr: &Expression<'a>) -> Option<f64> {
        match expr {
            Expression::Identifier(ident) => match ident.name.as_str() {
                "undefined" | "NaN" if self.is_global_reference(ident) => Some(f64::NAN),
                "Infinity" if self.is_global_reference(ident) => Some(f64::INFINITY),
                _ => None,
            },
            Expression::UnaryExpression(unary_expr) => match unary_expr.operator {
                UnaryOperator::UnaryPlus => self.eval_to_number(&unary_expr.argument),
                UnaryOperator::UnaryNegation => {
                    self.eval_to_number(&unary_expr.argument).map(|v| -v)
                }
                UnaryOperator::LogicalNot => {
                    self.get_boolean_value(expr).map(|b| if b { 1_f64 } else { 0_f64 })
                }
                UnaryOperator::Void => Some(f64::NAN),
                _ => None,
            },
            Expression::SequenceExpression(s) => {
                s.expressions.last().and_then(|e| self.eval_to_number(e))
            }
            Expression::ObjectExpression(e) if e.properties.is_empty() => Some(f64::NAN),
            expr => {
                use crate::ToNumber;
                expr.to_number()
            }
        }
    }

    fn eval_to_big_int(&self, expr: &Expression<'a>) -> Option<BigInt> {
        match expr {
            Expression::UnaryExpression(unary_expr) => match unary_expr.operator {
                UnaryOperator::UnaryPlus => self.eval_to_big_int(&unary_expr.argument),
                UnaryOperator::UnaryNegation => {
                    self.eval_to_big_int(&unary_expr.argument).map(|v| -v)
                }
                _ => None,
            },
            Expression::BigIntLiteral(_) => {
                use crate::ToBigInt;
                expr.to_big_int()
            }
            _ => None,
        }
    }

    fn eval_expression(&self, expr: &Expression<'a>) -> Option<ConstantValue<'a>> {
        match expr {
            Expression::GeneralBinaryExpression(e) => self.eval_general_binary_expression(e),
            Expression::LogicalExpression(e) => self.eval_logical_expression(e),
            Expression::UnaryExpression(e) => self.eval_unary_expression(e),
            Expression::Identifier(ident) => self.resolve_binding(ident),
            Expression::NumericLiteral(lit) => Some(ConstantValue::Number(lit.value)),
            Expression::NullLiteral(_) => Some(ConstantValue::Null),
            Expression::BooleanLiteral(lit) => Some(ConstantValue::Boolean(lit.value)),
            Expression::BigIntLiteral(lit) => lit.to_big_int().map(ConstantValue::BigInt),
            Expression::StringLiteral(lit) => {
                Some(ConstantValue::String(Cow::Borrowed(lit.value.as_str())))
            }
            Expression::StaticMemberExpression(e) => self.eval_static_member_expression(e),
            Expression::ElementAccessExpression(e) => self.eval_computed_member_expression(e),
            _ => None,
        }
    }

    fn eval_general_binary_expression(
        &self,
        e: &GeneralBinaryExpression<'a>,
    ) -> Option<ConstantValue<'a>> {
        self.eval_binary_operation(e.operator, &e.left, &e.right)
    }

    fn eval_binary_operation(
        &self,
        operator: GeneralBinaryOperator,
        left: &Expression<'a>,
        right: &Expression<'a>,
    ) -> Option<ConstantValue<'a>> {
        match operator {
            GeneralBinaryOperator::Addition => {
                if self.expression_may_have_side_efffects(left)
                    || self.expression_may_have_side_efffects(right)
                {
                    return None;
                }
                let left_type = ValueType::from(left);
                let right_type = ValueType::from(right);
                if left_type.is_string() || right_type.is_string() {
                    let lval = self.eval_expression(left)?;
                    let rval = self.eval_expression(right)?;
                    let lstr = lval.to_js_string()?;
                    let rstr = rval.to_js_string()?;
                    return Some(ConstantValue::String(lstr + rstr));
                }
                if left_type.is_number() || right_type.is_number() {
                    let lval = self.eval_expression(left)?;
                    let rval = self.eval_expression(right)?;
                    let lnum = lval.to_number()?;
                    let rnum = rval.to_number()?;
                    return Some(ConstantValue::Number(lnum + rnum));
                }
                None
            }
            GeneralBinaryOperator::Subtraction
            | GeneralBinaryOperator::Division
            | GeneralBinaryOperator::Remainder
            | GeneralBinaryOperator::Multiplication
            | GeneralBinaryOperator::Exponential => {
                let lval = self.eval_to_number(left)?;
                let rval = self.eval_to_number(right)?;
                let val = match operator {
                    GeneralBinaryOperator::Subtraction => lval - rval,
                    GeneralBinaryOperator::Division => lval / rval,
                    GeneralBinaryOperator::Remainder => {
                        if rval.is_zero() {
                            f64::NAN
                        } else {
                            lval % rval
                        }
                    }
                    GeneralBinaryOperator::Multiplication => lval * rval,
                    GeneralBinaryOperator::Exponential => lval.powf(rval),
                    _ => unreachable!(),
                };
                Some(ConstantValue::Number(val))
            }
            #[expect(clippy::cast_sign_loss)]
            GeneralBinaryOperator::ShiftLeft
            | GeneralBinaryOperator::ShiftRight
            | GeneralBinaryOperator::ShiftRightZeroFill => {
                let left = self.get_side_free_number_value(left)?;
                let right = self.get_side_free_number_value(right)?;
                let left = left.to_int_32();
                let right = (right.to_int_32() as u32) & 31;
                Some(ConstantValue::Number(match operator {
                    GeneralBinaryOperator::ShiftLeft => f64::from(left << right),
                    GeneralBinaryOperator::ShiftRight => f64::from(left >> right),
                    GeneralBinaryOperator::ShiftRightZeroFill => f64::from((left as u32) >> right),
                    _ => unreachable!(),
                }))
            }
            GeneralBinaryOperator::LessThan => {
                self.is_less_than(left, right).map(|value| match value {
                    ConstantValue::Undefined => ConstantValue::Boolean(false),
                    _ => value,
                })
            }
            GeneralBinaryOperator::GreaterThan => {
                self.is_less_than(right, left).map(|value| match value {
                    ConstantValue::Undefined => ConstantValue::Boolean(false),
                    _ => value,
                })
            }
            GeneralBinaryOperator::LessEqualThan => {
                self.is_less_than(right, left).map(|value| match value {
                    ConstantValue::Boolean(true) | ConstantValue::Undefined => {
                        ConstantValue::Boolean(false)
                    }
                    ConstantValue::Boolean(false) => ConstantValue::Boolean(true),
                    _ => unreachable!(),
                })
            }
            GeneralBinaryOperator::GreaterEqualThan => {
                self.is_less_than(left, right).map(|value| match value {
                    ConstantValue::Boolean(true) | ConstantValue::Undefined => {
                        ConstantValue::Boolean(false)
                    }
                    ConstantValue::Boolean(false) => ConstantValue::Boolean(true),
                    _ => unreachable!(),
                })
            }
            GeneralBinaryOperator::BitwiseAnd
            | GeneralBinaryOperator::BitwiseOR
            | GeneralBinaryOperator::BitwiseXOR => {
                if left.is_big_int_literal() && right.is_big_int_literal() {
                    let left_val = self.get_side_free_bigint_value(left)?;
                    let right_val = self.get_side_free_bigint_value(right)?;
                    let result_val: BigInt = match operator {
                        GeneralBinaryOperator::BitwiseAnd => left_val & right_val,
                        GeneralBinaryOperator::BitwiseOR => left_val | right_val,
                        GeneralBinaryOperator::BitwiseXOR => left_val ^ right_val,
                        _ => unreachable!(),
                    };
                    return Some(ConstantValue::BigInt(result_val));
                }
                let left_num = self.get_side_free_number_value(left);
                let right_num = self.get_side_free_number_value(right);
                if let (Some(left_val), Some(right_val)) = (left_num, right_num) {
                    let left_val_int = left_val.to_int_32();
                    let right_val_int = right_val.to_int_32();

                    let result_val: f64 = match operator {
                        GeneralBinaryOperator::BitwiseAnd => {
                            f64::from(left_val_int & right_val_int)
                        }
                        GeneralBinaryOperator::BitwiseOR => f64::from(left_val_int | right_val_int),
                        GeneralBinaryOperator::BitwiseXOR => {
                            f64::from(left_val_int ^ right_val_int)
                        }
                        _ => unreachable!(),
                    };
                    return Some(ConstantValue::Number(result_val));
                }
                None
            }
            GeneralBinaryOperator::Instanceof => {
                if self.expression_may_have_side_efffects(left) {
                    return None;
                }
                if let Expression::Identifier(right_ident) = right {
                    let name = right_ident.name.as_str();
                    if matches!(name, "Object" | "Number" | "Boolean" | "String")
                        && self.is_global_reference(right_ident)
                    {
                        return Some(ConstantValue::Boolean(
                            name == "Object" && ValueType::from(left).is_object(),
                        ));
                    }
                }
                None
            }
            _ => None,
        }
    }

    fn eval_logical_expression(&self, expr: &LogicalExpression<'a>) -> Option<ConstantValue<'a>> {
        match expr.operator {
            LogicalOperator::And => {
                if self.get_boolean_value(&expr.left) == Some(true) {
                    self.eval_expression(&expr.right)
                } else {
                    self.eval_expression(&expr.left)
                }
            }
            _ => None,
        }
    }

    fn eval_unary_expression(&self, expr: &UnaryExpression<'a>) -> Option<ConstantValue<'a>> {
        match expr.operator {
            UnaryOperator::Typeof => {
                let s = match &expr.argument {
                    Expression::ObjectExpression(_) | Expression::ArrayExpression(_)
                        if expr.argument.is_literal_value(true) =>
                    {
                        "object"
                    }
                    Expression::FunctionExpression(_) => "function",
                    Expression::StringLiteral(_) => "string",
                    Expression::NumericLiteral(_) => "number",
                    Expression::BooleanLiteral(_) => "boolean",
                    Expression::NullLiteral(_) => "object",
                    Expression::UnaryExpression(e) if e.operator == UnaryOperator::Void => {
                        "undefined"
                    }
                    Expression::BigIntLiteral(_) => "bigint",
                    Expression::Identifier(ident) => match ident.name.as_str() {
                        "undefined" if self.is_global_reference(ident) => "undefined",
                        "NaN" | "Infinity" if self.is_global_reference(ident) => "number",
                        _ => return None,
                    },
                    _ => return None,
                };
                Some(ConstantValue::String(Cow::Borrowed(s)))
            }
            UnaryOperator::Void => (expr.argument.is_literal()
                || !self.expression_may_have_side_efffects(&expr.argument))
            .then_some(ConstantValue::Undefined),
            UnaryOperator::LogicalNot => self
                .get_side_free_boolean_value(&expr.argument)
                .map(|b| !b)
                .map(ConstantValue::Boolean),
            UnaryOperator::UnaryPlus => {
                self.eval_to_number(&expr.argument).map(ConstantValue::Number)
            }
            UnaryOperator::UnaryNegation => match ValueType::from(&expr.argument) {
                ValueType::BigInt => {
                    self.eval_to_big_int(&expr.argument).map(|v| -v).map(ConstantValue::BigInt)
                }
                ValueType::Number => self
                    .eval_to_number(&expr.argument)
                    .map(|v| if v.is_nan() { v } else { -v })
                    .map(ConstantValue::Number),
                ValueType::Undefined => Some(ConstantValue::Number(f64::NAN)),
                ValueType::Null => Some(ConstantValue::Number(-0.0)),
                _ => None,
            },
            UnaryOperator::BitwiseNot => match ValueType::from(&expr.argument) {
                ValueType::BigInt => {
                    self.eval_to_big_int(&expr.argument).map(|v| !v).map(ConstantValue::BigInt)
                }
                #[expect(clippy::cast_lossless)]
                _ => self
                    .eval_to_number(&expr.argument)
                    .map(|v| (!v.to_int_32()) as f64)
                    .map(ConstantValue::Number),
            },
            UnaryOperator::Delete => None,
        }
    }

    fn eval_static_member_expression(
        &self,
        expr: &StaticMemberExpression<'a>,
    ) -> Option<ConstantValue<'a>> {
        match expr.property.name.as_str() {
            "length" => {
                if let Some(ConstantValue::String(s)) = self.eval_expression(&expr.object) {
                    Some(ConstantValue::Number(s.encode_utf16().count().to_f64().unwrap()))
                } else {
                    if self.expression_may_have_side_efffects(&expr.object) {
                        return None;
                    }
                    if let Expression::ArrayExpression(arr) = &expr.object {
                        Some(ConstantValue::Number(arr.elements.len().to_f64().unwrap()))
                    } else {
                        None
                    }
                }
            }
            _ => None,
        }
    }

    fn eval_computed_member_expression(
        &self,
        expr: &ElementAccessExpression<'a>,
    ) -> Option<ConstantValue<'a>> {
        match &expr.argument_expression {
            Expression::StringLiteral(s) if s.value == "length" => {
                if let Some(ConstantValue::String(s)) = self.eval_expression(&expr.object) {
                    Some(ConstantValue::Number(s.encode_utf16().count().to_f64().unwrap()))
                } else {
                    if self.expression_may_have_side_efffects(&expr.object) {
                        return None;
                    }
                    if let Expression::ArrayExpression(arr) = &expr.object {
                        Some(ConstantValue::Number(arr.elements.len().to_f64().unwrap()))
                    } else {
                        None
                    }
                }
            }
            _ => None,
        }
    }

    /// <https://tc39.es/ecma262/#sec-abstract-relational-comparison>
    fn is_less_than(&self, x: &Expression<'a>, y: &Expression<'a>) -> Option<ConstantValue<'a>> {
        // a. Let px be ? ToPrimitive(x, number).
        // b. Let py be ? ToPrimitive(y, number).
        let px = ValueType::from(x);

        // `.toString()` method is called and compared.
        // TODO: bigint is handled very differently in the spec
        if px.is_object() || px.is_undetermined() || px.is_bigint() {
            return None;
        }

        let py = ValueType::from(y);

        if py.is_object() || py.is_undetermined() || py.is_bigint() {
            return None;
        }

        if px.is_string() && py.is_string() {
            let left_string = self.get_side_free_string_value(x)?;
            let right_string = self.get_side_free_string_value(y)?;
            return Some(ConstantValue::Boolean(left_string.cmp(&right_string) == Ordering::Less));
        }

        let left_num = self.get_side_free_number_value(x)?;
        if left_num.is_nan() {
            return Some(ConstantValue::Undefined);
        }
        let right_num = self.get_side_free_number_value(y)?;
        if right_num.is_nan() {
            return Some(ConstantValue::Undefined);
        }
        Some(ConstantValue::Boolean(left_num < right_num))
    }
}
