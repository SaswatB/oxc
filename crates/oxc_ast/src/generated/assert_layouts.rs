// Auto-generated code, DO NOT EDIT DIRECTLY!
// To edit this generated file you have to edit `tasks/ast_tools/src/generators/assert_layouts.rs`

use std::mem::{align_of, offset_of, size_of};

use oxc_regular_expression::ast::*;

use crate::ast::*;

#[cfg(target_pointer_width = "64")]
const _: () = {
    assert!(size_of::<BooleanLiteral>() == 32usize);
    assert!(align_of::<BooleanLiteral>() == 8usize);
    assert!(offset_of!(BooleanLiteral, parent) == 0usize);
    assert!(offset_of!(BooleanLiteral, span) == 16usize);
    assert!(offset_of!(BooleanLiteral, value) == 24usize);

    assert!(size_of::<NullLiteral>() == 24usize);
    assert!(align_of::<NullLiteral>() == 8usize);
    assert!(offset_of!(NullLiteral, parent) == 0usize);
    assert!(offset_of!(NullLiteral, span) == 16usize);

    assert!(size_of::<NumericLiteral>() == 56usize);
    assert!(align_of::<NumericLiteral>() == 8usize);
    assert!(offset_of!(NumericLiteral, parent) == 0usize);
    assert!(offset_of!(NumericLiteral, span) == 16usize);
    assert!(offset_of!(NumericLiteral, value) == 24usize);
    assert!(offset_of!(NumericLiteral, raw) == 32usize);
    assert!(offset_of!(NumericLiteral, base) == 48usize);

    assert!(size_of::<StringLiteral>() == 56usize);
    assert!(align_of::<StringLiteral>() == 8usize);
    assert!(offset_of!(StringLiteral, parent) == 0usize);
    assert!(offset_of!(StringLiteral, span) == 16usize);
    assert!(offset_of!(StringLiteral, value) == 24usize);
    assert!(offset_of!(StringLiteral, raw) == 40usize);

    assert!(size_of::<BigIntLiteral>() == 48usize);
    assert!(align_of::<BigIntLiteral>() == 8usize);
    assert!(offset_of!(BigIntLiteral, parent) == 0usize);
    assert!(offset_of!(BigIntLiteral, span) == 16usize);
    assert!(offset_of!(BigIntLiteral, raw) == 24usize);
    assert!(offset_of!(BigIntLiteral, base) == 40usize);

    assert!(size_of::<RegExpLiteral>() == 72usize);
    assert!(align_of::<RegExpLiteral>() == 8usize);
    assert!(offset_of!(RegExpLiteral, parent) == 0usize);
    assert!(offset_of!(RegExpLiteral, span) == 16usize);
    assert!(offset_of!(RegExpLiteral, regex) == 24usize);
    assert!(offset_of!(RegExpLiteral, raw) == 56usize);

    assert!(size_of::<RegExp>() == 32usize);
    assert!(align_of::<RegExp>() == 8usize);
    assert!(offset_of!(RegExp, pattern) == 0usize);
    assert!(offset_of!(RegExp, flags) == 24usize);

    assert!(size_of::<RegExpPattern>() == 24usize);
    assert!(align_of::<RegExpPattern>() == 8usize);

    assert!(size_of::<Program>() == 176usize);
    assert!(align_of::<Program>() == 8usize);
    assert!(offset_of!(Program, span) == 0usize);
    assert!(offset_of!(Program, source_type) == 8usize);
    assert!(offset_of!(Program, source_text) == 16usize);
    assert!(offset_of!(Program, comments) == 32usize);
    assert!(offset_of!(Program, hashbang) == 64usize);
    assert!(offset_of!(Program, directives) == 104usize);
    assert!(offset_of!(Program, body) == 136usize);
    assert!(offset_of!(Program, scope_id) == 168usize);
    assert!(offset_of!(Program, id) == 172usize);

    assert!(size_of::<Expression>() == 16usize);
    assert!(align_of::<Expression>() == 8usize);

    assert!(size_of::<IdentifierName>() == 40usize);
    assert!(align_of::<IdentifierName>() == 8usize);
    assert!(offset_of!(IdentifierName, parent) == 0usize);
    assert!(offset_of!(IdentifierName, span) == 16usize);
    assert!(offset_of!(IdentifierName, name) == 24usize);

    assert!(size_of::<IdentifierReference>() == 48usize);
    assert!(align_of::<IdentifierReference>() == 8usize);
    assert!(offset_of!(IdentifierReference, parent) == 0usize);
    assert!(offset_of!(IdentifierReference, span) == 16usize);
    assert!(offset_of!(IdentifierReference, name) == 24usize);
    assert!(offset_of!(IdentifierReference, reference_id) == 40usize);

    assert!(size_of::<BindingIdentifier>() == 48usize);
    assert!(align_of::<BindingIdentifier>() == 8usize);
    assert!(offset_of!(BindingIdentifier, parent) == 0usize);
    assert!(offset_of!(BindingIdentifier, span) == 16usize);
    assert!(offset_of!(BindingIdentifier, name) == 24usize);
    assert!(offset_of!(BindingIdentifier, symbol_id) == 40usize);

    assert!(size_of::<LabelIdentifier>() == 40usize);
    assert!(align_of::<LabelIdentifier>() == 8usize);
    assert!(offset_of!(LabelIdentifier, parent) == 0usize);
    assert!(offset_of!(LabelIdentifier, span) == 16usize);
    assert!(offset_of!(LabelIdentifier, name) == 24usize);

    assert!(size_of::<ThisExpression>() == 24usize);
    assert!(align_of::<ThisExpression>() == 8usize);
    assert!(offset_of!(ThisExpression, parent) == 0usize);
    assert!(offset_of!(ThisExpression, span) == 16usize);

    assert!(size_of::<ArrayExpression>() == 72usize);
    assert!(align_of::<ArrayExpression>() == 8usize);
    assert!(offset_of!(ArrayExpression, parent) == 0usize);
    assert!(offset_of!(ArrayExpression, span) == 16usize);
    assert!(offset_of!(ArrayExpression, elements) == 24usize);
    assert!(offset_of!(ArrayExpression, trailing_comma) == 56usize);

    assert!(size_of::<ArrayExpressionElement>() == 32usize);
    assert!(align_of::<ArrayExpressionElement>() == 8usize);

    assert!(size_of::<Elision>() == 24usize);
    assert!(align_of::<Elision>() == 8usize);
    assert!(offset_of!(Elision, parent) == 0usize);
    assert!(offset_of!(Elision, span) == 16usize);

    assert!(size_of::<ObjectExpression>() == 72usize);
    assert!(align_of::<ObjectExpression>() == 8usize);
    assert!(offset_of!(ObjectExpression, parent) == 0usize);
    assert!(offset_of!(ObjectExpression, span) == 16usize);
    assert!(offset_of!(ObjectExpression, properties) == 24usize);
    assert!(offset_of!(ObjectExpression, trailing_comma) == 56usize);

    assert!(size_of::<ObjectPropertyKind>() == 16usize);
    assert!(align_of::<ObjectPropertyKind>() == 8usize);

    assert!(size_of::<ObjectProperty>() == 72usize);
    assert!(align_of::<ObjectProperty>() == 8usize);
    assert!(offset_of!(ObjectProperty, parent) == 0usize);
    assert!(offset_of!(ObjectProperty, span) == 16usize);
    assert!(offset_of!(ObjectProperty, kind) == 24usize);
    assert!(offset_of!(ObjectProperty, key) == 32usize);
    assert!(offset_of!(ObjectProperty, value) == 48usize);
    assert!(offset_of!(ObjectProperty, method) == 64usize);
    assert!(offset_of!(ObjectProperty, shorthand) == 65usize);
    assert!(offset_of!(ObjectProperty, computed) == 66usize);

    assert!(size_of::<PropertyKey>() == 16usize);
    assert!(align_of::<PropertyKey>() == 8usize);

    assert!(size_of::<PropertyKind>() == 1usize);
    assert!(align_of::<PropertyKind>() == 1usize);

    assert!(size_of::<TemplateLiteral>() == 88usize);
    assert!(align_of::<TemplateLiteral>() == 8usize);
    assert!(offset_of!(TemplateLiteral, parent) == 0usize);
    assert!(offset_of!(TemplateLiteral, span) == 16usize);
    assert!(offset_of!(TemplateLiteral, quasis) == 24usize);
    assert!(offset_of!(TemplateLiteral, expressions) == 56usize);

    assert!(size_of::<TaggedTemplateExpression>() == 136usize);
    assert!(align_of::<TaggedTemplateExpression>() == 8usize);
    assert!(offset_of!(TaggedTemplateExpression, parent) == 0usize);
    assert!(offset_of!(TaggedTemplateExpression, span) == 16usize);
    assert!(offset_of!(TaggedTemplateExpression, tag) == 24usize);
    assert!(offset_of!(TaggedTemplateExpression, quasi) == 40usize);
    assert!(offset_of!(TaggedTemplateExpression, type_parameters) == 128usize);

    assert!(size_of::<TemplateElement>() == 64usize);
    assert!(align_of::<TemplateElement>() == 8usize);
    assert!(offset_of!(TemplateElement, parent) == 0usize);
    assert!(offset_of!(TemplateElement, span) == 16usize);
    assert!(offset_of!(TemplateElement, tail) == 24usize);
    assert!(offset_of!(TemplateElement, value) == 32usize);

    assert!(size_of::<TemplateElementValue>() == 32usize);
    assert!(align_of::<TemplateElementValue>() == 8usize);
    assert!(offset_of!(TemplateElementValue, raw) == 0usize);
    assert!(offset_of!(TemplateElementValue, cooked) == 16usize);

    assert!(size_of::<MemberExpression>() == 16usize);
    assert!(align_of::<MemberExpression>() == 8usize);

    assert!(size_of::<ComputedMemberExpression>() == 64usize);
    assert!(align_of::<ComputedMemberExpression>() == 8usize);
    assert!(offset_of!(ComputedMemberExpression, parent) == 0usize);
    assert!(offset_of!(ComputedMemberExpression, span) == 16usize);
    assert!(offset_of!(ComputedMemberExpression, object) == 24usize);
    assert!(offset_of!(ComputedMemberExpression, expression) == 40usize);
    assert!(offset_of!(ComputedMemberExpression, optional) == 56usize);

    assert!(size_of::<StaticMemberExpression>() == 88usize);
    assert!(align_of::<StaticMemberExpression>() == 8usize);
    assert!(offset_of!(StaticMemberExpression, parent) == 0usize);
    assert!(offset_of!(StaticMemberExpression, span) == 16usize);
    assert!(offset_of!(StaticMemberExpression, object) == 24usize);
    assert!(offset_of!(StaticMemberExpression, property) == 40usize);
    assert!(offset_of!(StaticMemberExpression, optional) == 80usize);

    assert!(size_of::<PrivateFieldExpression>() == 88usize);
    assert!(align_of::<PrivateFieldExpression>() == 8usize);
    assert!(offset_of!(PrivateFieldExpression, parent) == 0usize);
    assert!(offset_of!(PrivateFieldExpression, span) == 16usize);
    assert!(offset_of!(PrivateFieldExpression, object) == 24usize);
    assert!(offset_of!(PrivateFieldExpression, field) == 40usize);
    assert!(offset_of!(PrivateFieldExpression, optional) == 80usize);

    assert!(size_of::<CallExpression>() == 88usize);
    assert!(align_of::<CallExpression>() == 8usize);
    assert!(offset_of!(CallExpression, parent) == 0usize);
    assert!(offset_of!(CallExpression, span) == 16usize);
    assert!(offset_of!(CallExpression, callee) == 24usize);
    assert!(offset_of!(CallExpression, type_parameters) == 40usize);
    assert!(offset_of!(CallExpression, arguments) == 48usize);
    assert!(offset_of!(CallExpression, optional) == 80usize);

    assert!(size_of::<NewExpression>() == 80usize);
    assert!(align_of::<NewExpression>() == 8usize);
    assert!(offset_of!(NewExpression, parent) == 0usize);
    assert!(offset_of!(NewExpression, span) == 16usize);
    assert!(offset_of!(NewExpression, callee) == 24usize);
    assert!(offset_of!(NewExpression, arguments) == 40usize);
    assert!(offset_of!(NewExpression, type_parameters) == 72usize);

    assert!(size_of::<MetaProperty>() == 104usize);
    assert!(align_of::<MetaProperty>() == 8usize);
    assert!(offset_of!(MetaProperty, parent) == 0usize);
    assert!(offset_of!(MetaProperty, span) == 16usize);
    assert!(offset_of!(MetaProperty, meta) == 24usize);
    assert!(offset_of!(MetaProperty, property) == 64usize);

    assert!(size_of::<SpreadElement>() == 40usize);
    assert!(align_of::<SpreadElement>() == 8usize);
    assert!(offset_of!(SpreadElement, parent) == 0usize);
    assert!(offset_of!(SpreadElement, span) == 16usize);
    assert!(offset_of!(SpreadElement, argument) == 24usize);

    assert!(size_of::<Argument>() == 16usize);
    assert!(align_of::<Argument>() == 8usize);

    assert!(size_of::<UpdateExpression>() == 48usize);
    assert!(align_of::<UpdateExpression>() == 8usize);
    assert!(offset_of!(UpdateExpression, parent) == 0usize);
    assert!(offset_of!(UpdateExpression, span) == 16usize);
    assert!(offset_of!(UpdateExpression, operator) == 24usize);
    assert!(offset_of!(UpdateExpression, prefix) == 25usize);
    assert!(offset_of!(UpdateExpression, argument) == 32usize);

    assert!(size_of::<UnaryExpression>() == 48usize);
    assert!(align_of::<UnaryExpression>() == 8usize);
    assert!(offset_of!(UnaryExpression, parent) == 0usize);
    assert!(offset_of!(UnaryExpression, span) == 16usize);
    assert!(offset_of!(UnaryExpression, operator) == 24usize);
    assert!(offset_of!(UnaryExpression, argument) == 32usize);

    assert!(size_of::<BinaryExpression>() == 64usize);
    assert!(align_of::<BinaryExpression>() == 8usize);
    assert!(offset_of!(BinaryExpression, parent) == 0usize);
    assert!(offset_of!(BinaryExpression, span) == 16usize);
    assert!(offset_of!(BinaryExpression, left) == 24usize);
    assert!(offset_of!(BinaryExpression, operator) == 40usize);
    assert!(offset_of!(BinaryExpression, right) == 48usize);

    assert!(size_of::<PrivateInExpression>() == 88usize);
    assert!(align_of::<PrivateInExpression>() == 8usize);
    assert!(offset_of!(PrivateInExpression, parent) == 0usize);
    assert!(offset_of!(PrivateInExpression, span) == 16usize);
    assert!(offset_of!(PrivateInExpression, left) == 24usize);
    assert!(offset_of!(PrivateInExpression, operator) == 64usize);
    assert!(offset_of!(PrivateInExpression, right) == 72usize);

    assert!(size_of::<LogicalExpression>() == 64usize);
    assert!(align_of::<LogicalExpression>() == 8usize);
    assert!(offset_of!(LogicalExpression, parent) == 0usize);
    assert!(offset_of!(LogicalExpression, span) == 16usize);
    assert!(offset_of!(LogicalExpression, left) == 24usize);
    assert!(offset_of!(LogicalExpression, operator) == 40usize);
    assert!(offset_of!(LogicalExpression, right) == 48usize);

    assert!(size_of::<ConditionalExpression>() == 72usize);
    assert!(align_of::<ConditionalExpression>() == 8usize);
    assert!(offset_of!(ConditionalExpression, parent) == 0usize);
    assert!(offset_of!(ConditionalExpression, span) == 16usize);
    assert!(offset_of!(ConditionalExpression, test) == 24usize);
    assert!(offset_of!(ConditionalExpression, consequent) == 40usize);
    assert!(offset_of!(ConditionalExpression, alternate) == 56usize);

    assert!(size_of::<AssignmentExpression>() == 64usize);
    assert!(align_of::<AssignmentExpression>() == 8usize);
    assert!(offset_of!(AssignmentExpression, parent) == 0usize);
    assert!(offset_of!(AssignmentExpression, span) == 16usize);
    assert!(offset_of!(AssignmentExpression, operator) == 24usize);
    assert!(offset_of!(AssignmentExpression, left) == 32usize);
    assert!(offset_of!(AssignmentExpression, right) == 48usize);

    assert!(size_of::<AssignmentTarget>() == 16usize);
    assert!(align_of::<AssignmentTarget>() == 8usize);

    assert!(size_of::<SimpleAssignmentTarget>() == 16usize);
    assert!(align_of::<SimpleAssignmentTarget>() == 8usize);

    assert!(size_of::<AssignmentTargetPattern>() == 16usize);
    assert!(align_of::<AssignmentTargetPattern>() == 8usize);

    assert!(size_of::<ArrayAssignmentTarget>() == 112usize);
    assert!(align_of::<ArrayAssignmentTarget>() == 8usize);
    assert!(offset_of!(ArrayAssignmentTarget, parent) == 0usize);
    assert!(offset_of!(ArrayAssignmentTarget, span) == 16usize);
    assert!(offset_of!(ArrayAssignmentTarget, elements) == 24usize);
    assert!(offset_of!(ArrayAssignmentTarget, rest) == 56usize);
    assert!(offset_of!(ArrayAssignmentTarget, trailing_comma) == 96usize);

    assert!(size_of::<ObjectAssignmentTarget>() == 96usize);
    assert!(align_of::<ObjectAssignmentTarget>() == 8usize);
    assert!(offset_of!(ObjectAssignmentTarget, parent) == 0usize);
    assert!(offset_of!(ObjectAssignmentTarget, span) == 16usize);
    assert!(offset_of!(ObjectAssignmentTarget, properties) == 24usize);
    assert!(offset_of!(ObjectAssignmentTarget, rest) == 56usize);

    assert!(size_of::<AssignmentTargetRest>() == 40usize);
    assert!(align_of::<AssignmentTargetRest>() == 8usize);
    assert!(offset_of!(AssignmentTargetRest, parent) == 0usize);
    assert!(offset_of!(AssignmentTargetRest, span) == 16usize);
    assert!(offset_of!(AssignmentTargetRest, target) == 24usize);

    assert!(size_of::<AssignmentTargetMaybeDefault>() == 16usize);
    assert!(align_of::<AssignmentTargetMaybeDefault>() == 8usize);

    assert!(size_of::<AssignmentTargetWithDefault>() == 56usize);
    assert!(align_of::<AssignmentTargetWithDefault>() == 8usize);
    assert!(offset_of!(AssignmentTargetWithDefault, parent) == 0usize);
    assert!(offset_of!(AssignmentTargetWithDefault, span) == 16usize);
    assert!(offset_of!(AssignmentTargetWithDefault, binding) == 24usize);
    assert!(offset_of!(AssignmentTargetWithDefault, init) == 40usize);

    assert!(size_of::<AssignmentTargetProperty>() == 16usize);
    assert!(align_of::<AssignmentTargetProperty>() == 8usize);

    assert!(size_of::<AssignmentTargetPropertyIdentifier>() == 88usize);
    assert!(align_of::<AssignmentTargetPropertyIdentifier>() == 8usize);
    assert!(offset_of!(AssignmentTargetPropertyIdentifier, parent) == 0usize);
    assert!(offset_of!(AssignmentTargetPropertyIdentifier, span) == 16usize);
    assert!(offset_of!(AssignmentTargetPropertyIdentifier, binding) == 24usize);
    assert!(offset_of!(AssignmentTargetPropertyIdentifier, init) == 72usize);

    assert!(size_of::<AssignmentTargetPropertyProperty>() == 64usize);
    assert!(align_of::<AssignmentTargetPropertyProperty>() == 8usize);
    assert!(offset_of!(AssignmentTargetPropertyProperty, parent) == 0usize);
    assert!(offset_of!(AssignmentTargetPropertyProperty, span) == 16usize);
    assert!(offset_of!(AssignmentTargetPropertyProperty, name) == 24usize);
    assert!(offset_of!(AssignmentTargetPropertyProperty, binding) == 40usize);
    assert!(offset_of!(AssignmentTargetPropertyProperty, computed) == 56usize);

    assert!(size_of::<SequenceExpression>() == 56usize);
    assert!(align_of::<SequenceExpression>() == 8usize);
    assert!(offset_of!(SequenceExpression, parent) == 0usize);
    assert!(offset_of!(SequenceExpression, span) == 16usize);
    assert!(offset_of!(SequenceExpression, expressions) == 24usize);

    assert!(size_of::<Super>() == 24usize);
    assert!(align_of::<Super>() == 8usize);
    assert!(offset_of!(Super, parent) == 0usize);
    assert!(offset_of!(Super, span) == 16usize);

    assert!(size_of::<AwaitExpression>() == 40usize);
    assert!(align_of::<AwaitExpression>() == 8usize);
    assert!(offset_of!(AwaitExpression, parent) == 0usize);
    assert!(offset_of!(AwaitExpression, span) == 16usize);
    assert!(offset_of!(AwaitExpression, argument) == 24usize);

    assert!(size_of::<ChainExpression>() == 40usize);
    assert!(align_of::<ChainExpression>() == 8usize);
    assert!(offset_of!(ChainExpression, parent) == 0usize);
    assert!(offset_of!(ChainExpression, span) == 16usize);
    assert!(offset_of!(ChainExpression, expression) == 24usize);

    assert!(size_of::<ChainElement>() == 16usize);
    assert!(align_of::<ChainElement>() == 8usize);

    assert!(size_of::<ParenthesizedExpression>() == 40usize);
    assert!(align_of::<ParenthesizedExpression>() == 8usize);
    assert!(offset_of!(ParenthesizedExpression, parent) == 0usize);
    assert!(offset_of!(ParenthesizedExpression, span) == 16usize);
    assert!(offset_of!(ParenthesizedExpression, expression) == 24usize);

    assert!(size_of::<Statement>() == 16usize);
    assert!(align_of::<Statement>() == 8usize);

    assert!(size_of::<Directive>() == 96usize);
    assert!(align_of::<Directive>() == 8usize);
    assert!(offset_of!(Directive, parent) == 0usize);
    assert!(offset_of!(Directive, span) == 16usize);
    assert!(offset_of!(Directive, expression) == 24usize);
    assert!(offset_of!(Directive, directive) == 80usize);

    assert!(size_of::<Hashbang>() == 40usize);
    assert!(align_of::<Hashbang>() == 8usize);
    assert!(offset_of!(Hashbang, parent) == 0usize);
    assert!(offset_of!(Hashbang, span) == 16usize);
    assert!(offset_of!(Hashbang, value) == 24usize);

    assert!(size_of::<BlockStatement>() == 64usize);
    assert!(align_of::<BlockStatement>() == 8usize);
    assert!(offset_of!(BlockStatement, parent) == 0usize);
    assert!(offset_of!(BlockStatement, span) == 16usize);
    assert!(offset_of!(BlockStatement, body) == 24usize);
    assert!(offset_of!(BlockStatement, scope_id) == 56usize);

    assert!(size_of::<Declaration>() == 16usize);
    assert!(align_of::<Declaration>() == 8usize);

    assert!(size_of::<VariableDeclaration>() == 72usize);
    assert!(align_of::<VariableDeclaration>() == 8usize);
    assert!(offset_of!(VariableDeclaration, parent) == 0usize);
    assert!(offset_of!(VariableDeclaration, span) == 16usize);
    assert!(offset_of!(VariableDeclaration, kind) == 24usize);
    assert!(offset_of!(VariableDeclaration, declarations) == 32usize);
    assert!(offset_of!(VariableDeclaration, declare) == 64usize);

    assert!(size_of::<VariableDeclarationKind>() == 1usize);
    assert!(align_of::<VariableDeclarationKind>() == 1usize);

    assert!(size_of::<VariableDeclarator>() == 88usize);
    assert!(align_of::<VariableDeclarator>() == 8usize);
    assert!(offset_of!(VariableDeclarator, parent) == 0usize);
    assert!(offset_of!(VariableDeclarator, span) == 16usize);
    assert!(offset_of!(VariableDeclarator, kind) == 24usize);
    assert!(offset_of!(VariableDeclarator, id) == 32usize);
    assert!(offset_of!(VariableDeclarator, init) == 64usize);
    assert!(offset_of!(VariableDeclarator, definite) == 80usize);

    assert!(size_of::<EmptyStatement>() == 24usize);
    assert!(align_of::<EmptyStatement>() == 8usize);
    assert!(offset_of!(EmptyStatement, parent) == 0usize);
    assert!(offset_of!(EmptyStatement, span) == 16usize);

    assert!(size_of::<ExpressionStatement>() == 40usize);
    assert!(align_of::<ExpressionStatement>() == 8usize);
    assert!(offset_of!(ExpressionStatement, parent) == 0usize);
    assert!(offset_of!(ExpressionStatement, span) == 16usize);
    assert!(offset_of!(ExpressionStatement, expression) == 24usize);

    assert!(size_of::<IfStatement>() == 72usize);
    assert!(align_of::<IfStatement>() == 8usize);
    assert!(offset_of!(IfStatement, parent) == 0usize);
    assert!(offset_of!(IfStatement, span) == 16usize);
    assert!(offset_of!(IfStatement, test) == 24usize);
    assert!(offset_of!(IfStatement, consequent) == 40usize);
    assert!(offset_of!(IfStatement, alternate) == 56usize);

    assert!(size_of::<DoWhileStatement>() == 56usize);
    assert!(align_of::<DoWhileStatement>() == 8usize);
    assert!(offset_of!(DoWhileStatement, parent) == 0usize);
    assert!(offset_of!(DoWhileStatement, span) == 16usize);
    assert!(offset_of!(DoWhileStatement, body) == 24usize);
    assert!(offset_of!(DoWhileStatement, test) == 40usize);

    assert!(size_of::<WhileStatement>() == 56usize);
    assert!(align_of::<WhileStatement>() == 8usize);
    assert!(offset_of!(WhileStatement, parent) == 0usize);
    assert!(offset_of!(WhileStatement, span) == 16usize);
    assert!(offset_of!(WhileStatement, test) == 24usize);
    assert!(offset_of!(WhileStatement, body) == 40usize);

    assert!(size_of::<ForStatement>() == 96usize);
    assert!(align_of::<ForStatement>() == 8usize);
    assert!(offset_of!(ForStatement, parent) == 0usize);
    assert!(offset_of!(ForStatement, span) == 16usize);
    assert!(offset_of!(ForStatement, init) == 24usize);
    assert!(offset_of!(ForStatement, test) == 40usize);
    assert!(offset_of!(ForStatement, update) == 56usize);
    assert!(offset_of!(ForStatement, body) == 72usize);
    assert!(offset_of!(ForStatement, scope_id) == 88usize);

    assert!(size_of::<ForStatementInit>() == 16usize);
    assert!(align_of::<ForStatementInit>() == 8usize);

    assert!(size_of::<ForInStatement>() == 80usize);
    assert!(align_of::<ForInStatement>() == 8usize);
    assert!(offset_of!(ForInStatement, parent) == 0usize);
    assert!(offset_of!(ForInStatement, span) == 16usize);
    assert!(offset_of!(ForInStatement, left) == 24usize);
    assert!(offset_of!(ForInStatement, right) == 40usize);
    assert!(offset_of!(ForInStatement, body) == 56usize);
    assert!(offset_of!(ForInStatement, scope_id) == 72usize);

    assert!(size_of::<ForStatementLeft>() == 16usize);
    assert!(align_of::<ForStatementLeft>() == 8usize);

    assert!(size_of::<ForOfStatement>() == 88usize);
    assert!(align_of::<ForOfStatement>() == 8usize);
    assert!(offset_of!(ForOfStatement, parent) == 0usize);
    assert!(offset_of!(ForOfStatement, span) == 16usize);
    assert!(offset_of!(ForOfStatement, r#await) == 24usize);
    assert!(offset_of!(ForOfStatement, left) == 32usize);
    assert!(offset_of!(ForOfStatement, right) == 48usize);
    assert!(offset_of!(ForOfStatement, body) == 64usize);
    assert!(offset_of!(ForOfStatement, scope_id) == 80usize);

    assert!(size_of::<ContinueStatement>() == 64usize);
    assert!(align_of::<ContinueStatement>() == 8usize);
    assert!(offset_of!(ContinueStatement, parent) == 0usize);
    assert!(offset_of!(ContinueStatement, span) == 16usize);
    assert!(offset_of!(ContinueStatement, label) == 24usize);

    assert!(size_of::<BreakStatement>() == 64usize);
    assert!(align_of::<BreakStatement>() == 8usize);
    assert!(offset_of!(BreakStatement, parent) == 0usize);
    assert!(offset_of!(BreakStatement, span) == 16usize);
    assert!(offset_of!(BreakStatement, label) == 24usize);

    assert!(size_of::<ReturnStatement>() == 40usize);
    assert!(align_of::<ReturnStatement>() == 8usize);
    assert!(offset_of!(ReturnStatement, parent) == 0usize);
    assert!(offset_of!(ReturnStatement, span) == 16usize);
    assert!(offset_of!(ReturnStatement, argument) == 24usize);

    assert!(size_of::<WithStatement>() == 56usize);
    assert!(align_of::<WithStatement>() == 8usize);
    assert!(offset_of!(WithStatement, parent) == 0usize);
    assert!(offset_of!(WithStatement, span) == 16usize);
    assert!(offset_of!(WithStatement, object) == 24usize);
    assert!(offset_of!(WithStatement, body) == 40usize);

    assert!(size_of::<SwitchStatement>() == 80usize);
    assert!(align_of::<SwitchStatement>() == 8usize);
    assert!(offset_of!(SwitchStatement, parent) == 0usize);
    assert!(offset_of!(SwitchStatement, span) == 16usize);
    assert!(offset_of!(SwitchStatement, discriminant) == 24usize);
    assert!(offset_of!(SwitchStatement, cases) == 40usize);
    assert!(offset_of!(SwitchStatement, scope_id) == 72usize);

    assert!(size_of::<SwitchCase>() == 72usize);
    assert!(align_of::<SwitchCase>() == 8usize);
    assert!(offset_of!(SwitchCase, parent) == 0usize);
    assert!(offset_of!(SwitchCase, span) == 16usize);
    assert!(offset_of!(SwitchCase, test) == 24usize);
    assert!(offset_of!(SwitchCase, consequent) == 40usize);

    assert!(size_of::<LabeledStatement>() == 80usize);
    assert!(align_of::<LabeledStatement>() == 8usize);
    assert!(offset_of!(LabeledStatement, parent) == 0usize);
    assert!(offset_of!(LabeledStatement, span) == 16usize);
    assert!(offset_of!(LabeledStatement, label) == 24usize);
    assert!(offset_of!(LabeledStatement, body) == 64usize);

    assert!(size_of::<ThrowStatement>() == 40usize);
    assert!(align_of::<ThrowStatement>() == 8usize);
    assert!(offset_of!(ThrowStatement, parent) == 0usize);
    assert!(offset_of!(ThrowStatement, span) == 16usize);
    assert!(offset_of!(ThrowStatement, argument) == 24usize);

    assert!(size_of::<TryStatement>() == 48usize);
    assert!(align_of::<TryStatement>() == 8usize);
    assert!(offset_of!(TryStatement, parent) == 0usize);
    assert!(offset_of!(TryStatement, span) == 16usize);
    assert!(offset_of!(TryStatement, block) == 24usize);
    assert!(offset_of!(TryStatement, handler) == 32usize);
    assert!(offset_of!(TryStatement, finalizer) == 40usize);

    assert!(size_of::<CatchClause>() == 96usize);
    assert!(align_of::<CatchClause>() == 8usize);
    assert!(offset_of!(CatchClause, parent) == 0usize);
    assert!(offset_of!(CatchClause, span) == 16usize);
    assert!(offset_of!(CatchClause, param) == 24usize);
    assert!(offset_of!(CatchClause, body) == 80usize);
    assert!(offset_of!(CatchClause, scope_id) == 88usize);

    assert!(size_of::<CatchParameter>() == 56usize);
    assert!(align_of::<CatchParameter>() == 8usize);
    assert!(offset_of!(CatchParameter, parent) == 0usize);
    assert!(offset_of!(CatchParameter, span) == 16usize);
    assert!(offset_of!(CatchParameter, pattern) == 24usize);

    assert!(size_of::<DebuggerStatement>() == 24usize);
    assert!(align_of::<DebuggerStatement>() == 8usize);
    assert!(offset_of!(DebuggerStatement, parent) == 0usize);
    assert!(offset_of!(DebuggerStatement, span) == 16usize);

    assert!(size_of::<BindingPattern>() == 32usize);
    assert!(align_of::<BindingPattern>() == 8usize);
    assert!(offset_of!(BindingPattern, kind) == 0usize);
    assert!(offset_of!(BindingPattern, type_annotation) == 16usize);
    assert!(offset_of!(BindingPattern, optional) == 24usize);

    assert!(size_of::<BindingPatternKind>() == 16usize);
    assert!(align_of::<BindingPatternKind>() == 8usize);

    assert!(size_of::<AssignmentPattern>() == 72usize);
    assert!(align_of::<AssignmentPattern>() == 8usize);
    assert!(offset_of!(AssignmentPattern, parent) == 0usize);
    assert!(offset_of!(AssignmentPattern, span) == 16usize);
    assert!(offset_of!(AssignmentPattern, left) == 24usize);
    assert!(offset_of!(AssignmentPattern, right) == 56usize);

    assert!(size_of::<ObjectPattern>() == 64usize);
    assert!(align_of::<ObjectPattern>() == 8usize);
    assert!(offset_of!(ObjectPattern, parent) == 0usize);
    assert!(offset_of!(ObjectPattern, span) == 16usize);
    assert!(offset_of!(ObjectPattern, properties) == 24usize);
    assert!(offset_of!(ObjectPattern, rest) == 56usize);

    assert!(size_of::<BindingProperty>() == 80usize);
    assert!(align_of::<BindingProperty>() == 8usize);
    assert!(offset_of!(BindingProperty, parent) == 0usize);
    assert!(offset_of!(BindingProperty, span) == 16usize);
    assert!(offset_of!(BindingProperty, key) == 24usize);
    assert!(offset_of!(BindingProperty, value) == 40usize);
    assert!(offset_of!(BindingProperty, shorthand) == 72usize);
    assert!(offset_of!(BindingProperty, computed) == 73usize);

    assert!(size_of::<ArrayPattern>() == 64usize);
    assert!(align_of::<ArrayPattern>() == 8usize);
    assert!(offset_of!(ArrayPattern, parent) == 0usize);
    assert!(offset_of!(ArrayPattern, span) == 16usize);
    assert!(offset_of!(ArrayPattern, elements) == 24usize);
    assert!(offset_of!(ArrayPattern, rest) == 56usize);

    assert!(size_of::<BindingRestElement>() == 56usize);
    assert!(align_of::<BindingRestElement>() == 8usize);
    assert!(offset_of!(BindingRestElement, parent) == 0usize);
    assert!(offset_of!(BindingRestElement, span) == 16usize);
    assert!(offset_of!(BindingRestElement, argument) == 24usize);

    assert!(size_of::<Function>() == 136usize);
    assert!(align_of::<Function>() == 8usize);
    assert!(offset_of!(Function, parent) == 0usize);
    assert!(offset_of!(Function, span) == 16usize);
    assert!(offset_of!(Function, r#type) == 24usize);
    assert!(offset_of!(Function, id) == 32usize);
    assert!(offset_of!(Function, generator) == 80usize);
    assert!(offset_of!(Function, r#async) == 81usize);
    assert!(offset_of!(Function, declare) == 82usize);
    assert!(offset_of!(Function, type_parameters) == 88usize);
    assert!(offset_of!(Function, this_param) == 96usize);
    assert!(offset_of!(Function, params) == 104usize);
    assert!(offset_of!(Function, return_type) == 112usize);
    assert!(offset_of!(Function, body) == 120usize);
    assert!(offset_of!(Function, scope_id) == 128usize);

    assert!(size_of::<FunctionType>() == 1usize);
    assert!(align_of::<FunctionType>() == 1usize);

    assert!(size_of::<FormalParameters>() == 72usize);
    assert!(align_of::<FormalParameters>() == 8usize);
    assert!(offset_of!(FormalParameters, parent) == 0usize);
    assert!(offset_of!(FormalParameters, span) == 16usize);
    assert!(offset_of!(FormalParameters, kind) == 24usize);
    assert!(offset_of!(FormalParameters, items) == 32usize);
    assert!(offset_of!(FormalParameters, rest) == 64usize);

    assert!(size_of::<FormalParameter>() == 96usize);
    assert!(align_of::<FormalParameter>() == 8usize);
    assert!(offset_of!(FormalParameter, parent) == 0usize);
    assert!(offset_of!(FormalParameter, span) == 16usize);
    assert!(offset_of!(FormalParameter, decorators) == 24usize);
    assert!(offset_of!(FormalParameter, pattern) == 56usize);
    assert!(offset_of!(FormalParameter, accessibility) == 88usize);
    assert!(offset_of!(FormalParameter, readonly) == 89usize);
    assert!(offset_of!(FormalParameter, r#override) == 90usize);

    assert!(size_of::<FormalParameterKind>() == 1usize);
    assert!(align_of::<FormalParameterKind>() == 1usize);

    assert!(size_of::<FunctionBody>() == 88usize);
    assert!(align_of::<FunctionBody>() == 8usize);
    assert!(offset_of!(FunctionBody, parent) == 0usize);
    assert!(offset_of!(FunctionBody, span) == 16usize);
    assert!(offset_of!(FunctionBody, directives) == 24usize);
    assert!(offset_of!(FunctionBody, statements) == 56usize);

    assert!(size_of::<ArrowFunctionExpression>() == 72usize);
    assert!(align_of::<ArrowFunctionExpression>() == 8usize);
    assert!(offset_of!(ArrowFunctionExpression, parent) == 0usize);
    assert!(offset_of!(ArrowFunctionExpression, span) == 16usize);
    assert!(offset_of!(ArrowFunctionExpression, expression) == 24usize);
    assert!(offset_of!(ArrowFunctionExpression, r#async) == 25usize);
    assert!(offset_of!(ArrowFunctionExpression, type_parameters) == 32usize);
    assert!(offset_of!(ArrowFunctionExpression, params) == 40usize);
    assert!(offset_of!(ArrowFunctionExpression, return_type) == 48usize);
    assert!(offset_of!(ArrowFunctionExpression, body) == 56usize);
    assert!(offset_of!(ArrowFunctionExpression, scope_id) == 64usize);

    assert!(size_of::<YieldExpression>() == 48usize);
    assert!(align_of::<YieldExpression>() == 8usize);
    assert!(offset_of!(YieldExpression, parent) == 0usize);
    assert!(offset_of!(YieldExpression, span) == 16usize);
    assert!(offset_of!(YieldExpression, delegate) == 24usize);
    assert!(offset_of!(YieldExpression, argument) == 32usize);

    assert!(size_of::<Class>() == 192usize);
    assert!(align_of::<Class>() == 8usize);
    assert!(offset_of!(Class, parent) == 0usize);
    assert!(offset_of!(Class, span) == 16usize);
    assert!(offset_of!(Class, r#type) == 24usize);
    assert!(offset_of!(Class, decorators) == 32usize);
    assert!(offset_of!(Class, id) == 64usize);
    assert!(offset_of!(Class, type_parameters) == 112usize);
    assert!(offset_of!(Class, super_class) == 120usize);
    assert!(offset_of!(Class, super_type_parameters) == 136usize);
    assert!(offset_of!(Class, implements) == 144usize);
    assert!(offset_of!(Class, body) == 176usize);
    assert!(offset_of!(Class, r#abstract) == 184usize);
    assert!(offset_of!(Class, declare) == 185usize);
    assert!(offset_of!(Class, scope_id) == 188usize);

    assert!(size_of::<ClassType>() == 1usize);
    assert!(align_of::<ClassType>() == 1usize);

    assert!(size_of::<ClassBody>() == 56usize);
    assert!(align_of::<ClassBody>() == 8usize);
    assert!(offset_of!(ClassBody, parent) == 0usize);
    assert!(offset_of!(ClassBody, span) == 16usize);
    assert!(offset_of!(ClassBody, body) == 24usize);

    assert!(size_of::<ClassElement>() == 16usize);
    assert!(align_of::<ClassElement>() == 8usize);

    assert!(size_of::<MethodDefinition>() == 96usize);
    assert!(align_of::<MethodDefinition>() == 8usize);
    assert!(offset_of!(MethodDefinition, parent) == 0usize);
    assert!(offset_of!(MethodDefinition, span) == 16usize);
    assert!(offset_of!(MethodDefinition, r#type) == 24usize);
    assert!(offset_of!(MethodDefinition, decorators) == 32usize);
    assert!(offset_of!(MethodDefinition, key) == 64usize);
    assert!(offset_of!(MethodDefinition, value) == 80usize);
    assert!(offset_of!(MethodDefinition, kind) == 88usize);
    assert!(offset_of!(MethodDefinition, computed) == 89usize);
    assert!(offset_of!(MethodDefinition, r#static) == 90usize);
    assert!(offset_of!(MethodDefinition, r#override) == 91usize);
    assert!(offset_of!(MethodDefinition, optional) == 92usize);
    assert!(offset_of!(MethodDefinition, accessibility) == 93usize);

    assert!(size_of::<MethodDefinitionType>() == 1usize);
    assert!(align_of::<MethodDefinitionType>() == 1usize);

    assert!(size_of::<PropertyDefinition>() == 120usize);
    assert!(align_of::<PropertyDefinition>() == 8usize);
    assert!(offset_of!(PropertyDefinition, parent) == 0usize);
    assert!(offset_of!(PropertyDefinition, span) == 16usize);
    assert!(offset_of!(PropertyDefinition, r#type) == 24usize);
    assert!(offset_of!(PropertyDefinition, decorators) == 32usize);
    assert!(offset_of!(PropertyDefinition, key) == 64usize);
    assert!(offset_of!(PropertyDefinition, value) == 80usize);
    assert!(offset_of!(PropertyDefinition, computed) == 96usize);
    assert!(offset_of!(PropertyDefinition, r#static) == 97usize);
    assert!(offset_of!(PropertyDefinition, declare) == 98usize);
    assert!(offset_of!(PropertyDefinition, r#override) == 99usize);
    assert!(offset_of!(PropertyDefinition, optional) == 100usize);
    assert!(offset_of!(PropertyDefinition, definite) == 101usize);
    assert!(offset_of!(PropertyDefinition, readonly) == 102usize);
    assert!(offset_of!(PropertyDefinition, type_annotation) == 104usize);
    assert!(offset_of!(PropertyDefinition, accessibility) == 112usize);

    assert!(size_of::<PropertyDefinitionType>() == 1usize);
    assert!(align_of::<PropertyDefinitionType>() == 1usize);

    assert!(size_of::<MethodDefinitionKind>() == 1usize);
    assert!(align_of::<MethodDefinitionKind>() == 1usize);

    assert!(size_of::<PrivateIdentifier>() == 40usize);
    assert!(align_of::<PrivateIdentifier>() == 8usize);
    assert!(offset_of!(PrivateIdentifier, parent) == 0usize);
    assert!(offset_of!(PrivateIdentifier, span) == 16usize);
    assert!(offset_of!(PrivateIdentifier, name) == 24usize);

    assert!(size_of::<StaticBlock>() == 64usize);
    assert!(align_of::<StaticBlock>() == 8usize);
    assert!(offset_of!(StaticBlock, parent) == 0usize);
    assert!(offset_of!(StaticBlock, span) == 16usize);
    assert!(offset_of!(StaticBlock, body) == 24usize);
    assert!(offset_of!(StaticBlock, scope_id) == 56usize);

    assert!(size_of::<ModuleDeclaration>() == 16usize);
    assert!(align_of::<ModuleDeclaration>() == 8usize);

    assert!(size_of::<AccessorPropertyType>() == 1usize);
    assert!(align_of::<AccessorPropertyType>() == 1usize);

    assert!(size_of::<AccessorProperty>() == 120usize);
    assert!(align_of::<AccessorProperty>() == 8usize);
    assert!(offset_of!(AccessorProperty, parent) == 0usize);
    assert!(offset_of!(AccessorProperty, span) == 16usize);
    assert!(offset_of!(AccessorProperty, r#type) == 24usize);
    assert!(offset_of!(AccessorProperty, decorators) == 32usize);
    assert!(offset_of!(AccessorProperty, key) == 64usize);
    assert!(offset_of!(AccessorProperty, value) == 80usize);
    assert!(offset_of!(AccessorProperty, computed) == 96usize);
    assert!(offset_of!(AccessorProperty, r#static) == 97usize);
    assert!(offset_of!(AccessorProperty, definite) == 98usize);
    assert!(offset_of!(AccessorProperty, type_annotation) == 104usize);
    assert!(offset_of!(AccessorProperty, accessibility) == 112usize);

    assert!(size_of::<ImportExpression>() == 80usize);
    assert!(align_of::<ImportExpression>() == 8usize);
    assert!(offset_of!(ImportExpression, parent) == 0usize);
    assert!(offset_of!(ImportExpression, span) == 16usize);
    assert!(offset_of!(ImportExpression, source) == 24usize);
    assert!(offset_of!(ImportExpression, arguments) == 40usize);
    assert!(offset_of!(ImportExpression, phase) == 72usize);

    assert!(size_of::<ImportDeclaration>() == 136usize);
    assert!(align_of::<ImportDeclaration>() == 8usize);
    assert!(offset_of!(ImportDeclaration, parent) == 0usize);
    assert!(offset_of!(ImportDeclaration, span) == 16usize);
    assert!(offset_of!(ImportDeclaration, specifiers) == 24usize);
    assert!(offset_of!(ImportDeclaration, source) == 56usize);
    assert!(offset_of!(ImportDeclaration, phase) == 112usize);
    assert!(offset_of!(ImportDeclaration, with_clause) == 120usize);
    assert!(offset_of!(ImportDeclaration, import_kind) == 128usize);

    assert!(size_of::<ImportPhase>() == 1usize);
    assert!(align_of::<ImportPhase>() == 1usize);

    assert!(size_of::<ImportDeclarationSpecifier>() == 16usize);
    assert!(align_of::<ImportDeclarationSpecifier>() == 8usize);

    assert!(size_of::<ImportSpecifier>() == 144usize);
    assert!(align_of::<ImportSpecifier>() == 8usize);
    assert!(offset_of!(ImportSpecifier, parent) == 0usize);
    assert!(offset_of!(ImportSpecifier, span) == 16usize);
    assert!(offset_of!(ImportSpecifier, imported) == 24usize);
    assert!(offset_of!(ImportSpecifier, local) == 88usize);
    assert!(offset_of!(ImportSpecifier, import_kind) == 136usize);

    assert!(size_of::<ImportDefaultSpecifier>() == 72usize);
    assert!(align_of::<ImportDefaultSpecifier>() == 8usize);
    assert!(offset_of!(ImportDefaultSpecifier, parent) == 0usize);
    assert!(offset_of!(ImportDefaultSpecifier, span) == 16usize);
    assert!(offset_of!(ImportDefaultSpecifier, local) == 24usize);

    assert!(size_of::<ImportNamespaceSpecifier>() == 72usize);
    assert!(align_of::<ImportNamespaceSpecifier>() == 8usize);
    assert!(offset_of!(ImportNamespaceSpecifier, parent) == 0usize);
    assert!(offset_of!(ImportNamespaceSpecifier, span) == 16usize);
    assert!(offset_of!(ImportNamespaceSpecifier, local) == 24usize);

    assert!(size_of::<WithClause>() == 96usize);
    assert!(align_of::<WithClause>() == 8usize);
    assert!(offset_of!(WithClause, parent) == 0usize);
    assert!(offset_of!(WithClause, span) == 16usize);
    assert!(offset_of!(WithClause, attributes_keyword) == 24usize);
    assert!(offset_of!(WithClause, with_entries) == 64usize);

    assert!(size_of::<ImportAttribute>() == 144usize);
    assert!(align_of::<ImportAttribute>() == 8usize);
    assert!(offset_of!(ImportAttribute, parent) == 0usize);
    assert!(offset_of!(ImportAttribute, span) == 16usize);
    assert!(offset_of!(ImportAttribute, key) == 24usize);
    assert!(offset_of!(ImportAttribute, value) == 88usize);

    assert!(size_of::<ImportAttributeKey>() == 64usize);
    assert!(align_of::<ImportAttributeKey>() == 8usize);

    assert!(size_of::<ExportNamedDeclaration>() == 144usize);
    assert!(align_of::<ExportNamedDeclaration>() == 8usize);
    assert!(offset_of!(ExportNamedDeclaration, parent) == 0usize);
    assert!(offset_of!(ExportNamedDeclaration, span) == 16usize);
    assert!(offset_of!(ExportNamedDeclaration, declaration) == 24usize);
    assert!(offset_of!(ExportNamedDeclaration, specifiers) == 40usize);
    assert!(offset_of!(ExportNamedDeclaration, source) == 72usize);
    assert!(offset_of!(ExportNamedDeclaration, export_kind) == 128usize);
    assert!(offset_of!(ExportNamedDeclaration, with_clause) == 136usize);

    assert!(size_of::<ExportDefaultDeclaration>() == 104usize);
    assert!(align_of::<ExportDefaultDeclaration>() == 8usize);
    assert!(offset_of!(ExportDefaultDeclaration, parent) == 0usize);
    assert!(offset_of!(ExportDefaultDeclaration, span) == 16usize);
    assert!(offset_of!(ExportDefaultDeclaration, declaration) == 24usize);
    assert!(offset_of!(ExportDefaultDeclaration, exported) == 40usize);

    assert!(size_of::<ExportAllDeclaration>() == 160usize);
    assert!(align_of::<ExportAllDeclaration>() == 8usize);
    assert!(offset_of!(ExportAllDeclaration, parent) == 0usize);
    assert!(offset_of!(ExportAllDeclaration, span) == 16usize);
    assert!(offset_of!(ExportAllDeclaration, exported) == 24usize);
    assert!(offset_of!(ExportAllDeclaration, source) == 88usize);
    assert!(offset_of!(ExportAllDeclaration, with_clause) == 144usize);
    assert!(offset_of!(ExportAllDeclaration, export_kind) == 152usize);

    assert!(size_of::<ExportSpecifier>() == 160usize);
    assert!(align_of::<ExportSpecifier>() == 8usize);
    assert!(offset_of!(ExportSpecifier, parent) == 0usize);
    assert!(offset_of!(ExportSpecifier, span) == 16usize);
    assert!(offset_of!(ExportSpecifier, local) == 24usize);
    assert!(offset_of!(ExportSpecifier, exported) == 88usize);
    assert!(offset_of!(ExportSpecifier, export_kind) == 152usize);

    assert!(size_of::<ExportDefaultDeclarationKind>() == 16usize);
    assert!(align_of::<ExportDefaultDeclarationKind>() == 8usize);

    assert!(size_of::<ModuleExportName>() == 64usize);
    assert!(align_of::<ModuleExportName>() == 8usize);

    assert!(size_of::<TSThisParameter>() == 40usize);
    assert!(align_of::<TSThisParameter>() == 8usize);
    assert!(offset_of!(TSThisParameter, parent) == 0usize);
    assert!(offset_of!(TSThisParameter, span) == 16usize);
    assert!(offset_of!(TSThisParameter, this_span) == 24usize);
    assert!(offset_of!(TSThisParameter, type_annotation) == 32usize);

    assert!(size_of::<TSEnumDeclaration>() == 112usize);
    assert!(align_of::<TSEnumDeclaration>() == 8usize);
    assert!(offset_of!(TSEnumDeclaration, parent) == 0usize);
    assert!(offset_of!(TSEnumDeclaration, span) == 16usize);
    assert!(offset_of!(TSEnumDeclaration, id) == 24usize);
    assert!(offset_of!(TSEnumDeclaration, members) == 72usize);
    assert!(offset_of!(TSEnumDeclaration, r#const) == 104usize);
    assert!(offset_of!(TSEnumDeclaration, declare) == 105usize);
    assert!(offset_of!(TSEnumDeclaration, scope_id) == 108usize);

    assert!(size_of::<TSEnumMember>() == 56usize);
    assert!(align_of::<TSEnumMember>() == 8usize);
    assert!(offset_of!(TSEnumMember, parent) == 0usize);
    assert!(offset_of!(TSEnumMember, span) == 16usize);
    assert!(offset_of!(TSEnumMember, id) == 24usize);
    assert!(offset_of!(TSEnumMember, initializer) == 40usize);

    assert!(size_of::<TSEnumMemberName>() == 16usize);
    assert!(align_of::<TSEnumMemberName>() == 8usize);

    assert!(size_of::<TSTypeAnnotation>() == 40usize);
    assert!(align_of::<TSTypeAnnotation>() == 8usize);
    assert!(offset_of!(TSTypeAnnotation, parent) == 0usize);
    assert!(offset_of!(TSTypeAnnotation, span) == 16usize);
    assert!(offset_of!(TSTypeAnnotation, type_annotation) == 24usize);

    assert!(size_of::<TSLiteralType>() == 40usize);
    assert!(align_of::<TSLiteralType>() == 8usize);
    assert!(offset_of!(TSLiteralType, parent) == 0usize);
    assert!(offset_of!(TSLiteralType, span) == 16usize);
    assert!(offset_of!(TSLiteralType, literal) == 24usize);

    assert!(size_of::<TSLiteral>() == 16usize);
    assert!(align_of::<TSLiteral>() == 8usize);

    assert!(size_of::<TSType>() == 16usize);
    assert!(align_of::<TSType>() == 8usize);

    assert!(size_of::<TSConditionalType>() == 96usize);
    assert!(align_of::<TSConditionalType>() == 8usize);
    assert!(offset_of!(TSConditionalType, parent) == 0usize);
    assert!(offset_of!(TSConditionalType, span) == 16usize);
    assert!(offset_of!(TSConditionalType, check_type) == 24usize);
    assert!(offset_of!(TSConditionalType, extends_type) == 40usize);
    assert!(offset_of!(TSConditionalType, true_type) == 56usize);
    assert!(offset_of!(TSConditionalType, false_type) == 72usize);
    assert!(offset_of!(TSConditionalType, scope_id) == 88usize);

    assert!(size_of::<TSUnionType>() == 56usize);
    assert!(align_of::<TSUnionType>() == 8usize);
    assert!(offset_of!(TSUnionType, parent) == 0usize);
    assert!(offset_of!(TSUnionType, span) == 16usize);
    assert!(offset_of!(TSUnionType, types) == 24usize);

    assert!(size_of::<TSIntersectionType>() == 56usize);
    assert!(align_of::<TSIntersectionType>() == 8usize);
    assert!(offset_of!(TSIntersectionType, parent) == 0usize);
    assert!(offset_of!(TSIntersectionType, span) == 16usize);
    assert!(offset_of!(TSIntersectionType, types) == 24usize);

    assert!(size_of::<TSParenthesizedType>() == 40usize);
    assert!(align_of::<TSParenthesizedType>() == 8usize);
    assert!(offset_of!(TSParenthesizedType, parent) == 0usize);
    assert!(offset_of!(TSParenthesizedType, span) == 16usize);
    assert!(offset_of!(TSParenthesizedType, type_annotation) == 24usize);

    assert!(size_of::<TSTypeOperator>() == 48usize);
    assert!(align_of::<TSTypeOperator>() == 8usize);
    assert!(offset_of!(TSTypeOperator, parent) == 0usize);
    assert!(offset_of!(TSTypeOperator, span) == 16usize);
    assert!(offset_of!(TSTypeOperator, operator) == 24usize);
    assert!(offset_of!(TSTypeOperator, type_annotation) == 32usize);

    assert!(size_of::<TSTypeOperatorOperator>() == 1usize);
    assert!(align_of::<TSTypeOperatorOperator>() == 1usize);

    assert!(size_of::<TSArrayType>() == 40usize);
    assert!(align_of::<TSArrayType>() == 8usize);
    assert!(offset_of!(TSArrayType, parent) == 0usize);
    assert!(offset_of!(TSArrayType, span) == 16usize);
    assert!(offset_of!(TSArrayType, element_type) == 24usize);

    assert!(size_of::<TSIndexedAccessType>() == 56usize);
    assert!(align_of::<TSIndexedAccessType>() == 8usize);
    assert!(offset_of!(TSIndexedAccessType, parent) == 0usize);
    assert!(offset_of!(TSIndexedAccessType, span) == 16usize);
    assert!(offset_of!(TSIndexedAccessType, object_type) == 24usize);
    assert!(offset_of!(TSIndexedAccessType, index_type) == 40usize);

    assert!(size_of::<TSTupleType>() == 56usize);
    assert!(align_of::<TSTupleType>() == 8usize);
    assert!(offset_of!(TSTupleType, parent) == 0usize);
    assert!(offset_of!(TSTupleType, span) == 16usize);
    assert!(offset_of!(TSTupleType, element_types) == 24usize);

    assert!(size_of::<TSNamedTupleMember>() == 88usize);
    assert!(align_of::<TSNamedTupleMember>() == 8usize);
    assert!(offset_of!(TSNamedTupleMember, parent) == 0usize);
    assert!(offset_of!(TSNamedTupleMember, span) == 16usize);
    assert!(offset_of!(TSNamedTupleMember, element_type) == 24usize);
    assert!(offset_of!(TSNamedTupleMember, label) == 40usize);
    assert!(offset_of!(TSNamedTupleMember, optional) == 80usize);

    assert!(size_of::<TSOptionalType>() == 40usize);
    assert!(align_of::<TSOptionalType>() == 8usize);
    assert!(offset_of!(TSOptionalType, parent) == 0usize);
    assert!(offset_of!(TSOptionalType, span) == 16usize);
    assert!(offset_of!(TSOptionalType, type_annotation) == 24usize);

    assert!(size_of::<TSRestType>() == 40usize);
    assert!(align_of::<TSRestType>() == 8usize);
    assert!(offset_of!(TSRestType, parent) == 0usize);
    assert!(offset_of!(TSRestType, span) == 16usize);
    assert!(offset_of!(TSRestType, type_annotation) == 24usize);

    assert!(size_of::<TSTupleElement>() == 16usize);
    assert!(align_of::<TSTupleElement>() == 8usize);

    assert!(size_of::<TSAnyKeyword>() == 24usize);
    assert!(align_of::<TSAnyKeyword>() == 8usize);
    assert!(offset_of!(TSAnyKeyword, parent) == 0usize);
    assert!(offset_of!(TSAnyKeyword, span) == 16usize);

    assert!(size_of::<TSStringKeyword>() == 24usize);
    assert!(align_of::<TSStringKeyword>() == 8usize);
    assert!(offset_of!(TSStringKeyword, parent) == 0usize);
    assert!(offset_of!(TSStringKeyword, span) == 16usize);

    assert!(size_of::<TSBooleanKeyword>() == 24usize);
    assert!(align_of::<TSBooleanKeyword>() == 8usize);
    assert!(offset_of!(TSBooleanKeyword, parent) == 0usize);
    assert!(offset_of!(TSBooleanKeyword, span) == 16usize);

    assert!(size_of::<TSNumberKeyword>() == 24usize);
    assert!(align_of::<TSNumberKeyword>() == 8usize);
    assert!(offset_of!(TSNumberKeyword, parent) == 0usize);
    assert!(offset_of!(TSNumberKeyword, span) == 16usize);

    assert!(size_of::<TSNeverKeyword>() == 24usize);
    assert!(align_of::<TSNeverKeyword>() == 8usize);
    assert!(offset_of!(TSNeverKeyword, parent) == 0usize);
    assert!(offset_of!(TSNeverKeyword, span) == 16usize);

    assert!(size_of::<TSIntrinsicKeyword>() == 24usize);
    assert!(align_of::<TSIntrinsicKeyword>() == 8usize);
    assert!(offset_of!(TSIntrinsicKeyword, parent) == 0usize);
    assert!(offset_of!(TSIntrinsicKeyword, span) == 16usize);

    assert!(size_of::<TSUnknownKeyword>() == 24usize);
    assert!(align_of::<TSUnknownKeyword>() == 8usize);
    assert!(offset_of!(TSUnknownKeyword, parent) == 0usize);
    assert!(offset_of!(TSUnknownKeyword, span) == 16usize);

    assert!(size_of::<TSNullKeyword>() == 24usize);
    assert!(align_of::<TSNullKeyword>() == 8usize);
    assert!(offset_of!(TSNullKeyword, parent) == 0usize);
    assert!(offset_of!(TSNullKeyword, span) == 16usize);

    assert!(size_of::<TSUndefinedKeyword>() == 24usize);
    assert!(align_of::<TSUndefinedKeyword>() == 8usize);
    assert!(offset_of!(TSUndefinedKeyword, parent) == 0usize);
    assert!(offset_of!(TSUndefinedKeyword, span) == 16usize);

    assert!(size_of::<TSVoidKeyword>() == 24usize);
    assert!(align_of::<TSVoidKeyword>() == 8usize);
    assert!(offset_of!(TSVoidKeyword, parent) == 0usize);
    assert!(offset_of!(TSVoidKeyword, span) == 16usize);

    assert!(size_of::<TSSymbolKeyword>() == 24usize);
    assert!(align_of::<TSSymbolKeyword>() == 8usize);
    assert!(offset_of!(TSSymbolKeyword, parent) == 0usize);
    assert!(offset_of!(TSSymbolKeyword, span) == 16usize);

    assert!(size_of::<TSThisType>() == 24usize);
    assert!(align_of::<TSThisType>() == 8usize);
    assert!(offset_of!(TSThisType, parent) == 0usize);
    assert!(offset_of!(TSThisType, span) == 16usize);

    assert!(size_of::<TSObjectKeyword>() == 24usize);
    assert!(align_of::<TSObjectKeyword>() == 8usize);
    assert!(offset_of!(TSObjectKeyword, parent) == 0usize);
    assert!(offset_of!(TSObjectKeyword, span) == 16usize);

    assert!(size_of::<TSBigIntKeyword>() == 24usize);
    assert!(align_of::<TSBigIntKeyword>() == 8usize);
    assert!(offset_of!(TSBigIntKeyword, parent) == 0usize);
    assert!(offset_of!(TSBigIntKeyword, span) == 16usize);

    assert!(size_of::<TSTypeReference>() == 48usize);
    assert!(align_of::<TSTypeReference>() == 8usize);
    assert!(offset_of!(TSTypeReference, parent) == 0usize);
    assert!(offset_of!(TSTypeReference, span) == 16usize);
    assert!(offset_of!(TSTypeReference, type_name) == 24usize);
    assert!(offset_of!(TSTypeReference, type_parameters) == 40usize);

    assert!(size_of::<TSTypeName>() == 16usize);
    assert!(align_of::<TSTypeName>() == 8usize);

    assert!(size_of::<TSQualifiedName>() == 80usize);
    assert!(align_of::<TSQualifiedName>() == 8usize);
    assert!(offset_of!(TSQualifiedName, parent) == 0usize);
    assert!(offset_of!(TSQualifiedName, span) == 16usize);
    assert!(offset_of!(TSQualifiedName, left) == 24usize);
    assert!(offset_of!(TSQualifiedName, right) == 40usize);

    assert!(size_of::<TSTypeParameterInstantiation>() == 56usize);
    assert!(align_of::<TSTypeParameterInstantiation>() == 8usize);
    assert!(offset_of!(TSTypeParameterInstantiation, parent) == 0usize);
    assert!(offset_of!(TSTypeParameterInstantiation, span) == 16usize);
    assert!(offset_of!(TSTypeParameterInstantiation, params) == 24usize);

    assert!(size_of::<TSTypeParameter>() == 112usize);
    assert!(align_of::<TSTypeParameter>() == 8usize);
    assert!(offset_of!(TSTypeParameter, parent) == 0usize);
    assert!(offset_of!(TSTypeParameter, span) == 16usize);
    assert!(offset_of!(TSTypeParameter, name) == 24usize);
    assert!(offset_of!(TSTypeParameter, constraint) == 72usize);
    assert!(offset_of!(TSTypeParameter, default) == 88usize);
    assert!(offset_of!(TSTypeParameter, r#in) == 104usize);
    assert!(offset_of!(TSTypeParameter, out) == 105usize);
    assert!(offset_of!(TSTypeParameter, r#const) == 106usize);

    assert!(size_of::<TSTypeParameterDeclaration>() == 56usize);
    assert!(align_of::<TSTypeParameterDeclaration>() == 8usize);
    assert!(offset_of!(TSTypeParameterDeclaration, parent) == 0usize);
    assert!(offset_of!(TSTypeParameterDeclaration, span) == 16usize);
    assert!(offset_of!(TSTypeParameterDeclaration, params) == 24usize);

    assert!(size_of::<TSTypeAliasDeclaration>() == 104usize);
    assert!(align_of::<TSTypeAliasDeclaration>() == 8usize);
    assert!(offset_of!(TSTypeAliasDeclaration, parent) == 0usize);
    assert!(offset_of!(TSTypeAliasDeclaration, span) == 16usize);
    assert!(offset_of!(TSTypeAliasDeclaration, id) == 24usize);
    assert!(offset_of!(TSTypeAliasDeclaration, type_parameters) == 72usize);
    assert!(offset_of!(TSTypeAliasDeclaration, type_annotation) == 80usize);
    assert!(offset_of!(TSTypeAliasDeclaration, declare) == 96usize);
    assert!(offset_of!(TSTypeAliasDeclaration, scope_id) == 100usize);

    assert!(size_of::<TSAccessibility>() == 1usize);
    assert!(align_of::<TSAccessibility>() == 1usize);

    assert!(size_of::<TSClassImplements>() == 48usize);
    assert!(align_of::<TSClassImplements>() == 8usize);
    assert!(offset_of!(TSClassImplements, parent) == 0usize);
    assert!(offset_of!(TSClassImplements, span) == 16usize);
    assert!(offset_of!(TSClassImplements, expression) == 24usize);
    assert!(offset_of!(TSClassImplements, type_parameters) == 40usize);

    assert!(size_of::<TSInterfaceDeclaration>() == 128usize);
    assert!(align_of::<TSInterfaceDeclaration>() == 8usize);
    assert!(offset_of!(TSInterfaceDeclaration, parent) == 0usize);
    assert!(offset_of!(TSInterfaceDeclaration, span) == 16usize);
    assert!(offset_of!(TSInterfaceDeclaration, id) == 24usize);
    assert!(offset_of!(TSInterfaceDeclaration, extends) == 72usize);
    assert!(offset_of!(TSInterfaceDeclaration, type_parameters) == 104usize);
    assert!(offset_of!(TSInterfaceDeclaration, body) == 112usize);
    assert!(offset_of!(TSInterfaceDeclaration, declare) == 120usize);
    assert!(offset_of!(TSInterfaceDeclaration, scope_id) == 124usize);

    assert!(size_of::<TSInterfaceBody>() == 56usize);
    assert!(align_of::<TSInterfaceBody>() == 8usize);
    assert!(offset_of!(TSInterfaceBody, parent) == 0usize);
    assert!(offset_of!(TSInterfaceBody, span) == 16usize);
    assert!(offset_of!(TSInterfaceBody, body) == 24usize);

    assert!(size_of::<TSPropertySignature>() == 56usize);
    assert!(align_of::<TSPropertySignature>() == 8usize);
    assert!(offset_of!(TSPropertySignature, parent) == 0usize);
    assert!(offset_of!(TSPropertySignature, span) == 16usize);
    assert!(offset_of!(TSPropertySignature, computed) == 24usize);
    assert!(offset_of!(TSPropertySignature, optional) == 25usize);
    assert!(offset_of!(TSPropertySignature, readonly) == 26usize);
    assert!(offset_of!(TSPropertySignature, key) == 32usize);
    assert!(offset_of!(TSPropertySignature, type_annotation) == 48usize);

    assert!(size_of::<TSSignature>() == 16usize);
    assert!(align_of::<TSSignature>() == 8usize);

    assert!(size_of::<TSIndexSignature>() == 72usize);
    assert!(align_of::<TSIndexSignature>() == 8usize);
    assert!(offset_of!(TSIndexSignature, parent) == 0usize);
    assert!(offset_of!(TSIndexSignature, span) == 16usize);
    assert!(offset_of!(TSIndexSignature, parameters) == 24usize);
    assert!(offset_of!(TSIndexSignature, type_annotation) == 56usize);
    assert!(offset_of!(TSIndexSignature, readonly) == 64usize);
    assert!(offset_of!(TSIndexSignature, r#static) == 65usize);

    assert!(size_of::<TSCallSignatureDeclaration>() == 88usize);
    assert!(align_of::<TSCallSignatureDeclaration>() == 8usize);
    assert!(offset_of!(TSCallSignatureDeclaration, parent) == 0usize);
    assert!(offset_of!(TSCallSignatureDeclaration, span) == 16usize);
    assert!(offset_of!(TSCallSignatureDeclaration, type_parameters) == 24usize);
    assert!(offset_of!(TSCallSignatureDeclaration, this_param) == 32usize);
    assert!(offset_of!(TSCallSignatureDeclaration, params) == 72usize);
    assert!(offset_of!(TSCallSignatureDeclaration, return_type) == 80usize);

    assert!(size_of::<TSMethodSignatureKind>() == 1usize);
    assert!(align_of::<TSMethodSignatureKind>() == 1usize);

    assert!(size_of::<TSMethodSignature>() == 88usize);
    assert!(align_of::<TSMethodSignature>() == 8usize);
    assert!(offset_of!(TSMethodSignature, parent) == 0usize);
    assert!(offset_of!(TSMethodSignature, span) == 16usize);
    assert!(offset_of!(TSMethodSignature, key) == 24usize);
    assert!(offset_of!(TSMethodSignature, computed) == 40usize);
    assert!(offset_of!(TSMethodSignature, optional) == 41usize);
    assert!(offset_of!(TSMethodSignature, kind) == 42usize);
    assert!(offset_of!(TSMethodSignature, type_parameters) == 48usize);
    assert!(offset_of!(TSMethodSignature, this_param) == 56usize);
    assert!(offset_of!(TSMethodSignature, params) == 64usize);
    assert!(offset_of!(TSMethodSignature, return_type) == 72usize);
    assert!(offset_of!(TSMethodSignature, scope_id) == 80usize);

    assert!(size_of::<TSConstructSignatureDeclaration>() == 56usize);
    assert!(align_of::<TSConstructSignatureDeclaration>() == 8usize);
    assert!(offset_of!(TSConstructSignatureDeclaration, parent) == 0usize);
    assert!(offset_of!(TSConstructSignatureDeclaration, span) == 16usize);
    assert!(offset_of!(TSConstructSignatureDeclaration, type_parameters) == 24usize);
    assert!(offset_of!(TSConstructSignatureDeclaration, params) == 32usize);
    assert!(offset_of!(TSConstructSignatureDeclaration, return_type) == 40usize);
    assert!(offset_of!(TSConstructSignatureDeclaration, scope_id) == 48usize);

    assert!(size_of::<TSIndexSignatureName>() == 48usize);
    assert!(align_of::<TSIndexSignatureName>() == 8usize);
    assert!(offset_of!(TSIndexSignatureName, parent) == 0usize);
    assert!(offset_of!(TSIndexSignatureName, span) == 16usize);
    assert!(offset_of!(TSIndexSignatureName, name) == 24usize);
    assert!(offset_of!(TSIndexSignatureName, type_annotation) == 40usize);

    assert!(size_of::<TSInterfaceHeritage>() == 48usize);
    assert!(align_of::<TSInterfaceHeritage>() == 8usize);
    assert!(offset_of!(TSInterfaceHeritage, parent) == 0usize);
    assert!(offset_of!(TSInterfaceHeritage, span) == 16usize);
    assert!(offset_of!(TSInterfaceHeritage, expression) == 24usize);
    assert!(offset_of!(TSInterfaceHeritage, type_parameters) == 40usize);

    assert!(size_of::<TSTypePredicate>() == 72usize);
    assert!(align_of::<TSTypePredicate>() == 8usize);
    assert!(offset_of!(TSTypePredicate, parent) == 0usize);
    assert!(offset_of!(TSTypePredicate, span) == 16usize);
    assert!(offset_of!(TSTypePredicate, parameter_name) == 24usize);
    assert!(offset_of!(TSTypePredicate, asserts) == 56usize);
    assert!(offset_of!(TSTypePredicate, type_annotation) == 64usize);

    assert!(size_of::<TSTypePredicateName>() == 32usize);
    assert!(align_of::<TSTypePredicateName>() == 8usize);

    assert!(size_of::<TSModuleDeclaration>() == 112usize);
    assert!(align_of::<TSModuleDeclaration>() == 8usize);
    assert!(offset_of!(TSModuleDeclaration, parent) == 0usize);
    assert!(offset_of!(TSModuleDeclaration, span) == 16usize);
    assert!(offset_of!(TSModuleDeclaration, id) == 24usize);
    assert!(offset_of!(TSModuleDeclaration, body) == 88usize);
    assert!(offset_of!(TSModuleDeclaration, kind) == 104usize);
    assert!(offset_of!(TSModuleDeclaration, declare) == 105usize);
    assert!(offset_of!(TSModuleDeclaration, scope_id) == 108usize);

    assert!(size_of::<TSModuleDeclarationKind>() == 1usize);
    assert!(align_of::<TSModuleDeclarationKind>() == 1usize);

    assert!(size_of::<TSModuleDeclarationName>() == 64usize);
    assert!(align_of::<TSModuleDeclarationName>() == 8usize);

    assert!(size_of::<TSModuleDeclarationBody>() == 16usize);
    assert!(align_of::<TSModuleDeclarationBody>() == 8usize);

    assert!(size_of::<TSModuleBlock>() == 88usize);
    assert!(align_of::<TSModuleBlock>() == 8usize);
    assert!(offset_of!(TSModuleBlock, parent) == 0usize);
    assert!(offset_of!(TSModuleBlock, span) == 16usize);
    assert!(offset_of!(TSModuleBlock, directives) == 24usize);
    assert!(offset_of!(TSModuleBlock, body) == 56usize);

    assert!(size_of::<TSTypeLiteral>() == 56usize);
    assert!(align_of::<TSTypeLiteral>() == 8usize);
    assert!(offset_of!(TSTypeLiteral, parent) == 0usize);
    assert!(offset_of!(TSTypeLiteral, span) == 16usize);
    assert!(offset_of!(TSTypeLiteral, members) == 24usize);

    assert!(size_of::<TSInferType>() == 32usize);
    assert!(align_of::<TSInferType>() == 8usize);
    assert!(offset_of!(TSInferType, parent) == 0usize);
    assert!(offset_of!(TSInferType, span) == 16usize);
    assert!(offset_of!(TSInferType, type_parameter) == 24usize);

    assert!(size_of::<TSTypeQuery>() == 48usize);
    assert!(align_of::<TSTypeQuery>() == 8usize);
    assert!(offset_of!(TSTypeQuery, parent) == 0usize);
    assert!(offset_of!(TSTypeQuery, span) == 16usize);
    assert!(offset_of!(TSTypeQuery, expr_name) == 24usize);
    assert!(offset_of!(TSTypeQuery, type_parameters) == 40usize);

    assert!(size_of::<TSTypeQueryExprName>() == 16usize);
    assert!(align_of::<TSTypeQueryExprName>() == 8usize);

    assert!(size_of::<TSImportType>() == 80usize);
    assert!(align_of::<TSImportType>() == 8usize);
    assert!(offset_of!(TSImportType, parent) == 0usize);
    assert!(offset_of!(TSImportType, span) == 16usize);
    assert!(offset_of!(TSImportType, is_type_of) == 24usize);
    assert!(offset_of!(TSImportType, parameter) == 32usize);
    assert!(offset_of!(TSImportType, qualifier) == 48usize);
    assert!(offset_of!(TSImportType, attributes) == 64usize);
    assert!(offset_of!(TSImportType, type_parameters) == 72usize);

    assert!(size_of::<TSImportAttributes>() == 96usize);
    assert!(align_of::<TSImportAttributes>() == 8usize);
    assert!(offset_of!(TSImportAttributes, parent) == 0usize);
    assert!(offset_of!(TSImportAttributes, span) == 16usize);
    assert!(offset_of!(TSImportAttributes, attributes_keyword) == 24usize);
    assert!(offset_of!(TSImportAttributes, elements) == 64usize);

    assert!(size_of::<TSImportAttribute>() == 104usize);
    assert!(align_of::<TSImportAttribute>() == 8usize);
    assert!(offset_of!(TSImportAttribute, parent) == 0usize);
    assert!(offset_of!(TSImportAttribute, span) == 16usize);
    assert!(offset_of!(TSImportAttribute, name) == 24usize);
    assert!(offset_of!(TSImportAttribute, value) == 88usize);

    assert!(size_of::<TSImportAttributeName>() == 64usize);
    assert!(align_of::<TSImportAttributeName>() == 8usize);

    assert!(size_of::<TSFunctionType>() == 56usize);
    assert!(align_of::<TSFunctionType>() == 8usize);
    assert!(offset_of!(TSFunctionType, parent) == 0usize);
    assert!(offset_of!(TSFunctionType, span) == 16usize);
    assert!(offset_of!(TSFunctionType, type_parameters) == 24usize);
    assert!(offset_of!(TSFunctionType, this_param) == 32usize);
    assert!(offset_of!(TSFunctionType, params) == 40usize);
    assert!(offset_of!(TSFunctionType, return_type) == 48usize);

    assert!(size_of::<TSConstructorType>() == 56usize);
    assert!(align_of::<TSConstructorType>() == 8usize);
    assert!(offset_of!(TSConstructorType, parent) == 0usize);
    assert!(offset_of!(TSConstructorType, span) == 16usize);
    assert!(offset_of!(TSConstructorType, r#abstract) == 24usize);
    assert!(offset_of!(TSConstructorType, type_parameters) == 32usize);
    assert!(offset_of!(TSConstructorType, params) == 40usize);
    assert!(offset_of!(TSConstructorType, return_type) == 48usize);

    assert!(size_of::<TSMappedType>() == 72usize);
    assert!(align_of::<TSMappedType>() == 8usize);
    assert!(offset_of!(TSMappedType, parent) == 0usize);
    assert!(offset_of!(TSMappedType, span) == 16usize);
    assert!(offset_of!(TSMappedType, type_parameter) == 24usize);
    assert!(offset_of!(TSMappedType, name_type) == 32usize);
    assert!(offset_of!(TSMappedType, type_annotation) == 48usize);
    assert!(offset_of!(TSMappedType, optional) == 64usize);
    assert!(offset_of!(TSMappedType, readonly) == 65usize);
    assert!(offset_of!(TSMappedType, scope_id) == 68usize);

    assert!(size_of::<TSMappedTypeModifierOperator>() == 1usize);
    assert!(align_of::<TSMappedTypeModifierOperator>() == 1usize);

    assert!(size_of::<TSTemplateLiteralType>() == 88usize);
    assert!(align_of::<TSTemplateLiteralType>() == 8usize);
    assert!(offset_of!(TSTemplateLiteralType, parent) == 0usize);
    assert!(offset_of!(TSTemplateLiteralType, span) == 16usize);
    assert!(offset_of!(TSTemplateLiteralType, quasis) == 24usize);
    assert!(offset_of!(TSTemplateLiteralType, types) == 56usize);

    assert!(size_of::<TSAsExpression>() == 56usize);
    assert!(align_of::<TSAsExpression>() == 8usize);
    assert!(offset_of!(TSAsExpression, parent) == 0usize);
    assert!(offset_of!(TSAsExpression, span) == 16usize);
    assert!(offset_of!(TSAsExpression, expression) == 24usize);
    assert!(offset_of!(TSAsExpression, type_annotation) == 40usize);

    assert!(size_of::<TSSatisfiesExpression>() == 56usize);
    assert!(align_of::<TSSatisfiesExpression>() == 8usize);
    assert!(offset_of!(TSSatisfiesExpression, parent) == 0usize);
    assert!(offset_of!(TSSatisfiesExpression, span) == 16usize);
    assert!(offset_of!(TSSatisfiesExpression, expression) == 24usize);
    assert!(offset_of!(TSSatisfiesExpression, type_annotation) == 40usize);

    assert!(size_of::<TSTypeAssertion>() == 56usize);
    assert!(align_of::<TSTypeAssertion>() == 8usize);
    assert!(offset_of!(TSTypeAssertion, parent) == 0usize);
    assert!(offset_of!(TSTypeAssertion, span) == 16usize);
    assert!(offset_of!(TSTypeAssertion, expression) == 24usize);
    assert!(offset_of!(TSTypeAssertion, type_annotation) == 40usize);

    assert!(size_of::<TSImportEqualsDeclaration>() == 96usize);
    assert!(align_of::<TSImportEqualsDeclaration>() == 8usize);
    assert!(offset_of!(TSImportEqualsDeclaration, parent) == 0usize);
    assert!(offset_of!(TSImportEqualsDeclaration, span) == 16usize);
    assert!(offset_of!(TSImportEqualsDeclaration, id) == 24usize);
    assert!(offset_of!(TSImportEqualsDeclaration, module_reference) == 72usize);
    assert!(offset_of!(TSImportEqualsDeclaration, import_kind) == 88usize);

    assert!(size_of::<TSModuleReference>() == 16usize);
    assert!(align_of::<TSModuleReference>() == 8usize);

    assert!(size_of::<TSExternalModuleReference>() == 80usize);
    assert!(align_of::<TSExternalModuleReference>() == 8usize);
    assert!(offset_of!(TSExternalModuleReference, parent) == 0usize);
    assert!(offset_of!(TSExternalModuleReference, span) == 16usize);
    assert!(offset_of!(TSExternalModuleReference, expression) == 24usize);

    assert!(size_of::<TSNonNullExpression>() == 40usize);
    assert!(align_of::<TSNonNullExpression>() == 8usize);
    assert!(offset_of!(TSNonNullExpression, parent) == 0usize);
    assert!(offset_of!(TSNonNullExpression, span) == 16usize);
    assert!(offset_of!(TSNonNullExpression, expression) == 24usize);

    assert!(size_of::<Decorator>() == 40usize);
    assert!(align_of::<Decorator>() == 8usize);
    assert!(offset_of!(Decorator, parent) == 0usize);
    assert!(offset_of!(Decorator, span) == 16usize);
    assert!(offset_of!(Decorator, expression) == 24usize);

    assert!(size_of::<TSExportAssignment>() == 40usize);
    assert!(align_of::<TSExportAssignment>() == 8usize);
    assert!(offset_of!(TSExportAssignment, parent) == 0usize);
    assert!(offset_of!(TSExportAssignment, span) == 16usize);
    assert!(offset_of!(TSExportAssignment, expression) == 24usize);

    assert!(size_of::<TSNamespaceExportDeclaration>() == 64usize);
    assert!(align_of::<TSNamespaceExportDeclaration>() == 8usize);
    assert!(offset_of!(TSNamespaceExportDeclaration, parent) == 0usize);
    assert!(offset_of!(TSNamespaceExportDeclaration, span) == 16usize);
    assert!(offset_of!(TSNamespaceExportDeclaration, id) == 24usize);

    assert!(size_of::<TSInstantiationExpression>() == 48usize);
    assert!(align_of::<TSInstantiationExpression>() == 8usize);
    assert!(offset_of!(TSInstantiationExpression, parent) == 0usize);
    assert!(offset_of!(TSInstantiationExpression, span) == 16usize);
    assert!(offset_of!(TSInstantiationExpression, expression) == 24usize);
    assert!(offset_of!(TSInstantiationExpression, type_parameters) == 40usize);

    assert!(size_of::<ImportOrExportKind>() == 1usize);
    assert!(align_of::<ImportOrExportKind>() == 1usize);

    assert!(size_of::<JSDocNullableType>() == 48usize);
    assert!(align_of::<JSDocNullableType>() == 8usize);
    assert!(offset_of!(JSDocNullableType, parent) == 0usize);
    assert!(offset_of!(JSDocNullableType, span) == 16usize);
    assert!(offset_of!(JSDocNullableType, type_annotation) == 24usize);
    assert!(offset_of!(JSDocNullableType, postfix) == 40usize);

    assert!(size_of::<JSDocNonNullableType>() == 48usize);
    assert!(align_of::<JSDocNonNullableType>() == 8usize);
    assert!(offset_of!(JSDocNonNullableType, parent) == 0usize);
    assert!(offset_of!(JSDocNonNullableType, span) == 16usize);
    assert!(offset_of!(JSDocNonNullableType, type_annotation) == 24usize);
    assert!(offset_of!(JSDocNonNullableType, postfix) == 40usize);

    assert!(size_of::<JSDocUnknownType>() == 24usize);
    assert!(align_of::<JSDocUnknownType>() == 8usize);
    assert!(offset_of!(JSDocUnknownType, parent) == 0usize);
    assert!(offset_of!(JSDocUnknownType, span) == 16usize);

    assert!(size_of::<JSXElement>() == 72usize);
    assert!(align_of::<JSXElement>() == 8usize);
    assert!(offset_of!(JSXElement, parent) == 0usize);
    assert!(offset_of!(JSXElement, span) == 16usize);
    assert!(offset_of!(JSXElement, opening_element) == 24usize);
    assert!(offset_of!(JSXElement, closing_element) == 32usize);
    assert!(offset_of!(JSXElement, children) == 40usize);

    assert!(size_of::<JSXOpeningElement>() == 88usize);
    assert!(align_of::<JSXOpeningElement>() == 8usize);
    assert!(offset_of!(JSXOpeningElement, parent) == 0usize);
    assert!(offset_of!(JSXOpeningElement, span) == 16usize);
    assert!(offset_of!(JSXOpeningElement, self_closing) == 24usize);
    assert!(offset_of!(JSXOpeningElement, name) == 32usize);
    assert!(offset_of!(JSXOpeningElement, attributes) == 48usize);
    assert!(offset_of!(JSXOpeningElement, type_parameters) == 80usize);

    assert!(size_of::<JSXClosingElement>() == 40usize);
    assert!(align_of::<JSXClosingElement>() == 8usize);
    assert!(offset_of!(JSXClosingElement, parent) == 0usize);
    assert!(offset_of!(JSXClosingElement, span) == 16usize);
    assert!(offset_of!(JSXClosingElement, name) == 24usize);

    assert!(size_of::<JSXFragment>() == 104usize);
    assert!(align_of::<JSXFragment>() == 8usize);
    assert!(offset_of!(JSXFragment, parent) == 0usize);
    assert!(offset_of!(JSXFragment, span) == 16usize);
    assert!(offset_of!(JSXFragment, opening_fragment) == 24usize);
    assert!(offset_of!(JSXFragment, closing_fragment) == 48usize);
    assert!(offset_of!(JSXFragment, children) == 72usize);

    assert!(size_of::<JSXOpeningFragment>() == 24usize);
    assert!(align_of::<JSXOpeningFragment>() == 8usize);
    assert!(offset_of!(JSXOpeningFragment, parent) == 0usize);
    assert!(offset_of!(JSXOpeningFragment, span) == 16usize);

    assert!(size_of::<JSXClosingFragment>() == 24usize);
    assert!(align_of::<JSXClosingFragment>() == 8usize);
    assert!(offset_of!(JSXClosingFragment, parent) == 0usize);
    assert!(offset_of!(JSXClosingFragment, span) == 16usize);

    assert!(size_of::<JSXElementName>() == 16usize);
    assert!(align_of::<JSXElementName>() == 8usize);

    assert!(size_of::<JSXNamespacedName>() == 104usize);
    assert!(align_of::<JSXNamespacedName>() == 8usize);
    assert!(offset_of!(JSXNamespacedName, parent) == 0usize);
    assert!(offset_of!(JSXNamespacedName, span) == 16usize);
    assert!(offset_of!(JSXNamespacedName, namespace) == 24usize);
    assert!(offset_of!(JSXNamespacedName, property) == 64usize);

    assert!(size_of::<JSXMemberExpression>() == 80usize);
    assert!(align_of::<JSXMemberExpression>() == 8usize);
    assert!(offset_of!(JSXMemberExpression, parent) == 0usize);
    assert!(offset_of!(JSXMemberExpression, span) == 16usize);
    assert!(offset_of!(JSXMemberExpression, object) == 24usize);
    assert!(offset_of!(JSXMemberExpression, property) == 40usize);

    assert!(size_of::<JSXMemberExpressionObject>() == 16usize);
    assert!(align_of::<JSXMemberExpressionObject>() == 8usize);

    assert!(size_of::<JSXExpressionContainer>() == 56usize);
    assert!(align_of::<JSXExpressionContainer>() == 8usize);
    assert!(offset_of!(JSXExpressionContainer, parent) == 0usize);
    assert!(offset_of!(JSXExpressionContainer, span) == 16usize);
    assert!(offset_of!(JSXExpressionContainer, expression) == 24usize);

    assert!(size_of::<JSXExpression>() == 32usize);
    assert!(align_of::<JSXExpression>() == 8usize);

    assert!(size_of::<JSXEmptyExpression>() == 24usize);
    assert!(align_of::<JSXEmptyExpression>() == 8usize);
    assert!(offset_of!(JSXEmptyExpression, parent) == 0usize);
    assert!(offset_of!(JSXEmptyExpression, span) == 16usize);

    assert!(size_of::<JSXAttributeItem>() == 16usize);
    assert!(align_of::<JSXAttributeItem>() == 8usize);

    assert!(size_of::<JSXAttribute>() == 56usize);
    assert!(align_of::<JSXAttribute>() == 8usize);
    assert!(offset_of!(JSXAttribute, parent) == 0usize);
    assert!(offset_of!(JSXAttribute, span) == 16usize);
    assert!(offset_of!(JSXAttribute, name) == 24usize);
    assert!(offset_of!(JSXAttribute, value) == 40usize);

    assert!(size_of::<JSXSpreadAttribute>() == 40usize);
    assert!(align_of::<JSXSpreadAttribute>() == 8usize);
    assert!(offset_of!(JSXSpreadAttribute, parent) == 0usize);
    assert!(offset_of!(JSXSpreadAttribute, span) == 16usize);
    assert!(offset_of!(JSXSpreadAttribute, argument) == 24usize);

    assert!(size_of::<JSXAttributeName>() == 16usize);
    assert!(align_of::<JSXAttributeName>() == 8usize);

    assert!(size_of::<JSXAttributeValue>() == 16usize);
    assert!(align_of::<JSXAttributeValue>() == 8usize);

    assert!(size_of::<JSXIdentifier>() == 40usize);
    assert!(align_of::<JSXIdentifier>() == 8usize);
    assert!(offset_of!(JSXIdentifier, parent) == 0usize);
    assert!(offset_of!(JSXIdentifier, span) == 16usize);
    assert!(offset_of!(JSXIdentifier, name) == 24usize);

    assert!(size_of::<JSXChild>() == 16usize);
    assert!(align_of::<JSXChild>() == 8usize);

    assert!(size_of::<JSXSpreadChild>() == 40usize);
    assert!(align_of::<JSXSpreadChild>() == 8usize);
    assert!(offset_of!(JSXSpreadChild, parent) == 0usize);
    assert!(offset_of!(JSXSpreadChild, span) == 16usize);
    assert!(offset_of!(JSXSpreadChild, expression) == 24usize);

    assert!(size_of::<JSXText>() == 40usize);
    assert!(align_of::<JSXText>() == 8usize);
    assert!(offset_of!(JSXText, parent) == 0usize);
    assert!(offset_of!(JSXText, span) == 16usize);
    assert!(offset_of!(JSXText, value) == 24usize);

    assert!(size_of::<CommentKind>() == 1usize);
    assert!(align_of::<CommentKind>() == 1usize);

    assert!(size_of::<CommentPosition>() == 1usize);
    assert!(align_of::<CommentPosition>() == 1usize);

    assert!(size_of::<Comment>() == 16usize);
    assert!(align_of::<Comment>() == 8usize);
    assert!(offset_of!(Comment, span) == 0usize);
    assert!(offset_of!(Comment, attached_to) == 8usize);
    assert!(offset_of!(Comment, kind) == 12usize);
    assert!(offset_of!(Comment, position) == 13usize);
    assert!(offset_of!(Comment, preceded_by_newline) == 14usize);
    assert!(offset_of!(Comment, followed_by_newline) == 15usize);

    assert!(size_of::<NumberBase>() == 1usize);
    assert!(align_of::<NumberBase>() == 1usize);

    assert!(size_of::<BigintBase>() == 1usize);
    assert!(align_of::<BigintBase>() == 1usize);

    assert!(size_of::<AssignmentOperator>() == 1usize);
    assert!(align_of::<AssignmentOperator>() == 1usize);

    assert!(size_of::<BinaryOperator>() == 1usize);
    assert!(align_of::<BinaryOperator>() == 1usize);

    assert!(size_of::<LogicalOperator>() == 1usize);
    assert!(align_of::<LogicalOperator>() == 1usize);

    assert!(size_of::<UnaryOperator>() == 1usize);
    assert!(align_of::<UnaryOperator>() == 1usize);

    assert!(size_of::<UpdateOperator>() == 1usize);
    assert!(align_of::<UpdateOperator>() == 1usize);

    assert!(size_of::<Span>() == 8usize);
    assert!(align_of::<Span>() == 8usize);
    assert!(offset_of!(Span, start) == 0usize);
    assert!(offset_of!(Span, end) == 4usize);

    assert!(size_of::<SourceType>() == 3usize);
    assert!(align_of::<SourceType>() == 1usize);

    assert!(size_of::<Language>() == 1usize);
    assert!(align_of::<Language>() == 1usize);

    assert!(size_of::<ModuleKind>() == 1usize);
    assert!(align_of::<ModuleKind>() == 1usize);

    assert!(size_of::<LanguageVariant>() == 1usize);
    assert!(align_of::<LanguageVariant>() == 1usize);

    assert!(size_of::<Pattern>() == 48usize);
    assert!(align_of::<Pattern>() == 8usize);
    assert!(offset_of!(Pattern, span) == 0usize);
    assert!(offset_of!(Pattern, body) == 8usize);

    assert!(size_of::<Disjunction>() == 40usize);
    assert!(align_of::<Disjunction>() == 8usize);
    assert!(offset_of!(Disjunction, span) == 0usize);
    assert!(offset_of!(Disjunction, body) == 8usize);

    assert!(size_of::<Alternative>() == 40usize);
    assert!(align_of::<Alternative>() == 8usize);
    assert!(offset_of!(Alternative, span) == 0usize);
    assert!(offset_of!(Alternative, body) == 8usize);

    assert!(size_of::<Term>() == 16usize);
    assert!(align_of::<Term>() == 8usize);

    assert!(size_of::<BoundaryAssertion>() == 16usize);
    assert!(align_of::<BoundaryAssertion>() == 8usize);
    assert!(offset_of!(BoundaryAssertion, span) == 0usize);
    assert!(offset_of!(BoundaryAssertion, kind) == 8usize);

    assert!(size_of::<BoundaryAssertionKind>() == 1usize);
    assert!(align_of::<BoundaryAssertionKind>() == 1usize);

    assert!(size_of::<LookAroundAssertion>() == 56usize);
    assert!(align_of::<LookAroundAssertion>() == 8usize);
    assert!(offset_of!(LookAroundAssertion, span) == 0usize);
    assert!(offset_of!(LookAroundAssertion, kind) == 8usize);
    assert!(offset_of!(LookAroundAssertion, body) == 16usize);

    assert!(size_of::<LookAroundAssertionKind>() == 1usize);
    assert!(align_of::<LookAroundAssertionKind>() == 1usize);

    assert!(size_of::<Quantifier>() == 56usize);
    assert!(align_of::<Quantifier>() == 8usize);
    assert!(offset_of!(Quantifier, span) == 0usize);
    assert!(offset_of!(Quantifier, min) == 8usize);
    assert!(offset_of!(Quantifier, max) == 16usize);
    assert!(offset_of!(Quantifier, greedy) == 32usize);
    assert!(offset_of!(Quantifier, body) == 40usize);

    assert!(size_of::<Character>() == 16usize);
    assert!(align_of::<Character>() == 8usize);
    assert!(offset_of!(Character, span) == 0usize);
    assert!(offset_of!(Character, kind) == 8usize);
    assert!(offset_of!(Character, value) == 12usize);

    assert!(size_of::<CharacterKind>() == 1usize);
    assert!(align_of::<CharacterKind>() == 1usize);

    assert!(size_of::<CharacterClassEscape>() == 16usize);
    assert!(align_of::<CharacterClassEscape>() == 8usize);
    assert!(offset_of!(CharacterClassEscape, span) == 0usize);
    assert!(offset_of!(CharacterClassEscape, kind) == 8usize);

    assert!(size_of::<CharacterClassEscapeKind>() == 1usize);
    assert!(align_of::<CharacterClassEscapeKind>() == 1usize);

    assert!(size_of::<UnicodePropertyEscape>() == 48usize);
    assert!(align_of::<UnicodePropertyEscape>() == 8usize);
    assert!(offset_of!(UnicodePropertyEscape, span) == 0usize);
    assert!(offset_of!(UnicodePropertyEscape, negative) == 8usize);
    assert!(offset_of!(UnicodePropertyEscape, strings) == 9usize);
    assert!(offset_of!(UnicodePropertyEscape, name) == 16usize);
    assert!(offset_of!(UnicodePropertyEscape, value) == 32usize);

    assert!(size_of::<Dot>() == 8usize);
    assert!(align_of::<Dot>() == 8usize);
    assert!(offset_of!(Dot, span) == 0usize);

    assert!(size_of::<CharacterClass>() == 48usize);
    assert!(align_of::<CharacterClass>() == 8usize);
    assert!(offset_of!(CharacterClass, span) == 0usize);
    assert!(offset_of!(CharacterClass, negative) == 8usize);
    assert!(offset_of!(CharacterClass, strings) == 9usize);
    assert!(offset_of!(CharacterClass, kind) == 10usize);
    assert!(offset_of!(CharacterClass, body) == 16usize);

    assert!(size_of::<CharacterClassContentsKind>() == 1usize);
    assert!(align_of::<CharacterClassContentsKind>() == 1usize);

    assert!(size_of::<CharacterClassContents>() == 16usize);
    assert!(align_of::<CharacterClassContents>() == 8usize);

    assert!(size_of::<CharacterClassRange>() == 40usize);
    assert!(align_of::<CharacterClassRange>() == 8usize);
    assert!(offset_of!(CharacterClassRange, span) == 0usize);
    assert!(offset_of!(CharacterClassRange, min) == 8usize);
    assert!(offset_of!(CharacterClassRange, max) == 24usize);

    assert!(size_of::<ClassStringDisjunction>() == 48usize);
    assert!(align_of::<ClassStringDisjunction>() == 8usize);
    assert!(offset_of!(ClassStringDisjunction, span) == 0usize);
    assert!(offset_of!(ClassStringDisjunction, strings) == 8usize);
    assert!(offset_of!(ClassStringDisjunction, body) == 16usize);

    assert!(size_of::<ClassString>() == 48usize);
    assert!(align_of::<ClassString>() == 8usize);
    assert!(offset_of!(ClassString, span) == 0usize);
    assert!(offset_of!(ClassString, strings) == 8usize);
    assert!(offset_of!(ClassString, body) == 16usize);

    assert!(size_of::<CapturingGroup>() == 64usize);
    assert!(align_of::<CapturingGroup>() == 8usize);
    assert!(offset_of!(CapturingGroup, span) == 0usize);
    assert!(offset_of!(CapturingGroup, name) == 8usize);
    assert!(offset_of!(CapturingGroup, body) == 24usize);

    assert!(size_of::<IgnoreGroup>() == 64usize);
    assert!(align_of::<IgnoreGroup>() == 8usize);
    assert!(offset_of!(IgnoreGroup, span) == 0usize);
    assert!(offset_of!(IgnoreGroup, modifiers) == 8usize);
    assert!(offset_of!(IgnoreGroup, body) == 24usize);

    assert!(size_of::<Modifiers>() == 16usize);
    assert!(align_of::<Modifiers>() == 8usize);
    assert!(offset_of!(Modifiers, span) == 0usize);
    assert!(offset_of!(Modifiers, enabling) == 8usize);
    assert!(offset_of!(Modifiers, disabling) == 11usize);

    assert!(size_of::<Modifier>() == 3usize);
    assert!(align_of::<Modifier>() == 1usize);
    assert!(offset_of!(Modifier, ignore_case) == 0usize);
    assert!(offset_of!(Modifier, multiline) == 1usize);
    assert!(offset_of!(Modifier, sticky) == 2usize);

    assert!(size_of::<IndexedReference>() == 16usize);
    assert!(align_of::<IndexedReference>() == 8usize);
    assert!(offset_of!(IndexedReference, span) == 0usize);
    assert!(offset_of!(IndexedReference, index) == 8usize);

    assert!(size_of::<NamedReference>() == 24usize);
    assert!(align_of::<NamedReference>() == 8usize);
    assert!(offset_of!(NamedReference, span) == 0usize);
    assert!(offset_of!(NamedReference, name) == 8usize);
};

#[cfg(target_pointer_width = "32")]
const _: () = {
    assert!(size_of::<BooleanLiteral>() == 32usize);
    assert!(align_of::<BooleanLiteral>() == 8usize);
    assert!(offset_of!(BooleanLiteral, parent) == 0usize);
    assert!(offset_of!(BooleanLiteral, span) == 16usize);
    assert!(offset_of!(BooleanLiteral, value) == 24usize);

    assert!(size_of::<NullLiteral>() == 24usize);
    assert!(align_of::<NullLiteral>() == 8usize);
    assert!(offset_of!(NullLiteral, parent) == 0usize);
    assert!(offset_of!(NullLiteral, span) == 16usize);

    assert!(size_of::<NumericLiteral>() == 48usize);
    assert!(align_of::<NumericLiteral>() == 8usize);
    assert!(offset_of!(NumericLiteral, parent) == 0usize);
    assert!(offset_of!(NumericLiteral, span) == 16usize);
    assert!(offset_of!(NumericLiteral, value) == 24usize);
    assert!(offset_of!(NumericLiteral, raw) == 32usize);
    assert!(offset_of!(NumericLiteral, base) == 40usize);

    assert!(size_of::<StringLiteral>() == 40usize);
    assert!(align_of::<StringLiteral>() == 8usize);
    assert!(offset_of!(StringLiteral, parent) == 0usize);
    assert!(offset_of!(StringLiteral, span) == 16usize);
    assert!(offset_of!(StringLiteral, value) == 24usize);
    assert!(offset_of!(StringLiteral, raw) == 32usize);

    assert!(size_of::<BigIntLiteral>() == 40usize);
    assert!(align_of::<BigIntLiteral>() == 8usize);
    assert!(offset_of!(BigIntLiteral, parent) == 0usize);
    assert!(offset_of!(BigIntLiteral, span) == 16usize);
    assert!(offset_of!(BigIntLiteral, raw) == 24usize);
    assert!(offset_of!(BigIntLiteral, base) == 32usize);

    assert!(size_of::<RegExpLiteral>() == 48usize);
    assert!(align_of::<RegExpLiteral>() == 8usize);
    assert!(offset_of!(RegExpLiteral, parent) == 0usize);
    assert!(offset_of!(RegExpLiteral, span) == 16usize);
    assert!(offset_of!(RegExpLiteral, regex) == 24usize);
    assert!(offset_of!(RegExpLiteral, raw) == 40usize);

    assert!(size_of::<RegExp>() == 16usize);
    assert!(align_of::<RegExp>() == 4usize);
    assert!(offset_of!(RegExp, pattern) == 0usize);
    assert!(offset_of!(RegExp, flags) == 12usize);

    assert!(size_of::<RegExpPattern>() == 12usize);
    assert!(align_of::<RegExpPattern>() == 4usize);

    assert!(size_of::<Program>() == 112usize);
    assert!(align_of::<Program>() == 8usize);
    assert!(offset_of!(Program, span) == 0usize);
    assert!(offset_of!(Program, source_type) == 8usize);
    assert!(offset_of!(Program, source_text) == 12usize);
    assert!(offset_of!(Program, comments) == 20usize);
    assert!(offset_of!(Program, hashbang) == 40usize);
    assert!(offset_of!(Program, directives) == 72usize);
    assert!(offset_of!(Program, body) == 88usize);
    assert!(offset_of!(Program, scope_id) == 104usize);
    assert!(offset_of!(Program, id) == 108usize);

    assert!(size_of::<Expression>() == 8usize);
    assert!(align_of::<Expression>() == 4usize);

    assert!(size_of::<IdentifierName>() == 32usize);
    assert!(align_of::<IdentifierName>() == 8usize);
    assert!(offset_of!(IdentifierName, parent) == 0usize);
    assert!(offset_of!(IdentifierName, span) == 16usize);
    assert!(offset_of!(IdentifierName, name) == 24usize);

    assert!(size_of::<IdentifierReference>() == 40usize);
    assert!(align_of::<IdentifierReference>() == 8usize);
    assert!(offset_of!(IdentifierReference, parent) == 0usize);
    assert!(offset_of!(IdentifierReference, span) == 16usize);
    assert!(offset_of!(IdentifierReference, name) == 24usize);
    assert!(offset_of!(IdentifierReference, reference_id) == 32usize);

    assert!(size_of::<BindingIdentifier>() == 40usize);
    assert!(align_of::<BindingIdentifier>() == 8usize);
    assert!(offset_of!(BindingIdentifier, parent) == 0usize);
    assert!(offset_of!(BindingIdentifier, span) == 16usize);
    assert!(offset_of!(BindingIdentifier, name) == 24usize);
    assert!(offset_of!(BindingIdentifier, symbol_id) == 32usize);

    assert!(size_of::<LabelIdentifier>() == 32usize);
    assert!(align_of::<LabelIdentifier>() == 8usize);
    assert!(offset_of!(LabelIdentifier, parent) == 0usize);
    assert!(offset_of!(LabelIdentifier, span) == 16usize);
    assert!(offset_of!(LabelIdentifier, name) == 24usize);

    assert!(size_of::<ThisExpression>() == 24usize);
    assert!(align_of::<ThisExpression>() == 8usize);
    assert!(offset_of!(ThisExpression, parent) == 0usize);
    assert!(offset_of!(ThisExpression, span) == 16usize);

    assert!(size_of::<ArrayExpression>() == 56usize);
    assert!(align_of::<ArrayExpression>() == 8usize);
    assert!(offset_of!(ArrayExpression, parent) == 0usize);
    assert!(offset_of!(ArrayExpression, span) == 16usize);
    assert!(offset_of!(ArrayExpression, elements) == 24usize);
    assert!(offset_of!(ArrayExpression, trailing_comma) == 40usize);

    assert!(size_of::<ArrayExpressionElement>() == 32usize);
    assert!(align_of::<ArrayExpressionElement>() == 8usize);

    assert!(size_of::<Elision>() == 24usize);
    assert!(align_of::<Elision>() == 8usize);
    assert!(offset_of!(Elision, parent) == 0usize);
    assert!(offset_of!(Elision, span) == 16usize);

    assert!(size_of::<ObjectExpression>() == 56usize);
    assert!(align_of::<ObjectExpression>() == 8usize);
    assert!(offset_of!(ObjectExpression, parent) == 0usize);
    assert!(offset_of!(ObjectExpression, span) == 16usize);
    assert!(offset_of!(ObjectExpression, properties) == 24usize);
    assert!(offset_of!(ObjectExpression, trailing_comma) == 40usize);

    assert!(size_of::<ObjectPropertyKind>() == 8usize);
    assert!(align_of::<ObjectPropertyKind>() == 4usize);

    assert!(size_of::<ObjectProperty>() == 48usize);
    assert!(align_of::<ObjectProperty>() == 8usize);
    assert!(offset_of!(ObjectProperty, parent) == 0usize);
    assert!(offset_of!(ObjectProperty, span) == 16usize);
    assert!(offset_of!(ObjectProperty, kind) == 24usize);
    assert!(offset_of!(ObjectProperty, key) == 28usize);
    assert!(offset_of!(ObjectProperty, value) == 36usize);
    assert!(offset_of!(ObjectProperty, method) == 44usize);
    assert!(offset_of!(ObjectProperty, shorthand) == 45usize);
    assert!(offset_of!(ObjectProperty, computed) == 46usize);

    assert!(size_of::<PropertyKey>() == 8usize);
    assert!(align_of::<PropertyKey>() == 4usize);

    assert!(size_of::<PropertyKind>() == 1usize);
    assert!(align_of::<PropertyKind>() == 1usize);

    assert!(size_of::<TemplateLiteral>() == 56usize);
    assert!(align_of::<TemplateLiteral>() == 8usize);
    assert!(offset_of!(TemplateLiteral, parent) == 0usize);
    assert!(offset_of!(TemplateLiteral, span) == 16usize);
    assert!(offset_of!(TemplateLiteral, quasis) == 24usize);
    assert!(offset_of!(TemplateLiteral, expressions) == 40usize);

    assert!(size_of::<TaggedTemplateExpression>() == 96usize);
    assert!(align_of::<TaggedTemplateExpression>() == 8usize);
    assert!(offset_of!(TaggedTemplateExpression, parent) == 0usize);
    assert!(offset_of!(TaggedTemplateExpression, span) == 16usize);
    assert!(offset_of!(TaggedTemplateExpression, tag) == 24usize);
    assert!(offset_of!(TaggedTemplateExpression, quasi) == 32usize);
    assert!(offset_of!(TaggedTemplateExpression, type_parameters) == 88usize);

    assert!(size_of::<TemplateElement>() == 48usize);
    assert!(align_of::<TemplateElement>() == 8usize);
    assert!(offset_of!(TemplateElement, parent) == 0usize);
    assert!(offset_of!(TemplateElement, span) == 16usize);
    assert!(offset_of!(TemplateElement, tail) == 24usize);
    assert!(offset_of!(TemplateElement, value) == 28usize);

    assert!(size_of::<TemplateElementValue>() == 16usize);
    assert!(align_of::<TemplateElementValue>() == 4usize);
    assert!(offset_of!(TemplateElementValue, raw) == 0usize);
    assert!(offset_of!(TemplateElementValue, cooked) == 8usize);

    assert!(size_of::<MemberExpression>() == 8usize);
    assert!(align_of::<MemberExpression>() == 4usize);

    assert!(size_of::<ComputedMemberExpression>() == 48usize);
    assert!(align_of::<ComputedMemberExpression>() == 8usize);
    assert!(offset_of!(ComputedMemberExpression, parent) == 0usize);
    assert!(offset_of!(ComputedMemberExpression, span) == 16usize);
    assert!(offset_of!(ComputedMemberExpression, object) == 24usize);
    assert!(offset_of!(ComputedMemberExpression, expression) == 32usize);
    assert!(offset_of!(ComputedMemberExpression, optional) == 40usize);

    assert!(size_of::<StaticMemberExpression>() == 72usize);
    assert!(align_of::<StaticMemberExpression>() == 8usize);
    assert!(offset_of!(StaticMemberExpression, parent) == 0usize);
    assert!(offset_of!(StaticMemberExpression, span) == 16usize);
    assert!(offset_of!(StaticMemberExpression, object) == 24usize);
    assert!(offset_of!(StaticMemberExpression, property) == 32usize);
    assert!(offset_of!(StaticMemberExpression, optional) == 64usize);

    assert!(size_of::<PrivateFieldExpression>() == 72usize);
    assert!(align_of::<PrivateFieldExpression>() == 8usize);
    assert!(offset_of!(PrivateFieldExpression, parent) == 0usize);
    assert!(offset_of!(PrivateFieldExpression, span) == 16usize);
    assert!(offset_of!(PrivateFieldExpression, object) == 24usize);
    assert!(offset_of!(PrivateFieldExpression, field) == 32usize);
    assert!(offset_of!(PrivateFieldExpression, optional) == 64usize);

    assert!(size_of::<CallExpression>() == 56usize);
    assert!(align_of::<CallExpression>() == 8usize);
    assert!(offset_of!(CallExpression, parent) == 0usize);
    assert!(offset_of!(CallExpression, span) == 16usize);
    assert!(offset_of!(CallExpression, callee) == 24usize);
    assert!(offset_of!(CallExpression, type_parameters) == 32usize);
    assert!(offset_of!(CallExpression, arguments) == 36usize);
    assert!(offset_of!(CallExpression, optional) == 52usize);

    assert!(size_of::<NewExpression>() == 56usize);
    assert!(align_of::<NewExpression>() == 8usize);
    assert!(offset_of!(NewExpression, parent) == 0usize);
    assert!(offset_of!(NewExpression, span) == 16usize);
    assert!(offset_of!(NewExpression, callee) == 24usize);
    assert!(offset_of!(NewExpression, arguments) == 32usize);
    assert!(offset_of!(NewExpression, type_parameters) == 48usize);

    assert!(size_of::<MetaProperty>() == 88usize);
    assert!(align_of::<MetaProperty>() == 8usize);
    assert!(offset_of!(MetaProperty, parent) == 0usize);
    assert!(offset_of!(MetaProperty, span) == 16usize);
    assert!(offset_of!(MetaProperty, meta) == 24usize);
    assert!(offset_of!(MetaProperty, property) == 56usize);

    assert!(size_of::<SpreadElement>() == 32usize);
    assert!(align_of::<SpreadElement>() == 8usize);
    assert!(offset_of!(SpreadElement, parent) == 0usize);
    assert!(offset_of!(SpreadElement, span) == 16usize);
    assert!(offset_of!(SpreadElement, argument) == 24usize);

    assert!(size_of::<Argument>() == 8usize);
    assert!(align_of::<Argument>() == 4usize);

    assert!(size_of::<UpdateExpression>() == 40usize);
    assert!(align_of::<UpdateExpression>() == 8usize);
    assert!(offset_of!(UpdateExpression, parent) == 0usize);
    assert!(offset_of!(UpdateExpression, span) == 16usize);
    assert!(offset_of!(UpdateExpression, operator) == 24usize);
    assert!(offset_of!(UpdateExpression, prefix) == 25usize);
    assert!(offset_of!(UpdateExpression, argument) == 28usize);

    assert!(size_of::<UnaryExpression>() == 40usize);
    assert!(align_of::<UnaryExpression>() == 8usize);
    assert!(offset_of!(UnaryExpression, parent) == 0usize);
    assert!(offset_of!(UnaryExpression, span) == 16usize);
    assert!(offset_of!(UnaryExpression, operator) == 24usize);
    assert!(offset_of!(UnaryExpression, argument) == 28usize);

    assert!(size_of::<BinaryExpression>() == 48usize);
    assert!(align_of::<BinaryExpression>() == 8usize);
    assert!(offset_of!(BinaryExpression, parent) == 0usize);
    assert!(offset_of!(BinaryExpression, span) == 16usize);
    assert!(offset_of!(BinaryExpression, left) == 24usize);
    assert!(offset_of!(BinaryExpression, operator) == 32usize);
    assert!(offset_of!(BinaryExpression, right) == 36usize);

    assert!(size_of::<PrivateInExpression>() == 72usize);
    assert!(align_of::<PrivateInExpression>() == 8usize);
    assert!(offset_of!(PrivateInExpression, parent) == 0usize);
    assert!(offset_of!(PrivateInExpression, span) == 16usize);
    assert!(offset_of!(PrivateInExpression, left) == 24usize);
    assert!(offset_of!(PrivateInExpression, operator) == 56usize);
    assert!(offset_of!(PrivateInExpression, right) == 60usize);

    assert!(size_of::<LogicalExpression>() == 48usize);
    assert!(align_of::<LogicalExpression>() == 8usize);
    assert!(offset_of!(LogicalExpression, parent) == 0usize);
    assert!(offset_of!(LogicalExpression, span) == 16usize);
    assert!(offset_of!(LogicalExpression, left) == 24usize);
    assert!(offset_of!(LogicalExpression, operator) == 32usize);
    assert!(offset_of!(LogicalExpression, right) == 36usize);

    assert!(size_of::<ConditionalExpression>() == 48usize);
    assert!(align_of::<ConditionalExpression>() == 8usize);
    assert!(offset_of!(ConditionalExpression, parent) == 0usize);
    assert!(offset_of!(ConditionalExpression, span) == 16usize);
    assert!(offset_of!(ConditionalExpression, test) == 24usize);
    assert!(offset_of!(ConditionalExpression, consequent) == 32usize);
    assert!(offset_of!(ConditionalExpression, alternate) == 40usize);

    assert!(size_of::<AssignmentExpression>() == 48usize);
    assert!(align_of::<AssignmentExpression>() == 8usize);
    assert!(offset_of!(AssignmentExpression, parent) == 0usize);
    assert!(offset_of!(AssignmentExpression, span) == 16usize);
    assert!(offset_of!(AssignmentExpression, operator) == 24usize);
    assert!(offset_of!(AssignmentExpression, left) == 28usize);
    assert!(offset_of!(AssignmentExpression, right) == 36usize);

    assert!(size_of::<AssignmentTarget>() == 8usize);
    assert!(align_of::<AssignmentTarget>() == 4usize);

    assert!(size_of::<SimpleAssignmentTarget>() == 8usize);
    assert!(align_of::<SimpleAssignmentTarget>() == 4usize);

    assert!(size_of::<AssignmentTargetPattern>() == 8usize);
    assert!(align_of::<AssignmentTargetPattern>() == 4usize);

    assert!(size_of::<ArrayAssignmentTarget>() == 88usize);
    assert!(align_of::<ArrayAssignmentTarget>() == 8usize);
    assert!(offset_of!(ArrayAssignmentTarget, parent) == 0usize);
    assert!(offset_of!(ArrayAssignmentTarget, span) == 16usize);
    assert!(offset_of!(ArrayAssignmentTarget, elements) == 24usize);
    assert!(offset_of!(ArrayAssignmentTarget, rest) == 40usize);
    assert!(offset_of!(ArrayAssignmentTarget, trailing_comma) == 72usize);

    assert!(size_of::<ObjectAssignmentTarget>() == 72usize);
    assert!(align_of::<ObjectAssignmentTarget>() == 8usize);
    assert!(offset_of!(ObjectAssignmentTarget, parent) == 0usize);
    assert!(offset_of!(ObjectAssignmentTarget, span) == 16usize);
    assert!(offset_of!(ObjectAssignmentTarget, properties) == 24usize);
    assert!(offset_of!(ObjectAssignmentTarget, rest) == 40usize);

    assert!(size_of::<AssignmentTargetRest>() == 32usize);
    assert!(align_of::<AssignmentTargetRest>() == 8usize);
    assert!(offset_of!(AssignmentTargetRest, parent) == 0usize);
    assert!(offset_of!(AssignmentTargetRest, span) == 16usize);
    assert!(offset_of!(AssignmentTargetRest, target) == 24usize);

    assert!(size_of::<AssignmentTargetMaybeDefault>() == 8usize);
    assert!(align_of::<AssignmentTargetMaybeDefault>() == 4usize);

    assert!(size_of::<AssignmentTargetWithDefault>() == 40usize);
    assert!(align_of::<AssignmentTargetWithDefault>() == 8usize);
    assert!(offset_of!(AssignmentTargetWithDefault, parent) == 0usize);
    assert!(offset_of!(AssignmentTargetWithDefault, span) == 16usize);
    assert!(offset_of!(AssignmentTargetWithDefault, binding) == 24usize);
    assert!(offset_of!(AssignmentTargetWithDefault, init) == 32usize);

    assert!(size_of::<AssignmentTargetProperty>() == 8usize);
    assert!(align_of::<AssignmentTargetProperty>() == 4usize);

    assert!(size_of::<AssignmentTargetPropertyIdentifier>() == 72usize);
    assert!(align_of::<AssignmentTargetPropertyIdentifier>() == 8usize);
    assert!(offset_of!(AssignmentTargetPropertyIdentifier, parent) == 0usize);
    assert!(offset_of!(AssignmentTargetPropertyIdentifier, span) == 16usize);
    assert!(offset_of!(AssignmentTargetPropertyIdentifier, binding) == 24usize);
    assert!(offset_of!(AssignmentTargetPropertyIdentifier, init) == 64usize);

    assert!(size_of::<AssignmentTargetPropertyProperty>() == 48usize);
    assert!(align_of::<AssignmentTargetPropertyProperty>() == 8usize);
    assert!(offset_of!(AssignmentTargetPropertyProperty, parent) == 0usize);
    assert!(offset_of!(AssignmentTargetPropertyProperty, span) == 16usize);
    assert!(offset_of!(AssignmentTargetPropertyProperty, name) == 24usize);
    assert!(offset_of!(AssignmentTargetPropertyProperty, binding) == 32usize);
    assert!(offset_of!(AssignmentTargetPropertyProperty, computed) == 40usize);

    assert!(size_of::<SequenceExpression>() == 40usize);
    assert!(align_of::<SequenceExpression>() == 8usize);
    assert!(offset_of!(SequenceExpression, parent) == 0usize);
    assert!(offset_of!(SequenceExpression, span) == 16usize);
    assert!(offset_of!(SequenceExpression, expressions) == 24usize);

    assert!(size_of::<Super>() == 24usize);
    assert!(align_of::<Super>() == 8usize);
    assert!(offset_of!(Super, parent) == 0usize);
    assert!(offset_of!(Super, span) == 16usize);

    assert!(size_of::<AwaitExpression>() == 32usize);
    assert!(align_of::<AwaitExpression>() == 8usize);
    assert!(offset_of!(AwaitExpression, parent) == 0usize);
    assert!(offset_of!(AwaitExpression, span) == 16usize);
    assert!(offset_of!(AwaitExpression, argument) == 24usize);

    assert!(size_of::<ChainExpression>() == 32usize);
    assert!(align_of::<ChainExpression>() == 8usize);
    assert!(offset_of!(ChainExpression, parent) == 0usize);
    assert!(offset_of!(ChainExpression, span) == 16usize);
    assert!(offset_of!(ChainExpression, expression) == 24usize);

    assert!(size_of::<ChainElement>() == 8usize);
    assert!(align_of::<ChainElement>() == 4usize);

    assert!(size_of::<ParenthesizedExpression>() == 32usize);
    assert!(align_of::<ParenthesizedExpression>() == 8usize);
    assert!(offset_of!(ParenthesizedExpression, parent) == 0usize);
    assert!(offset_of!(ParenthesizedExpression, span) == 16usize);
    assert!(offset_of!(ParenthesizedExpression, expression) == 24usize);

    assert!(size_of::<Statement>() == 8usize);
    assert!(align_of::<Statement>() == 4usize);

    assert!(size_of::<Directive>() == 72usize);
    assert!(align_of::<Directive>() == 8usize);
    assert!(offset_of!(Directive, parent) == 0usize);
    assert!(offset_of!(Directive, span) == 16usize);
    assert!(offset_of!(Directive, expression) == 24usize);
    assert!(offset_of!(Directive, directive) == 64usize);

    assert!(size_of::<Hashbang>() == 32usize);
    assert!(align_of::<Hashbang>() == 8usize);
    assert!(offset_of!(Hashbang, parent) == 0usize);
    assert!(offset_of!(Hashbang, span) == 16usize);
    assert!(offset_of!(Hashbang, value) == 24usize);

    assert!(size_of::<BlockStatement>() == 48usize);
    assert!(align_of::<BlockStatement>() == 8usize);
    assert!(offset_of!(BlockStatement, parent) == 0usize);
    assert!(offset_of!(BlockStatement, span) == 16usize);
    assert!(offset_of!(BlockStatement, body) == 24usize);
    assert!(offset_of!(BlockStatement, scope_id) == 40usize);

    assert!(size_of::<Declaration>() == 8usize);
    assert!(align_of::<Declaration>() == 4usize);

    assert!(size_of::<VariableDeclaration>() == 48usize);
    assert!(align_of::<VariableDeclaration>() == 8usize);
    assert!(offset_of!(VariableDeclaration, parent) == 0usize);
    assert!(offset_of!(VariableDeclaration, span) == 16usize);
    assert!(offset_of!(VariableDeclaration, kind) == 24usize);
    assert!(offset_of!(VariableDeclaration, declarations) == 28usize);
    assert!(offset_of!(VariableDeclaration, declare) == 44usize);

    assert!(size_of::<VariableDeclarationKind>() == 1usize);
    assert!(align_of::<VariableDeclarationKind>() == 1usize);

    assert!(size_of::<VariableDeclarator>() == 56usize);
    assert!(align_of::<VariableDeclarator>() == 8usize);
    assert!(offset_of!(VariableDeclarator, parent) == 0usize);
    assert!(offset_of!(VariableDeclarator, span) == 16usize);
    assert!(offset_of!(VariableDeclarator, kind) == 24usize);
    assert!(offset_of!(VariableDeclarator, id) == 28usize);
    assert!(offset_of!(VariableDeclarator, init) == 44usize);
    assert!(offset_of!(VariableDeclarator, definite) == 52usize);

    assert!(size_of::<EmptyStatement>() == 24usize);
    assert!(align_of::<EmptyStatement>() == 8usize);
    assert!(offset_of!(EmptyStatement, parent) == 0usize);
    assert!(offset_of!(EmptyStatement, span) == 16usize);

    assert!(size_of::<ExpressionStatement>() == 32usize);
    assert!(align_of::<ExpressionStatement>() == 8usize);
    assert!(offset_of!(ExpressionStatement, parent) == 0usize);
    assert!(offset_of!(ExpressionStatement, span) == 16usize);
    assert!(offset_of!(ExpressionStatement, expression) == 24usize);

    assert!(size_of::<IfStatement>() == 48usize);
    assert!(align_of::<IfStatement>() == 8usize);
    assert!(offset_of!(IfStatement, parent) == 0usize);
    assert!(offset_of!(IfStatement, span) == 16usize);
    assert!(offset_of!(IfStatement, test) == 24usize);
    assert!(offset_of!(IfStatement, consequent) == 32usize);
    assert!(offset_of!(IfStatement, alternate) == 40usize);

    assert!(size_of::<DoWhileStatement>() == 40usize);
    assert!(align_of::<DoWhileStatement>() == 8usize);
    assert!(offset_of!(DoWhileStatement, parent) == 0usize);
    assert!(offset_of!(DoWhileStatement, span) == 16usize);
    assert!(offset_of!(DoWhileStatement, body) == 24usize);
    assert!(offset_of!(DoWhileStatement, test) == 32usize);

    assert!(size_of::<WhileStatement>() == 40usize);
    assert!(align_of::<WhileStatement>() == 8usize);
    assert!(offset_of!(WhileStatement, parent) == 0usize);
    assert!(offset_of!(WhileStatement, span) == 16usize);
    assert!(offset_of!(WhileStatement, test) == 24usize);
    assert!(offset_of!(WhileStatement, body) == 32usize);

    assert!(size_of::<ForStatement>() == 64usize);
    assert!(align_of::<ForStatement>() == 8usize);
    assert!(offset_of!(ForStatement, parent) == 0usize);
    assert!(offset_of!(ForStatement, span) == 16usize);
    assert!(offset_of!(ForStatement, init) == 24usize);
    assert!(offset_of!(ForStatement, test) == 32usize);
    assert!(offset_of!(ForStatement, update) == 40usize);
    assert!(offset_of!(ForStatement, body) == 48usize);
    assert!(offset_of!(ForStatement, scope_id) == 56usize);

    assert!(size_of::<ForStatementInit>() == 8usize);
    assert!(align_of::<ForStatementInit>() == 4usize);

    assert!(size_of::<ForInStatement>() == 56usize);
    assert!(align_of::<ForInStatement>() == 8usize);
    assert!(offset_of!(ForInStatement, parent) == 0usize);
    assert!(offset_of!(ForInStatement, span) == 16usize);
    assert!(offset_of!(ForInStatement, left) == 24usize);
    assert!(offset_of!(ForInStatement, right) == 32usize);
    assert!(offset_of!(ForInStatement, body) == 40usize);
    assert!(offset_of!(ForInStatement, scope_id) == 48usize);

    assert!(size_of::<ForStatementLeft>() == 8usize);
    assert!(align_of::<ForStatementLeft>() == 4usize);

    assert!(size_of::<ForOfStatement>() == 56usize);
    assert!(align_of::<ForOfStatement>() == 8usize);
    assert!(offset_of!(ForOfStatement, parent) == 0usize);
    assert!(offset_of!(ForOfStatement, span) == 16usize);
    assert!(offset_of!(ForOfStatement, r#await) == 24usize);
    assert!(offset_of!(ForOfStatement, left) == 28usize);
    assert!(offset_of!(ForOfStatement, right) == 36usize);
    assert!(offset_of!(ForOfStatement, body) == 44usize);
    assert!(offset_of!(ForOfStatement, scope_id) == 52usize);

    assert!(size_of::<ContinueStatement>() == 56usize);
    assert!(align_of::<ContinueStatement>() == 8usize);
    assert!(offset_of!(ContinueStatement, parent) == 0usize);
    assert!(offset_of!(ContinueStatement, span) == 16usize);
    assert!(offset_of!(ContinueStatement, label) == 24usize);

    assert!(size_of::<BreakStatement>() == 56usize);
    assert!(align_of::<BreakStatement>() == 8usize);
    assert!(offset_of!(BreakStatement, parent) == 0usize);
    assert!(offset_of!(BreakStatement, span) == 16usize);
    assert!(offset_of!(BreakStatement, label) == 24usize);

    assert!(size_of::<ReturnStatement>() == 32usize);
    assert!(align_of::<ReturnStatement>() == 8usize);
    assert!(offset_of!(ReturnStatement, parent) == 0usize);
    assert!(offset_of!(ReturnStatement, span) == 16usize);
    assert!(offset_of!(ReturnStatement, argument) == 24usize);

    assert!(size_of::<WithStatement>() == 40usize);
    assert!(align_of::<WithStatement>() == 8usize);
    assert!(offset_of!(WithStatement, parent) == 0usize);
    assert!(offset_of!(WithStatement, span) == 16usize);
    assert!(offset_of!(WithStatement, object) == 24usize);
    assert!(offset_of!(WithStatement, body) == 32usize);

    assert!(size_of::<SwitchStatement>() == 56usize);
    assert!(align_of::<SwitchStatement>() == 8usize);
    assert!(offset_of!(SwitchStatement, parent) == 0usize);
    assert!(offset_of!(SwitchStatement, span) == 16usize);
    assert!(offset_of!(SwitchStatement, discriminant) == 24usize);
    assert!(offset_of!(SwitchStatement, cases) == 32usize);
    assert!(offset_of!(SwitchStatement, scope_id) == 48usize);

    assert!(size_of::<SwitchCase>() == 48usize);
    assert!(align_of::<SwitchCase>() == 8usize);
    assert!(offset_of!(SwitchCase, parent) == 0usize);
    assert!(offset_of!(SwitchCase, span) == 16usize);
    assert!(offset_of!(SwitchCase, test) == 24usize);
    assert!(offset_of!(SwitchCase, consequent) == 32usize);

    assert!(size_of::<LabeledStatement>() == 64usize);
    assert!(align_of::<LabeledStatement>() == 8usize);
    assert!(offset_of!(LabeledStatement, parent) == 0usize);
    assert!(offset_of!(LabeledStatement, span) == 16usize);
    assert!(offset_of!(LabeledStatement, label) == 24usize);
    assert!(offset_of!(LabeledStatement, body) == 56usize);

    assert!(size_of::<ThrowStatement>() == 32usize);
    assert!(align_of::<ThrowStatement>() == 8usize);
    assert!(offset_of!(ThrowStatement, parent) == 0usize);
    assert!(offset_of!(ThrowStatement, span) == 16usize);
    assert!(offset_of!(ThrowStatement, argument) == 24usize);

    assert!(size_of::<TryStatement>() == 40usize);
    assert!(align_of::<TryStatement>() == 8usize);
    assert!(offset_of!(TryStatement, parent) == 0usize);
    assert!(offset_of!(TryStatement, span) == 16usize);
    assert!(offset_of!(TryStatement, block) == 24usize);
    assert!(offset_of!(TryStatement, handler) == 28usize);
    assert!(offset_of!(TryStatement, finalizer) == 32usize);

    assert!(size_of::<CatchClause>() == 72usize);
    assert!(align_of::<CatchClause>() == 8usize);
    assert!(offset_of!(CatchClause, parent) == 0usize);
    assert!(offset_of!(CatchClause, span) == 16usize);
    assert!(offset_of!(CatchClause, param) == 24usize);
    assert!(offset_of!(CatchClause, body) == 64usize);
    assert!(offset_of!(CatchClause, scope_id) == 68usize);

    assert!(size_of::<CatchParameter>() == 40usize);
    assert!(align_of::<CatchParameter>() == 8usize);
    assert!(offset_of!(CatchParameter, parent) == 0usize);
    assert!(offset_of!(CatchParameter, span) == 16usize);
    assert!(offset_of!(CatchParameter, pattern) == 24usize);

    assert!(size_of::<DebuggerStatement>() == 24usize);
    assert!(align_of::<DebuggerStatement>() == 8usize);
    assert!(offset_of!(DebuggerStatement, parent) == 0usize);
    assert!(offset_of!(DebuggerStatement, span) == 16usize);

    assert!(size_of::<BindingPattern>() == 16usize);
    assert!(align_of::<BindingPattern>() == 4usize);
    assert!(offset_of!(BindingPattern, kind) == 0usize);
    assert!(offset_of!(BindingPattern, type_annotation) == 8usize);
    assert!(offset_of!(BindingPattern, optional) == 12usize);

    assert!(size_of::<BindingPatternKind>() == 8usize);
    assert!(align_of::<BindingPatternKind>() == 4usize);

    assert!(size_of::<AssignmentPattern>() == 48usize);
    assert!(align_of::<AssignmentPattern>() == 8usize);
    assert!(offset_of!(AssignmentPattern, parent) == 0usize);
    assert!(offset_of!(AssignmentPattern, span) == 16usize);
    assert!(offset_of!(AssignmentPattern, left) == 24usize);
    assert!(offset_of!(AssignmentPattern, right) == 40usize);

    assert!(size_of::<ObjectPattern>() == 48usize);
    assert!(align_of::<ObjectPattern>() == 8usize);
    assert!(offset_of!(ObjectPattern, parent) == 0usize);
    assert!(offset_of!(ObjectPattern, span) == 16usize);
    assert!(offset_of!(ObjectPattern, properties) == 24usize);
    assert!(offset_of!(ObjectPattern, rest) == 40usize);

    assert!(size_of::<BindingProperty>() == 56usize);
    assert!(align_of::<BindingProperty>() == 8usize);
    assert!(offset_of!(BindingProperty, parent) == 0usize);
    assert!(offset_of!(BindingProperty, span) == 16usize);
    assert!(offset_of!(BindingProperty, key) == 24usize);
    assert!(offset_of!(BindingProperty, value) == 32usize);
    assert!(offset_of!(BindingProperty, shorthand) == 48usize);
    assert!(offset_of!(BindingProperty, computed) == 49usize);

    assert!(size_of::<ArrayPattern>() == 48usize);
    assert!(align_of::<ArrayPattern>() == 8usize);
    assert!(offset_of!(ArrayPattern, parent) == 0usize);
    assert!(offset_of!(ArrayPattern, span) == 16usize);
    assert!(offset_of!(ArrayPattern, elements) == 24usize);
    assert!(offset_of!(ArrayPattern, rest) == 40usize);

    assert!(size_of::<BindingRestElement>() == 40usize);
    assert!(align_of::<BindingRestElement>() == 8usize);
    assert!(offset_of!(BindingRestElement, parent) == 0usize);
    assert!(offset_of!(BindingRestElement, span) == 16usize);
    assert!(offset_of!(BindingRestElement, argument) == 24usize);

    assert!(size_of::<Function>() == 104usize);
    assert!(align_of::<Function>() == 8usize);
    assert!(offset_of!(Function, parent) == 0usize);
    assert!(offset_of!(Function, span) == 16usize);
    assert!(offset_of!(Function, r#type) == 24usize);
    assert!(offset_of!(Function, id) == 32usize);
    assert!(offset_of!(Function, generator) == 72usize);
    assert!(offset_of!(Function, r#async) == 73usize);
    assert!(offset_of!(Function, declare) == 74usize);
    assert!(offset_of!(Function, type_parameters) == 76usize);
    assert!(offset_of!(Function, this_param) == 80usize);
    assert!(offset_of!(Function, params) == 84usize);
    assert!(offset_of!(Function, return_type) == 88usize);
    assert!(offset_of!(Function, body) == 92usize);
    assert!(offset_of!(Function, scope_id) == 96usize);

    assert!(size_of::<FunctionType>() == 1usize);
    assert!(align_of::<FunctionType>() == 1usize);

    assert!(size_of::<FormalParameters>() == 48usize);
    assert!(align_of::<FormalParameters>() == 8usize);
    assert!(offset_of!(FormalParameters, parent) == 0usize);
    assert!(offset_of!(FormalParameters, span) == 16usize);
    assert!(offset_of!(FormalParameters, kind) == 24usize);
    assert!(offset_of!(FormalParameters, items) == 28usize);
    assert!(offset_of!(FormalParameters, rest) == 44usize);

    assert!(size_of::<FormalParameter>() == 64usize);
    assert!(align_of::<FormalParameter>() == 8usize);
    assert!(offset_of!(FormalParameter, parent) == 0usize);
    assert!(offset_of!(FormalParameter, span) == 16usize);
    assert!(offset_of!(FormalParameter, decorators) == 24usize);
    assert!(offset_of!(FormalParameter, pattern) == 40usize);
    assert!(offset_of!(FormalParameter, accessibility) == 56usize);
    assert!(offset_of!(FormalParameter, readonly) == 57usize);
    assert!(offset_of!(FormalParameter, r#override) == 58usize);

    assert!(size_of::<FormalParameterKind>() == 1usize);
    assert!(align_of::<FormalParameterKind>() == 1usize);

    assert!(size_of::<FunctionBody>() == 56usize);
    assert!(align_of::<FunctionBody>() == 8usize);
    assert!(offset_of!(FunctionBody, parent) == 0usize);
    assert!(offset_of!(FunctionBody, span) == 16usize);
    assert!(offset_of!(FunctionBody, directives) == 24usize);
    assert!(offset_of!(FunctionBody, statements) == 40usize);

    assert!(size_of::<ArrowFunctionExpression>() == 48usize);
    assert!(align_of::<ArrowFunctionExpression>() == 8usize);
    assert!(offset_of!(ArrowFunctionExpression, parent) == 0usize);
    assert!(offset_of!(ArrowFunctionExpression, span) == 16usize);
    assert!(offset_of!(ArrowFunctionExpression, expression) == 24usize);
    assert!(offset_of!(ArrowFunctionExpression, r#async) == 25usize);
    assert!(offset_of!(ArrowFunctionExpression, type_parameters) == 28usize);
    assert!(offset_of!(ArrowFunctionExpression, params) == 32usize);
    assert!(offset_of!(ArrowFunctionExpression, return_type) == 36usize);
    assert!(offset_of!(ArrowFunctionExpression, body) == 40usize);
    assert!(offset_of!(ArrowFunctionExpression, scope_id) == 44usize);

    assert!(size_of::<YieldExpression>() == 40usize);
    assert!(align_of::<YieldExpression>() == 8usize);
    assert!(offset_of!(YieldExpression, parent) == 0usize);
    assert!(offset_of!(YieldExpression, span) == 16usize);
    assert!(offset_of!(YieldExpression, delegate) == 24usize);
    assert!(offset_of!(YieldExpression, argument) == 28usize);

    assert!(size_of::<Class>() == 136usize);
    assert!(align_of::<Class>() == 8usize);
    assert!(offset_of!(Class, parent) == 0usize);
    assert!(offset_of!(Class, span) == 16usize);
    assert!(offset_of!(Class, r#type) == 24usize);
    assert!(offset_of!(Class, decorators) == 28usize);
    assert!(offset_of!(Class, id) == 48usize);
    assert!(offset_of!(Class, type_parameters) == 88usize);
    assert!(offset_of!(Class, super_class) == 92usize);
    assert!(offset_of!(Class, super_type_parameters) == 100usize);
    assert!(offset_of!(Class, implements) == 104usize);
    assert!(offset_of!(Class, body) == 120usize);
    assert!(offset_of!(Class, r#abstract) == 124usize);
    assert!(offset_of!(Class, declare) == 125usize);
    assert!(offset_of!(Class, scope_id) == 128usize);

    assert!(size_of::<ClassType>() == 1usize);
    assert!(align_of::<ClassType>() == 1usize);

    assert!(size_of::<ClassBody>() == 40usize);
    assert!(align_of::<ClassBody>() == 8usize);
    assert!(offset_of!(ClassBody, parent) == 0usize);
    assert!(offset_of!(ClassBody, span) == 16usize);
    assert!(offset_of!(ClassBody, body) == 24usize);

    assert!(size_of::<ClassElement>() == 8usize);
    assert!(align_of::<ClassElement>() == 4usize);

    assert!(size_of::<MethodDefinition>() == 64usize);
    assert!(align_of::<MethodDefinition>() == 8usize);
    assert!(offset_of!(MethodDefinition, parent) == 0usize);
    assert!(offset_of!(MethodDefinition, span) == 16usize);
    assert!(offset_of!(MethodDefinition, r#type) == 24usize);
    assert!(offset_of!(MethodDefinition, decorators) == 28usize);
    assert!(offset_of!(MethodDefinition, key) == 44usize);
    assert!(offset_of!(MethodDefinition, value) == 52usize);
    assert!(offset_of!(MethodDefinition, kind) == 56usize);
    assert!(offset_of!(MethodDefinition, computed) == 57usize);
    assert!(offset_of!(MethodDefinition, r#static) == 58usize);
    assert!(offset_of!(MethodDefinition, r#override) == 59usize);
    assert!(offset_of!(MethodDefinition, optional) == 60usize);
    assert!(offset_of!(MethodDefinition, accessibility) == 61usize);

    assert!(size_of::<MethodDefinitionType>() == 1usize);
    assert!(align_of::<MethodDefinitionType>() == 1usize);

    assert!(size_of::<PropertyDefinition>() == 80usize);
    assert!(align_of::<PropertyDefinition>() == 8usize);
    assert!(offset_of!(PropertyDefinition, parent) == 0usize);
    assert!(offset_of!(PropertyDefinition, span) == 16usize);
    assert!(offset_of!(PropertyDefinition, r#type) == 24usize);
    assert!(offset_of!(PropertyDefinition, decorators) == 28usize);
    assert!(offset_of!(PropertyDefinition, key) == 44usize);
    assert!(offset_of!(PropertyDefinition, value) == 52usize);
    assert!(offset_of!(PropertyDefinition, computed) == 60usize);
    assert!(offset_of!(PropertyDefinition, r#static) == 61usize);
    assert!(offset_of!(PropertyDefinition, declare) == 62usize);
    assert!(offset_of!(PropertyDefinition, r#override) == 63usize);
    assert!(offset_of!(PropertyDefinition, optional) == 64usize);
    assert!(offset_of!(PropertyDefinition, definite) == 65usize);
    assert!(offset_of!(PropertyDefinition, readonly) == 66usize);
    assert!(offset_of!(PropertyDefinition, type_annotation) == 68usize);
    assert!(offset_of!(PropertyDefinition, accessibility) == 72usize);

    assert!(size_of::<PropertyDefinitionType>() == 1usize);
    assert!(align_of::<PropertyDefinitionType>() == 1usize);

    assert!(size_of::<MethodDefinitionKind>() == 1usize);
    assert!(align_of::<MethodDefinitionKind>() == 1usize);

    assert!(size_of::<PrivateIdentifier>() == 32usize);
    assert!(align_of::<PrivateIdentifier>() == 8usize);
    assert!(offset_of!(PrivateIdentifier, parent) == 0usize);
    assert!(offset_of!(PrivateIdentifier, span) == 16usize);
    assert!(offset_of!(PrivateIdentifier, name) == 24usize);

    assert!(size_of::<StaticBlock>() == 48usize);
    assert!(align_of::<StaticBlock>() == 8usize);
    assert!(offset_of!(StaticBlock, parent) == 0usize);
    assert!(offset_of!(StaticBlock, span) == 16usize);
    assert!(offset_of!(StaticBlock, body) == 24usize);
    assert!(offset_of!(StaticBlock, scope_id) == 40usize);

    assert!(size_of::<ModuleDeclaration>() == 8usize);
    assert!(align_of::<ModuleDeclaration>() == 4usize);

    assert!(size_of::<AccessorPropertyType>() == 1usize);
    assert!(align_of::<AccessorPropertyType>() == 1usize);

    assert!(size_of::<AccessorProperty>() == 72usize);
    assert!(align_of::<AccessorProperty>() == 8usize);
    assert!(offset_of!(AccessorProperty, parent) == 0usize);
    assert!(offset_of!(AccessorProperty, span) == 16usize);
    assert!(offset_of!(AccessorProperty, r#type) == 24usize);
    assert!(offset_of!(AccessorProperty, decorators) == 28usize);
    assert!(offset_of!(AccessorProperty, key) == 44usize);
    assert!(offset_of!(AccessorProperty, value) == 52usize);
    assert!(offset_of!(AccessorProperty, computed) == 60usize);
    assert!(offset_of!(AccessorProperty, r#static) == 61usize);
    assert!(offset_of!(AccessorProperty, definite) == 62usize);
    assert!(offset_of!(AccessorProperty, type_annotation) == 64usize);
    assert!(offset_of!(AccessorProperty, accessibility) == 68usize);

    assert!(size_of::<ImportExpression>() == 56usize);
    assert!(align_of::<ImportExpression>() == 8usize);
    assert!(offset_of!(ImportExpression, parent) == 0usize);
    assert!(offset_of!(ImportExpression, span) == 16usize);
    assert!(offset_of!(ImportExpression, source) == 24usize);
    assert!(offset_of!(ImportExpression, arguments) == 32usize);
    assert!(offset_of!(ImportExpression, phase) == 48usize);

    assert!(size_of::<ImportDeclaration>() == 96usize);
    assert!(align_of::<ImportDeclaration>() == 8usize);
    assert!(offset_of!(ImportDeclaration, parent) == 0usize);
    assert!(offset_of!(ImportDeclaration, span) == 16usize);
    assert!(offset_of!(ImportDeclaration, specifiers) == 24usize);
    assert!(offset_of!(ImportDeclaration, source) == 40usize);
    assert!(offset_of!(ImportDeclaration, phase) == 80usize);
    assert!(offset_of!(ImportDeclaration, with_clause) == 84usize);
    assert!(offset_of!(ImportDeclaration, import_kind) == 88usize);

    assert!(size_of::<ImportPhase>() == 1usize);
    assert!(align_of::<ImportPhase>() == 1usize);

    assert!(size_of::<ImportDeclarationSpecifier>() == 8usize);
    assert!(align_of::<ImportDeclarationSpecifier>() == 4usize);

    assert!(size_of::<ImportSpecifier>() == 120usize);
    assert!(align_of::<ImportSpecifier>() == 8usize);
    assert!(offset_of!(ImportSpecifier, parent) == 0usize);
    assert!(offset_of!(ImportSpecifier, span) == 16usize);
    assert!(offset_of!(ImportSpecifier, imported) == 24usize);
    assert!(offset_of!(ImportSpecifier, local) == 72usize);
    assert!(offset_of!(ImportSpecifier, import_kind) == 112usize);

    assert!(size_of::<ImportDefaultSpecifier>() == 64usize);
    assert!(align_of::<ImportDefaultSpecifier>() == 8usize);
    assert!(offset_of!(ImportDefaultSpecifier, parent) == 0usize);
    assert!(offset_of!(ImportDefaultSpecifier, span) == 16usize);
    assert!(offset_of!(ImportDefaultSpecifier, local) == 24usize);

    assert!(size_of::<ImportNamespaceSpecifier>() == 64usize);
    assert!(align_of::<ImportNamespaceSpecifier>() == 8usize);
    assert!(offset_of!(ImportNamespaceSpecifier, parent) == 0usize);
    assert!(offset_of!(ImportNamespaceSpecifier, span) == 16usize);
    assert!(offset_of!(ImportNamespaceSpecifier, local) == 24usize);

    assert!(size_of::<WithClause>() == 72usize);
    assert!(align_of::<WithClause>() == 8usize);
    assert!(offset_of!(WithClause, parent) == 0usize);
    assert!(offset_of!(WithClause, span) == 16usize);
    assert!(offset_of!(WithClause, attributes_keyword) == 24usize);
    assert!(offset_of!(WithClause, with_entries) == 56usize);

    assert!(size_of::<ImportAttribute>() == 112usize);
    assert!(align_of::<ImportAttribute>() == 8usize);
    assert!(offset_of!(ImportAttribute, parent) == 0usize);
    assert!(offset_of!(ImportAttribute, span) == 16usize);
    assert!(offset_of!(ImportAttribute, key) == 24usize);
    assert!(offset_of!(ImportAttribute, value) == 72usize);

    assert!(size_of::<ImportAttributeKey>() == 48usize);
    assert!(align_of::<ImportAttributeKey>() == 8usize);

    assert!(size_of::<ExportNamedDeclaration>() == 96usize);
    assert!(align_of::<ExportNamedDeclaration>() == 8usize);
    assert!(offset_of!(ExportNamedDeclaration, parent) == 0usize);
    assert!(offset_of!(ExportNamedDeclaration, span) == 16usize);
    assert!(offset_of!(ExportNamedDeclaration, declaration) == 24usize);
    assert!(offset_of!(ExportNamedDeclaration, specifiers) == 32usize);
    assert!(offset_of!(ExportNamedDeclaration, source) == 48usize);
    assert!(offset_of!(ExportNamedDeclaration, export_kind) == 88usize);
    assert!(offset_of!(ExportNamedDeclaration, with_clause) == 92usize);

    assert!(size_of::<ExportDefaultDeclaration>() == 80usize);
    assert!(align_of::<ExportDefaultDeclaration>() == 8usize);
    assert!(offset_of!(ExportDefaultDeclaration, parent) == 0usize);
    assert!(offset_of!(ExportDefaultDeclaration, span) == 16usize);
    assert!(offset_of!(ExportDefaultDeclaration, declaration) == 24usize);
    assert!(offset_of!(ExportDefaultDeclaration, exported) == 32usize);

    assert!(size_of::<ExportAllDeclaration>() == 120usize);
    assert!(align_of::<ExportAllDeclaration>() == 8usize);
    assert!(offset_of!(ExportAllDeclaration, parent) == 0usize);
    assert!(offset_of!(ExportAllDeclaration, span) == 16usize);
    assert!(offset_of!(ExportAllDeclaration, exported) == 24usize);
    assert!(offset_of!(ExportAllDeclaration, source) == 72usize);
    assert!(offset_of!(ExportAllDeclaration, with_clause) == 112usize);
    assert!(offset_of!(ExportAllDeclaration, export_kind) == 116usize);

    assert!(size_of::<ExportSpecifier>() == 128usize);
    assert!(align_of::<ExportSpecifier>() == 8usize);
    assert!(offset_of!(ExportSpecifier, parent) == 0usize);
    assert!(offset_of!(ExportSpecifier, span) == 16usize);
    assert!(offset_of!(ExportSpecifier, local) == 24usize);
    assert!(offset_of!(ExportSpecifier, exported) == 72usize);
    assert!(offset_of!(ExportSpecifier, export_kind) == 120usize);

    assert!(size_of::<ExportDefaultDeclarationKind>() == 8usize);
    assert!(align_of::<ExportDefaultDeclarationKind>() == 4usize);

    assert!(size_of::<ModuleExportName>() == 48usize);
    assert!(align_of::<ModuleExportName>() == 8usize);

    assert!(size_of::<TSThisParameter>() == 40usize);
    assert!(align_of::<TSThisParameter>() == 8usize);
    assert!(offset_of!(TSThisParameter, parent) == 0usize);
    assert!(offset_of!(TSThisParameter, span) == 16usize);
    assert!(offset_of!(TSThisParameter, this_span) == 24usize);
    assert!(offset_of!(TSThisParameter, type_annotation) == 32usize);

    assert!(size_of::<TSEnumDeclaration>() == 88usize);
    assert!(align_of::<TSEnumDeclaration>() == 8usize);
    assert!(offset_of!(TSEnumDeclaration, parent) == 0usize);
    assert!(offset_of!(TSEnumDeclaration, span) == 16usize);
    assert!(offset_of!(TSEnumDeclaration, id) == 24usize);
    assert!(offset_of!(TSEnumDeclaration, members) == 64usize);
    assert!(offset_of!(TSEnumDeclaration, r#const) == 80usize);
    assert!(offset_of!(TSEnumDeclaration, declare) == 81usize);
    assert!(offset_of!(TSEnumDeclaration, scope_id) == 84usize);

    assert!(size_of::<TSEnumMember>() == 40usize);
    assert!(align_of::<TSEnumMember>() == 8usize);
    assert!(offset_of!(TSEnumMember, parent) == 0usize);
    assert!(offset_of!(TSEnumMember, span) == 16usize);
    assert!(offset_of!(TSEnumMember, id) == 24usize);
    assert!(offset_of!(TSEnumMember, initializer) == 32usize);

    assert!(size_of::<TSEnumMemberName>() == 8usize);
    assert!(align_of::<TSEnumMemberName>() == 4usize);

    assert!(size_of::<TSTypeAnnotation>() == 32usize);
    assert!(align_of::<TSTypeAnnotation>() == 8usize);
    assert!(offset_of!(TSTypeAnnotation, parent) == 0usize);
    assert!(offset_of!(TSTypeAnnotation, span) == 16usize);
    assert!(offset_of!(TSTypeAnnotation, type_annotation) == 24usize);

    assert!(size_of::<TSLiteralType>() == 32usize);
    assert!(align_of::<TSLiteralType>() == 8usize);
    assert!(offset_of!(TSLiteralType, parent) == 0usize);
    assert!(offset_of!(TSLiteralType, span) == 16usize);
    assert!(offset_of!(TSLiteralType, literal) == 24usize);

    assert!(size_of::<TSLiteral>() == 8usize);
    assert!(align_of::<TSLiteral>() == 4usize);

    assert!(size_of::<TSType>() == 8usize);
    assert!(align_of::<TSType>() == 4usize);

    assert!(size_of::<TSConditionalType>() == 64usize);
    assert!(align_of::<TSConditionalType>() == 8usize);
    assert!(offset_of!(TSConditionalType, parent) == 0usize);
    assert!(offset_of!(TSConditionalType, span) == 16usize);
    assert!(offset_of!(TSConditionalType, check_type) == 24usize);
    assert!(offset_of!(TSConditionalType, extends_type) == 32usize);
    assert!(offset_of!(TSConditionalType, true_type) == 40usize);
    assert!(offset_of!(TSConditionalType, false_type) == 48usize);
    assert!(offset_of!(TSConditionalType, scope_id) == 56usize);

    assert!(size_of::<TSUnionType>() == 40usize);
    assert!(align_of::<TSUnionType>() == 8usize);
    assert!(offset_of!(TSUnionType, parent) == 0usize);
    assert!(offset_of!(TSUnionType, span) == 16usize);
    assert!(offset_of!(TSUnionType, types) == 24usize);

    assert!(size_of::<TSIntersectionType>() == 40usize);
    assert!(align_of::<TSIntersectionType>() == 8usize);
    assert!(offset_of!(TSIntersectionType, parent) == 0usize);
    assert!(offset_of!(TSIntersectionType, span) == 16usize);
    assert!(offset_of!(TSIntersectionType, types) == 24usize);

    assert!(size_of::<TSParenthesizedType>() == 32usize);
    assert!(align_of::<TSParenthesizedType>() == 8usize);
    assert!(offset_of!(TSParenthesizedType, parent) == 0usize);
    assert!(offset_of!(TSParenthesizedType, span) == 16usize);
    assert!(offset_of!(TSParenthesizedType, type_annotation) == 24usize);

    assert!(size_of::<TSTypeOperator>() == 40usize);
    assert!(align_of::<TSTypeOperator>() == 8usize);
    assert!(offset_of!(TSTypeOperator, parent) == 0usize);
    assert!(offset_of!(TSTypeOperator, span) == 16usize);
    assert!(offset_of!(TSTypeOperator, operator) == 24usize);
    assert!(offset_of!(TSTypeOperator, type_annotation) == 28usize);

    assert!(size_of::<TSTypeOperatorOperator>() == 1usize);
    assert!(align_of::<TSTypeOperatorOperator>() == 1usize);

    assert!(size_of::<TSArrayType>() == 32usize);
    assert!(align_of::<TSArrayType>() == 8usize);
    assert!(offset_of!(TSArrayType, parent) == 0usize);
    assert!(offset_of!(TSArrayType, span) == 16usize);
    assert!(offset_of!(TSArrayType, element_type) == 24usize);

    assert!(size_of::<TSIndexedAccessType>() == 40usize);
    assert!(align_of::<TSIndexedAccessType>() == 8usize);
    assert!(offset_of!(TSIndexedAccessType, parent) == 0usize);
    assert!(offset_of!(TSIndexedAccessType, span) == 16usize);
    assert!(offset_of!(TSIndexedAccessType, object_type) == 24usize);
    assert!(offset_of!(TSIndexedAccessType, index_type) == 32usize);

    assert!(size_of::<TSTupleType>() == 40usize);
    assert!(align_of::<TSTupleType>() == 8usize);
    assert!(offset_of!(TSTupleType, parent) == 0usize);
    assert!(offset_of!(TSTupleType, span) == 16usize);
    assert!(offset_of!(TSTupleType, element_types) == 24usize);

    assert!(size_of::<TSNamedTupleMember>() == 72usize);
    assert!(align_of::<TSNamedTupleMember>() == 8usize);
    assert!(offset_of!(TSNamedTupleMember, parent) == 0usize);
    assert!(offset_of!(TSNamedTupleMember, span) == 16usize);
    assert!(offset_of!(TSNamedTupleMember, element_type) == 24usize);
    assert!(offset_of!(TSNamedTupleMember, label) == 32usize);
    assert!(offset_of!(TSNamedTupleMember, optional) == 64usize);

    assert!(size_of::<TSOptionalType>() == 32usize);
    assert!(align_of::<TSOptionalType>() == 8usize);
    assert!(offset_of!(TSOptionalType, parent) == 0usize);
    assert!(offset_of!(TSOptionalType, span) == 16usize);
    assert!(offset_of!(TSOptionalType, type_annotation) == 24usize);

    assert!(size_of::<TSRestType>() == 32usize);
    assert!(align_of::<TSRestType>() == 8usize);
    assert!(offset_of!(TSRestType, parent) == 0usize);
    assert!(offset_of!(TSRestType, span) == 16usize);
    assert!(offset_of!(TSRestType, type_annotation) == 24usize);

    assert!(size_of::<TSTupleElement>() == 8usize);
    assert!(align_of::<TSTupleElement>() == 4usize);

    assert!(size_of::<TSAnyKeyword>() == 24usize);
    assert!(align_of::<TSAnyKeyword>() == 8usize);
    assert!(offset_of!(TSAnyKeyword, parent) == 0usize);
    assert!(offset_of!(TSAnyKeyword, span) == 16usize);

    assert!(size_of::<TSStringKeyword>() == 24usize);
    assert!(align_of::<TSStringKeyword>() == 8usize);
    assert!(offset_of!(TSStringKeyword, parent) == 0usize);
    assert!(offset_of!(TSStringKeyword, span) == 16usize);

    assert!(size_of::<TSBooleanKeyword>() == 24usize);
    assert!(align_of::<TSBooleanKeyword>() == 8usize);
    assert!(offset_of!(TSBooleanKeyword, parent) == 0usize);
    assert!(offset_of!(TSBooleanKeyword, span) == 16usize);

    assert!(size_of::<TSNumberKeyword>() == 24usize);
    assert!(align_of::<TSNumberKeyword>() == 8usize);
    assert!(offset_of!(TSNumberKeyword, parent) == 0usize);
    assert!(offset_of!(TSNumberKeyword, span) == 16usize);

    assert!(size_of::<TSNeverKeyword>() == 24usize);
    assert!(align_of::<TSNeverKeyword>() == 8usize);
    assert!(offset_of!(TSNeverKeyword, parent) == 0usize);
    assert!(offset_of!(TSNeverKeyword, span) == 16usize);

    assert!(size_of::<TSIntrinsicKeyword>() == 24usize);
    assert!(align_of::<TSIntrinsicKeyword>() == 8usize);
    assert!(offset_of!(TSIntrinsicKeyword, parent) == 0usize);
    assert!(offset_of!(TSIntrinsicKeyword, span) == 16usize);

    assert!(size_of::<TSUnknownKeyword>() == 24usize);
    assert!(align_of::<TSUnknownKeyword>() == 8usize);
    assert!(offset_of!(TSUnknownKeyword, parent) == 0usize);
    assert!(offset_of!(TSUnknownKeyword, span) == 16usize);

    assert!(size_of::<TSNullKeyword>() == 24usize);
    assert!(align_of::<TSNullKeyword>() == 8usize);
    assert!(offset_of!(TSNullKeyword, parent) == 0usize);
    assert!(offset_of!(TSNullKeyword, span) == 16usize);

    assert!(size_of::<TSUndefinedKeyword>() == 24usize);
    assert!(align_of::<TSUndefinedKeyword>() == 8usize);
    assert!(offset_of!(TSUndefinedKeyword, parent) == 0usize);
    assert!(offset_of!(TSUndefinedKeyword, span) == 16usize);

    assert!(size_of::<TSVoidKeyword>() == 24usize);
    assert!(align_of::<TSVoidKeyword>() == 8usize);
    assert!(offset_of!(TSVoidKeyword, parent) == 0usize);
    assert!(offset_of!(TSVoidKeyword, span) == 16usize);

    assert!(size_of::<TSSymbolKeyword>() == 24usize);
    assert!(align_of::<TSSymbolKeyword>() == 8usize);
    assert!(offset_of!(TSSymbolKeyword, parent) == 0usize);
    assert!(offset_of!(TSSymbolKeyword, span) == 16usize);

    assert!(size_of::<TSThisType>() == 24usize);
    assert!(align_of::<TSThisType>() == 8usize);
    assert!(offset_of!(TSThisType, parent) == 0usize);
    assert!(offset_of!(TSThisType, span) == 16usize);

    assert!(size_of::<TSObjectKeyword>() == 24usize);
    assert!(align_of::<TSObjectKeyword>() == 8usize);
    assert!(offset_of!(TSObjectKeyword, parent) == 0usize);
    assert!(offset_of!(TSObjectKeyword, span) == 16usize);

    assert!(size_of::<TSBigIntKeyword>() == 24usize);
    assert!(align_of::<TSBigIntKeyword>() == 8usize);
    assert!(offset_of!(TSBigIntKeyword, parent) == 0usize);
    assert!(offset_of!(TSBigIntKeyword, span) == 16usize);

    assert!(size_of::<TSTypeReference>() == 40usize);
    assert!(align_of::<TSTypeReference>() == 8usize);
    assert!(offset_of!(TSTypeReference, parent) == 0usize);
    assert!(offset_of!(TSTypeReference, span) == 16usize);
    assert!(offset_of!(TSTypeReference, type_name) == 24usize);
    assert!(offset_of!(TSTypeReference, type_parameters) == 32usize);

    assert!(size_of::<TSTypeName>() == 8usize);
    assert!(align_of::<TSTypeName>() == 4usize);

    assert!(size_of::<TSQualifiedName>() == 64usize);
    assert!(align_of::<TSQualifiedName>() == 8usize);
    assert!(offset_of!(TSQualifiedName, parent) == 0usize);
    assert!(offset_of!(TSQualifiedName, span) == 16usize);
    assert!(offset_of!(TSQualifiedName, left) == 24usize);
    assert!(offset_of!(TSQualifiedName, right) == 32usize);

    assert!(size_of::<TSTypeParameterInstantiation>() == 40usize);
    assert!(align_of::<TSTypeParameterInstantiation>() == 8usize);
    assert!(offset_of!(TSTypeParameterInstantiation, parent) == 0usize);
    assert!(offset_of!(TSTypeParameterInstantiation, span) == 16usize);
    assert!(offset_of!(TSTypeParameterInstantiation, params) == 24usize);

    assert!(size_of::<TSTypeParameter>() == 88usize);
    assert!(align_of::<TSTypeParameter>() == 8usize);
    assert!(offset_of!(TSTypeParameter, parent) == 0usize);
    assert!(offset_of!(TSTypeParameter, span) == 16usize);
    assert!(offset_of!(TSTypeParameter, name) == 24usize);
    assert!(offset_of!(TSTypeParameter, constraint) == 64usize);
    assert!(offset_of!(TSTypeParameter, default) == 72usize);
    assert!(offset_of!(TSTypeParameter, r#in) == 80usize);
    assert!(offset_of!(TSTypeParameter, out) == 81usize);
    assert!(offset_of!(TSTypeParameter, r#const) == 82usize);

    assert!(size_of::<TSTypeParameterDeclaration>() == 40usize);
    assert!(align_of::<TSTypeParameterDeclaration>() == 8usize);
    assert!(offset_of!(TSTypeParameterDeclaration, parent) == 0usize);
    assert!(offset_of!(TSTypeParameterDeclaration, span) == 16usize);
    assert!(offset_of!(TSTypeParameterDeclaration, params) == 24usize);

    assert!(size_of::<TSTypeAliasDeclaration>() == 88usize);
    assert!(align_of::<TSTypeAliasDeclaration>() == 8usize);
    assert!(offset_of!(TSTypeAliasDeclaration, parent) == 0usize);
    assert!(offset_of!(TSTypeAliasDeclaration, span) == 16usize);
    assert!(offset_of!(TSTypeAliasDeclaration, id) == 24usize);
    assert!(offset_of!(TSTypeAliasDeclaration, type_parameters) == 64usize);
    assert!(offset_of!(TSTypeAliasDeclaration, type_annotation) == 68usize);
    assert!(offset_of!(TSTypeAliasDeclaration, declare) == 76usize);
    assert!(offset_of!(TSTypeAliasDeclaration, scope_id) == 80usize);

    assert!(size_of::<TSAccessibility>() == 1usize);
    assert!(align_of::<TSAccessibility>() == 1usize);

    assert!(size_of::<TSClassImplements>() == 40usize);
    assert!(align_of::<TSClassImplements>() == 8usize);
    assert!(offset_of!(TSClassImplements, parent) == 0usize);
    assert!(offset_of!(TSClassImplements, span) == 16usize);
    assert!(offset_of!(TSClassImplements, expression) == 24usize);
    assert!(offset_of!(TSClassImplements, type_parameters) == 32usize);

    assert!(size_of::<TSInterfaceDeclaration>() == 96usize);
    assert!(align_of::<TSInterfaceDeclaration>() == 8usize);
    assert!(offset_of!(TSInterfaceDeclaration, parent) == 0usize);
    assert!(offset_of!(TSInterfaceDeclaration, span) == 16usize);
    assert!(offset_of!(TSInterfaceDeclaration, id) == 24usize);
    assert!(offset_of!(TSInterfaceDeclaration, extends) == 64usize);
    assert!(offset_of!(TSInterfaceDeclaration, type_parameters) == 80usize);
    assert!(offset_of!(TSInterfaceDeclaration, body) == 84usize);
    assert!(offset_of!(TSInterfaceDeclaration, declare) == 88usize);
    assert!(offset_of!(TSInterfaceDeclaration, scope_id) == 92usize);

    assert!(size_of::<TSInterfaceBody>() == 40usize);
    assert!(align_of::<TSInterfaceBody>() == 8usize);
    assert!(offset_of!(TSInterfaceBody, parent) == 0usize);
    assert!(offset_of!(TSInterfaceBody, span) == 16usize);
    assert!(offset_of!(TSInterfaceBody, body) == 24usize);

    assert!(size_of::<TSPropertySignature>() == 40usize);
    assert!(align_of::<TSPropertySignature>() == 8usize);
    assert!(offset_of!(TSPropertySignature, parent) == 0usize);
    assert!(offset_of!(TSPropertySignature, span) == 16usize);
    assert!(offset_of!(TSPropertySignature, computed) == 24usize);
    assert!(offset_of!(TSPropertySignature, optional) == 25usize);
    assert!(offset_of!(TSPropertySignature, readonly) == 26usize);
    assert!(offset_of!(TSPropertySignature, key) == 28usize);
    assert!(offset_of!(TSPropertySignature, type_annotation) == 36usize);

    assert!(size_of::<TSSignature>() == 8usize);
    assert!(align_of::<TSSignature>() == 4usize);

    assert!(size_of::<TSIndexSignature>() == 48usize);
    assert!(align_of::<TSIndexSignature>() == 8usize);
    assert!(offset_of!(TSIndexSignature, parent) == 0usize);
    assert!(offset_of!(TSIndexSignature, span) == 16usize);
    assert!(offset_of!(TSIndexSignature, parameters) == 24usize);
    assert!(offset_of!(TSIndexSignature, type_annotation) == 40usize);
    assert!(offset_of!(TSIndexSignature, readonly) == 44usize);
    assert!(offset_of!(TSIndexSignature, r#static) == 45usize);

    assert!(size_of::<TSCallSignatureDeclaration>() == 80usize);
    assert!(align_of::<TSCallSignatureDeclaration>() == 8usize);
    assert!(offset_of!(TSCallSignatureDeclaration, parent) == 0usize);
    assert!(offset_of!(TSCallSignatureDeclaration, span) == 16usize);
    assert!(offset_of!(TSCallSignatureDeclaration, type_parameters) == 24usize);
    assert!(offset_of!(TSCallSignatureDeclaration, this_param) == 32usize);
    assert!(offset_of!(TSCallSignatureDeclaration, params) == 72usize);
    assert!(offset_of!(TSCallSignatureDeclaration, return_type) == 76usize);

    assert!(size_of::<TSMethodSignatureKind>() == 1usize);
    assert!(align_of::<TSMethodSignatureKind>() == 1usize);

    assert!(size_of::<TSMethodSignature>() == 56usize);
    assert!(align_of::<TSMethodSignature>() == 8usize);
    assert!(offset_of!(TSMethodSignature, parent) == 0usize);
    assert!(offset_of!(TSMethodSignature, span) == 16usize);
    assert!(offset_of!(TSMethodSignature, key) == 24usize);
    assert!(offset_of!(TSMethodSignature, computed) == 32usize);
    assert!(offset_of!(TSMethodSignature, optional) == 33usize);
    assert!(offset_of!(TSMethodSignature, kind) == 34usize);
    assert!(offset_of!(TSMethodSignature, type_parameters) == 36usize);
    assert!(offset_of!(TSMethodSignature, this_param) == 40usize);
    assert!(offset_of!(TSMethodSignature, params) == 44usize);
    assert!(offset_of!(TSMethodSignature, return_type) == 48usize);
    assert!(offset_of!(TSMethodSignature, scope_id) == 52usize);

    assert!(size_of::<TSConstructSignatureDeclaration>() == 40usize);
    assert!(align_of::<TSConstructSignatureDeclaration>() == 8usize);
    assert!(offset_of!(TSConstructSignatureDeclaration, parent) == 0usize);
    assert!(offset_of!(TSConstructSignatureDeclaration, span) == 16usize);
    assert!(offset_of!(TSConstructSignatureDeclaration, type_parameters) == 24usize);
    assert!(offset_of!(TSConstructSignatureDeclaration, params) == 28usize);
    assert!(offset_of!(TSConstructSignatureDeclaration, return_type) == 32usize);
    assert!(offset_of!(TSConstructSignatureDeclaration, scope_id) == 36usize);

    assert!(size_of::<TSIndexSignatureName>() == 40usize);
    assert!(align_of::<TSIndexSignatureName>() == 8usize);
    assert!(offset_of!(TSIndexSignatureName, parent) == 0usize);
    assert!(offset_of!(TSIndexSignatureName, span) == 16usize);
    assert!(offset_of!(TSIndexSignatureName, name) == 24usize);
    assert!(offset_of!(TSIndexSignatureName, type_annotation) == 32usize);

    assert!(size_of::<TSInterfaceHeritage>() == 40usize);
    assert!(align_of::<TSInterfaceHeritage>() == 8usize);
    assert!(offset_of!(TSInterfaceHeritage, parent) == 0usize);
    assert!(offset_of!(TSInterfaceHeritage, span) == 16usize);
    assert!(offset_of!(TSInterfaceHeritage, expression) == 24usize);
    assert!(offset_of!(TSInterfaceHeritage, type_parameters) == 32usize);

    assert!(size_of::<TSTypePredicate>() == 64usize);
    assert!(align_of::<TSTypePredicate>() == 8usize);
    assert!(offset_of!(TSTypePredicate, parent) == 0usize);
    assert!(offset_of!(TSTypePredicate, span) == 16usize);
    assert!(offset_of!(TSTypePredicate, parameter_name) == 24usize);
    assert!(offset_of!(TSTypePredicate, asserts) == 56usize);
    assert!(offset_of!(TSTypePredicate, type_annotation) == 60usize);

    assert!(size_of::<TSTypePredicateName>() == 32usize);
    assert!(align_of::<TSTypePredicateName>() == 8usize);

    assert!(size_of::<TSModuleDeclaration>() == 88usize);
    assert!(align_of::<TSModuleDeclaration>() == 8usize);
    assert!(offset_of!(TSModuleDeclaration, parent) == 0usize);
    assert!(offset_of!(TSModuleDeclaration, span) == 16usize);
    assert!(offset_of!(TSModuleDeclaration, id) == 24usize);
    assert!(offset_of!(TSModuleDeclaration, body) == 72usize);
    assert!(offset_of!(TSModuleDeclaration, kind) == 80usize);
    assert!(offset_of!(TSModuleDeclaration, declare) == 81usize);
    assert!(offset_of!(TSModuleDeclaration, scope_id) == 84usize);

    assert!(size_of::<TSModuleDeclarationKind>() == 1usize);
    assert!(align_of::<TSModuleDeclarationKind>() == 1usize);

    assert!(size_of::<TSModuleDeclarationName>() == 48usize);
    assert!(align_of::<TSModuleDeclarationName>() == 8usize);

    assert!(size_of::<TSModuleDeclarationBody>() == 8usize);
    assert!(align_of::<TSModuleDeclarationBody>() == 4usize);

    assert!(size_of::<TSModuleBlock>() == 56usize);
    assert!(align_of::<TSModuleBlock>() == 8usize);
    assert!(offset_of!(TSModuleBlock, parent) == 0usize);
    assert!(offset_of!(TSModuleBlock, span) == 16usize);
    assert!(offset_of!(TSModuleBlock, directives) == 24usize);
    assert!(offset_of!(TSModuleBlock, body) == 40usize);

    assert!(size_of::<TSTypeLiteral>() == 40usize);
    assert!(align_of::<TSTypeLiteral>() == 8usize);
    assert!(offset_of!(TSTypeLiteral, parent) == 0usize);
    assert!(offset_of!(TSTypeLiteral, span) == 16usize);
    assert!(offset_of!(TSTypeLiteral, members) == 24usize);

    assert!(size_of::<TSInferType>() == 32usize);
    assert!(align_of::<TSInferType>() == 8usize);
    assert!(offset_of!(TSInferType, parent) == 0usize);
    assert!(offset_of!(TSInferType, span) == 16usize);
    assert!(offset_of!(TSInferType, type_parameter) == 24usize);

    assert!(size_of::<TSTypeQuery>() == 40usize);
    assert!(align_of::<TSTypeQuery>() == 8usize);
    assert!(offset_of!(TSTypeQuery, parent) == 0usize);
    assert!(offset_of!(TSTypeQuery, span) == 16usize);
    assert!(offset_of!(TSTypeQuery, expr_name) == 24usize);
    assert!(offset_of!(TSTypeQuery, type_parameters) == 32usize);

    assert!(size_of::<TSTypeQueryExprName>() == 8usize);
    assert!(align_of::<TSTypeQueryExprName>() == 4usize);

    assert!(size_of::<TSImportType>() == 56usize);
    assert!(align_of::<TSImportType>() == 8usize);
    assert!(offset_of!(TSImportType, parent) == 0usize);
    assert!(offset_of!(TSImportType, span) == 16usize);
    assert!(offset_of!(TSImportType, is_type_of) == 24usize);
    assert!(offset_of!(TSImportType, parameter) == 28usize);
    assert!(offset_of!(TSImportType, qualifier) == 36usize);
    assert!(offset_of!(TSImportType, attributes) == 44usize);
    assert!(offset_of!(TSImportType, type_parameters) == 48usize);

    assert!(size_of::<TSImportAttributes>() == 72usize);
    assert!(align_of::<TSImportAttributes>() == 8usize);
    assert!(offset_of!(TSImportAttributes, parent) == 0usize);
    assert!(offset_of!(TSImportAttributes, span) == 16usize);
    assert!(offset_of!(TSImportAttributes, attributes_keyword) == 24usize);
    assert!(offset_of!(TSImportAttributes, elements) == 56usize);

    assert!(size_of::<TSImportAttribute>() == 80usize);
    assert!(align_of::<TSImportAttribute>() == 8usize);
    assert!(offset_of!(TSImportAttribute, parent) == 0usize);
    assert!(offset_of!(TSImportAttribute, span) == 16usize);
    assert!(offset_of!(TSImportAttribute, name) == 24usize);
    assert!(offset_of!(TSImportAttribute, value) == 72usize);

    assert!(size_of::<TSImportAttributeName>() == 48usize);
    assert!(align_of::<TSImportAttributeName>() == 8usize);

    assert!(size_of::<TSFunctionType>() == 40usize);
    assert!(align_of::<TSFunctionType>() == 8usize);
    assert!(offset_of!(TSFunctionType, parent) == 0usize);
    assert!(offset_of!(TSFunctionType, span) == 16usize);
    assert!(offset_of!(TSFunctionType, type_parameters) == 24usize);
    assert!(offset_of!(TSFunctionType, this_param) == 28usize);
    assert!(offset_of!(TSFunctionType, params) == 32usize);
    assert!(offset_of!(TSFunctionType, return_type) == 36usize);

    assert!(size_of::<TSConstructorType>() == 40usize);
    assert!(align_of::<TSConstructorType>() == 8usize);
    assert!(offset_of!(TSConstructorType, parent) == 0usize);
    assert!(offset_of!(TSConstructorType, span) == 16usize);
    assert!(offset_of!(TSConstructorType, r#abstract) == 24usize);
    assert!(offset_of!(TSConstructorType, type_parameters) == 28usize);
    assert!(offset_of!(TSConstructorType, params) == 32usize);
    assert!(offset_of!(TSConstructorType, return_type) == 36usize);

    assert!(size_of::<TSMappedType>() == 56usize);
    assert!(align_of::<TSMappedType>() == 8usize);
    assert!(offset_of!(TSMappedType, parent) == 0usize);
    assert!(offset_of!(TSMappedType, span) == 16usize);
    assert!(offset_of!(TSMappedType, type_parameter) == 24usize);
    assert!(offset_of!(TSMappedType, name_type) == 28usize);
    assert!(offset_of!(TSMappedType, type_annotation) == 36usize);
    assert!(offset_of!(TSMappedType, optional) == 44usize);
    assert!(offset_of!(TSMappedType, readonly) == 45usize);
    assert!(offset_of!(TSMappedType, scope_id) == 48usize);

    assert!(size_of::<TSMappedTypeModifierOperator>() == 1usize);
    assert!(align_of::<TSMappedTypeModifierOperator>() == 1usize);

    assert!(size_of::<TSTemplateLiteralType>() == 56usize);
    assert!(align_of::<TSTemplateLiteralType>() == 8usize);
    assert!(offset_of!(TSTemplateLiteralType, parent) == 0usize);
    assert!(offset_of!(TSTemplateLiteralType, span) == 16usize);
    assert!(offset_of!(TSTemplateLiteralType, quasis) == 24usize);
    assert!(offset_of!(TSTemplateLiteralType, types) == 40usize);

    assert!(size_of::<TSAsExpression>() == 40usize);
    assert!(align_of::<TSAsExpression>() == 8usize);
    assert!(offset_of!(TSAsExpression, parent) == 0usize);
    assert!(offset_of!(TSAsExpression, span) == 16usize);
    assert!(offset_of!(TSAsExpression, expression) == 24usize);
    assert!(offset_of!(TSAsExpression, type_annotation) == 32usize);

    assert!(size_of::<TSSatisfiesExpression>() == 40usize);
    assert!(align_of::<TSSatisfiesExpression>() == 8usize);
    assert!(offset_of!(TSSatisfiesExpression, parent) == 0usize);
    assert!(offset_of!(TSSatisfiesExpression, span) == 16usize);
    assert!(offset_of!(TSSatisfiesExpression, expression) == 24usize);
    assert!(offset_of!(TSSatisfiesExpression, type_annotation) == 32usize);

    assert!(size_of::<TSTypeAssertion>() == 40usize);
    assert!(align_of::<TSTypeAssertion>() == 8usize);
    assert!(offset_of!(TSTypeAssertion, parent) == 0usize);
    assert!(offset_of!(TSTypeAssertion, span) == 16usize);
    assert!(offset_of!(TSTypeAssertion, expression) == 24usize);
    assert!(offset_of!(TSTypeAssertion, type_annotation) == 32usize);

    assert!(size_of::<TSImportEqualsDeclaration>() == 80usize);
    assert!(align_of::<TSImportEqualsDeclaration>() == 8usize);
    assert!(offset_of!(TSImportEqualsDeclaration, parent) == 0usize);
    assert!(offset_of!(TSImportEqualsDeclaration, span) == 16usize);
    assert!(offset_of!(TSImportEqualsDeclaration, id) == 24usize);
    assert!(offset_of!(TSImportEqualsDeclaration, module_reference) == 64usize);
    assert!(offset_of!(TSImportEqualsDeclaration, import_kind) == 72usize);

    assert!(size_of::<TSModuleReference>() == 8usize);
    assert!(align_of::<TSModuleReference>() == 4usize);

    assert!(size_of::<TSExternalModuleReference>() == 64usize);
    assert!(align_of::<TSExternalModuleReference>() == 8usize);
    assert!(offset_of!(TSExternalModuleReference, parent) == 0usize);
    assert!(offset_of!(TSExternalModuleReference, span) == 16usize);
    assert!(offset_of!(TSExternalModuleReference, expression) == 24usize);

    assert!(size_of::<TSNonNullExpression>() == 32usize);
    assert!(align_of::<TSNonNullExpression>() == 8usize);
    assert!(offset_of!(TSNonNullExpression, parent) == 0usize);
    assert!(offset_of!(TSNonNullExpression, span) == 16usize);
    assert!(offset_of!(TSNonNullExpression, expression) == 24usize);

    assert!(size_of::<Decorator>() == 32usize);
    assert!(align_of::<Decorator>() == 8usize);
    assert!(offset_of!(Decorator, parent) == 0usize);
    assert!(offset_of!(Decorator, span) == 16usize);
    assert!(offset_of!(Decorator, expression) == 24usize);

    assert!(size_of::<TSExportAssignment>() == 32usize);
    assert!(align_of::<TSExportAssignment>() == 8usize);
    assert!(offset_of!(TSExportAssignment, parent) == 0usize);
    assert!(offset_of!(TSExportAssignment, span) == 16usize);
    assert!(offset_of!(TSExportAssignment, expression) == 24usize);

    assert!(size_of::<TSNamespaceExportDeclaration>() == 56usize);
    assert!(align_of::<TSNamespaceExportDeclaration>() == 8usize);
    assert!(offset_of!(TSNamespaceExportDeclaration, parent) == 0usize);
    assert!(offset_of!(TSNamespaceExportDeclaration, span) == 16usize);
    assert!(offset_of!(TSNamespaceExportDeclaration, id) == 24usize);

    assert!(size_of::<TSInstantiationExpression>() == 40usize);
    assert!(align_of::<TSInstantiationExpression>() == 8usize);
    assert!(offset_of!(TSInstantiationExpression, parent) == 0usize);
    assert!(offset_of!(TSInstantiationExpression, span) == 16usize);
    assert!(offset_of!(TSInstantiationExpression, expression) == 24usize);
    assert!(offset_of!(TSInstantiationExpression, type_parameters) == 32usize);

    assert!(size_of::<ImportOrExportKind>() == 1usize);
    assert!(align_of::<ImportOrExportKind>() == 1usize);

    assert!(size_of::<JSDocNullableType>() == 40usize);
    assert!(align_of::<JSDocNullableType>() == 8usize);
    assert!(offset_of!(JSDocNullableType, parent) == 0usize);
    assert!(offset_of!(JSDocNullableType, span) == 16usize);
    assert!(offset_of!(JSDocNullableType, type_annotation) == 24usize);
    assert!(offset_of!(JSDocNullableType, postfix) == 32usize);

    assert!(size_of::<JSDocNonNullableType>() == 40usize);
    assert!(align_of::<JSDocNonNullableType>() == 8usize);
    assert!(offset_of!(JSDocNonNullableType, parent) == 0usize);
    assert!(offset_of!(JSDocNonNullableType, span) == 16usize);
    assert!(offset_of!(JSDocNonNullableType, type_annotation) == 24usize);
    assert!(offset_of!(JSDocNonNullableType, postfix) == 32usize);

    assert!(size_of::<JSDocUnknownType>() == 24usize);
    assert!(align_of::<JSDocUnknownType>() == 8usize);
    assert!(offset_of!(JSDocUnknownType, parent) == 0usize);
    assert!(offset_of!(JSDocUnknownType, span) == 16usize);

    assert!(size_of::<JSXElement>() == 48usize);
    assert!(align_of::<JSXElement>() == 8usize);
    assert!(offset_of!(JSXElement, parent) == 0usize);
    assert!(offset_of!(JSXElement, span) == 16usize);
    assert!(offset_of!(JSXElement, opening_element) == 24usize);
    assert!(offset_of!(JSXElement, closing_element) == 28usize);
    assert!(offset_of!(JSXElement, children) == 32usize);

    assert!(size_of::<JSXOpeningElement>() == 56usize);
    assert!(align_of::<JSXOpeningElement>() == 8usize);
    assert!(offset_of!(JSXOpeningElement, parent) == 0usize);
    assert!(offset_of!(JSXOpeningElement, span) == 16usize);
    assert!(offset_of!(JSXOpeningElement, self_closing) == 24usize);
    assert!(offset_of!(JSXOpeningElement, name) == 28usize);
    assert!(offset_of!(JSXOpeningElement, attributes) == 36usize);
    assert!(offset_of!(JSXOpeningElement, type_parameters) == 52usize);

    assert!(size_of::<JSXClosingElement>() == 32usize);
    assert!(align_of::<JSXClosingElement>() == 8usize);
    assert!(offset_of!(JSXClosingElement, parent) == 0usize);
    assert!(offset_of!(JSXClosingElement, span) == 16usize);
    assert!(offset_of!(JSXClosingElement, name) == 24usize);

    assert!(size_of::<JSXFragment>() == 88usize);
    assert!(align_of::<JSXFragment>() == 8usize);
    assert!(offset_of!(JSXFragment, parent) == 0usize);
    assert!(offset_of!(JSXFragment, span) == 16usize);
    assert!(offset_of!(JSXFragment, opening_fragment) == 24usize);
    assert!(offset_of!(JSXFragment, closing_fragment) == 48usize);
    assert!(offset_of!(JSXFragment, children) == 72usize);

    assert!(size_of::<JSXOpeningFragment>() == 24usize);
    assert!(align_of::<JSXOpeningFragment>() == 8usize);
    assert!(offset_of!(JSXOpeningFragment, parent) == 0usize);
    assert!(offset_of!(JSXOpeningFragment, span) == 16usize);

    assert!(size_of::<JSXClosingFragment>() == 24usize);
    assert!(align_of::<JSXClosingFragment>() == 8usize);
    assert!(offset_of!(JSXClosingFragment, parent) == 0usize);
    assert!(offset_of!(JSXClosingFragment, span) == 16usize);

    assert!(size_of::<JSXElementName>() == 8usize);
    assert!(align_of::<JSXElementName>() == 4usize);

    assert!(size_of::<JSXNamespacedName>() == 88usize);
    assert!(align_of::<JSXNamespacedName>() == 8usize);
    assert!(offset_of!(JSXNamespacedName, parent) == 0usize);
    assert!(offset_of!(JSXNamespacedName, span) == 16usize);
    assert!(offset_of!(JSXNamespacedName, namespace) == 24usize);
    assert!(offset_of!(JSXNamespacedName, property) == 56usize);

    assert!(size_of::<JSXMemberExpression>() == 64usize);
    assert!(align_of::<JSXMemberExpression>() == 8usize);
    assert!(offset_of!(JSXMemberExpression, parent) == 0usize);
    assert!(offset_of!(JSXMemberExpression, span) == 16usize);
    assert!(offset_of!(JSXMemberExpression, object) == 24usize);
    assert!(offset_of!(JSXMemberExpression, property) == 32usize);

    assert!(size_of::<JSXMemberExpressionObject>() == 8usize);
    assert!(align_of::<JSXMemberExpressionObject>() == 4usize);

    assert!(size_of::<JSXExpressionContainer>() == 56usize);
    assert!(align_of::<JSXExpressionContainer>() == 8usize);
    assert!(offset_of!(JSXExpressionContainer, parent) == 0usize);
    assert!(offset_of!(JSXExpressionContainer, span) == 16usize);
    assert!(offset_of!(JSXExpressionContainer, expression) == 24usize);

    assert!(size_of::<JSXExpression>() == 32usize);
    assert!(align_of::<JSXExpression>() == 8usize);

    assert!(size_of::<JSXEmptyExpression>() == 24usize);
    assert!(align_of::<JSXEmptyExpression>() == 8usize);
    assert!(offset_of!(JSXEmptyExpression, parent) == 0usize);
    assert!(offset_of!(JSXEmptyExpression, span) == 16usize);

    assert!(size_of::<JSXAttributeItem>() == 8usize);
    assert!(align_of::<JSXAttributeItem>() == 4usize);

    assert!(size_of::<JSXAttribute>() == 40usize);
    assert!(align_of::<JSXAttribute>() == 8usize);
    assert!(offset_of!(JSXAttribute, parent) == 0usize);
    assert!(offset_of!(JSXAttribute, span) == 16usize);
    assert!(offset_of!(JSXAttribute, name) == 24usize);
    assert!(offset_of!(JSXAttribute, value) == 32usize);

    assert!(size_of::<JSXSpreadAttribute>() == 32usize);
    assert!(align_of::<JSXSpreadAttribute>() == 8usize);
    assert!(offset_of!(JSXSpreadAttribute, parent) == 0usize);
    assert!(offset_of!(JSXSpreadAttribute, span) == 16usize);
    assert!(offset_of!(JSXSpreadAttribute, argument) == 24usize);

    assert!(size_of::<JSXAttributeName>() == 8usize);
    assert!(align_of::<JSXAttributeName>() == 4usize);

    assert!(size_of::<JSXAttributeValue>() == 8usize);
    assert!(align_of::<JSXAttributeValue>() == 4usize);

    assert!(size_of::<JSXIdentifier>() == 32usize);
    assert!(align_of::<JSXIdentifier>() == 8usize);
    assert!(offset_of!(JSXIdentifier, parent) == 0usize);
    assert!(offset_of!(JSXIdentifier, span) == 16usize);
    assert!(offset_of!(JSXIdentifier, name) == 24usize);

    assert!(size_of::<JSXChild>() == 8usize);
    assert!(align_of::<JSXChild>() == 4usize);

    assert!(size_of::<JSXSpreadChild>() == 32usize);
    assert!(align_of::<JSXSpreadChild>() == 8usize);
    assert!(offset_of!(JSXSpreadChild, parent) == 0usize);
    assert!(offset_of!(JSXSpreadChild, span) == 16usize);
    assert!(offset_of!(JSXSpreadChild, expression) == 24usize);

    assert!(size_of::<JSXText>() == 32usize);
    assert!(align_of::<JSXText>() == 8usize);
    assert!(offset_of!(JSXText, parent) == 0usize);
    assert!(offset_of!(JSXText, span) == 16usize);
    assert!(offset_of!(JSXText, value) == 24usize);

    assert!(size_of::<CommentKind>() == 1usize);
    assert!(align_of::<CommentKind>() == 1usize);

    assert!(size_of::<CommentPosition>() == 1usize);
    assert!(align_of::<CommentPosition>() == 1usize);

    assert!(size_of::<Comment>() == 16usize);
    assert!(align_of::<Comment>() == 4usize);
    assert!(offset_of!(Comment, span) == 0usize);
    assert!(offset_of!(Comment, attached_to) == 8usize);
    assert!(offset_of!(Comment, kind) == 12usize);
    assert!(offset_of!(Comment, position) == 13usize);
    assert!(offset_of!(Comment, preceded_by_newline) == 14usize);
    assert!(offset_of!(Comment, followed_by_newline) == 15usize);

    assert!(size_of::<NumberBase>() == 1usize);
    assert!(align_of::<NumberBase>() == 1usize);

    assert!(size_of::<BigintBase>() == 1usize);
    assert!(align_of::<BigintBase>() == 1usize);

    assert!(size_of::<AssignmentOperator>() == 1usize);
    assert!(align_of::<AssignmentOperator>() == 1usize);

    assert!(size_of::<BinaryOperator>() == 1usize);
    assert!(align_of::<BinaryOperator>() == 1usize);

    assert!(size_of::<LogicalOperator>() == 1usize);
    assert!(align_of::<LogicalOperator>() == 1usize);

    assert!(size_of::<UnaryOperator>() == 1usize);
    assert!(align_of::<UnaryOperator>() == 1usize);

    assert!(size_of::<UpdateOperator>() == 1usize);
    assert!(align_of::<UpdateOperator>() == 1usize);

    assert!(size_of::<Span>() == 8usize);
    assert!(align_of::<Span>() == 4usize);
    assert!(offset_of!(Span, start) == 0usize);
    assert!(offset_of!(Span, end) == 4usize);

    assert!(size_of::<SourceType>() == 3usize);
    assert!(align_of::<SourceType>() == 1usize);

    assert!(size_of::<Language>() == 1usize);
    assert!(align_of::<Language>() == 1usize);

    assert!(size_of::<ModuleKind>() == 1usize);
    assert!(align_of::<ModuleKind>() == 1usize);

    assert!(size_of::<LanguageVariant>() == 1usize);
    assert!(align_of::<LanguageVariant>() == 1usize);

    assert!(size_of::<Pattern>() == 32usize);
    assert!(align_of::<Pattern>() == 4usize);
    assert!(offset_of!(Pattern, span) == 0usize);
    assert!(offset_of!(Pattern, body) == 8usize);

    assert!(size_of::<Disjunction>() == 24usize);
    assert!(align_of::<Disjunction>() == 4usize);
    assert!(offset_of!(Disjunction, span) == 0usize);
    assert!(offset_of!(Disjunction, body) == 8usize);

    assert!(size_of::<Alternative>() == 24usize);
    assert!(align_of::<Alternative>() == 4usize);
    assert!(offset_of!(Alternative, span) == 0usize);
    assert!(offset_of!(Alternative, body) == 8usize);

    assert!(size_of::<Term>() == 12usize);
    assert!(align_of::<Term>() == 4usize);

    assert!(size_of::<BoundaryAssertion>() == 12usize);
    assert!(align_of::<BoundaryAssertion>() == 4usize);
    assert!(offset_of!(BoundaryAssertion, span) == 0usize);
    assert!(offset_of!(BoundaryAssertion, kind) == 8usize);

    assert!(size_of::<BoundaryAssertionKind>() == 1usize);
    assert!(align_of::<BoundaryAssertionKind>() == 1usize);

    assert!(size_of::<LookAroundAssertion>() == 36usize);
    assert!(align_of::<LookAroundAssertion>() == 4usize);
    assert!(offset_of!(LookAroundAssertion, span) == 0usize);
    assert!(offset_of!(LookAroundAssertion, kind) == 8usize);
    assert!(offset_of!(LookAroundAssertion, body) == 12usize);

    assert!(size_of::<LookAroundAssertionKind>() == 1usize);
    assert!(align_of::<LookAroundAssertionKind>() == 1usize);

    assert!(size_of::<Quantifier>() == 48usize);
    assert!(align_of::<Quantifier>() == 8usize);
    assert!(offset_of!(Quantifier, span) == 0usize);
    assert!(offset_of!(Quantifier, min) == 8usize);
    assert!(offset_of!(Quantifier, max) == 16usize);
    assert!(offset_of!(Quantifier, greedy) == 32usize);
    assert!(offset_of!(Quantifier, body) == 36usize);

    assert!(size_of::<Character>() == 16usize);
    assert!(align_of::<Character>() == 4usize);
    assert!(offset_of!(Character, span) == 0usize);
    assert!(offset_of!(Character, kind) == 8usize);
    assert!(offset_of!(Character, value) == 12usize);

    assert!(size_of::<CharacterKind>() == 1usize);
    assert!(align_of::<CharacterKind>() == 1usize);

    assert!(size_of::<CharacterClassEscape>() == 12usize);
    assert!(align_of::<CharacterClassEscape>() == 4usize);
    assert!(offset_of!(CharacterClassEscape, span) == 0usize);
    assert!(offset_of!(CharacterClassEscape, kind) == 8usize);

    assert!(size_of::<CharacterClassEscapeKind>() == 1usize);
    assert!(align_of::<CharacterClassEscapeKind>() == 1usize);

    assert!(size_of::<UnicodePropertyEscape>() == 28usize);
    assert!(align_of::<UnicodePropertyEscape>() == 4usize);
    assert!(offset_of!(UnicodePropertyEscape, span) == 0usize);
    assert!(offset_of!(UnicodePropertyEscape, negative) == 8usize);
    assert!(offset_of!(UnicodePropertyEscape, strings) == 9usize);
    assert!(offset_of!(UnicodePropertyEscape, name) == 12usize);
    assert!(offset_of!(UnicodePropertyEscape, value) == 20usize);

    assert!(size_of::<Dot>() == 8usize);
    assert!(align_of::<Dot>() == 4usize);
    assert!(offset_of!(Dot, span) == 0usize);

    assert!(size_of::<CharacterClass>() == 28usize);
    assert!(align_of::<CharacterClass>() == 4usize);
    assert!(offset_of!(CharacterClass, span) == 0usize);
    assert!(offset_of!(CharacterClass, negative) == 8usize);
    assert!(offset_of!(CharacterClass, strings) == 9usize);
    assert!(offset_of!(CharacterClass, kind) == 10usize);
    assert!(offset_of!(CharacterClass, body) == 12usize);

    assert!(size_of::<CharacterClassContentsKind>() == 1usize);
    assert!(align_of::<CharacterClassContentsKind>() == 1usize);

    assert!(size_of::<CharacterClassContents>() == 8usize);
    assert!(align_of::<CharacterClassContents>() == 4usize);

    assert!(size_of::<CharacterClassRange>() == 40usize);
    assert!(align_of::<CharacterClassRange>() == 4usize);
    assert!(offset_of!(CharacterClassRange, span) == 0usize);
    assert!(offset_of!(CharacterClassRange, min) == 8usize);
    assert!(offset_of!(CharacterClassRange, max) == 24usize);

    assert!(size_of::<ClassStringDisjunction>() == 28usize);
    assert!(align_of::<ClassStringDisjunction>() == 4usize);
    assert!(offset_of!(ClassStringDisjunction, span) == 0usize);
    assert!(offset_of!(ClassStringDisjunction, strings) == 8usize);
    assert!(offset_of!(ClassStringDisjunction, body) == 12usize);

    assert!(size_of::<ClassString>() == 28usize);
    assert!(align_of::<ClassString>() == 4usize);
    assert!(offset_of!(ClassString, span) == 0usize);
    assert!(offset_of!(ClassString, strings) == 8usize);
    assert!(offset_of!(ClassString, body) == 12usize);

    assert!(size_of::<CapturingGroup>() == 40usize);
    assert!(align_of::<CapturingGroup>() == 4usize);
    assert!(offset_of!(CapturingGroup, span) == 0usize);
    assert!(offset_of!(CapturingGroup, name) == 8usize);
    assert!(offset_of!(CapturingGroup, body) == 16usize);

    assert!(size_of::<IgnoreGroup>() == 48usize);
    assert!(align_of::<IgnoreGroup>() == 4usize);
    assert!(offset_of!(IgnoreGroup, span) == 0usize);
    assert!(offset_of!(IgnoreGroup, modifiers) == 8usize);
    assert!(offset_of!(IgnoreGroup, body) == 24usize);

    assert!(size_of::<Modifiers>() == 16usize);
    assert!(align_of::<Modifiers>() == 4usize);
    assert!(offset_of!(Modifiers, span) == 0usize);
    assert!(offset_of!(Modifiers, enabling) == 8usize);
    assert!(offset_of!(Modifiers, disabling) == 11usize);

    assert!(size_of::<Modifier>() == 3usize);
    assert!(align_of::<Modifier>() == 1usize);
    assert!(offset_of!(Modifier, ignore_case) == 0usize);
    assert!(offset_of!(Modifier, multiline) == 1usize);
    assert!(offset_of!(Modifier, sticky) == 2usize);

    assert!(size_of::<IndexedReference>() == 12usize);
    assert!(align_of::<IndexedReference>() == 4usize);
    assert!(offset_of!(IndexedReference, span) == 0usize);
    assert!(offset_of!(IndexedReference, index) == 8usize);

    assert!(size_of::<NamedReference>() == 16usize);
    assert!(align_of::<NamedReference>() == 4usize);
    assert!(offset_of!(NamedReference, span) == 0usize);
    assert!(offset_of!(NamedReference, name) == 8usize);
};

#[cfg(not(any(target_pointer_width = "64", target_pointer_width = "32")))]
const _: () = panic!("Platforms with pointer width other than 64 or 32 bit are not supported");
