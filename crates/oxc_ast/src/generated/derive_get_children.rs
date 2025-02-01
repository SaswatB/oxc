// Auto-generated code, DO NOT EDIT DIRECTLY!
// To edit this generated file you have to edit `tasks/ast_tools/src/derives/get_children.rs`

#![allow(clippy::match_same_arms)]
#![allow(unused_variables)]

use crate::AstKind;
pub use crate::GetChildren;

use crate::ast::js::*;

use crate::ast::jsx::*;

use crate::ast::literal::*;

use crate::ast::ts::*;

impl<'a> GetChildren<'a> for BooleanLiteral {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetChildren<'a> for NullLiteral {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetChildren<'a> for NumericLiteral<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetChildren<'a> for StringLiteral<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetChildren<'a> for BigIntLiteral<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetChildren<'a> for RegExpLiteral<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetChildren<'a> for Program<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        if let Some(field) = &self.hashbang {
            children.push(AstKind::Hashbang(field));
        }
        for item in &self.directives {
            children.push(AstKind::Directive(item));
        }
        for item in &self.body {
            children.push(AstKind::from_statement(item));
        }
        children
    }
}

impl<'a> GetChildren<'a> for Expression<'a> {
    #[allow(unused_variables, clippy::match_same_arms)]
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        match self {
            Self::BooleanLiteral(_) => vec![],
            Self::NullLiteral(_) => vec![],
            Self::NumericLiteral(_) => vec![],
            Self::BigIntLiteral(_) => vec![],
            Self::RegExpLiteral(_) => vec![],
            Self::StringLiteral(_) => vec![],
            Self::TemplateLiteral(_) => vec![],
            Self::Identifier(_) => vec![],
            Self::MetaProperty(_) => vec![],
            Self::Super(_) => vec![],
            Self::ArrayExpression(_) => vec![],
            Self::ArrowFunctionExpression(_) => vec![],
            Self::AssignmentExpression(_) => vec![],
            Self::AwaitExpression(_) => vec![],
            Self::BinaryExpression(_) => vec![],
            Self::CallExpression(_) => vec![],
            Self::ChainExpression(_) => vec![],
            Self::ClassExpression(_) => vec![],
            Self::ConditionalExpression(_) => vec![],
            Self::FunctionExpression(_) => vec![],
            Self::ImportExpression(_) => vec![],
            Self::LogicalExpression(_) => vec![],
            Self::NewExpression(_) => vec![],
            Self::ObjectExpression(_) => vec![],
            Self::ParenthesizedExpression(_) => vec![],
            Self::SequenceExpression(_) => vec![],
            Self::TaggedTemplateExpression(_) => vec![],
            Self::ThisExpression(_) => vec![],
            Self::UnaryExpression(_) => vec![],
            Self::UpdateExpression(_) => vec![],
            Self::YieldExpression(_) => vec![],
            Self::PrivateInExpression(_) => vec![],
            Self::JSXElement(_) => vec![],
            Self::JSXFragment(_) => vec![],
            Self::TSAsExpression(_) => vec![],
            Self::TSSatisfiesExpression(_) => vec![],
            Self::TSTypeAssertion(_) => vec![],
            Self::TSNonNullExpression(_) => vec![],
            Self::TSInstantiationExpression(_) => vec![],
            Self::ComputedMemberExpression(_) => vec![],
            Self::StaticMemberExpression(_) => vec![],
            Self::PrivateFieldExpression(_) => vec![],
        }
    }
}

impl<'a> GetChildren<'a> for IdentifierName<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetChildren<'a> for IdentifierReference<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetChildren<'a> for BindingIdentifier<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetChildren<'a> for LabelIdentifier<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetChildren<'a> for ThisExpression {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetChildren<'a> for ArrayExpression<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        for item in &self.elements {
            children.push(AstKind::ArrayExpressionElement(item));
        }
        children
    }
}

impl<'a> GetChildren<'a> for ArrayExpressionElement<'a> {
    #[allow(unused_variables, clippy::match_same_arms)]
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        match self {
            Self::SpreadElement(_) => vec![],
            Self::Elision(child) => vec![AstKind::Elision(child)],
            Self::BooleanLiteral(_) => vec![],
            Self::NullLiteral(_) => vec![],
            Self::NumericLiteral(_) => vec![],
            Self::BigIntLiteral(_) => vec![],
            Self::RegExpLiteral(_) => vec![],
            Self::StringLiteral(_) => vec![],
            Self::TemplateLiteral(_) => vec![],
            Self::Identifier(_) => vec![],
            Self::MetaProperty(_) => vec![],
            Self::Super(_) => vec![],
            Self::ArrayExpression(_) => vec![],
            Self::ArrowFunctionExpression(_) => vec![],
            Self::AssignmentExpression(_) => vec![],
            Self::AwaitExpression(_) => vec![],
            Self::BinaryExpression(_) => vec![],
            Self::CallExpression(_) => vec![],
            Self::ChainExpression(_) => vec![],
            Self::ClassExpression(_) => vec![],
            Self::ConditionalExpression(_) => vec![],
            Self::FunctionExpression(_) => vec![],
            Self::ImportExpression(_) => vec![],
            Self::LogicalExpression(_) => vec![],
            Self::NewExpression(_) => vec![],
            Self::ObjectExpression(_) => vec![],
            Self::ParenthesizedExpression(_) => vec![],
            Self::SequenceExpression(_) => vec![],
            Self::TaggedTemplateExpression(_) => vec![],
            Self::ThisExpression(_) => vec![],
            Self::UnaryExpression(_) => vec![],
            Self::UpdateExpression(_) => vec![],
            Self::YieldExpression(_) => vec![],
            Self::PrivateInExpression(_) => vec![],
            Self::JSXElement(_) => vec![],
            Self::JSXFragment(_) => vec![],
            Self::TSAsExpression(_) => vec![],
            Self::TSSatisfiesExpression(_) => vec![],
            Self::TSTypeAssertion(_) => vec![],
            Self::TSNonNullExpression(_) => vec![],
            Self::TSInstantiationExpression(_) => vec![],
            Self::ComputedMemberExpression(_) => vec![],
            Self::StaticMemberExpression(_) => vec![],
            Self::PrivateFieldExpression(_) => vec![],
        }
    }
}

impl<'a> GetChildren<'a> for Elision {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetChildren<'a> for ObjectExpression<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        for item in &self.properties {
            children.push(AstKind::from_object_property_kind(item));
        }
        children
    }
}

impl<'a> GetChildren<'a> for ObjectPropertyKind<'a> {
    #[allow(unused_variables, clippy::match_same_arms)]
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        match self {
            Self::ObjectProperty(_) => vec![],
            Self::SpreadProperty(_) => vec![],
        }
    }
}

impl<'a> GetChildren<'a> for ObjectProperty<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::PropertyKey(&self.key));
        children.push(AstKind::from_expression(&self.value));
        children
    }
}

impl<'a> GetChildren<'a> for PropertyKey<'a> {
    #[allow(unused_variables, clippy::match_same_arms)]
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        match self {
            Self::StaticIdentifier(_) => vec![],
            Self::PrivateIdentifier(_) => vec![],
            Self::BooleanLiteral(_) => vec![],
            Self::NullLiteral(_) => vec![],
            Self::NumericLiteral(_) => vec![],
            Self::BigIntLiteral(_) => vec![],
            Self::RegExpLiteral(_) => vec![],
            Self::StringLiteral(_) => vec![],
            Self::TemplateLiteral(_) => vec![],
            Self::Identifier(_) => vec![],
            Self::MetaProperty(_) => vec![],
            Self::Super(_) => vec![],
            Self::ArrayExpression(_) => vec![],
            Self::ArrowFunctionExpression(_) => vec![],
            Self::AssignmentExpression(_) => vec![],
            Self::AwaitExpression(_) => vec![],
            Self::BinaryExpression(_) => vec![],
            Self::CallExpression(_) => vec![],
            Self::ChainExpression(_) => vec![],
            Self::ClassExpression(_) => vec![],
            Self::ConditionalExpression(_) => vec![],
            Self::FunctionExpression(_) => vec![],
            Self::ImportExpression(_) => vec![],
            Self::LogicalExpression(_) => vec![],
            Self::NewExpression(_) => vec![],
            Self::ObjectExpression(_) => vec![],
            Self::ParenthesizedExpression(_) => vec![],
            Self::SequenceExpression(_) => vec![],
            Self::TaggedTemplateExpression(_) => vec![],
            Self::ThisExpression(_) => vec![],
            Self::UnaryExpression(_) => vec![],
            Self::UpdateExpression(_) => vec![],
            Self::YieldExpression(_) => vec![],
            Self::PrivateInExpression(_) => vec![],
            Self::JSXElement(_) => vec![],
            Self::JSXFragment(_) => vec![],
            Self::TSAsExpression(_) => vec![],
            Self::TSSatisfiesExpression(_) => vec![],
            Self::TSTypeAssertion(_) => vec![],
            Self::TSNonNullExpression(_) => vec![],
            Self::TSInstantiationExpression(_) => vec![],
            Self::ComputedMemberExpression(_) => vec![],
            Self::StaticMemberExpression(_) => vec![],
            Self::PrivateFieldExpression(_) => vec![],
        }
    }
}

impl<'a> GetChildren<'a> for TemplateLiteral<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        for item in &self.quasis {
            children.push(AstKind::TemplateElement(item));
        }
        for item in &self.expressions {
            children.push(AstKind::from_expression(item));
        }
        children
    }
}

impl<'a> GetChildren<'a> for TaggedTemplateExpression<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_expression(&self.tag));
        children.push(AstKind::TemplateLiteral(&self.quasi));
        if let Some(field) = &self.type_parameters {
            children.push(AstKind::TSTypeParameterInstantiation(field));
        }
        children
    }
}

impl<'a> GetChildren<'a> for TemplateElement<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetChildren<'a> for MemberExpression<'a> {
    #[allow(unused_variables, clippy::match_same_arms)]
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        match self {
            Self::ComputedMemberExpression(_) => vec![],
            Self::StaticMemberExpression(_) => vec![],
            Self::PrivateFieldExpression(_) => vec![],
        }
    }
}

impl<'a> GetChildren<'a> for ComputedMemberExpression<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_expression(&self.object));
        children.push(AstKind::from_expression(&self.expression));
        children
    }
}

impl<'a> GetChildren<'a> for StaticMemberExpression<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_expression(&self.object));
        children.push(AstKind::IdentifierName(&self.property));
        children
    }
}

impl<'a> GetChildren<'a> for PrivateFieldExpression<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_expression(&self.object));
        children.push(AstKind::PrivateIdentifier(&self.field));
        children
    }
}

impl<'a> GetChildren<'a> for CallExpression<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_expression(&self.callee));
        if let Some(field) = &self.type_parameters {
            children.push(AstKind::TSTypeParameterInstantiation(field));
        }
        for item in &self.arguments {
            children.push(AstKind::Argument(item));
        }
        children
    }
}

impl<'a> GetChildren<'a> for NewExpression<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_expression(&self.callee));
        for item in &self.arguments {
            children.push(AstKind::Argument(item));
        }
        if let Some(field) = &self.type_parameters {
            children.push(AstKind::TSTypeParameterInstantiation(field));
        }
        children
    }
}

impl<'a> GetChildren<'a> for MetaProperty<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::IdentifierName(&self.meta));
        children.push(AstKind::IdentifierName(&self.property));
        children
    }
}

impl<'a> GetChildren<'a> for SpreadElement<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_expression(&self.argument));
        children
    }
}

impl<'a> GetChildren<'a> for Argument<'a> {
    #[allow(unused_variables, clippy::match_same_arms)]
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        match self {
            Self::SpreadElement(_) => vec![],
            Self::BooleanLiteral(_) => vec![],
            Self::NullLiteral(_) => vec![],
            Self::NumericLiteral(_) => vec![],
            Self::BigIntLiteral(_) => vec![],
            Self::RegExpLiteral(_) => vec![],
            Self::StringLiteral(_) => vec![],
            Self::TemplateLiteral(_) => vec![],
            Self::Identifier(_) => vec![],
            Self::MetaProperty(_) => vec![],
            Self::Super(_) => vec![],
            Self::ArrayExpression(_) => vec![],
            Self::ArrowFunctionExpression(_) => vec![],
            Self::AssignmentExpression(_) => vec![],
            Self::AwaitExpression(_) => vec![],
            Self::BinaryExpression(_) => vec![],
            Self::CallExpression(_) => vec![],
            Self::ChainExpression(_) => vec![],
            Self::ClassExpression(_) => vec![],
            Self::ConditionalExpression(_) => vec![],
            Self::FunctionExpression(_) => vec![],
            Self::ImportExpression(_) => vec![],
            Self::LogicalExpression(_) => vec![],
            Self::NewExpression(_) => vec![],
            Self::ObjectExpression(_) => vec![],
            Self::ParenthesizedExpression(_) => vec![],
            Self::SequenceExpression(_) => vec![],
            Self::TaggedTemplateExpression(_) => vec![],
            Self::ThisExpression(_) => vec![],
            Self::UnaryExpression(_) => vec![],
            Self::UpdateExpression(_) => vec![],
            Self::YieldExpression(_) => vec![],
            Self::PrivateInExpression(_) => vec![],
            Self::JSXElement(_) => vec![],
            Self::JSXFragment(_) => vec![],
            Self::TSAsExpression(_) => vec![],
            Self::TSSatisfiesExpression(_) => vec![],
            Self::TSTypeAssertion(_) => vec![],
            Self::TSNonNullExpression(_) => vec![],
            Self::TSInstantiationExpression(_) => vec![],
            Self::ComputedMemberExpression(_) => vec![],
            Self::StaticMemberExpression(_) => vec![],
            Self::PrivateFieldExpression(_) => vec![],
        }
    }
}

impl<'a> GetChildren<'a> for UpdateExpression<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::SimpleAssignmentTarget(&self.argument));
        children
    }
}

impl<'a> GetChildren<'a> for UnaryExpression<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_expression(&self.argument));
        children
    }
}

impl<'a> GetChildren<'a> for BinaryExpression<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_expression(&self.left));
        children.push(AstKind::from_expression(&self.right));
        children
    }
}

impl<'a> GetChildren<'a> for PrivateInExpression<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::PrivateIdentifier(&self.left));
        children.push(AstKind::from_expression(&self.right));
        children
    }
}

impl<'a> GetChildren<'a> for LogicalExpression<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_expression(&self.left));
        children.push(AstKind::from_expression(&self.right));
        children
    }
}

impl<'a> GetChildren<'a> for ConditionalExpression<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_expression(&self.test));
        children.push(AstKind::from_expression(&self.consequent));
        children.push(AstKind::from_expression(&self.alternate));
        children
    }
}

impl<'a> GetChildren<'a> for AssignmentExpression<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::AssignmentTarget(&self.left));
        children.push(AstKind::from_expression(&self.right));
        children
    }
}

impl<'a> GetChildren<'a> for AssignmentTarget<'a> {
    #[allow(unused_variables, clippy::match_same_arms)]
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        match self {
            Self::AssignmentTargetIdentifier(_) => vec![],
            Self::TSAsExpression(_) => vec![],
            Self::TSSatisfiesExpression(_) => vec![],
            Self::TSNonNullExpression(_) => vec![],
            Self::TSTypeAssertion(_) => vec![],
            Self::TSInstantiationExpression(_) => vec![],
            Self::ComputedMemberExpression(_) => vec![],
            Self::StaticMemberExpression(_) => vec![],
            Self::PrivateFieldExpression(_) => vec![],
            Self::ArrayAssignmentTarget(_) => vec![],
            Self::ObjectAssignmentTarget(_) => vec![],
        }
    }
}

impl<'a> GetChildren<'a> for SimpleAssignmentTarget<'a> {
    #[allow(unused_variables, clippy::match_same_arms)]
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        match self {
            Self::AssignmentTargetIdentifier(_) => vec![],
            Self::TSAsExpression(_) => vec![],
            Self::TSSatisfiesExpression(_) => vec![],
            Self::TSNonNullExpression(_) => vec![],
            Self::TSTypeAssertion(_) => vec![],
            Self::TSInstantiationExpression(_) => vec![],
            Self::ComputedMemberExpression(_) => vec![],
            Self::StaticMemberExpression(_) => vec![],
            Self::PrivateFieldExpression(_) => vec![],
        }
    }
}

impl<'a> GetChildren<'a> for AssignmentTargetPattern<'a> {
    #[allow(unused_variables, clippy::match_same_arms)]
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        match self {
            Self::ArrayAssignmentTarget(_) => vec![],
            Self::ObjectAssignmentTarget(_) => vec![],
        }
    }
}

impl<'a> GetChildren<'a> for ArrayAssignmentTarget<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        for opt_item in &self.elements {
            if let Some(item) = opt_item {
                children.push(AstKind::from_assignment_target_maybe_default(item));
            }
        }
        if let Some(field) = &self.rest {
            children.push(AstKind::AssignmentTargetRest(field));
        }
        children
    }
}

impl<'a> GetChildren<'a> for ObjectAssignmentTarget<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        for item in &self.properties {
            children.push(AstKind::from_assignment_target_property(item));
        }
        if let Some(field) = &self.rest {
            children.push(AstKind::AssignmentTargetRest(field));
        }
        children
    }
}

impl<'a> GetChildren<'a> for AssignmentTargetRest<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::AssignmentTarget(&self.target));
        children
    }
}

impl<'a> GetChildren<'a> for AssignmentTargetMaybeDefault<'a> {
    #[allow(unused_variables, clippy::match_same_arms)]
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        match self {
            Self::AssignmentTargetWithDefault(_) => vec![],
            Self::AssignmentTargetIdentifier(_) => vec![],
            Self::TSAsExpression(_) => vec![],
            Self::TSSatisfiesExpression(_) => vec![],
            Self::TSNonNullExpression(_) => vec![],
            Self::TSTypeAssertion(_) => vec![],
            Self::TSInstantiationExpression(_) => vec![],
            Self::ComputedMemberExpression(_) => vec![],
            Self::StaticMemberExpression(_) => vec![],
            Self::PrivateFieldExpression(_) => vec![],
            Self::ArrayAssignmentTarget(_) => vec![],
            Self::ObjectAssignmentTarget(_) => vec![],
        }
    }
}

impl<'a> GetChildren<'a> for AssignmentTargetWithDefault<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::AssignmentTarget(&self.binding));
        children.push(AstKind::from_expression(&self.init));
        children
    }
}

impl<'a> GetChildren<'a> for AssignmentTargetProperty<'a> {
    #[allow(unused_variables, clippy::match_same_arms)]
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        match self {
            Self::AssignmentTargetPropertyIdentifier(_) => vec![],
            Self::AssignmentTargetPropertyProperty(_) => vec![],
        }
    }
}

impl<'a> GetChildren<'a> for AssignmentTargetPropertyIdentifier<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::IdentifierReference(&self.binding));
        if let Some(field) = &self.init {
            children.push(AstKind::from_expression(field));
        }
        children
    }
}

impl<'a> GetChildren<'a> for AssignmentTargetPropertyProperty<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::PropertyKey(&self.name));
        children.push(AstKind::from_assignment_target_maybe_default(&self.binding));
        children
    }
}

impl<'a> GetChildren<'a> for SequenceExpression<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        for item in &self.expressions {
            children.push(AstKind::from_expression(item));
        }
        children
    }
}

impl<'a> GetChildren<'a> for Super {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetChildren<'a> for AwaitExpression<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_expression(&self.argument));
        children
    }
}

impl<'a> GetChildren<'a> for ChainExpression<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_chain_element(&self.expression));
        children
    }
}

impl<'a> GetChildren<'a> for ChainElement<'a> {
    #[allow(unused_variables, clippy::match_same_arms)]
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        match self {
            Self::CallExpression(_) => vec![],
            Self::TSNonNullExpression(_) => vec![],
            Self::ComputedMemberExpression(_) => vec![],
            Self::StaticMemberExpression(_) => vec![],
            Self::PrivateFieldExpression(_) => vec![],
        }
    }
}

impl<'a> GetChildren<'a> for ParenthesizedExpression<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_expression(&self.expression));
        children
    }
}

impl<'a> GetChildren<'a> for Statement<'a> {
    #[allow(unused_variables, clippy::match_same_arms)]
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        match self {
            Self::BlockStatement(_) => vec![],
            Self::BreakStatement(_) => vec![],
            Self::ContinueStatement(_) => vec![],
            Self::DebuggerStatement(_) => vec![],
            Self::DoWhileStatement(_) => vec![],
            Self::EmptyStatement(_) => vec![],
            Self::ExpressionStatement(_) => vec![],
            Self::ForInStatement(_) => vec![],
            Self::ForOfStatement(_) => vec![],
            Self::ForStatement(_) => vec![],
            Self::IfStatement(_) => vec![],
            Self::LabeledStatement(_) => vec![],
            Self::ReturnStatement(_) => vec![],
            Self::SwitchStatement(_) => vec![],
            Self::ThrowStatement(_) => vec![],
            Self::TryStatement(_) => vec![],
            Self::WhileStatement(_) => vec![],
            Self::WithStatement(_) => vec![],
            Self::VariableDeclaration(_) => vec![],
            Self::FunctionDeclaration(_) => vec![],
            Self::ClassDeclaration(_) => vec![],
            Self::TSTypeAliasDeclaration(_) => vec![],
            Self::TSInterfaceDeclaration(_) => vec![],
            Self::TSEnumDeclaration(_) => vec![],
            Self::TSModuleDeclaration(_) => vec![],
            Self::TSImportEqualsDeclaration(_) => vec![],
            Self::ImportDeclaration(_) => vec![],
            Self::ExportAllDeclaration(_) => vec![],
            Self::ExportDefaultDeclaration(_) => vec![],
            Self::ExportNamedDeclaration(_) => vec![],
            Self::TSExportAssignment(_) => vec![],
            Self::TSNamespaceExportDeclaration(_) => vec![],
        }
    }
}

impl<'a> GetChildren<'a> for Directive<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::StringLiteral(&self.expression));
        children
    }
}

impl<'a> GetChildren<'a> for Hashbang<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetChildren<'a> for BlockStatement<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        for item in &self.body {
            children.push(AstKind::from_statement(item));
        }
        children
    }
}

impl<'a> GetChildren<'a> for Declaration<'a> {
    #[allow(unused_variables, clippy::match_same_arms)]
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        match self {
            Self::VariableDeclaration(_) => vec![],
            Self::FunctionDeclaration(_) => vec![],
            Self::ClassDeclaration(_) => vec![],
            Self::TSTypeAliasDeclaration(_) => vec![],
            Self::TSInterfaceDeclaration(_) => vec![],
            Self::TSEnumDeclaration(_) => vec![],
            Self::TSModuleDeclaration(_) => vec![],
            Self::TSImportEqualsDeclaration(_) => vec![],
        }
    }
}

impl<'a> GetChildren<'a> for VariableDeclaration<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        for item in &self.declarations {
            children.push(AstKind::VariableDeclarator(item));
        }
        children
    }
}

impl<'a> GetChildren<'a> for VariableDeclarator<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::BindingPattern(&self.id));
        if let Some(field) = &self.init {
            children.push(AstKind::from_expression(field));
        }
        children
    }
}

impl<'a> GetChildren<'a> for EmptyStatement {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetChildren<'a> for ExpressionStatement<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_expression(&self.expression));
        children
    }
}

impl<'a> GetChildren<'a> for IfStatement<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_expression(&self.test));
        children.push(AstKind::from_statement(&self.consequent));
        if let Some(field) = &self.alternate {
            children.push(AstKind::from_statement(field));
        }
        children
    }
}

impl<'a> GetChildren<'a> for DoWhileStatement<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_statement(&self.body));
        children.push(AstKind::from_expression(&self.test));
        children
    }
}

impl<'a> GetChildren<'a> for WhileStatement<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_expression(&self.test));
        children.push(AstKind::from_statement(&self.body));
        children
    }
}

impl<'a> GetChildren<'a> for ForStatement<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        if let Some(field) = &self.init {
            children.push(AstKind::ForStatementInit(field));
        }
        if let Some(field) = &self.test {
            children.push(AstKind::from_expression(field));
        }
        if let Some(field) = &self.update {
            children.push(AstKind::from_expression(field));
        }
        children.push(AstKind::from_statement(&self.body));
        children
    }
}

impl<'a> GetChildren<'a> for ForStatementInit<'a> {
    #[allow(unused_variables, clippy::match_same_arms)]
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        match self {
            Self::VariableDeclaration(_) => vec![],
            Self::BooleanLiteral(_) => vec![],
            Self::NullLiteral(_) => vec![],
            Self::NumericLiteral(_) => vec![],
            Self::BigIntLiteral(_) => vec![],
            Self::RegExpLiteral(_) => vec![],
            Self::StringLiteral(_) => vec![],
            Self::TemplateLiteral(_) => vec![],
            Self::Identifier(_) => vec![],
            Self::MetaProperty(_) => vec![],
            Self::Super(_) => vec![],
            Self::ArrayExpression(_) => vec![],
            Self::ArrowFunctionExpression(_) => vec![],
            Self::AssignmentExpression(_) => vec![],
            Self::AwaitExpression(_) => vec![],
            Self::BinaryExpression(_) => vec![],
            Self::CallExpression(_) => vec![],
            Self::ChainExpression(_) => vec![],
            Self::ClassExpression(_) => vec![],
            Self::ConditionalExpression(_) => vec![],
            Self::FunctionExpression(_) => vec![],
            Self::ImportExpression(_) => vec![],
            Self::LogicalExpression(_) => vec![],
            Self::NewExpression(_) => vec![],
            Self::ObjectExpression(_) => vec![],
            Self::ParenthesizedExpression(_) => vec![],
            Self::SequenceExpression(_) => vec![],
            Self::TaggedTemplateExpression(_) => vec![],
            Self::ThisExpression(_) => vec![],
            Self::UnaryExpression(_) => vec![],
            Self::UpdateExpression(_) => vec![],
            Self::YieldExpression(_) => vec![],
            Self::PrivateInExpression(_) => vec![],
            Self::JSXElement(_) => vec![],
            Self::JSXFragment(_) => vec![],
            Self::TSAsExpression(_) => vec![],
            Self::TSSatisfiesExpression(_) => vec![],
            Self::TSTypeAssertion(_) => vec![],
            Self::TSNonNullExpression(_) => vec![],
            Self::TSInstantiationExpression(_) => vec![],
            Self::ComputedMemberExpression(_) => vec![],
            Self::StaticMemberExpression(_) => vec![],
            Self::PrivateFieldExpression(_) => vec![],
        }
    }
}

impl<'a> GetChildren<'a> for ForInStatement<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_for_statement_left(&self.left));
        children.push(AstKind::from_expression(&self.right));
        children.push(AstKind::from_statement(&self.body));
        children
    }
}

impl<'a> GetChildren<'a> for ForStatementLeft<'a> {
    #[allow(unused_variables, clippy::match_same_arms)]
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        match self {
            Self::VariableDeclaration(_) => vec![],
            Self::AssignmentTargetIdentifier(_) => vec![],
            Self::TSAsExpression(_) => vec![],
            Self::TSSatisfiesExpression(_) => vec![],
            Self::TSNonNullExpression(_) => vec![],
            Self::TSTypeAssertion(_) => vec![],
            Self::TSInstantiationExpression(_) => vec![],
            Self::ComputedMemberExpression(_) => vec![],
            Self::StaticMemberExpression(_) => vec![],
            Self::PrivateFieldExpression(_) => vec![],
            Self::ArrayAssignmentTarget(_) => vec![],
            Self::ObjectAssignmentTarget(_) => vec![],
        }
    }
}

impl<'a> GetChildren<'a> for ForOfStatement<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_for_statement_left(&self.left));
        children.push(AstKind::from_expression(&self.right));
        children.push(AstKind::from_statement(&self.body));
        children
    }
}

impl<'a> GetChildren<'a> for ContinueStatement<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        if let Some(field) = &self.label {
            children.push(AstKind::LabelIdentifier(field));
        }
        children
    }
}

impl<'a> GetChildren<'a> for BreakStatement<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        if let Some(field) = &self.label {
            children.push(AstKind::LabelIdentifier(field));
        }
        children
    }
}

impl<'a> GetChildren<'a> for ReturnStatement<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        if let Some(field) = &self.argument {
            children.push(AstKind::from_expression(field));
        }
        children
    }
}

impl<'a> GetChildren<'a> for WithStatement<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_expression(&self.object));
        children.push(AstKind::from_statement(&self.body));
        children
    }
}

impl<'a> GetChildren<'a> for SwitchStatement<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_expression(&self.discriminant));
        for item in &self.cases {
            children.push(AstKind::SwitchCase(item));
        }
        children
    }
}

impl<'a> GetChildren<'a> for SwitchCase<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        if let Some(field) = &self.test {
            children.push(AstKind::from_expression(field));
        }
        for item in &self.consequent {
            children.push(AstKind::from_statement(item));
        }
        children
    }
}

impl<'a> GetChildren<'a> for LabeledStatement<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::LabelIdentifier(&self.label));
        children.push(AstKind::from_statement(&self.body));
        children
    }
}

impl<'a> GetChildren<'a> for ThrowStatement<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_expression(&self.argument));
        children
    }
}

impl<'a> GetChildren<'a> for TryStatement<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::BlockStatement(&*self.block));
        if let Some(field) = &self.handler {
            children.push(AstKind::CatchClause(field));
        }
        if let Some(field) = &self.finalizer {
            children.push(AstKind::BlockStatement(field));
        }
        children
    }
}

impl<'a> GetChildren<'a> for CatchClause<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        if let Some(field) = &self.param {
            children.push(AstKind::CatchParameter(field));
        }
        children.push(AstKind::BlockStatement(&*self.body));
        children
    }
}

impl<'a> GetChildren<'a> for CatchParameter<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::BindingPattern(&self.pattern));
        children
    }
}

impl<'a> GetChildren<'a> for DebuggerStatement {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetChildren<'a> for BindingPattern<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_binding_pattern_kind(&self.kind));
        if let Some(field) = &self.type_annotation {
            children.push(AstKind::TSTypeAnnotation(field));
        }
        children
    }
}

impl<'a> GetChildren<'a> for BindingPatternKind<'a> {
    #[allow(unused_variables, clippy::match_same_arms)]
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        match self {
            Self::BindingIdentifier(_) => vec![],
            Self::ObjectPattern(_) => vec![],
            Self::ArrayPattern(_) => vec![],
            Self::AssignmentPattern(_) => vec![],
        }
    }
}

impl<'a> GetChildren<'a> for AssignmentPattern<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::BindingPattern(&self.left));
        children.push(AstKind::from_expression(&self.right));
        children
    }
}

impl<'a> GetChildren<'a> for ObjectPattern<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        for item in &self.properties {
            children.push(AstKind::BindingProperty(item));
        }
        if let Some(field) = &self.rest {
            children.push(AstKind::BindingRestElement(field));
        }
        children
    }
}

impl<'a> GetChildren<'a> for BindingProperty<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::PropertyKey(&self.key));
        children.push(AstKind::BindingPattern(&self.value));
        children
    }
}

impl<'a> GetChildren<'a> for ArrayPattern<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        for opt_item in &self.elements {
            if let Some(item) = opt_item {
                children.push(AstKind::BindingPattern(item));
            }
        }
        if let Some(field) = &self.rest {
            children.push(AstKind::BindingRestElement(field));
        }
        children
    }
}

impl<'a> GetChildren<'a> for BindingRestElement<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::BindingPattern(&self.argument));
        children
    }
}

impl<'a> GetChildren<'a> for Function<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        if let Some(field) = &self.id {
            children.push(AstKind::BindingIdentifier(field));
        }
        if let Some(field) = &self.type_parameters {
            children.push(AstKind::TSTypeParameterDeclaration(field));
        }
        if let Some(field) = &self.this_param {
            children.push(AstKind::TSThisParameter(field));
        }
        children.push(AstKind::FormalParameters(&*self.params));
        if let Some(field) = &self.return_type {
            children.push(AstKind::TSTypeAnnotation(field));
        }
        if let Some(field) = &self.body {
            children.push(AstKind::FunctionBody(field));
        }
        children
    }
}

impl<'a> GetChildren<'a> for FormalParameters<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        for item in &self.items {
            children.push(AstKind::FormalParameter(item));
        }
        if let Some(field) = &self.rest {
            children.push(AstKind::BindingRestElement(field));
        }
        children
    }
}

impl<'a> GetChildren<'a> for FormalParameter<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        for item in &self.decorators {
            children.push(AstKind::Decorator(item));
        }
        children.push(AstKind::BindingPattern(&self.pattern));
        children
    }
}

impl<'a> GetChildren<'a> for FunctionBody<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        for item in &self.directives {
            children.push(AstKind::Directive(item));
        }
        for item in &self.statements {
            children.push(AstKind::from_statement(item));
        }
        children
    }
}

impl<'a> GetChildren<'a> for ArrowFunctionExpression<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        if let Some(field) = &self.type_parameters {
            children.push(AstKind::TSTypeParameterDeclaration(field));
        }
        children.push(AstKind::FormalParameters(&*self.params));
        if let Some(field) = &self.return_type {
            children.push(AstKind::TSTypeAnnotation(field));
        }
        children.push(AstKind::FunctionBody(&*self.body));
        children
    }
}

impl<'a> GetChildren<'a> for YieldExpression<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        if let Some(field) = &self.argument {
            children.push(AstKind::from_expression(field));
        }
        children
    }
}

impl<'a> GetChildren<'a> for Class<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        for item in &self.decorators {
            children.push(AstKind::Decorator(item));
        }
        if let Some(field) = &self.id {
            children.push(AstKind::BindingIdentifier(field));
        }
        if let Some(field) = &self.type_parameters {
            children.push(AstKind::TSTypeParameterDeclaration(field));
        }
        if let Some(field) = &self.super_class {
            children.push(AstKind::from_expression(field));
        }
        if let Some(field) = &self.super_type_parameters {
            children.push(AstKind::TSTypeParameterInstantiation(field));
        }
        if let Some(vec) = &self.implements {
            for item in vec {
                children.push(AstKind::TSClassImplements(item));
            }
        }
        children.push(AstKind::ClassBody(&*self.body));
        children
    }
}

impl<'a> GetChildren<'a> for ClassBody<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        for item in &self.body {
            children.push(AstKind::from_class_element(item));
        }
        children
    }
}

impl<'a> GetChildren<'a> for ClassElement<'a> {
    #[allow(unused_variables, clippy::match_same_arms)]
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        match self {
            Self::StaticBlock(_) => vec![],
            Self::MethodDefinition(_) => vec![],
            Self::PropertyDefinition(_) => vec![],
            Self::AccessorProperty(_) => vec![],
            Self::TSIndexSignature(_) => vec![],
        }
    }
}

impl<'a> GetChildren<'a> for MethodDefinition<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        for item in &self.decorators {
            children.push(AstKind::Decorator(item));
        }
        children.push(AstKind::PropertyKey(&self.key));
        children.push(AstKind::Function(&*self.value));
        children
    }
}

impl<'a> GetChildren<'a> for PropertyDefinition<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        for item in &self.decorators {
            children.push(AstKind::Decorator(item));
        }
        children.push(AstKind::PropertyKey(&self.key));
        if let Some(field) = &self.value {
            children.push(AstKind::from_expression(field));
        }
        if let Some(field) = &self.type_annotation {
            children.push(AstKind::TSTypeAnnotation(field));
        }
        children
    }
}

impl<'a> GetChildren<'a> for PrivateIdentifier<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetChildren<'a> for StaticBlock<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        for item in &self.body {
            children.push(AstKind::from_statement(item));
        }
        children
    }
}

impl<'a> GetChildren<'a> for ModuleDeclaration<'a> {
    #[allow(unused_variables, clippy::match_same_arms)]
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        match self {
            Self::ImportDeclaration(_) => vec![],
            Self::ExportAllDeclaration(_) => vec![],
            Self::ExportDefaultDeclaration(_) => vec![],
            Self::ExportNamedDeclaration(_) => vec![],
            Self::TSExportAssignment(_) => vec![],
            Self::TSNamespaceExportDeclaration(_) => vec![],
        }
    }
}

impl<'a> GetChildren<'a> for AccessorProperty<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        for item in &self.decorators {
            children.push(AstKind::Decorator(item));
        }
        children.push(AstKind::PropertyKey(&self.key));
        if let Some(field) = &self.value {
            children.push(AstKind::from_expression(field));
        }
        if let Some(field) = &self.type_annotation {
            children.push(AstKind::TSTypeAnnotation(field));
        }
        children
    }
}

impl<'a> GetChildren<'a> for ImportExpression<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_expression(&self.source));
        for item in &self.arguments {
            children.push(AstKind::from_expression(item));
        }
        children
    }
}

impl<'a> GetChildren<'a> for ImportDeclaration<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        if let Some(vec) = &self.specifiers {
            for item in vec {
                children.push(AstKind::from_import_declaration_specifier(item));
            }
        }
        children.push(AstKind::StringLiteral(&self.source));
        if let Some(field) = &self.with_clause {
            children.push(AstKind::WithClause(field));
        }
        children
    }
}

impl<'a> GetChildren<'a> for ImportDeclarationSpecifier<'a> {
    #[allow(unused_variables, clippy::match_same_arms)]
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        match self {
            Self::ImportSpecifier(_) => vec![],
            Self::ImportDefaultSpecifier(_) => vec![],
            Self::ImportNamespaceSpecifier(_) => vec![],
        }
    }
}

impl<'a> GetChildren<'a> for ImportSpecifier<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_module_export_name(&self.imported));
        children.push(AstKind::BindingIdentifier(&self.local));
        children
    }
}

impl<'a> GetChildren<'a> for ImportDefaultSpecifier<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::BindingIdentifier(&self.local));
        children
    }
}

impl<'a> GetChildren<'a> for ImportNamespaceSpecifier<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::BindingIdentifier(&self.local));
        children
    }
}

impl<'a> GetChildren<'a> for WithClause<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::IdentifierName(&self.attributes_keyword));
        for item in &self.with_entries {
            children.push(AstKind::ImportAttribute(item));
        }
        children
    }
}

impl<'a> GetChildren<'a> for ImportAttribute<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_import_attribute_key(&self.key));
        children.push(AstKind::StringLiteral(&self.value));
        children
    }
}

impl<'a> GetChildren<'a> for ImportAttributeKey<'a> {
    #[allow(unused_variables, clippy::match_same_arms)]
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        match self {
            Self::Identifier(child) => vec![AstKind::IdentifierName(child)],
            Self::StringLiteral(child) => vec![AstKind::StringLiteral(child)],
        }
    }
}

impl<'a> GetChildren<'a> for ExportNamedDeclaration<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        if let Some(field) = &self.declaration {
            children.push(AstKind::from_declaration(field));
        }
        for item in &self.specifiers {
            children.push(AstKind::ExportSpecifier(item));
        }
        if let Some(field) = &self.source {
            children.push(AstKind::StringLiteral(field));
        }
        if let Some(field) = &self.with_clause {
            children.push(AstKind::WithClause(field));
        }
        children
    }
}

impl<'a> GetChildren<'a> for ExportDefaultDeclaration<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_export_default_declaration_kind(&self.declaration));
        children.push(AstKind::from_module_export_name(&self.exported));
        children
    }
}

impl<'a> GetChildren<'a> for ExportAllDeclaration<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        if let Some(field) = &self.exported {
            children.push(AstKind::from_module_export_name(field));
        }
        children.push(AstKind::StringLiteral(&self.source));
        if let Some(field) = &self.with_clause {
            children.push(AstKind::WithClause(field));
        }
        children
    }
}

impl<'a> GetChildren<'a> for ExportSpecifier<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_module_export_name(&self.local));
        children.push(AstKind::from_module_export_name(&self.exported));
        children
    }
}

impl<'a> GetChildren<'a> for ExportDefaultDeclarationKind<'a> {
    #[allow(unused_variables, clippy::match_same_arms)]
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        match self {
            Self::FunctionDeclaration(_) => vec![],
            Self::ClassDeclaration(_) => vec![],
            Self::TSInterfaceDeclaration(_) => vec![],
            Self::BooleanLiteral(_) => vec![],
            Self::NullLiteral(_) => vec![],
            Self::NumericLiteral(_) => vec![],
            Self::BigIntLiteral(_) => vec![],
            Self::RegExpLiteral(_) => vec![],
            Self::StringLiteral(_) => vec![],
            Self::TemplateLiteral(_) => vec![],
            Self::Identifier(_) => vec![],
            Self::MetaProperty(_) => vec![],
            Self::Super(_) => vec![],
            Self::ArrayExpression(_) => vec![],
            Self::ArrowFunctionExpression(_) => vec![],
            Self::AssignmentExpression(_) => vec![],
            Self::AwaitExpression(_) => vec![],
            Self::BinaryExpression(_) => vec![],
            Self::CallExpression(_) => vec![],
            Self::ChainExpression(_) => vec![],
            Self::ClassExpression(_) => vec![],
            Self::ConditionalExpression(_) => vec![],
            Self::FunctionExpression(_) => vec![],
            Self::ImportExpression(_) => vec![],
            Self::LogicalExpression(_) => vec![],
            Self::NewExpression(_) => vec![],
            Self::ObjectExpression(_) => vec![],
            Self::ParenthesizedExpression(_) => vec![],
            Self::SequenceExpression(_) => vec![],
            Self::TaggedTemplateExpression(_) => vec![],
            Self::ThisExpression(_) => vec![],
            Self::UnaryExpression(_) => vec![],
            Self::UpdateExpression(_) => vec![],
            Self::YieldExpression(_) => vec![],
            Self::PrivateInExpression(_) => vec![],
            Self::JSXElement(_) => vec![],
            Self::JSXFragment(_) => vec![],
            Self::TSAsExpression(_) => vec![],
            Self::TSSatisfiesExpression(_) => vec![],
            Self::TSTypeAssertion(_) => vec![],
            Self::TSNonNullExpression(_) => vec![],
            Self::TSInstantiationExpression(_) => vec![],
            Self::ComputedMemberExpression(_) => vec![],
            Self::StaticMemberExpression(_) => vec![],
            Self::PrivateFieldExpression(_) => vec![],
        }
    }
}

impl<'a> GetChildren<'a> for ModuleExportName<'a> {
    #[allow(unused_variables, clippy::match_same_arms)]
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        match self {
            Self::IdentifierName(child) => vec![AstKind::IdentifierName(child)],
            Self::IdentifierReference(child) => vec![AstKind::IdentifierReference(child)],
            Self::StringLiteral(child) => vec![AstKind::StringLiteral(child)],
        }
    }
}

impl<'a> GetChildren<'a> for TSThisParameter<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        if let Some(field) = &self.type_annotation {
            children.push(AstKind::TSTypeAnnotation(field));
        }
        children
    }
}

impl<'a> GetChildren<'a> for TSEnumDeclaration<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::BindingIdentifier(&self.id));
        for item in &self.members {
            children.push(AstKind::TSEnumMember(item));
        }
        children
    }
}

impl<'a> GetChildren<'a> for TSEnumMember<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_ts_enum_member_name(&self.id));
        if let Some(field) = &self.initializer {
            children.push(AstKind::from_expression(field));
        }
        children
    }
}

impl<'a> GetChildren<'a> for TSEnumMemberName<'a> {
    #[allow(unused_variables, clippy::match_same_arms)]
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        match self {
            Self::Identifier(_) => vec![],
            Self::String(_) => vec![],
        }
    }
}

impl<'a> GetChildren<'a> for TSTypeAnnotation<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_ts_type(&self.type_annotation));
        children
    }
}

impl<'a> GetChildren<'a> for TSLiteralType<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_ts_literal(&self.literal));
        children
    }
}

impl<'a> GetChildren<'a> for TSLiteral<'a> {
    #[allow(unused_variables, clippy::match_same_arms)]
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        match self {
            Self::BooleanLiteral(_) => vec![],
            Self::NullLiteral(_) => vec![],
            Self::NumericLiteral(_) => vec![],
            Self::BigIntLiteral(_) => vec![],
            Self::RegExpLiteral(_) => vec![],
            Self::StringLiteral(_) => vec![],
            Self::TemplateLiteral(_) => vec![],
            Self::UnaryExpression(_) => vec![],
        }
    }
}

impl<'a> GetChildren<'a> for TSType<'a> {
    #[allow(unused_variables, clippy::match_same_arms)]
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        match self {
            Self::TSAnyKeyword(_) => vec![],
            Self::TSBigIntKeyword(_) => vec![],
            Self::TSBooleanKeyword(_) => vec![],
            Self::TSIntrinsicKeyword(_) => vec![],
            Self::TSNeverKeyword(_) => vec![],
            Self::TSNullKeyword(_) => vec![],
            Self::TSNumberKeyword(_) => vec![],
            Self::TSObjectKeyword(_) => vec![],
            Self::TSStringKeyword(_) => vec![],
            Self::TSSymbolKeyword(_) => vec![],
            Self::TSUndefinedKeyword(_) => vec![],
            Self::TSUnknownKeyword(_) => vec![],
            Self::TSVoidKeyword(_) => vec![],
            Self::TSArrayType(_) => vec![],
            Self::TSConditionalType(_) => vec![],
            Self::TSConstructorType(_) => vec![],
            Self::TSFunctionType(_) => vec![],
            Self::TSImportType(_) => vec![],
            Self::TSIndexedAccessType(_) => vec![],
            Self::TSInferType(_) => vec![],
            Self::TSIntersectionType(_) => vec![],
            Self::TSLiteralType(_) => vec![],
            Self::TSMappedType(_) => vec![],
            Self::TSNamedTupleMember(_) => vec![],
            Self::TSQualifiedName(_) => vec![],
            Self::TSTemplateLiteralType(_) => vec![],
            Self::TSThisType(_) => vec![],
            Self::TSTupleType(_) => vec![],
            Self::TSTypeLiteral(_) => vec![],
            Self::TSTypeOperatorType(_) => vec![],
            Self::TSTypePredicate(_) => vec![],
            Self::TSTypeQuery(_) => vec![],
            Self::TSTypeReference(_) => vec![],
            Self::TSUnionType(_) => vec![],
            Self::TSParenthesizedType(_) => vec![],
            Self::JSDocNullableType(_) => vec![],
            Self::JSDocNonNullableType(_) => vec![],
            Self::JSDocUnknownType(_) => vec![],
        }
    }
}

impl<'a> GetChildren<'a> for TSConditionalType<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_ts_type(&self.check_type));
        children.push(AstKind::from_ts_type(&self.extends_type));
        children.push(AstKind::from_ts_type(&self.true_type));
        children.push(AstKind::from_ts_type(&self.false_type));
        children
    }
}

impl<'a> GetChildren<'a> for TSUnionType<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        for item in &self.types {
            children.push(AstKind::from_ts_type(item));
        }
        children
    }
}

impl<'a> GetChildren<'a> for TSIntersectionType<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        for item in &self.types {
            children.push(AstKind::from_ts_type(item));
        }
        children
    }
}

impl<'a> GetChildren<'a> for TSParenthesizedType<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_ts_type(&self.type_annotation));
        children
    }
}

impl<'a> GetChildren<'a> for TSTypeOperator<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_ts_type(&self.type_annotation));
        children
    }
}

impl<'a> GetChildren<'a> for TSArrayType<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_ts_type(&self.element_type));
        children
    }
}

impl<'a> GetChildren<'a> for TSIndexedAccessType<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_ts_type(&self.object_type));
        children.push(AstKind::from_ts_type(&self.index_type));
        children
    }
}

impl<'a> GetChildren<'a> for TSTupleType<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        for item in &self.element_types {
            children.push(AstKind::from_ts_tuple_element(item));
        }
        children
    }
}

impl<'a> GetChildren<'a> for TSNamedTupleMember<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_ts_tuple_element(&self.element_type));
        children.push(AstKind::IdentifierName(&self.label));
        children
    }
}

impl<'a> GetChildren<'a> for TSOptionalType<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_ts_type(&self.type_annotation));
        children
    }
}

impl<'a> GetChildren<'a> for TSRestType<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_ts_type(&self.type_annotation));
        children
    }
}

impl<'a> GetChildren<'a> for TSTupleElement<'a> {
    #[allow(unused_variables, clippy::match_same_arms)]
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        match self {
            Self::TSOptionalType(_) => vec![],
            Self::TSRestType(_) => vec![],
            Self::TSAnyKeyword(_) => vec![],
            Self::TSBigIntKeyword(_) => vec![],
            Self::TSBooleanKeyword(_) => vec![],
            Self::TSIntrinsicKeyword(_) => vec![],
            Self::TSNeverKeyword(_) => vec![],
            Self::TSNullKeyword(_) => vec![],
            Self::TSNumberKeyword(_) => vec![],
            Self::TSObjectKeyword(_) => vec![],
            Self::TSStringKeyword(_) => vec![],
            Self::TSSymbolKeyword(_) => vec![],
            Self::TSUndefinedKeyword(_) => vec![],
            Self::TSUnknownKeyword(_) => vec![],
            Self::TSVoidKeyword(_) => vec![],
            Self::TSArrayType(_) => vec![],
            Self::TSConditionalType(_) => vec![],
            Self::TSConstructorType(_) => vec![],
            Self::TSFunctionType(_) => vec![],
            Self::TSImportType(_) => vec![],
            Self::TSIndexedAccessType(_) => vec![],
            Self::TSInferType(_) => vec![],
            Self::TSIntersectionType(_) => vec![],
            Self::TSLiteralType(_) => vec![],
            Self::TSMappedType(_) => vec![],
            Self::TSNamedTupleMember(_) => vec![],
            Self::TSQualifiedName(_) => vec![],
            Self::TSTemplateLiteralType(_) => vec![],
            Self::TSThisType(_) => vec![],
            Self::TSTupleType(_) => vec![],
            Self::TSTypeLiteral(_) => vec![],
            Self::TSTypeOperatorType(_) => vec![],
            Self::TSTypePredicate(_) => vec![],
            Self::TSTypeQuery(_) => vec![],
            Self::TSTypeReference(_) => vec![],
            Self::TSUnionType(_) => vec![],
            Self::TSParenthesizedType(_) => vec![],
            Self::JSDocNullableType(_) => vec![],
            Self::JSDocNonNullableType(_) => vec![],
            Self::JSDocUnknownType(_) => vec![],
        }
    }
}

impl<'a> GetChildren<'a> for TSAnyKeyword {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetChildren<'a> for TSStringKeyword {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetChildren<'a> for TSBooleanKeyword {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetChildren<'a> for TSNumberKeyword {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetChildren<'a> for TSNeverKeyword {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetChildren<'a> for TSIntrinsicKeyword {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetChildren<'a> for TSUnknownKeyword {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetChildren<'a> for TSNullKeyword {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetChildren<'a> for TSUndefinedKeyword {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetChildren<'a> for TSVoidKeyword {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetChildren<'a> for TSSymbolKeyword {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetChildren<'a> for TSThisType {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetChildren<'a> for TSObjectKeyword {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetChildren<'a> for TSBigIntKeyword {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetChildren<'a> for TSTypeReference<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::TSTypeName(&self.type_name));
        if let Some(field) = &self.type_parameters {
            children.push(AstKind::TSTypeParameterInstantiation(field));
        }
        children
    }
}

impl<'a> GetChildren<'a> for TSTypeName<'a> {
    #[allow(unused_variables, clippy::match_same_arms)]
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        match self {
            Self::IdentifierReference(_) => vec![],
            Self::QualifiedName(_) => vec![],
        }
    }
}

impl<'a> GetChildren<'a> for TSQualifiedName<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::TSTypeName(&self.left));
        children.push(AstKind::IdentifierName(&self.right));
        children
    }
}

impl<'a> GetChildren<'a> for TSTypeParameterInstantiation<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        for item in &self.params {
            children.push(AstKind::from_ts_type(item));
        }
        children
    }
}

impl<'a> GetChildren<'a> for TSTypeParameter<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::BindingIdentifier(&self.name));
        if let Some(field) = &self.constraint {
            children.push(AstKind::from_ts_type(field));
        }
        if let Some(field) = &self.default {
            children.push(AstKind::from_ts_type(field));
        }
        children
    }
}

impl<'a> GetChildren<'a> for TSTypeParameterDeclaration<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        for item in &self.params {
            children.push(AstKind::TSTypeParameter(item));
        }
        children
    }
}

impl<'a> GetChildren<'a> for TSTypeAliasDeclaration<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::BindingIdentifier(&self.id));
        if let Some(field) = &self.type_parameters {
            children.push(AstKind::TSTypeParameterDeclaration(field));
        }
        children.push(AstKind::from_ts_type(&self.type_annotation));
        children
    }
}

impl<'a> GetChildren<'a> for TSClassImplements<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::TSTypeName(&self.expression));
        if let Some(field) = &self.type_parameters {
            children.push(AstKind::TSTypeParameterInstantiation(field));
        }
        children
    }
}

impl<'a> GetChildren<'a> for TSInterfaceDeclaration<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::BindingIdentifier(&self.id));
        if let Some(vec) = &self.extends {
            for item in vec {
                children.push(AstKind::TSInterfaceHeritage(item));
            }
        }
        if let Some(field) = &self.type_parameters {
            children.push(AstKind::TSTypeParameterDeclaration(field));
        }
        children.push(AstKind::TSInterfaceBody(&*self.body));
        children
    }
}

impl<'a> GetChildren<'a> for TSInterfaceBody<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        for item in &self.body {
            children.push(AstKind::from_ts_signature(item));
        }
        children
    }
}

impl<'a> GetChildren<'a> for TSPropertySignature<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::PropertyKey(&self.key));
        if let Some(field) = &self.type_annotation {
            children.push(AstKind::TSTypeAnnotation(field));
        }
        children
    }
}

impl<'a> GetChildren<'a> for TSSignature<'a> {
    #[allow(unused_variables, clippy::match_same_arms)]
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        match self {
            Self::TSIndexSignature(_) => vec![],
            Self::TSPropertySignature(_) => vec![],
            Self::TSCallSignatureDeclaration(_) => vec![],
            Self::TSConstructSignatureDeclaration(_) => vec![],
            Self::TSMethodSignature(_) => vec![],
        }
    }
}

impl<'a> GetChildren<'a> for TSIndexSignature<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        for item in &self.parameters {
            children.push(AstKind::TSIndexSignatureName(item));
        }
        children.push(AstKind::TSTypeAnnotation(&*self.type_annotation));
        children
    }
}

impl<'a> GetChildren<'a> for TSCallSignatureDeclaration<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        if let Some(field) = &self.type_parameters {
            children.push(AstKind::TSTypeParameterDeclaration(field));
        }
        if let Some(field) = &self.this_param {
            children.push(AstKind::TSThisParameter(field));
        }
        children.push(AstKind::FormalParameters(&*self.params));
        if let Some(field) = &self.return_type {
            children.push(AstKind::TSTypeAnnotation(field));
        }
        children
    }
}

impl<'a> GetChildren<'a> for TSMethodSignature<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::PropertyKey(&self.key));
        if let Some(field) = &self.type_parameters {
            children.push(AstKind::TSTypeParameterDeclaration(field));
        }
        if let Some(field) = &self.this_param {
            children.push(AstKind::TSThisParameter(field));
        }
        children.push(AstKind::FormalParameters(&*self.params));
        if let Some(field) = &self.return_type {
            children.push(AstKind::TSTypeAnnotation(field));
        }
        children
    }
}

impl<'a> GetChildren<'a> for TSConstructSignatureDeclaration<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        if let Some(field) = &self.type_parameters {
            children.push(AstKind::TSTypeParameterDeclaration(field));
        }
        children.push(AstKind::FormalParameters(&*self.params));
        if let Some(field) = &self.return_type {
            children.push(AstKind::TSTypeAnnotation(field));
        }
        children
    }
}

impl<'a> GetChildren<'a> for TSIndexSignatureName<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::TSTypeAnnotation(&*self.type_annotation));
        children
    }
}

impl<'a> GetChildren<'a> for TSInterfaceHeritage<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_expression(&self.expression));
        if let Some(field) = &self.type_parameters {
            children.push(AstKind::TSTypeParameterInstantiation(field));
        }
        children
    }
}

impl<'a> GetChildren<'a> for TSTypePredicate<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_ts_type_predicate_name(&self.parameter_name));
        if let Some(field) = &self.type_annotation {
            children.push(AstKind::TSTypeAnnotation(field));
        }
        children
    }
}

impl<'a> GetChildren<'a> for TSTypePredicateName<'a> {
    #[allow(unused_variables, clippy::match_same_arms)]
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        match self {
            Self::Identifier(_) => vec![],
            Self::This(child) => vec![AstKind::TSThisType(child)],
        }
    }
}

impl<'a> GetChildren<'a> for TSModuleDeclaration<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_ts_module_declaration_name(&self.id));
        if let Some(field) = &self.body {
            children.push(AstKind::from_ts_module_declaration_body(field));
        }
        children
    }
}

impl<'a> GetChildren<'a> for TSModuleDeclarationName<'a> {
    #[allow(unused_variables, clippy::match_same_arms)]
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        match self {
            Self::Identifier(child) => vec![AstKind::BindingIdentifier(child)],
            Self::StringLiteral(child) => vec![AstKind::StringLiteral(child)],
        }
    }
}

impl<'a> GetChildren<'a> for TSModuleDeclarationBody<'a> {
    #[allow(unused_variables, clippy::match_same_arms)]
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        match self {
            Self::TSModuleDeclaration(_) => vec![],
            Self::TSModuleBlock(_) => vec![],
        }
    }
}

impl<'a> GetChildren<'a> for TSModuleBlock<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        for item in &self.directives {
            children.push(AstKind::Directive(item));
        }
        for item in &self.body {
            children.push(AstKind::from_statement(item));
        }
        children
    }
}

impl<'a> GetChildren<'a> for TSTypeLiteral<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        for item in &self.members {
            children.push(AstKind::from_ts_signature(item));
        }
        children
    }
}

impl<'a> GetChildren<'a> for TSInferType<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::TSTypeParameter(&*self.type_parameter));
        children
    }
}

impl<'a> GetChildren<'a> for TSTypeQuery<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_ts_type_query_expr_name(&self.expr_name));
        if let Some(field) = &self.type_parameters {
            children.push(AstKind::TSTypeParameterInstantiation(field));
        }
        children
    }
}

impl<'a> GetChildren<'a> for TSTypeQueryExprName<'a> {
    #[allow(unused_variables, clippy::match_same_arms)]
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        match self {
            Self::TSImportType(_) => vec![],
            Self::IdentifierReference(_) => vec![],
            Self::QualifiedName(_) => vec![],
        }
    }
}

impl<'a> GetChildren<'a> for TSImportType<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_ts_type(&self.parameter));
        if let Some(field) = &self.qualifier {
            children.push(AstKind::TSTypeName(field));
        }
        if let Some(field) = &self.attributes {
            children.push(AstKind::TSImportAttributes(field));
        }
        if let Some(field) = &self.type_parameters {
            children.push(AstKind::TSTypeParameterInstantiation(field));
        }
        children
    }
}

impl<'a> GetChildren<'a> for TSImportAttributes<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::IdentifierName(&self.attributes_keyword));
        for item in &self.elements {
            children.push(AstKind::TSImportAttribute(item));
        }
        children
    }
}

impl<'a> GetChildren<'a> for TSImportAttribute<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_ts_import_attribute_name(&self.name));
        children.push(AstKind::from_expression(&self.value));
        children
    }
}

impl<'a> GetChildren<'a> for TSImportAttributeName<'a> {
    #[allow(unused_variables, clippy::match_same_arms)]
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        match self {
            Self::Identifier(child) => vec![AstKind::IdentifierName(child)],
            Self::StringLiteral(child) => vec![AstKind::StringLiteral(child)],
        }
    }
}

impl<'a> GetChildren<'a> for TSFunctionType<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        if let Some(field) = &self.type_parameters {
            children.push(AstKind::TSTypeParameterDeclaration(field));
        }
        if let Some(field) = &self.this_param {
            children.push(AstKind::TSThisParameter(field));
        }
        children.push(AstKind::FormalParameters(&*self.params));
        children.push(AstKind::TSTypeAnnotation(&*self.return_type));
        children
    }
}

impl<'a> GetChildren<'a> for TSConstructorType<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        if let Some(field) = &self.type_parameters {
            children.push(AstKind::TSTypeParameterDeclaration(field));
        }
        children.push(AstKind::FormalParameters(&*self.params));
        children.push(AstKind::TSTypeAnnotation(&*self.return_type));
        children
    }
}

impl<'a> GetChildren<'a> for TSMappedType<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::TSTypeParameter(&*self.type_parameter));
        if let Some(field) = &self.name_type {
            children.push(AstKind::from_ts_type(field));
        }
        if let Some(field) = &self.type_annotation {
            children.push(AstKind::from_ts_type(field));
        }
        children
    }
}

impl<'a> GetChildren<'a> for TSTemplateLiteralType<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        for item in &self.quasis {
            children.push(AstKind::TemplateElement(item));
        }
        for item in &self.types {
            children.push(AstKind::from_ts_type(item));
        }
        children
    }
}

impl<'a> GetChildren<'a> for TSAsExpression<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_expression(&self.expression));
        children.push(AstKind::from_ts_type(&self.type_annotation));
        children
    }
}

impl<'a> GetChildren<'a> for TSSatisfiesExpression<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_expression(&self.expression));
        children.push(AstKind::from_ts_type(&self.type_annotation));
        children
    }
}

impl<'a> GetChildren<'a> for TSTypeAssertion<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_expression(&self.expression));
        children.push(AstKind::from_ts_type(&self.type_annotation));
        children
    }
}

impl<'a> GetChildren<'a> for TSImportEqualsDeclaration<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::BindingIdentifier(&self.id));
        children.push(AstKind::TSModuleReference(&self.module_reference));
        children
    }
}

impl<'a> GetChildren<'a> for TSModuleReference<'a> {
    #[allow(unused_variables, clippy::match_same_arms)]
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        match self {
            Self::ExternalModuleReference(_) => vec![],
            Self::IdentifierReference(_) => vec![],
            Self::QualifiedName(_) => vec![],
        }
    }
}

impl<'a> GetChildren<'a> for TSExternalModuleReference<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::StringLiteral(&self.expression));
        children
    }
}

impl<'a> GetChildren<'a> for TSNonNullExpression<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_expression(&self.expression));
        children
    }
}

impl<'a> GetChildren<'a> for Decorator<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_expression(&self.expression));
        children
    }
}

impl<'a> GetChildren<'a> for TSExportAssignment<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_expression(&self.expression));
        children
    }
}

impl<'a> GetChildren<'a> for TSNamespaceExportDeclaration<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::IdentifierName(&self.id));
        children
    }
}

impl<'a> GetChildren<'a> for TSInstantiationExpression<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_expression(&self.expression));
        children.push(AstKind::TSTypeParameterInstantiation(&*self.type_parameters));
        children
    }
}

impl<'a> GetChildren<'a> for JSDocNullableType<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_ts_type(&self.type_annotation));
        children
    }
}

impl<'a> GetChildren<'a> for JSDocNonNullableType<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_ts_type(&self.type_annotation));
        children
    }
}

impl<'a> GetChildren<'a> for JSDocUnknownType {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetChildren<'a> for JSXElement<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::JSXOpeningElement(&*self.opening_element));
        if let Some(field) = &self.closing_element {
            children.push(AstKind::JSXClosingElement(field));
        }
        for item in &self.children {
            children.push(AstKind::from_jsx_child(item));
        }
        children
    }
}

impl<'a> GetChildren<'a> for JSXOpeningElement<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::JSXElementName(&self.name));
        for item in &self.attributes {
            children.push(AstKind::JSXAttributeItem(item));
        }
        if let Some(field) = &self.type_parameters {
            children.push(AstKind::TSTypeParameterInstantiation(field));
        }
        children
    }
}

impl<'a> GetChildren<'a> for JSXClosingElement<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::JSXElementName(&self.name));
        children
    }
}

impl<'a> GetChildren<'a> for JSXFragment<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        for item in &self.children {
            children.push(AstKind::from_jsx_child(item));
        }
        children
    }
}

impl<'a> GetChildren<'a> for JSXOpeningFragment {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetChildren<'a> for JSXClosingFragment {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetChildren<'a> for JSXElementName<'a> {
    #[allow(unused_variables, clippy::match_same_arms)]
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        match self {
            Self::Identifier(_) => vec![],
            Self::IdentifierReference(_) => vec![],
            Self::NamespacedName(_) => vec![],
            Self::MemberExpression(_) => vec![],
            Self::ThisExpression(_) => vec![],
        }
    }
}

impl<'a> GetChildren<'a> for JSXNamespacedName<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::JSXIdentifier(&self.namespace));
        children.push(AstKind::JSXIdentifier(&self.property));
        children
    }
}

impl<'a> GetChildren<'a> for JSXMemberExpression<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::JSXMemberExpressionObject(&self.object));
        children.push(AstKind::JSXIdentifier(&self.property));
        children
    }
}

impl<'a> GetChildren<'a> for JSXMemberExpressionObject<'a> {
    #[allow(unused_variables, clippy::match_same_arms)]
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        match self {
            Self::IdentifierReference(_) => vec![],
            Self::MemberExpression(_) => vec![],
            Self::ThisExpression(_) => vec![],
        }
    }
}

impl<'a> GetChildren<'a> for JSXExpressionContainer<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_jsx_expression(&self.expression));
        children
    }
}

impl<'a> GetChildren<'a> for JSXExpression<'a> {
    #[allow(unused_variables, clippy::match_same_arms)]
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        match self {
            Self::EmptyExpression(child) => vec![AstKind::JSXEmptyExpression(child)],
            Self::BooleanLiteral(_) => vec![],
            Self::NullLiteral(_) => vec![],
            Self::NumericLiteral(_) => vec![],
            Self::BigIntLiteral(_) => vec![],
            Self::RegExpLiteral(_) => vec![],
            Self::StringLiteral(_) => vec![],
            Self::TemplateLiteral(_) => vec![],
            Self::Identifier(_) => vec![],
            Self::MetaProperty(_) => vec![],
            Self::Super(_) => vec![],
            Self::ArrayExpression(_) => vec![],
            Self::ArrowFunctionExpression(_) => vec![],
            Self::AssignmentExpression(_) => vec![],
            Self::AwaitExpression(_) => vec![],
            Self::BinaryExpression(_) => vec![],
            Self::CallExpression(_) => vec![],
            Self::ChainExpression(_) => vec![],
            Self::ClassExpression(_) => vec![],
            Self::ConditionalExpression(_) => vec![],
            Self::FunctionExpression(_) => vec![],
            Self::ImportExpression(_) => vec![],
            Self::LogicalExpression(_) => vec![],
            Self::NewExpression(_) => vec![],
            Self::ObjectExpression(_) => vec![],
            Self::ParenthesizedExpression(_) => vec![],
            Self::SequenceExpression(_) => vec![],
            Self::TaggedTemplateExpression(_) => vec![],
            Self::ThisExpression(_) => vec![],
            Self::UnaryExpression(_) => vec![],
            Self::UpdateExpression(_) => vec![],
            Self::YieldExpression(_) => vec![],
            Self::PrivateInExpression(_) => vec![],
            Self::JSXElement(_) => vec![],
            Self::JSXFragment(_) => vec![],
            Self::TSAsExpression(_) => vec![],
            Self::TSSatisfiesExpression(_) => vec![],
            Self::TSTypeAssertion(_) => vec![],
            Self::TSNonNullExpression(_) => vec![],
            Self::TSInstantiationExpression(_) => vec![],
            Self::ComputedMemberExpression(_) => vec![],
            Self::StaticMemberExpression(_) => vec![],
            Self::PrivateFieldExpression(_) => vec![],
        }
    }
}

impl<'a> GetChildren<'a> for JSXEmptyExpression {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetChildren<'a> for JSXAttributeItem<'a> {
    #[allow(unused_variables, clippy::match_same_arms)]
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        match self {
            Self::Attribute(_) => vec![],
            Self::SpreadAttribute(_) => vec![],
        }
    }
}

impl<'a> GetChildren<'a> for JSXAttribute<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_jsx_attribute_name(&self.name));
        if let Some(field) = &self.value {
            children.push(AstKind::from_jsx_attribute_value(field));
        }
        children
    }
}

impl<'a> GetChildren<'a> for JSXSpreadAttribute<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_expression(&self.argument));
        children
    }
}

impl<'a> GetChildren<'a> for JSXAttributeName<'a> {
    #[allow(unused_variables, clippy::match_same_arms)]
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        match self {
            Self::Identifier(_) => vec![],
            Self::NamespacedName(_) => vec![],
        }
    }
}

impl<'a> GetChildren<'a> for JSXAttributeValue<'a> {
    #[allow(unused_variables, clippy::match_same_arms)]
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        match self {
            Self::StringLiteral(_) => vec![],
            Self::ExpressionContainer(_) => vec![],
            Self::Element(_) => vec![],
            Self::Fragment(_) => vec![],
        }
    }
}

impl<'a> GetChildren<'a> for JSXIdentifier<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetChildren<'a> for JSXChild<'a> {
    #[allow(unused_variables, clippy::match_same_arms)]
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        match self {
            Self::Text(_) => vec![],
            Self::Element(_) => vec![],
            Self::Fragment(_) => vec![],
            Self::ExpressionContainer(_) => vec![],
            Self::Spread(_) => vec![],
        }
    }
}

impl<'a> GetChildren<'a> for JSXSpreadChild<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_expression(&self.expression));
        children
    }
}

impl<'a> GetChildren<'a> for JSXText<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}
