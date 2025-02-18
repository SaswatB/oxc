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
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::BooleanLiteral(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for NullLiteral {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::NullLiteral(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for NumericLiteral<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::NumericLiteral(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for StringLiteral<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::StringLiteral(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for BigIntLiteral<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::BigIntLiteral(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for RegExpLiteral<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::RegExpLiteral(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for SourceFile<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        if let Some(field) = &self.hashbang {
            children.push(AstKind::Hashbang(field));
        }
        for item in &self.directives {
            children.push(AstKind::Directive(item));
        }
        for item in &self.body {
            children.push((*item).to_ast_kind());
        }
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::SourceFile(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
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
            Self::GeneralBinaryExpression(_) => vec![],
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
            Self::ElementAccessExpression(_) => vec![],
            Self::PropertyAccessExpression(_) => vec![],
            Self::PrivateFieldExpression(_) => vec![],
        }
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        match self {
            Self::BooleanLiteral(e) => AstKind::BooleanLiteral(e),
            Self::NullLiteral(e) => AstKind::NullLiteral(e),
            Self::NumericLiteral(e) => AstKind::NumericLiteral(e),
            Self::BigIntLiteral(e) => AstKind::BigIntLiteral(e),
            Self::RegExpLiteral(e) => AstKind::RegExpLiteral(e),
            Self::StringLiteral(e) => AstKind::StringLiteral(e),
            Self::TemplateLiteral(e) => AstKind::TemplateLiteral(e),
            Self::Identifier(e) => AstKind::IdentifierReference(e),
            Self::MetaProperty(e) => AstKind::MetaProperty(e),
            Self::Super(e) => AstKind::Super(e),
            Self::ArrayExpression(e) => AstKind::ArrayExpression(e),
            Self::ArrowFunctionExpression(e) => AstKind::ArrowFunctionExpression(e),
            Self::AssignmentExpression(e) => AstKind::AssignmentExpression(e),
            Self::AwaitExpression(e) => AstKind::AwaitExpression(e),
            Self::GeneralBinaryExpression(e) => AstKind::GeneralBinaryExpression(e),
            Self::CallExpression(e) => AstKind::CallExpression(e),
            Self::ChainExpression(e) => AstKind::ChainExpression(e),
            Self::ClassExpression(e) => AstKind::Class(e),
            Self::ConditionalExpression(e) => AstKind::ConditionalExpression(e),
            Self::FunctionExpression(e) => AstKind::Function(e),
            Self::ImportExpression(e) => AstKind::ImportExpression(e),
            Self::LogicalExpression(e) => AstKind::LogicalExpression(e),
            Self::NewExpression(e) => AstKind::NewExpression(e),
            Self::ObjectExpression(e) => AstKind::ObjectExpression(e),
            Self::ParenthesizedExpression(e) => AstKind::ParenthesizedExpression(e),
            Self::SequenceExpression(e) => AstKind::SequenceExpression(e),
            Self::TaggedTemplateExpression(e) => AstKind::TaggedTemplateExpression(e),
            Self::ThisExpression(e) => AstKind::ThisExpression(e),
            Self::UnaryExpression(e) => AstKind::UnaryExpression(e),
            Self::UpdateExpression(e) => AstKind::UpdateExpression(e),
            Self::YieldExpression(e) => AstKind::YieldExpression(e),
            Self::PrivateInExpression(e) => AstKind::PrivateInExpression(e),
            Self::JSXElement(e) => AstKind::JSXElement(e),
            Self::JSXFragment(e) => AstKind::JSXFragment(e),
            Self::TSAsExpression(e) => AstKind::TSAsExpression(e),
            Self::TSSatisfiesExpression(e) => AstKind::TSSatisfiesExpression(e),
            Self::TSTypeAssertion(e) => AstKind::TSTypeAssertion(e),
            Self::TSNonNullExpression(e) => AstKind::TSNonNullExpression(e),
            Self::TSInstantiationExpression(e) => AstKind::TSInstantiationExpression(e),
            Self::ElementAccessExpression(e) => AstKind::ElementAccessExpression(e),
            Self::PropertyAccessExpression(e) => AstKind::PropertyAccessExpression(e),
            Self::PrivateFieldExpression(e) => AstKind::PrivateFieldExpression(e),
        }
    }
    fn get_node_id(&'a self) -> u32 {
        match self {
            Self::BooleanLiteral(e) => e.node_id,
            Self::NullLiteral(e) => e.node_id,
            Self::NumericLiteral(e) => e.node_id,
            Self::BigIntLiteral(e) => e.node_id,
            Self::RegExpLiteral(e) => e.node_id,
            Self::StringLiteral(e) => e.node_id,
            Self::TemplateLiteral(e) => e.node_id,
            Self::Identifier(e) => e.node_id,
            Self::MetaProperty(e) => e.node_id,
            Self::Super(e) => e.node_id,
            Self::ArrayExpression(e) => e.node_id,
            Self::ArrowFunctionExpression(e) => e.node_id,
            Self::AssignmentExpression(e) => e.node_id,
            Self::AwaitExpression(e) => e.node_id,
            Self::GeneralBinaryExpression(e) => e.node_id,
            Self::CallExpression(e) => e.node_id,
            Self::ChainExpression(e) => e.node_id,
            Self::ClassExpression(e) => e.node_id,
            Self::ConditionalExpression(e) => e.node_id,
            Self::FunctionExpression(e) => e.node_id,
            Self::ImportExpression(e) => e.node_id,
            Self::LogicalExpression(e) => e.node_id,
            Self::NewExpression(e) => e.node_id,
            Self::ObjectExpression(e) => e.node_id,
            Self::ParenthesizedExpression(e) => e.node_id,
            Self::SequenceExpression(e) => e.node_id,
            Self::TaggedTemplateExpression(e) => e.node_id,
            Self::ThisExpression(e) => e.node_id,
            Self::UnaryExpression(e) => e.node_id,
            Self::UpdateExpression(e) => e.node_id,
            Self::YieldExpression(e) => e.node_id,
            Self::PrivateInExpression(e) => e.node_id,
            Self::JSXElement(e) => e.node_id,
            Self::JSXFragment(e) => e.node_id,
            Self::TSAsExpression(e) => e.node_id,
            Self::TSSatisfiesExpression(e) => e.node_id,
            Self::TSTypeAssertion(e) => e.node_id,
            Self::TSNonNullExpression(e) => e.node_id,
            Self::TSInstantiationExpression(e) => e.node_id,
            Self::ElementAccessExpression(e) => e.node_id,
            Self::PropertyAccessExpression(e) => e.node_id,
            Self::PrivateFieldExpression(e) => e.node_id,
        }
    }
}

impl<'a> GetChildren<'a> for IdentifierName<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::IdentifierName(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for IdentifierReference<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::IdentifierReference(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for BindingIdentifier<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::BindingIdentifier(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for LabelIdentifier<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::LabelIdentifier(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for ThisExpression {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::ThisExpression(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for ArrayExpression<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        for item in &self.elements {
            children.push((*item).to_ast_kind());
        }
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::ArrayExpression(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for ArrayExpressionElement<'a> {
    #[allow(unused_variables, clippy::match_same_arms)]
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        match self {
            Self::SpreadElement(_) => vec![],
            Self::OmittedExpression(child) => vec![AstKind::OmittedExpression(child)],
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
            Self::GeneralBinaryExpression(_) => vec![],
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
            Self::ElementAccessExpression(_) => vec![],
            Self::PropertyAccessExpression(_) => vec![],
            Self::PrivateFieldExpression(_) => vec![],
        }
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        match self {
            Self::SpreadElement(e) => AstKind::SpreadElement(e),
            Self::OmittedExpression(e) => AstKind::OmittedExpression(e),
            Self::BooleanLiteral(e) => AstKind::BooleanLiteral(e),
            Self::NullLiteral(e) => AstKind::NullLiteral(e),
            Self::NumericLiteral(e) => AstKind::NumericLiteral(e),
            Self::BigIntLiteral(e) => AstKind::BigIntLiteral(e),
            Self::RegExpLiteral(e) => AstKind::RegExpLiteral(e),
            Self::StringLiteral(e) => AstKind::StringLiteral(e),
            Self::TemplateLiteral(e) => AstKind::TemplateLiteral(e),
            Self::Identifier(e) => AstKind::IdentifierReference(e),
            Self::MetaProperty(e) => AstKind::MetaProperty(e),
            Self::Super(e) => AstKind::Super(e),
            Self::ArrayExpression(e) => AstKind::ArrayExpression(e),
            Self::ArrowFunctionExpression(e) => AstKind::ArrowFunctionExpression(e),
            Self::AssignmentExpression(e) => AstKind::AssignmentExpression(e),
            Self::AwaitExpression(e) => AstKind::AwaitExpression(e),
            Self::GeneralBinaryExpression(e) => AstKind::GeneralBinaryExpression(e),
            Self::CallExpression(e) => AstKind::CallExpression(e),
            Self::ChainExpression(e) => AstKind::ChainExpression(e),
            Self::ClassExpression(e) => AstKind::Class(e),
            Self::ConditionalExpression(e) => AstKind::ConditionalExpression(e),
            Self::FunctionExpression(e) => AstKind::Function(e),
            Self::ImportExpression(e) => AstKind::ImportExpression(e),
            Self::LogicalExpression(e) => AstKind::LogicalExpression(e),
            Self::NewExpression(e) => AstKind::NewExpression(e),
            Self::ObjectExpression(e) => AstKind::ObjectExpression(e),
            Self::ParenthesizedExpression(e) => AstKind::ParenthesizedExpression(e),
            Self::SequenceExpression(e) => AstKind::SequenceExpression(e),
            Self::TaggedTemplateExpression(e) => AstKind::TaggedTemplateExpression(e),
            Self::ThisExpression(e) => AstKind::ThisExpression(e),
            Self::UnaryExpression(e) => AstKind::UnaryExpression(e),
            Self::UpdateExpression(e) => AstKind::UpdateExpression(e),
            Self::YieldExpression(e) => AstKind::YieldExpression(e),
            Self::PrivateInExpression(e) => AstKind::PrivateInExpression(e),
            Self::JSXElement(e) => AstKind::JSXElement(e),
            Self::JSXFragment(e) => AstKind::JSXFragment(e),
            Self::TSAsExpression(e) => AstKind::TSAsExpression(e),
            Self::TSSatisfiesExpression(e) => AstKind::TSSatisfiesExpression(e),
            Self::TSTypeAssertion(e) => AstKind::TSTypeAssertion(e),
            Self::TSNonNullExpression(e) => AstKind::TSNonNullExpression(e),
            Self::TSInstantiationExpression(e) => AstKind::TSInstantiationExpression(e),
            Self::ElementAccessExpression(e) => AstKind::ElementAccessExpression(e),
            Self::PropertyAccessExpression(e) => AstKind::PropertyAccessExpression(e),
            Self::PrivateFieldExpression(e) => AstKind::PrivateFieldExpression(e),
        }
    }
    fn get_node_id(&'a self) -> u32 {
        match self {
            Self::SpreadElement(e) => e.node_id,
            Self::OmittedExpression(e) => e.node_id,
            Self::BooleanLiteral(e) => e.node_id,
            Self::NullLiteral(e) => e.node_id,
            Self::NumericLiteral(e) => e.node_id,
            Self::BigIntLiteral(e) => e.node_id,
            Self::RegExpLiteral(e) => e.node_id,
            Self::StringLiteral(e) => e.node_id,
            Self::TemplateLiteral(e) => e.node_id,
            Self::Identifier(e) => e.node_id,
            Self::MetaProperty(e) => e.node_id,
            Self::Super(e) => e.node_id,
            Self::ArrayExpression(e) => e.node_id,
            Self::ArrowFunctionExpression(e) => e.node_id,
            Self::AssignmentExpression(e) => e.node_id,
            Self::AwaitExpression(e) => e.node_id,
            Self::GeneralBinaryExpression(e) => e.node_id,
            Self::CallExpression(e) => e.node_id,
            Self::ChainExpression(e) => e.node_id,
            Self::ClassExpression(e) => e.node_id,
            Self::ConditionalExpression(e) => e.node_id,
            Self::FunctionExpression(e) => e.node_id,
            Self::ImportExpression(e) => e.node_id,
            Self::LogicalExpression(e) => e.node_id,
            Self::NewExpression(e) => e.node_id,
            Self::ObjectExpression(e) => e.node_id,
            Self::ParenthesizedExpression(e) => e.node_id,
            Self::SequenceExpression(e) => e.node_id,
            Self::TaggedTemplateExpression(e) => e.node_id,
            Self::ThisExpression(e) => e.node_id,
            Self::UnaryExpression(e) => e.node_id,
            Self::UpdateExpression(e) => e.node_id,
            Self::YieldExpression(e) => e.node_id,
            Self::PrivateInExpression(e) => e.node_id,
            Self::JSXElement(e) => e.node_id,
            Self::JSXFragment(e) => e.node_id,
            Self::TSAsExpression(e) => e.node_id,
            Self::TSSatisfiesExpression(e) => e.node_id,
            Self::TSTypeAssertion(e) => e.node_id,
            Self::TSNonNullExpression(e) => e.node_id,
            Self::TSInstantiationExpression(e) => e.node_id,
            Self::ElementAccessExpression(e) => e.node_id,
            Self::PropertyAccessExpression(e) => e.node_id,
            Self::PrivateFieldExpression(e) => e.node_id,
        }
    }
}

impl<'a> GetChildren<'a> for OmittedExpression {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::OmittedExpression(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for ObjectExpression<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        for item in &self.properties {
            children.push((*item).to_ast_kind());
        }
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::ObjectExpression(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
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
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        match self {
            Self::ObjectProperty(e) => AstKind::ObjectProperty(e),
            Self::SpreadProperty(e) => AstKind::SpreadElement(e),
        }
    }
    fn get_node_id(&'a self) -> u32 {
        match self {
            Self::ObjectProperty(e) => e.node_id,
            Self::SpreadProperty(e) => e.node_id,
        }
    }
}

impl<'a> GetChildren<'a> for ObjectProperty<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.key).to_ast_kind());
        children.push((*&self.value).to_ast_kind());
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::ObjectProperty(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
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
            Self::GeneralBinaryExpression(_) => vec![],
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
            Self::ElementAccessExpression(_) => vec![],
            Self::PropertyAccessExpression(_) => vec![],
            Self::PrivateFieldExpression(_) => vec![],
        }
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        match self {
            Self::StaticIdentifier(e) => AstKind::IdentifierName(e),
            Self::PrivateIdentifier(e) => AstKind::PrivateIdentifier(e),
            Self::BooleanLiteral(e) => AstKind::BooleanLiteral(e),
            Self::NullLiteral(e) => AstKind::NullLiteral(e),
            Self::NumericLiteral(e) => AstKind::NumericLiteral(e),
            Self::BigIntLiteral(e) => AstKind::BigIntLiteral(e),
            Self::RegExpLiteral(e) => AstKind::RegExpLiteral(e),
            Self::StringLiteral(e) => AstKind::StringLiteral(e),
            Self::TemplateLiteral(e) => AstKind::TemplateLiteral(e),
            Self::Identifier(e) => AstKind::IdentifierReference(e),
            Self::MetaProperty(e) => AstKind::MetaProperty(e),
            Self::Super(e) => AstKind::Super(e),
            Self::ArrayExpression(e) => AstKind::ArrayExpression(e),
            Self::ArrowFunctionExpression(e) => AstKind::ArrowFunctionExpression(e),
            Self::AssignmentExpression(e) => AstKind::AssignmentExpression(e),
            Self::AwaitExpression(e) => AstKind::AwaitExpression(e),
            Self::GeneralBinaryExpression(e) => AstKind::GeneralBinaryExpression(e),
            Self::CallExpression(e) => AstKind::CallExpression(e),
            Self::ChainExpression(e) => AstKind::ChainExpression(e),
            Self::ClassExpression(e) => AstKind::Class(e),
            Self::ConditionalExpression(e) => AstKind::ConditionalExpression(e),
            Self::FunctionExpression(e) => AstKind::Function(e),
            Self::ImportExpression(e) => AstKind::ImportExpression(e),
            Self::LogicalExpression(e) => AstKind::LogicalExpression(e),
            Self::NewExpression(e) => AstKind::NewExpression(e),
            Self::ObjectExpression(e) => AstKind::ObjectExpression(e),
            Self::ParenthesizedExpression(e) => AstKind::ParenthesizedExpression(e),
            Self::SequenceExpression(e) => AstKind::SequenceExpression(e),
            Self::TaggedTemplateExpression(e) => AstKind::TaggedTemplateExpression(e),
            Self::ThisExpression(e) => AstKind::ThisExpression(e),
            Self::UnaryExpression(e) => AstKind::UnaryExpression(e),
            Self::UpdateExpression(e) => AstKind::UpdateExpression(e),
            Self::YieldExpression(e) => AstKind::YieldExpression(e),
            Self::PrivateInExpression(e) => AstKind::PrivateInExpression(e),
            Self::JSXElement(e) => AstKind::JSXElement(e),
            Self::JSXFragment(e) => AstKind::JSXFragment(e),
            Self::TSAsExpression(e) => AstKind::TSAsExpression(e),
            Self::TSSatisfiesExpression(e) => AstKind::TSSatisfiesExpression(e),
            Self::TSTypeAssertion(e) => AstKind::TSTypeAssertion(e),
            Self::TSNonNullExpression(e) => AstKind::TSNonNullExpression(e),
            Self::TSInstantiationExpression(e) => AstKind::TSInstantiationExpression(e),
            Self::ElementAccessExpression(e) => AstKind::ElementAccessExpression(e),
            Self::PropertyAccessExpression(e) => AstKind::PropertyAccessExpression(e),
            Self::PrivateFieldExpression(e) => AstKind::PrivateFieldExpression(e),
        }
    }
    fn get_node_id(&'a self) -> u32 {
        match self {
            Self::StaticIdentifier(e) => e.node_id,
            Self::PrivateIdentifier(e) => e.node_id,
            Self::BooleanLiteral(e) => e.node_id,
            Self::NullLiteral(e) => e.node_id,
            Self::NumericLiteral(e) => e.node_id,
            Self::BigIntLiteral(e) => e.node_id,
            Self::RegExpLiteral(e) => e.node_id,
            Self::StringLiteral(e) => e.node_id,
            Self::TemplateLiteral(e) => e.node_id,
            Self::Identifier(e) => e.node_id,
            Self::MetaProperty(e) => e.node_id,
            Self::Super(e) => e.node_id,
            Self::ArrayExpression(e) => e.node_id,
            Self::ArrowFunctionExpression(e) => e.node_id,
            Self::AssignmentExpression(e) => e.node_id,
            Self::AwaitExpression(e) => e.node_id,
            Self::GeneralBinaryExpression(e) => e.node_id,
            Self::CallExpression(e) => e.node_id,
            Self::ChainExpression(e) => e.node_id,
            Self::ClassExpression(e) => e.node_id,
            Self::ConditionalExpression(e) => e.node_id,
            Self::FunctionExpression(e) => e.node_id,
            Self::ImportExpression(e) => e.node_id,
            Self::LogicalExpression(e) => e.node_id,
            Self::NewExpression(e) => e.node_id,
            Self::ObjectExpression(e) => e.node_id,
            Self::ParenthesizedExpression(e) => e.node_id,
            Self::SequenceExpression(e) => e.node_id,
            Self::TaggedTemplateExpression(e) => e.node_id,
            Self::ThisExpression(e) => e.node_id,
            Self::UnaryExpression(e) => e.node_id,
            Self::UpdateExpression(e) => e.node_id,
            Self::YieldExpression(e) => e.node_id,
            Self::PrivateInExpression(e) => e.node_id,
            Self::JSXElement(e) => e.node_id,
            Self::JSXFragment(e) => e.node_id,
            Self::TSAsExpression(e) => e.node_id,
            Self::TSSatisfiesExpression(e) => e.node_id,
            Self::TSTypeAssertion(e) => e.node_id,
            Self::TSNonNullExpression(e) => e.node_id,
            Self::TSInstantiationExpression(e) => e.node_id,
            Self::ElementAccessExpression(e) => e.node_id,
            Self::PropertyAccessExpression(e) => e.node_id,
            Self::PrivateFieldExpression(e) => e.node_id,
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
            children.push((*item).to_ast_kind());
        }
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TemplateLiteral(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for TaggedTemplateExpression<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.tag).to_ast_kind());
        children.push(AstKind::TemplateLiteral(&self.quasi));
        if let Some(field) = &self.type_parameters {
            children.push(AstKind::TSTypeParameterInstantiation(field));
        }
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TaggedTemplateExpression(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for TemplateElement<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TemplateElement(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for MemberExpression<'a> {
    #[allow(unused_variables, clippy::match_same_arms)]
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        match self {
            Self::ElementAccessExpression(_) => vec![],
            Self::PropertyAccessExpression(_) => vec![],
            Self::PrivateFieldExpression(_) => vec![],
        }
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        match self {
            Self::ElementAccessExpression(e) => AstKind::ElementAccessExpression(e),
            Self::PropertyAccessExpression(e) => AstKind::PropertyAccessExpression(e),
            Self::PrivateFieldExpression(e) => AstKind::PrivateFieldExpression(e),
        }
    }
    fn get_node_id(&'a self) -> u32 {
        match self {
            Self::ElementAccessExpression(e) => e.node_id,
            Self::PropertyAccessExpression(e) => e.node_id,
            Self::PrivateFieldExpression(e) => e.node_id,
        }
    }
}

impl<'a> GetChildren<'a> for ElementAccessExpression<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.object).to_ast_kind());
        children.push((*&self.argument_expression).to_ast_kind());
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::ElementAccessExpression(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for PropertyAccessExpression<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.object).to_ast_kind());
        children.push(AstKind::IdentifierName(&self.property));
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::PropertyAccessExpression(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for PrivateFieldExpression<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.object).to_ast_kind());
        children.push(AstKind::PrivateIdentifier(&self.field));
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::PrivateFieldExpression(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for CallExpression<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.callee).to_ast_kind());
        if let Some(field) = &self.type_parameters {
            children.push(AstKind::TSTypeParameterInstantiation(field));
        }
        for item in &self.arguments {
            children.push((*item).to_ast_kind());
        }
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::CallExpression(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for NewExpression<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.callee).to_ast_kind());
        for item in &self.arguments {
            children.push((*item).to_ast_kind());
        }
        if let Some(field) = &self.type_parameters {
            children.push(AstKind::TSTypeParameterInstantiation(field));
        }
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::NewExpression(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for MetaProperty<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::IdentifierName(&self.meta));
        children.push(AstKind::IdentifierName(&self.property));
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::MetaProperty(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for SpreadElement<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.argument).to_ast_kind());
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::SpreadElement(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
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
            Self::GeneralBinaryExpression(_) => vec![],
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
            Self::ElementAccessExpression(_) => vec![],
            Self::PropertyAccessExpression(_) => vec![],
            Self::PrivateFieldExpression(_) => vec![],
        }
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        match self {
            Self::SpreadElement(e) => AstKind::SpreadElement(e),
            Self::BooleanLiteral(e) => AstKind::BooleanLiteral(e),
            Self::NullLiteral(e) => AstKind::NullLiteral(e),
            Self::NumericLiteral(e) => AstKind::NumericLiteral(e),
            Self::BigIntLiteral(e) => AstKind::BigIntLiteral(e),
            Self::RegExpLiteral(e) => AstKind::RegExpLiteral(e),
            Self::StringLiteral(e) => AstKind::StringLiteral(e),
            Self::TemplateLiteral(e) => AstKind::TemplateLiteral(e),
            Self::Identifier(e) => AstKind::IdentifierReference(e),
            Self::MetaProperty(e) => AstKind::MetaProperty(e),
            Self::Super(e) => AstKind::Super(e),
            Self::ArrayExpression(e) => AstKind::ArrayExpression(e),
            Self::ArrowFunctionExpression(e) => AstKind::ArrowFunctionExpression(e),
            Self::AssignmentExpression(e) => AstKind::AssignmentExpression(e),
            Self::AwaitExpression(e) => AstKind::AwaitExpression(e),
            Self::GeneralBinaryExpression(e) => AstKind::GeneralBinaryExpression(e),
            Self::CallExpression(e) => AstKind::CallExpression(e),
            Self::ChainExpression(e) => AstKind::ChainExpression(e),
            Self::ClassExpression(e) => AstKind::Class(e),
            Self::ConditionalExpression(e) => AstKind::ConditionalExpression(e),
            Self::FunctionExpression(e) => AstKind::Function(e),
            Self::ImportExpression(e) => AstKind::ImportExpression(e),
            Self::LogicalExpression(e) => AstKind::LogicalExpression(e),
            Self::NewExpression(e) => AstKind::NewExpression(e),
            Self::ObjectExpression(e) => AstKind::ObjectExpression(e),
            Self::ParenthesizedExpression(e) => AstKind::ParenthesizedExpression(e),
            Self::SequenceExpression(e) => AstKind::SequenceExpression(e),
            Self::TaggedTemplateExpression(e) => AstKind::TaggedTemplateExpression(e),
            Self::ThisExpression(e) => AstKind::ThisExpression(e),
            Self::UnaryExpression(e) => AstKind::UnaryExpression(e),
            Self::UpdateExpression(e) => AstKind::UpdateExpression(e),
            Self::YieldExpression(e) => AstKind::YieldExpression(e),
            Self::PrivateInExpression(e) => AstKind::PrivateInExpression(e),
            Self::JSXElement(e) => AstKind::JSXElement(e),
            Self::JSXFragment(e) => AstKind::JSXFragment(e),
            Self::TSAsExpression(e) => AstKind::TSAsExpression(e),
            Self::TSSatisfiesExpression(e) => AstKind::TSSatisfiesExpression(e),
            Self::TSTypeAssertion(e) => AstKind::TSTypeAssertion(e),
            Self::TSNonNullExpression(e) => AstKind::TSNonNullExpression(e),
            Self::TSInstantiationExpression(e) => AstKind::TSInstantiationExpression(e),
            Self::ElementAccessExpression(e) => AstKind::ElementAccessExpression(e),
            Self::PropertyAccessExpression(e) => AstKind::PropertyAccessExpression(e),
            Self::PrivateFieldExpression(e) => AstKind::PrivateFieldExpression(e),
        }
    }
    fn get_node_id(&'a self) -> u32 {
        match self {
            Self::SpreadElement(e) => e.node_id,
            Self::BooleanLiteral(e) => e.node_id,
            Self::NullLiteral(e) => e.node_id,
            Self::NumericLiteral(e) => e.node_id,
            Self::BigIntLiteral(e) => e.node_id,
            Self::RegExpLiteral(e) => e.node_id,
            Self::StringLiteral(e) => e.node_id,
            Self::TemplateLiteral(e) => e.node_id,
            Self::Identifier(e) => e.node_id,
            Self::MetaProperty(e) => e.node_id,
            Self::Super(e) => e.node_id,
            Self::ArrayExpression(e) => e.node_id,
            Self::ArrowFunctionExpression(e) => e.node_id,
            Self::AssignmentExpression(e) => e.node_id,
            Self::AwaitExpression(e) => e.node_id,
            Self::GeneralBinaryExpression(e) => e.node_id,
            Self::CallExpression(e) => e.node_id,
            Self::ChainExpression(e) => e.node_id,
            Self::ClassExpression(e) => e.node_id,
            Self::ConditionalExpression(e) => e.node_id,
            Self::FunctionExpression(e) => e.node_id,
            Self::ImportExpression(e) => e.node_id,
            Self::LogicalExpression(e) => e.node_id,
            Self::NewExpression(e) => e.node_id,
            Self::ObjectExpression(e) => e.node_id,
            Self::ParenthesizedExpression(e) => e.node_id,
            Self::SequenceExpression(e) => e.node_id,
            Self::TaggedTemplateExpression(e) => e.node_id,
            Self::ThisExpression(e) => e.node_id,
            Self::UnaryExpression(e) => e.node_id,
            Self::UpdateExpression(e) => e.node_id,
            Self::YieldExpression(e) => e.node_id,
            Self::PrivateInExpression(e) => e.node_id,
            Self::JSXElement(e) => e.node_id,
            Self::JSXFragment(e) => e.node_id,
            Self::TSAsExpression(e) => e.node_id,
            Self::TSSatisfiesExpression(e) => e.node_id,
            Self::TSTypeAssertion(e) => e.node_id,
            Self::TSNonNullExpression(e) => e.node_id,
            Self::TSInstantiationExpression(e) => e.node_id,
            Self::ElementAccessExpression(e) => e.node_id,
            Self::PropertyAccessExpression(e) => e.node_id,
            Self::PrivateFieldExpression(e) => e.node_id,
        }
    }
}

impl<'a> GetChildren<'a> for UpdateExpression<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.argument).to_ast_kind());
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::UpdateExpression(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for UnaryExpression<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.argument).to_ast_kind());
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::UnaryExpression(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for GeneralBinaryExpression<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.left).to_ast_kind());
        children.push((*&self.right).to_ast_kind());
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::GeneralBinaryExpression(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for PrivateInExpression<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::PrivateIdentifier(&self.left));
        children.push((*&self.right).to_ast_kind());
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::PrivateInExpression(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for LogicalExpression<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.left).to_ast_kind());
        children.push((*&self.right).to_ast_kind());
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::LogicalExpression(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for ConditionalExpression<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.test).to_ast_kind());
        children.push((*&self.consequent).to_ast_kind());
        children.push((*&self.alternate).to_ast_kind());
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::ConditionalExpression(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for AssignmentExpression<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.left).to_ast_kind());
        children.push((*&self.right).to_ast_kind());
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::AssignmentExpression(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
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
            Self::ElementAccessExpression(_) => vec![],
            Self::PropertyAccessExpression(_) => vec![],
            Self::PrivateFieldExpression(_) => vec![],
            Self::ArrayAssignmentTarget(_) => vec![],
            Self::ObjectAssignmentTarget(_) => vec![],
        }
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        match self {
            Self::AssignmentTargetIdentifier(e) => AstKind::IdentifierReference(e),
            Self::TSAsExpression(e) => AstKind::TSAsExpression(e),
            Self::TSSatisfiesExpression(e) => AstKind::TSSatisfiesExpression(e),
            Self::TSNonNullExpression(e) => AstKind::TSNonNullExpression(e),
            Self::TSTypeAssertion(e) => AstKind::TSTypeAssertion(e),
            Self::TSInstantiationExpression(e) => AstKind::TSInstantiationExpression(e),
            Self::ElementAccessExpression(e) => AstKind::ElementAccessExpression(e),
            Self::PropertyAccessExpression(e) => AstKind::PropertyAccessExpression(e),
            Self::PrivateFieldExpression(e) => AstKind::PrivateFieldExpression(e),
            Self::ArrayAssignmentTarget(e) => AstKind::ArrayAssignmentTarget(e),
            Self::ObjectAssignmentTarget(e) => AstKind::ObjectAssignmentTarget(e),
        }
    }
    fn get_node_id(&'a self) -> u32 {
        match self {
            Self::AssignmentTargetIdentifier(e) => e.node_id,
            Self::TSAsExpression(e) => e.node_id,
            Self::TSSatisfiesExpression(e) => e.node_id,
            Self::TSNonNullExpression(e) => e.node_id,
            Self::TSTypeAssertion(e) => e.node_id,
            Self::TSInstantiationExpression(e) => e.node_id,
            Self::ElementAccessExpression(e) => e.node_id,
            Self::PropertyAccessExpression(e) => e.node_id,
            Self::PrivateFieldExpression(e) => e.node_id,
            Self::ArrayAssignmentTarget(e) => e.node_id,
            Self::ObjectAssignmentTarget(e) => e.node_id,
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
            Self::ElementAccessExpression(_) => vec![],
            Self::PropertyAccessExpression(_) => vec![],
            Self::PrivateFieldExpression(_) => vec![],
        }
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        match self {
            Self::AssignmentTargetIdentifier(e) => AstKind::IdentifierReference(e),
            Self::TSAsExpression(e) => AstKind::TSAsExpression(e),
            Self::TSSatisfiesExpression(e) => AstKind::TSSatisfiesExpression(e),
            Self::TSNonNullExpression(e) => AstKind::TSNonNullExpression(e),
            Self::TSTypeAssertion(e) => AstKind::TSTypeAssertion(e),
            Self::TSInstantiationExpression(e) => AstKind::TSInstantiationExpression(e),
            Self::ElementAccessExpression(e) => AstKind::ElementAccessExpression(e),
            Self::PropertyAccessExpression(e) => AstKind::PropertyAccessExpression(e),
            Self::PrivateFieldExpression(e) => AstKind::PrivateFieldExpression(e),
        }
    }
    fn get_node_id(&'a self) -> u32 {
        match self {
            Self::AssignmentTargetIdentifier(e) => e.node_id,
            Self::TSAsExpression(e) => e.node_id,
            Self::TSSatisfiesExpression(e) => e.node_id,
            Self::TSNonNullExpression(e) => e.node_id,
            Self::TSTypeAssertion(e) => e.node_id,
            Self::TSInstantiationExpression(e) => e.node_id,
            Self::ElementAccessExpression(e) => e.node_id,
            Self::PropertyAccessExpression(e) => e.node_id,
            Self::PrivateFieldExpression(e) => e.node_id,
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
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        match self {
            Self::ArrayAssignmentTarget(e) => AstKind::ArrayAssignmentTarget(e),
            Self::ObjectAssignmentTarget(e) => AstKind::ObjectAssignmentTarget(e),
        }
    }
    fn get_node_id(&'a self) -> u32 {
        match self {
            Self::ArrayAssignmentTarget(e) => e.node_id,
            Self::ObjectAssignmentTarget(e) => e.node_id,
        }
    }
}

impl<'a> GetChildren<'a> for ArrayAssignmentTarget<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        for opt_item in &self.elements {
            if let Some(item) = opt_item {
                children.push((*item).to_ast_kind());
            }
        }
        if let Some(field) = &self.rest {
            children.push(AstKind::AssignmentTargetRest(field));
        }
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::ArrayAssignmentTarget(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for ObjectAssignmentTarget<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        for item in &self.properties {
            children.push((*item).to_ast_kind());
        }
        if let Some(field) = &self.rest {
            children.push(AstKind::AssignmentTargetRest(field));
        }
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::ObjectAssignmentTarget(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for AssignmentTargetRest<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.target).to_ast_kind());
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::AssignmentTargetRest(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
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
            Self::ElementAccessExpression(_) => vec![],
            Self::PropertyAccessExpression(_) => vec![],
            Self::PrivateFieldExpression(_) => vec![],
            Self::ArrayAssignmentTarget(_) => vec![],
            Self::ObjectAssignmentTarget(_) => vec![],
        }
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        match self {
            Self::AssignmentTargetWithDefault(e) => AstKind::AssignmentTargetWithDefault(e),
            Self::AssignmentTargetIdentifier(e) => AstKind::IdentifierReference(e),
            Self::TSAsExpression(e) => AstKind::TSAsExpression(e),
            Self::TSSatisfiesExpression(e) => AstKind::TSSatisfiesExpression(e),
            Self::TSNonNullExpression(e) => AstKind::TSNonNullExpression(e),
            Self::TSTypeAssertion(e) => AstKind::TSTypeAssertion(e),
            Self::TSInstantiationExpression(e) => AstKind::TSInstantiationExpression(e),
            Self::ElementAccessExpression(e) => AstKind::ElementAccessExpression(e),
            Self::PropertyAccessExpression(e) => AstKind::PropertyAccessExpression(e),
            Self::PrivateFieldExpression(e) => AstKind::PrivateFieldExpression(e),
            Self::ArrayAssignmentTarget(e) => AstKind::ArrayAssignmentTarget(e),
            Self::ObjectAssignmentTarget(e) => AstKind::ObjectAssignmentTarget(e),
        }
    }
    fn get_node_id(&'a self) -> u32 {
        match self {
            Self::AssignmentTargetWithDefault(e) => e.node_id,
            Self::AssignmentTargetIdentifier(e) => e.node_id,
            Self::TSAsExpression(e) => e.node_id,
            Self::TSSatisfiesExpression(e) => e.node_id,
            Self::TSNonNullExpression(e) => e.node_id,
            Self::TSTypeAssertion(e) => e.node_id,
            Self::TSInstantiationExpression(e) => e.node_id,
            Self::ElementAccessExpression(e) => e.node_id,
            Self::PropertyAccessExpression(e) => e.node_id,
            Self::PrivateFieldExpression(e) => e.node_id,
            Self::ArrayAssignmentTarget(e) => e.node_id,
            Self::ObjectAssignmentTarget(e) => e.node_id,
        }
    }
}

impl<'a> GetChildren<'a> for AssignmentTargetWithDefault<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.binding).to_ast_kind());
        children.push((*&self.init).to_ast_kind());
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::AssignmentTargetWithDefault(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
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
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        match self {
            Self::AssignmentTargetPropertyIdentifier(e) => {
                AstKind::AssignmentTargetPropertyIdentifier(e)
            }
            Self::AssignmentTargetPropertyProperty(e) => {
                AstKind::AssignmentTargetPropertyProperty(e)
            }
        }
    }
    fn get_node_id(&'a self) -> u32 {
        match self {
            Self::AssignmentTargetPropertyIdentifier(e) => e.node_id,
            Self::AssignmentTargetPropertyProperty(e) => e.node_id,
        }
    }
}

impl<'a> GetChildren<'a> for AssignmentTargetPropertyIdentifier<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::IdentifierReference(&self.binding));
        if let Some(field) = &self.init {
            children.push((*field).to_ast_kind());
        }
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::AssignmentTargetPropertyIdentifier(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for AssignmentTargetPropertyProperty<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.name).to_ast_kind());
        children.push((*&self.binding).to_ast_kind());
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::AssignmentTargetPropertyProperty(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for SequenceExpression<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        for item in &self.expressions {
            children.push((*item).to_ast_kind());
        }
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::SequenceExpression(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for Super {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::Super(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for AwaitExpression<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.argument).to_ast_kind());
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::AwaitExpression(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for ChainExpression<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.expression).to_ast_kind());
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::ChainExpression(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for ChainElement<'a> {
    #[allow(unused_variables, clippy::match_same_arms)]
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        match self {
            Self::CallExpression(_) => vec![],
            Self::TSNonNullExpression(_) => vec![],
            Self::ElementAccessExpression(_) => vec![],
            Self::PropertyAccessExpression(_) => vec![],
            Self::PrivateFieldExpression(_) => vec![],
        }
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        match self {
            Self::CallExpression(e) => AstKind::CallExpression(e),
            Self::TSNonNullExpression(e) => AstKind::TSNonNullExpression(e),
            Self::ElementAccessExpression(e) => AstKind::ElementAccessExpression(e),
            Self::PropertyAccessExpression(e) => AstKind::PropertyAccessExpression(e),
            Self::PrivateFieldExpression(e) => AstKind::PrivateFieldExpression(e),
        }
    }
    fn get_node_id(&'a self) -> u32 {
        match self {
            Self::CallExpression(e) => e.node_id,
            Self::TSNonNullExpression(e) => e.node_id,
            Self::ElementAccessExpression(e) => e.node_id,
            Self::PropertyAccessExpression(e) => e.node_id,
            Self::PrivateFieldExpression(e) => e.node_id,
        }
    }
}

impl<'a> GetChildren<'a> for ParenthesizedExpression<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.expression).to_ast_kind());
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::ParenthesizedExpression(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
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
            Self::VariableDeclarationList(_) => vec![],
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
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        match self {
            Self::BlockStatement(e) => AstKind::BlockStatement(e),
            Self::BreakStatement(e) => AstKind::BreakStatement(e),
            Self::ContinueStatement(e) => AstKind::ContinueStatement(e),
            Self::DebuggerStatement(e) => AstKind::DebuggerStatement(e),
            Self::DoWhileStatement(e) => AstKind::DoWhileStatement(e),
            Self::EmptyStatement(e) => AstKind::EmptyStatement(e),
            Self::ExpressionStatement(e) => AstKind::ExpressionStatement(e),
            Self::ForInStatement(e) => AstKind::ForInStatement(e),
            Self::ForOfStatement(e) => AstKind::ForOfStatement(e),
            Self::ForStatement(e) => AstKind::ForStatement(e),
            Self::IfStatement(e) => AstKind::IfStatement(e),
            Self::LabeledStatement(e) => AstKind::LabeledStatement(e),
            Self::ReturnStatement(e) => AstKind::ReturnStatement(e),
            Self::SwitchStatement(e) => AstKind::SwitchStatement(e),
            Self::ThrowStatement(e) => AstKind::ThrowStatement(e),
            Self::TryStatement(e) => AstKind::TryStatement(e),
            Self::WhileStatement(e) => AstKind::WhileStatement(e),
            Self::WithStatement(e) => AstKind::WithStatement(e),
            Self::VariableDeclarationList(e) => AstKind::VariableDeclarationList(e),
            Self::FunctionDeclaration(e) => AstKind::Function(e),
            Self::ClassDeclaration(e) => AstKind::Class(e),
            Self::TSTypeAliasDeclaration(e) => AstKind::TSTypeAliasDeclaration(e),
            Self::TSInterfaceDeclaration(e) => AstKind::TSInterfaceDeclaration(e),
            Self::TSEnumDeclaration(e) => AstKind::TSEnumDeclaration(e),
            Self::TSModuleDeclaration(e) => AstKind::TSModuleDeclaration(e),
            Self::TSImportEqualsDeclaration(e) => AstKind::TSImportEqualsDeclaration(e),
            Self::ImportDeclaration(e) => AstKind::ImportDeclaration(e),
            Self::ExportAllDeclaration(e) => AstKind::ExportAllDeclaration(e),
            Self::ExportDefaultDeclaration(e) => AstKind::ExportDefaultDeclaration(e),
            Self::ExportNamedDeclaration(e) => AstKind::ExportNamedDeclaration(e),
            Self::TSExportAssignment(e) => AstKind::TSExportAssignment(e),
            Self::TSNamespaceExportDeclaration(e) => AstKind::TSNamespaceExportDeclaration(e),
        }
    }
    fn get_node_id(&'a self) -> u32 {
        match self {
            Self::BlockStatement(e) => e.node_id,
            Self::BreakStatement(e) => e.node_id,
            Self::ContinueStatement(e) => e.node_id,
            Self::DebuggerStatement(e) => e.node_id,
            Self::DoWhileStatement(e) => e.node_id,
            Self::EmptyStatement(e) => e.node_id,
            Self::ExpressionStatement(e) => e.node_id,
            Self::ForInStatement(e) => e.node_id,
            Self::ForOfStatement(e) => e.node_id,
            Self::ForStatement(e) => e.node_id,
            Self::IfStatement(e) => e.node_id,
            Self::LabeledStatement(e) => e.node_id,
            Self::ReturnStatement(e) => e.node_id,
            Self::SwitchStatement(e) => e.node_id,
            Self::ThrowStatement(e) => e.node_id,
            Self::TryStatement(e) => e.node_id,
            Self::WhileStatement(e) => e.node_id,
            Self::WithStatement(e) => e.node_id,
            Self::VariableDeclarationList(e) => e.node_id,
            Self::FunctionDeclaration(e) => e.node_id,
            Self::ClassDeclaration(e) => e.node_id,
            Self::TSTypeAliasDeclaration(e) => e.node_id,
            Self::TSInterfaceDeclaration(e) => e.node_id,
            Self::TSEnumDeclaration(e) => e.node_id,
            Self::TSModuleDeclaration(e) => e.node_id,
            Self::TSImportEqualsDeclaration(e) => e.node_id,
            Self::ImportDeclaration(e) => e.node_id,
            Self::ExportAllDeclaration(e) => e.node_id,
            Self::ExportDefaultDeclaration(e) => e.node_id,
            Self::ExportNamedDeclaration(e) => e.node_id,
            Self::TSExportAssignment(e) => e.node_id,
            Self::TSNamespaceExportDeclaration(e) => e.node_id,
        }
    }
}

impl<'a> GetChildren<'a> for Directive<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::StringLiteral(&self.expression));
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::Directive(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for Hashbang<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::Hashbang(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for BlockStatement<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        for item in &self.body {
            children.push((*item).to_ast_kind());
        }
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::BlockStatement(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for Declaration<'a> {
    #[allow(unused_variables, clippy::match_same_arms)]
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        match self {
            Self::VariableDeclarationList(_) => vec![],
            Self::FunctionDeclaration(_) => vec![],
            Self::ClassDeclaration(_) => vec![],
            Self::TSTypeAliasDeclaration(_) => vec![],
            Self::TSInterfaceDeclaration(_) => vec![],
            Self::TSEnumDeclaration(_) => vec![],
            Self::TSModuleDeclaration(_) => vec![],
            Self::TSImportEqualsDeclaration(_) => vec![],
        }
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        match self {
            Self::VariableDeclarationList(e) => AstKind::VariableDeclarationList(e),
            Self::FunctionDeclaration(e) => AstKind::Function(e),
            Self::ClassDeclaration(e) => AstKind::Class(e),
            Self::TSTypeAliasDeclaration(e) => AstKind::TSTypeAliasDeclaration(e),
            Self::TSInterfaceDeclaration(e) => AstKind::TSInterfaceDeclaration(e),
            Self::TSEnumDeclaration(e) => AstKind::TSEnumDeclaration(e),
            Self::TSModuleDeclaration(e) => AstKind::TSModuleDeclaration(e),
            Self::TSImportEqualsDeclaration(e) => AstKind::TSImportEqualsDeclaration(e),
        }
    }
    fn get_node_id(&'a self) -> u32 {
        match self {
            Self::VariableDeclarationList(e) => e.node_id,
            Self::FunctionDeclaration(e) => e.node_id,
            Self::ClassDeclaration(e) => e.node_id,
            Self::TSTypeAliasDeclaration(e) => e.node_id,
            Self::TSInterfaceDeclaration(e) => e.node_id,
            Self::TSEnumDeclaration(e) => e.node_id,
            Self::TSModuleDeclaration(e) => e.node_id,
            Self::TSImportEqualsDeclaration(e) => e.node_id,
        }
    }
}

impl<'a> GetChildren<'a> for VariableDeclarationList<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        for item in &self.declarations {
            children.push(AstKind::VariableDeclarator(item));
        }
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::VariableDeclarationList(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for VariableDeclarator<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::DestructureBindingPattern(&self.id));
        if let Some(field) = &self.init {
            children.push((*field).to_ast_kind());
        }
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::VariableDeclarator(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for EmptyStatement {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::EmptyStatement(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for ExpressionStatement<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.expression).to_ast_kind());
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::ExpressionStatement(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for IfStatement<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.test).to_ast_kind());
        children.push((*&self.consequent).to_ast_kind());
        if let Some(field) = &self.alternate {
            children.push((*field).to_ast_kind());
        }
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::IfStatement(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for DoWhileStatement<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.body).to_ast_kind());
        children.push((*&self.test).to_ast_kind());
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::DoWhileStatement(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for WhileStatement<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.test).to_ast_kind());
        children.push((*&self.body).to_ast_kind());
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::WhileStatement(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for ForStatement<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        if let Some(field) = &self.init {
            children.push((*field).to_ast_kind());
        }
        if let Some(field) = &self.test {
            children.push((*field).to_ast_kind());
        }
        if let Some(field) = &self.update {
            children.push((*field).to_ast_kind());
        }
        children.push((*&self.body).to_ast_kind());
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::ForStatement(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for ForStatementInit<'a> {
    #[allow(unused_variables, clippy::match_same_arms)]
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        match self {
            Self::VariableDeclarationList(_) => vec![],
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
            Self::GeneralBinaryExpression(_) => vec![],
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
            Self::ElementAccessExpression(_) => vec![],
            Self::PropertyAccessExpression(_) => vec![],
            Self::PrivateFieldExpression(_) => vec![],
        }
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        match self {
            Self::VariableDeclarationList(e) => AstKind::VariableDeclarationList(e),
            Self::BooleanLiteral(e) => AstKind::BooleanLiteral(e),
            Self::NullLiteral(e) => AstKind::NullLiteral(e),
            Self::NumericLiteral(e) => AstKind::NumericLiteral(e),
            Self::BigIntLiteral(e) => AstKind::BigIntLiteral(e),
            Self::RegExpLiteral(e) => AstKind::RegExpLiteral(e),
            Self::StringLiteral(e) => AstKind::StringLiteral(e),
            Self::TemplateLiteral(e) => AstKind::TemplateLiteral(e),
            Self::Identifier(e) => AstKind::IdentifierReference(e),
            Self::MetaProperty(e) => AstKind::MetaProperty(e),
            Self::Super(e) => AstKind::Super(e),
            Self::ArrayExpression(e) => AstKind::ArrayExpression(e),
            Self::ArrowFunctionExpression(e) => AstKind::ArrowFunctionExpression(e),
            Self::AssignmentExpression(e) => AstKind::AssignmentExpression(e),
            Self::AwaitExpression(e) => AstKind::AwaitExpression(e),
            Self::GeneralBinaryExpression(e) => AstKind::GeneralBinaryExpression(e),
            Self::CallExpression(e) => AstKind::CallExpression(e),
            Self::ChainExpression(e) => AstKind::ChainExpression(e),
            Self::ClassExpression(e) => AstKind::Class(e),
            Self::ConditionalExpression(e) => AstKind::ConditionalExpression(e),
            Self::FunctionExpression(e) => AstKind::Function(e),
            Self::ImportExpression(e) => AstKind::ImportExpression(e),
            Self::LogicalExpression(e) => AstKind::LogicalExpression(e),
            Self::NewExpression(e) => AstKind::NewExpression(e),
            Self::ObjectExpression(e) => AstKind::ObjectExpression(e),
            Self::ParenthesizedExpression(e) => AstKind::ParenthesizedExpression(e),
            Self::SequenceExpression(e) => AstKind::SequenceExpression(e),
            Self::TaggedTemplateExpression(e) => AstKind::TaggedTemplateExpression(e),
            Self::ThisExpression(e) => AstKind::ThisExpression(e),
            Self::UnaryExpression(e) => AstKind::UnaryExpression(e),
            Self::UpdateExpression(e) => AstKind::UpdateExpression(e),
            Self::YieldExpression(e) => AstKind::YieldExpression(e),
            Self::PrivateInExpression(e) => AstKind::PrivateInExpression(e),
            Self::JSXElement(e) => AstKind::JSXElement(e),
            Self::JSXFragment(e) => AstKind::JSXFragment(e),
            Self::TSAsExpression(e) => AstKind::TSAsExpression(e),
            Self::TSSatisfiesExpression(e) => AstKind::TSSatisfiesExpression(e),
            Self::TSTypeAssertion(e) => AstKind::TSTypeAssertion(e),
            Self::TSNonNullExpression(e) => AstKind::TSNonNullExpression(e),
            Self::TSInstantiationExpression(e) => AstKind::TSInstantiationExpression(e),
            Self::ElementAccessExpression(e) => AstKind::ElementAccessExpression(e),
            Self::PropertyAccessExpression(e) => AstKind::PropertyAccessExpression(e),
            Self::PrivateFieldExpression(e) => AstKind::PrivateFieldExpression(e),
        }
    }
    fn get_node_id(&'a self) -> u32 {
        match self {
            Self::VariableDeclarationList(e) => e.node_id,
            Self::BooleanLiteral(e) => e.node_id,
            Self::NullLiteral(e) => e.node_id,
            Self::NumericLiteral(e) => e.node_id,
            Self::BigIntLiteral(e) => e.node_id,
            Self::RegExpLiteral(e) => e.node_id,
            Self::StringLiteral(e) => e.node_id,
            Self::TemplateLiteral(e) => e.node_id,
            Self::Identifier(e) => e.node_id,
            Self::MetaProperty(e) => e.node_id,
            Self::Super(e) => e.node_id,
            Self::ArrayExpression(e) => e.node_id,
            Self::ArrowFunctionExpression(e) => e.node_id,
            Self::AssignmentExpression(e) => e.node_id,
            Self::AwaitExpression(e) => e.node_id,
            Self::GeneralBinaryExpression(e) => e.node_id,
            Self::CallExpression(e) => e.node_id,
            Self::ChainExpression(e) => e.node_id,
            Self::ClassExpression(e) => e.node_id,
            Self::ConditionalExpression(e) => e.node_id,
            Self::FunctionExpression(e) => e.node_id,
            Self::ImportExpression(e) => e.node_id,
            Self::LogicalExpression(e) => e.node_id,
            Self::NewExpression(e) => e.node_id,
            Self::ObjectExpression(e) => e.node_id,
            Self::ParenthesizedExpression(e) => e.node_id,
            Self::SequenceExpression(e) => e.node_id,
            Self::TaggedTemplateExpression(e) => e.node_id,
            Self::ThisExpression(e) => e.node_id,
            Self::UnaryExpression(e) => e.node_id,
            Self::UpdateExpression(e) => e.node_id,
            Self::YieldExpression(e) => e.node_id,
            Self::PrivateInExpression(e) => e.node_id,
            Self::JSXElement(e) => e.node_id,
            Self::JSXFragment(e) => e.node_id,
            Self::TSAsExpression(e) => e.node_id,
            Self::TSSatisfiesExpression(e) => e.node_id,
            Self::TSTypeAssertion(e) => e.node_id,
            Self::TSNonNullExpression(e) => e.node_id,
            Self::TSInstantiationExpression(e) => e.node_id,
            Self::ElementAccessExpression(e) => e.node_id,
            Self::PropertyAccessExpression(e) => e.node_id,
            Self::PrivateFieldExpression(e) => e.node_id,
        }
    }
}

impl<'a> GetChildren<'a> for ForInStatement<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.left).to_ast_kind());
        children.push((*&self.right).to_ast_kind());
        children.push((*&self.body).to_ast_kind());
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::ForInStatement(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for ForStatementLeft<'a> {
    #[allow(unused_variables, clippy::match_same_arms)]
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        match self {
            Self::VariableDeclarationList(_) => vec![],
            Self::AssignmentTargetIdentifier(_) => vec![],
            Self::TSAsExpression(_) => vec![],
            Self::TSSatisfiesExpression(_) => vec![],
            Self::TSNonNullExpression(_) => vec![],
            Self::TSTypeAssertion(_) => vec![],
            Self::TSInstantiationExpression(_) => vec![],
            Self::ElementAccessExpression(_) => vec![],
            Self::PropertyAccessExpression(_) => vec![],
            Self::PrivateFieldExpression(_) => vec![],
            Self::ArrayAssignmentTarget(_) => vec![],
            Self::ObjectAssignmentTarget(_) => vec![],
        }
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        match self {
            Self::VariableDeclarationList(e) => AstKind::VariableDeclarationList(e),
            Self::AssignmentTargetIdentifier(e) => AstKind::IdentifierReference(e),
            Self::TSAsExpression(e) => AstKind::TSAsExpression(e),
            Self::TSSatisfiesExpression(e) => AstKind::TSSatisfiesExpression(e),
            Self::TSNonNullExpression(e) => AstKind::TSNonNullExpression(e),
            Self::TSTypeAssertion(e) => AstKind::TSTypeAssertion(e),
            Self::TSInstantiationExpression(e) => AstKind::TSInstantiationExpression(e),
            Self::ElementAccessExpression(e) => AstKind::ElementAccessExpression(e),
            Self::PropertyAccessExpression(e) => AstKind::PropertyAccessExpression(e),
            Self::PrivateFieldExpression(e) => AstKind::PrivateFieldExpression(e),
            Self::ArrayAssignmentTarget(e) => AstKind::ArrayAssignmentTarget(e),
            Self::ObjectAssignmentTarget(e) => AstKind::ObjectAssignmentTarget(e),
        }
    }
    fn get_node_id(&'a self) -> u32 {
        match self {
            Self::VariableDeclarationList(e) => e.node_id,
            Self::AssignmentTargetIdentifier(e) => e.node_id,
            Self::TSAsExpression(e) => e.node_id,
            Self::TSSatisfiesExpression(e) => e.node_id,
            Self::TSNonNullExpression(e) => e.node_id,
            Self::TSTypeAssertion(e) => e.node_id,
            Self::TSInstantiationExpression(e) => e.node_id,
            Self::ElementAccessExpression(e) => e.node_id,
            Self::PropertyAccessExpression(e) => e.node_id,
            Self::PrivateFieldExpression(e) => e.node_id,
            Self::ArrayAssignmentTarget(e) => e.node_id,
            Self::ObjectAssignmentTarget(e) => e.node_id,
        }
    }
}

impl<'a> GetChildren<'a> for ForOfStatement<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.left).to_ast_kind());
        children.push((*&self.right).to_ast_kind());
        children.push((*&self.body).to_ast_kind());
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::ForOfStatement(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
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
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::ContinueStatement(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
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
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::BreakStatement(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for ReturnStatement<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        if let Some(field) = &self.argument {
            children.push((*field).to_ast_kind());
        }
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::ReturnStatement(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for WithStatement<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.object).to_ast_kind());
        children.push((*&self.body).to_ast_kind());
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::WithStatement(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for SwitchStatement<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.discriminant).to_ast_kind());
        for item in &self.cases {
            children.push(AstKind::SwitchCase(item));
        }
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::SwitchStatement(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for SwitchCase<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        if let Some(field) = &self.test {
            children.push((*field).to_ast_kind());
        }
        for item in &self.consequent {
            children.push((*item).to_ast_kind());
        }
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::SwitchCase(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for LabeledStatement<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::LabelIdentifier(&self.label));
        children.push((*&self.body).to_ast_kind());
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::LabeledStatement(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for ThrowStatement<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.argument).to_ast_kind());
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::ThrowStatement(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
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
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TryStatement(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
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
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::CatchClause(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for CatchParameter<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::DestructureBindingPattern(&self.pattern));
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::CatchParameter(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for DebuggerStatement {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::DebuggerStatement(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for DestructureBindingPattern<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.kind).to_ast_kind());
        if let Some(field) = &self.type_annotation {
            children.push(AstKind::TSTypeAnnotation(field));
        }
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::DestructureBindingPattern(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for DestructureBindingPatternKind<'a> {
    #[allow(unused_variables, clippy::match_same_arms)]
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        match self {
            Self::BindingIdentifier(_) => vec![],
            Self::ObjectPattern(_) => vec![],
            Self::ArrayPattern(_) => vec![],
            Self::AssignmentPattern(_) => vec![],
        }
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        match self {
            Self::BindingIdentifier(e) => AstKind::BindingIdentifier(e),
            Self::ObjectPattern(e) => AstKind::ObjectPattern(e),
            Self::ArrayPattern(e) => AstKind::ArrayPattern(e),
            Self::AssignmentPattern(e) => AstKind::AssignmentPattern(e),
        }
    }
    fn get_node_id(&'a self) -> u32 {
        match self {
            Self::BindingIdentifier(e) => e.node_id,
            Self::ObjectPattern(e) => e.node_id,
            Self::ArrayPattern(e) => e.node_id,
            Self::AssignmentPattern(e) => e.node_id,
        }
    }
}

impl<'a> GetChildren<'a> for AssignmentPattern<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::DestructureBindingPattern(&self.left));
        children.push((*&self.right).to_ast_kind());
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::AssignmentPattern(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
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
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::ObjectPattern(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for BindingProperty<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.key).to_ast_kind());
        children.push(AstKind::DestructureBindingPattern(&self.value));
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::BindingProperty(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for ArrayPattern<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        for item in &self.elements {
            children.push(AstKind::ArrayPatternElement(item));
        }
        if let Some(field) = &self.rest {
            children.push(AstKind::BindingRestElement(field));
        }
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::ArrayPattern(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for ArrayPatternElement<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        if let Some(field) = &self.element {
            children.push(AstKind::DestructureBindingPattern(field));
        }
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::ArrayPatternElement(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for BindingRestElement<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::DestructureBindingPattern(&self.argument));
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::BindingRestElement(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
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
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::Function(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
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
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::FormalParameters(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for FormalParameter<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        for item in &self.decorators {
            children.push(AstKind::Decorator(item));
        }
        children.push(AstKind::DestructureBindingPattern(&self.pattern));
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::FormalParameter(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for FunctionBody<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        for item in &self.directives {
            children.push(AstKind::Directive(item));
        }
        for item in &self.statements {
            children.push((*item).to_ast_kind());
        }
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::FunctionBody(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
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
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::ArrowFunctionExpression(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for YieldExpression<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        if let Some(field) = &self.argument {
            children.push((*field).to_ast_kind());
        }
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::YieldExpression(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
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
            children.push((*field).to_ast_kind());
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
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::Class(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for ClassBody<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        for item in &self.body {
            children.push((*item).to_ast_kind());
        }
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::ClassBody(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
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
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        match self {
            Self::StaticBlock(e) => AstKind::StaticBlock(e),
            Self::MethodDefinition(e) => AstKind::MethodDefinition(e),
            Self::PropertyDefinition(e) => AstKind::PropertyDefinition(e),
            Self::AccessorProperty(e) => AstKind::AccessorProperty(e),
            Self::TSIndexSignature(e) => AstKind::TSIndexSignature(e),
        }
    }
    fn get_node_id(&'a self) -> u32 {
        match self {
            Self::StaticBlock(e) => e.node_id,
            Self::MethodDefinition(e) => e.node_id,
            Self::PropertyDefinition(e) => e.node_id,
            Self::AccessorProperty(e) => e.node_id,
            Self::TSIndexSignature(e) => e.node_id,
        }
    }
}

impl<'a> GetChildren<'a> for MethodDefinition<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        for item in &self.decorators {
            children.push(AstKind::Decorator(item));
        }
        children.push((*&self.key).to_ast_kind());
        children.push(AstKind::Function(&*self.value));
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::MethodDefinition(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for PropertyDefinition<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        for item in &self.decorators {
            children.push(AstKind::Decorator(item));
        }
        children.push((*&self.key).to_ast_kind());
        if let Some(field) = &self.value {
            children.push((*field).to_ast_kind());
        }
        if let Some(field) = &self.type_annotation {
            children.push(AstKind::TSTypeAnnotation(field));
        }
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::PropertyDefinition(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for PrivateIdentifier<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::PrivateIdentifier(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for StaticBlock<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        for item in &self.body {
            children.push((*item).to_ast_kind());
        }
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::StaticBlock(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
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
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        match self {
            Self::ImportDeclaration(e) => AstKind::ImportDeclaration(e),
            Self::ExportAllDeclaration(e) => AstKind::ExportAllDeclaration(e),
            Self::ExportDefaultDeclaration(e) => AstKind::ExportDefaultDeclaration(e),
            Self::ExportNamedDeclaration(e) => AstKind::ExportNamedDeclaration(e),
            Self::TSExportAssignment(e) => AstKind::TSExportAssignment(e),
            Self::TSNamespaceExportDeclaration(e) => AstKind::TSNamespaceExportDeclaration(e),
        }
    }
    fn get_node_id(&'a self) -> u32 {
        match self {
            Self::ImportDeclaration(e) => e.node_id,
            Self::ExportAllDeclaration(e) => e.node_id,
            Self::ExportDefaultDeclaration(e) => e.node_id,
            Self::ExportNamedDeclaration(e) => e.node_id,
            Self::TSExportAssignment(e) => e.node_id,
            Self::TSNamespaceExportDeclaration(e) => e.node_id,
        }
    }
}

impl<'a> GetChildren<'a> for AccessorProperty<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        for item in &self.decorators {
            children.push(AstKind::Decorator(item));
        }
        children.push((*&self.key).to_ast_kind());
        if let Some(field) = &self.value {
            children.push((*field).to_ast_kind());
        }
        if let Some(field) = &self.type_annotation {
            children.push(AstKind::TSTypeAnnotation(field));
        }
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::AccessorProperty(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for ImportExpression<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.source).to_ast_kind());
        for item in &self.arguments {
            children.push((*item).to_ast_kind());
        }
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::ImportExpression(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for ImportDeclaration<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        if let Some(vec) = &self.specifiers {
            for item in vec {
                children.push((*item).to_ast_kind());
            }
        }
        children.push(AstKind::StringLiteral(&self.source));
        if let Some(field) = &self.with_clause {
            children.push(AstKind::WithClause(field));
        }
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::ImportDeclaration(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
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
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        match self {
            Self::ImportSpecifier(e) => AstKind::ImportSpecifier(e),
            Self::ImportDefaultSpecifier(e) => AstKind::ImportDefaultSpecifier(e),
            Self::ImportNamespaceSpecifier(e) => AstKind::ImportNamespaceSpecifier(e),
        }
    }
    fn get_node_id(&'a self) -> u32 {
        match self {
            Self::ImportSpecifier(e) => e.node_id,
            Self::ImportDefaultSpecifier(e) => e.node_id,
            Self::ImportNamespaceSpecifier(e) => e.node_id,
        }
    }
}

impl<'a> GetChildren<'a> for ImportSpecifier<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.imported).to_ast_kind());
        children.push(AstKind::BindingIdentifier(&self.local));
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::ImportSpecifier(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for ImportDefaultSpecifier<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::BindingIdentifier(&self.local));
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::ImportDefaultSpecifier(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for ImportNamespaceSpecifier<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::BindingIdentifier(&self.local));
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::ImportNamespaceSpecifier(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
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
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::WithClause(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for ImportAttribute<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.key).to_ast_kind());
        children.push(AstKind::StringLiteral(&self.value));
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::ImportAttribute(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
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
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        match self {
            Self::Identifier(e) => AstKind::IdentifierName(e),
            Self::StringLiteral(e) => AstKind::StringLiteral(e),
        }
    }
    fn get_node_id(&'a self) -> u32 {
        match self {
            Self::Identifier(e) => e.node_id,
            Self::StringLiteral(e) => e.node_id,
        }
    }
}

impl<'a> GetChildren<'a> for ExportNamedDeclaration<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        if let Some(field) = &self.declaration {
            children.push((*field).to_ast_kind());
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
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::ExportNamedDeclaration(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for ExportDefaultDeclaration<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.declaration).to_ast_kind());
        children.push((*&self.exported).to_ast_kind());
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::ExportDefaultDeclaration(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for ExportAllDeclaration<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        if let Some(field) = &self.exported {
            children.push((*field).to_ast_kind());
        }
        children.push(AstKind::StringLiteral(&self.source));
        if let Some(field) = &self.with_clause {
            children.push(AstKind::WithClause(field));
        }
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::ExportAllDeclaration(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for ExportSpecifier<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.local).to_ast_kind());
        children.push((*&self.exported).to_ast_kind());
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::ExportSpecifier(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
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
            Self::GeneralBinaryExpression(_) => vec![],
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
            Self::ElementAccessExpression(_) => vec![],
            Self::PropertyAccessExpression(_) => vec![],
            Self::PrivateFieldExpression(_) => vec![],
        }
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        match self {
            Self::FunctionDeclaration(e) => AstKind::Function(e),
            Self::ClassDeclaration(e) => AstKind::Class(e),
            Self::TSInterfaceDeclaration(e) => AstKind::TSInterfaceDeclaration(e),
            Self::BooleanLiteral(e) => AstKind::BooleanLiteral(e),
            Self::NullLiteral(e) => AstKind::NullLiteral(e),
            Self::NumericLiteral(e) => AstKind::NumericLiteral(e),
            Self::BigIntLiteral(e) => AstKind::BigIntLiteral(e),
            Self::RegExpLiteral(e) => AstKind::RegExpLiteral(e),
            Self::StringLiteral(e) => AstKind::StringLiteral(e),
            Self::TemplateLiteral(e) => AstKind::TemplateLiteral(e),
            Self::Identifier(e) => AstKind::IdentifierReference(e),
            Self::MetaProperty(e) => AstKind::MetaProperty(e),
            Self::Super(e) => AstKind::Super(e),
            Self::ArrayExpression(e) => AstKind::ArrayExpression(e),
            Self::ArrowFunctionExpression(e) => AstKind::ArrowFunctionExpression(e),
            Self::AssignmentExpression(e) => AstKind::AssignmentExpression(e),
            Self::AwaitExpression(e) => AstKind::AwaitExpression(e),
            Self::GeneralBinaryExpression(e) => AstKind::GeneralBinaryExpression(e),
            Self::CallExpression(e) => AstKind::CallExpression(e),
            Self::ChainExpression(e) => AstKind::ChainExpression(e),
            Self::ClassExpression(e) => AstKind::Class(e),
            Self::ConditionalExpression(e) => AstKind::ConditionalExpression(e),
            Self::FunctionExpression(e) => AstKind::Function(e),
            Self::ImportExpression(e) => AstKind::ImportExpression(e),
            Self::LogicalExpression(e) => AstKind::LogicalExpression(e),
            Self::NewExpression(e) => AstKind::NewExpression(e),
            Self::ObjectExpression(e) => AstKind::ObjectExpression(e),
            Self::ParenthesizedExpression(e) => AstKind::ParenthesizedExpression(e),
            Self::SequenceExpression(e) => AstKind::SequenceExpression(e),
            Self::TaggedTemplateExpression(e) => AstKind::TaggedTemplateExpression(e),
            Self::ThisExpression(e) => AstKind::ThisExpression(e),
            Self::UnaryExpression(e) => AstKind::UnaryExpression(e),
            Self::UpdateExpression(e) => AstKind::UpdateExpression(e),
            Self::YieldExpression(e) => AstKind::YieldExpression(e),
            Self::PrivateInExpression(e) => AstKind::PrivateInExpression(e),
            Self::JSXElement(e) => AstKind::JSXElement(e),
            Self::JSXFragment(e) => AstKind::JSXFragment(e),
            Self::TSAsExpression(e) => AstKind::TSAsExpression(e),
            Self::TSSatisfiesExpression(e) => AstKind::TSSatisfiesExpression(e),
            Self::TSTypeAssertion(e) => AstKind::TSTypeAssertion(e),
            Self::TSNonNullExpression(e) => AstKind::TSNonNullExpression(e),
            Self::TSInstantiationExpression(e) => AstKind::TSInstantiationExpression(e),
            Self::ElementAccessExpression(e) => AstKind::ElementAccessExpression(e),
            Self::PropertyAccessExpression(e) => AstKind::PropertyAccessExpression(e),
            Self::PrivateFieldExpression(e) => AstKind::PrivateFieldExpression(e),
        }
    }
    fn get_node_id(&'a self) -> u32 {
        match self {
            Self::FunctionDeclaration(e) => e.node_id,
            Self::ClassDeclaration(e) => e.node_id,
            Self::TSInterfaceDeclaration(e) => e.node_id,
            Self::BooleanLiteral(e) => e.node_id,
            Self::NullLiteral(e) => e.node_id,
            Self::NumericLiteral(e) => e.node_id,
            Self::BigIntLiteral(e) => e.node_id,
            Self::RegExpLiteral(e) => e.node_id,
            Self::StringLiteral(e) => e.node_id,
            Self::TemplateLiteral(e) => e.node_id,
            Self::Identifier(e) => e.node_id,
            Self::MetaProperty(e) => e.node_id,
            Self::Super(e) => e.node_id,
            Self::ArrayExpression(e) => e.node_id,
            Self::ArrowFunctionExpression(e) => e.node_id,
            Self::AssignmentExpression(e) => e.node_id,
            Self::AwaitExpression(e) => e.node_id,
            Self::GeneralBinaryExpression(e) => e.node_id,
            Self::CallExpression(e) => e.node_id,
            Self::ChainExpression(e) => e.node_id,
            Self::ClassExpression(e) => e.node_id,
            Self::ConditionalExpression(e) => e.node_id,
            Self::FunctionExpression(e) => e.node_id,
            Self::ImportExpression(e) => e.node_id,
            Self::LogicalExpression(e) => e.node_id,
            Self::NewExpression(e) => e.node_id,
            Self::ObjectExpression(e) => e.node_id,
            Self::ParenthesizedExpression(e) => e.node_id,
            Self::SequenceExpression(e) => e.node_id,
            Self::TaggedTemplateExpression(e) => e.node_id,
            Self::ThisExpression(e) => e.node_id,
            Self::UnaryExpression(e) => e.node_id,
            Self::UpdateExpression(e) => e.node_id,
            Self::YieldExpression(e) => e.node_id,
            Self::PrivateInExpression(e) => e.node_id,
            Self::JSXElement(e) => e.node_id,
            Self::JSXFragment(e) => e.node_id,
            Self::TSAsExpression(e) => e.node_id,
            Self::TSSatisfiesExpression(e) => e.node_id,
            Self::TSTypeAssertion(e) => e.node_id,
            Self::TSNonNullExpression(e) => e.node_id,
            Self::TSInstantiationExpression(e) => e.node_id,
            Self::ElementAccessExpression(e) => e.node_id,
            Self::PropertyAccessExpression(e) => e.node_id,
            Self::PrivateFieldExpression(e) => e.node_id,
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
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        match self {
            Self::IdentifierName(e) => AstKind::IdentifierName(e),
            Self::IdentifierReference(e) => AstKind::IdentifierReference(e),
            Self::StringLiteral(e) => AstKind::StringLiteral(e),
        }
    }
    fn get_node_id(&'a self) -> u32 {
        match self {
            Self::IdentifierName(e) => e.node_id,
            Self::IdentifierReference(e) => e.node_id,
            Self::StringLiteral(e) => e.node_id,
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
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSThisParameter(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
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
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSEnumDeclaration(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for TSEnumMember<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.id).to_ast_kind());
        if let Some(field) = &self.initializer {
            children.push((*field).to_ast_kind());
        }
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSEnumMember(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
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
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        match self {
            Self::Identifier(e) => AstKind::IdentifierName(e),
            Self::String(e) => AstKind::StringLiteral(e),
        }
    }
    fn get_node_id(&'a self) -> u32 {
        match self {
            Self::Identifier(e) => e.node_id,
            Self::String(e) => e.node_id,
        }
    }
}

impl<'a> GetChildren<'a> for TSTypeAnnotation<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.type_annotation).to_ast_kind());
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSTypeAnnotation(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for TSLiteralType<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.literal).to_ast_kind());
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSLiteralType(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
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
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        match self {
            Self::BooleanLiteral(e) => AstKind::BooleanLiteral(e),
            Self::NullLiteral(e) => AstKind::NullLiteral(e),
            Self::NumericLiteral(e) => AstKind::NumericLiteral(e),
            Self::BigIntLiteral(e) => AstKind::BigIntLiteral(e),
            Self::RegExpLiteral(e) => AstKind::RegExpLiteral(e),
            Self::StringLiteral(e) => AstKind::StringLiteral(e),
            Self::TemplateLiteral(e) => AstKind::TemplateLiteral(e),
            Self::UnaryExpression(e) => AstKind::UnaryExpression(e),
        }
    }
    fn get_node_id(&'a self) -> u32 {
        match self {
            Self::BooleanLiteral(e) => e.node_id,
            Self::NullLiteral(e) => e.node_id,
            Self::NumericLiteral(e) => e.node_id,
            Self::BigIntLiteral(e) => e.node_id,
            Self::RegExpLiteral(e) => e.node_id,
            Self::StringLiteral(e) => e.node_id,
            Self::TemplateLiteral(e) => e.node_id,
            Self::UnaryExpression(e) => e.node_id,
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
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        match self {
            Self::TSAnyKeyword(e) => AstKind::TSAnyKeyword(e),
            Self::TSBigIntKeyword(e) => AstKind::TSBigIntKeyword(e),
            Self::TSBooleanKeyword(e) => AstKind::TSBooleanKeyword(e),
            Self::TSIntrinsicKeyword(e) => AstKind::TSIntrinsicKeyword(e),
            Self::TSNeverKeyword(e) => AstKind::TSNeverKeyword(e),
            Self::TSNullKeyword(e) => AstKind::TSNullKeyword(e),
            Self::TSNumberKeyword(e) => AstKind::TSNumberKeyword(e),
            Self::TSObjectKeyword(e) => AstKind::TSObjectKeyword(e),
            Self::TSStringKeyword(e) => AstKind::TSStringKeyword(e),
            Self::TSSymbolKeyword(e) => AstKind::TSSymbolKeyword(e),
            Self::TSUndefinedKeyword(e) => AstKind::TSUndefinedKeyword(e),
            Self::TSUnknownKeyword(e) => AstKind::TSUnknownKeyword(e),
            Self::TSVoidKeyword(e) => AstKind::TSVoidKeyword(e),
            Self::TSArrayType(e) => AstKind::TSArrayType(e),
            Self::TSConditionalType(e) => AstKind::TSConditionalType(e),
            Self::TSConstructorType(e) => AstKind::TSConstructorType(e),
            Self::TSFunctionType(e) => AstKind::TSFunctionType(e),
            Self::TSImportType(e) => AstKind::TSImportType(e),
            Self::TSIndexedAccessType(e) => AstKind::TSIndexedAccessType(e),
            Self::TSInferType(e) => AstKind::TSInferType(e),
            Self::TSIntersectionType(e) => AstKind::TSIntersectionType(e),
            Self::TSLiteralType(e) => AstKind::TSLiteralType(e),
            Self::TSMappedType(e) => AstKind::TSMappedType(e),
            Self::TSNamedTupleMember(e) => AstKind::TSNamedTupleMember(e),
            Self::TSQualifiedName(e) => AstKind::TSQualifiedName(e),
            Self::TSTemplateLiteralType(e) => AstKind::TSTemplateLiteralType(e),
            Self::TSThisType(e) => AstKind::TSThisType(e),
            Self::TSTupleType(e) => AstKind::TSTupleType(e),
            Self::TSTypeLiteral(e) => AstKind::TSTypeLiteral(e),
            Self::TSTypeOperatorType(e) => AstKind::TSTypeOperator(e),
            Self::TSTypePredicate(e) => AstKind::TSTypePredicate(e),
            Self::TSTypeQuery(e) => AstKind::TSTypeQuery(e),
            Self::TSTypeReference(e) => AstKind::TSTypeReference(e),
            Self::TSUnionType(e) => AstKind::TSUnionType(e),
            Self::TSParenthesizedType(e) => AstKind::TSParenthesizedType(e),
            Self::JSDocNullableType(e) => AstKind::JSDocNullableType(e),
            Self::JSDocNonNullableType(e) => AstKind::JSDocNonNullableType(e),
            Self::JSDocUnknownType(e) => AstKind::JSDocUnknownType(e),
        }
    }
    fn get_node_id(&'a self) -> u32 {
        match self {
            Self::TSAnyKeyword(e) => e.node_id,
            Self::TSBigIntKeyword(e) => e.node_id,
            Self::TSBooleanKeyword(e) => e.node_id,
            Self::TSIntrinsicKeyword(e) => e.node_id,
            Self::TSNeverKeyword(e) => e.node_id,
            Self::TSNullKeyword(e) => e.node_id,
            Self::TSNumberKeyword(e) => e.node_id,
            Self::TSObjectKeyword(e) => e.node_id,
            Self::TSStringKeyword(e) => e.node_id,
            Self::TSSymbolKeyword(e) => e.node_id,
            Self::TSUndefinedKeyword(e) => e.node_id,
            Self::TSUnknownKeyword(e) => e.node_id,
            Self::TSVoidKeyword(e) => e.node_id,
            Self::TSArrayType(e) => e.node_id,
            Self::TSConditionalType(e) => e.node_id,
            Self::TSConstructorType(e) => e.node_id,
            Self::TSFunctionType(e) => e.node_id,
            Self::TSImportType(e) => e.node_id,
            Self::TSIndexedAccessType(e) => e.node_id,
            Self::TSInferType(e) => e.node_id,
            Self::TSIntersectionType(e) => e.node_id,
            Self::TSLiteralType(e) => e.node_id,
            Self::TSMappedType(e) => e.node_id,
            Self::TSNamedTupleMember(e) => e.node_id,
            Self::TSQualifiedName(e) => e.node_id,
            Self::TSTemplateLiteralType(e) => e.node_id,
            Self::TSThisType(e) => e.node_id,
            Self::TSTupleType(e) => e.node_id,
            Self::TSTypeLiteral(e) => e.node_id,
            Self::TSTypeOperatorType(e) => e.node_id,
            Self::TSTypePredicate(e) => e.node_id,
            Self::TSTypeQuery(e) => e.node_id,
            Self::TSTypeReference(e) => e.node_id,
            Self::TSUnionType(e) => e.node_id,
            Self::TSParenthesizedType(e) => e.node_id,
            Self::JSDocNullableType(e) => e.node_id,
            Self::JSDocNonNullableType(e) => e.node_id,
            Self::JSDocUnknownType(e) => e.node_id,
        }
    }
}

impl<'a> GetChildren<'a> for TSConditionalType<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.check_type).to_ast_kind());
        children.push((*&self.extends_type).to_ast_kind());
        children.push((*&self.true_type).to_ast_kind());
        children.push((*&self.false_type).to_ast_kind());
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSConditionalType(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for TSUnionType<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        for item in &self.types {
            children.push((*item).to_ast_kind());
        }
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSUnionType(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for TSIntersectionType<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        for item in &self.types {
            children.push((*item).to_ast_kind());
        }
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSIntersectionType(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for TSParenthesizedType<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.type_annotation).to_ast_kind());
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSParenthesizedType(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for TSTypeOperator<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.type_annotation).to_ast_kind());
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSTypeOperator(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for TSArrayType<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.element_type).to_ast_kind());
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSArrayType(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for TSIndexedAccessType<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.object_type).to_ast_kind());
        children.push((*&self.index_type).to_ast_kind());
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSIndexedAccessType(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for TSTupleType<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        for item in &self.element_types {
            children.push((*item).to_ast_kind());
        }
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSTupleType(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for TSNamedTupleMember<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.element_type).to_ast_kind());
        children.push(AstKind::IdentifierName(&self.label));
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSNamedTupleMember(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for TSOptionalType<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.type_annotation).to_ast_kind());
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSOptionalType(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for TSRestType<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.type_annotation).to_ast_kind());
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSRestType(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
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
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        match self {
            Self::TSOptionalType(e) => AstKind::TSOptionalType(e),
            Self::TSRestType(e) => AstKind::TSRestType(e),
            Self::TSAnyKeyword(e) => AstKind::TSAnyKeyword(e),
            Self::TSBigIntKeyword(e) => AstKind::TSBigIntKeyword(e),
            Self::TSBooleanKeyword(e) => AstKind::TSBooleanKeyword(e),
            Self::TSIntrinsicKeyword(e) => AstKind::TSIntrinsicKeyword(e),
            Self::TSNeverKeyword(e) => AstKind::TSNeverKeyword(e),
            Self::TSNullKeyword(e) => AstKind::TSNullKeyword(e),
            Self::TSNumberKeyword(e) => AstKind::TSNumberKeyword(e),
            Self::TSObjectKeyword(e) => AstKind::TSObjectKeyword(e),
            Self::TSStringKeyword(e) => AstKind::TSStringKeyword(e),
            Self::TSSymbolKeyword(e) => AstKind::TSSymbolKeyword(e),
            Self::TSUndefinedKeyword(e) => AstKind::TSUndefinedKeyword(e),
            Self::TSUnknownKeyword(e) => AstKind::TSUnknownKeyword(e),
            Self::TSVoidKeyword(e) => AstKind::TSVoidKeyword(e),
            Self::TSArrayType(e) => AstKind::TSArrayType(e),
            Self::TSConditionalType(e) => AstKind::TSConditionalType(e),
            Self::TSConstructorType(e) => AstKind::TSConstructorType(e),
            Self::TSFunctionType(e) => AstKind::TSFunctionType(e),
            Self::TSImportType(e) => AstKind::TSImportType(e),
            Self::TSIndexedAccessType(e) => AstKind::TSIndexedAccessType(e),
            Self::TSInferType(e) => AstKind::TSInferType(e),
            Self::TSIntersectionType(e) => AstKind::TSIntersectionType(e),
            Self::TSLiteralType(e) => AstKind::TSLiteralType(e),
            Self::TSMappedType(e) => AstKind::TSMappedType(e),
            Self::TSNamedTupleMember(e) => AstKind::TSNamedTupleMember(e),
            Self::TSQualifiedName(e) => AstKind::TSQualifiedName(e),
            Self::TSTemplateLiteralType(e) => AstKind::TSTemplateLiteralType(e),
            Self::TSThisType(e) => AstKind::TSThisType(e),
            Self::TSTupleType(e) => AstKind::TSTupleType(e),
            Self::TSTypeLiteral(e) => AstKind::TSTypeLiteral(e),
            Self::TSTypeOperatorType(e) => AstKind::TSTypeOperator(e),
            Self::TSTypePredicate(e) => AstKind::TSTypePredicate(e),
            Self::TSTypeQuery(e) => AstKind::TSTypeQuery(e),
            Self::TSTypeReference(e) => AstKind::TSTypeReference(e),
            Self::TSUnionType(e) => AstKind::TSUnionType(e),
            Self::TSParenthesizedType(e) => AstKind::TSParenthesizedType(e),
            Self::JSDocNullableType(e) => AstKind::JSDocNullableType(e),
            Self::JSDocNonNullableType(e) => AstKind::JSDocNonNullableType(e),
            Self::JSDocUnknownType(e) => AstKind::JSDocUnknownType(e),
        }
    }
    fn get_node_id(&'a self) -> u32 {
        match self {
            Self::TSOptionalType(e) => e.node_id,
            Self::TSRestType(e) => e.node_id,
            Self::TSAnyKeyword(e) => e.node_id,
            Self::TSBigIntKeyword(e) => e.node_id,
            Self::TSBooleanKeyword(e) => e.node_id,
            Self::TSIntrinsicKeyword(e) => e.node_id,
            Self::TSNeverKeyword(e) => e.node_id,
            Self::TSNullKeyword(e) => e.node_id,
            Self::TSNumberKeyword(e) => e.node_id,
            Self::TSObjectKeyword(e) => e.node_id,
            Self::TSStringKeyword(e) => e.node_id,
            Self::TSSymbolKeyword(e) => e.node_id,
            Self::TSUndefinedKeyword(e) => e.node_id,
            Self::TSUnknownKeyword(e) => e.node_id,
            Self::TSVoidKeyword(e) => e.node_id,
            Self::TSArrayType(e) => e.node_id,
            Self::TSConditionalType(e) => e.node_id,
            Self::TSConstructorType(e) => e.node_id,
            Self::TSFunctionType(e) => e.node_id,
            Self::TSImportType(e) => e.node_id,
            Self::TSIndexedAccessType(e) => e.node_id,
            Self::TSInferType(e) => e.node_id,
            Self::TSIntersectionType(e) => e.node_id,
            Self::TSLiteralType(e) => e.node_id,
            Self::TSMappedType(e) => e.node_id,
            Self::TSNamedTupleMember(e) => e.node_id,
            Self::TSQualifiedName(e) => e.node_id,
            Self::TSTemplateLiteralType(e) => e.node_id,
            Self::TSThisType(e) => e.node_id,
            Self::TSTupleType(e) => e.node_id,
            Self::TSTypeLiteral(e) => e.node_id,
            Self::TSTypeOperatorType(e) => e.node_id,
            Self::TSTypePredicate(e) => e.node_id,
            Self::TSTypeQuery(e) => e.node_id,
            Self::TSTypeReference(e) => e.node_id,
            Self::TSUnionType(e) => e.node_id,
            Self::TSParenthesizedType(e) => e.node_id,
            Self::JSDocNullableType(e) => e.node_id,
            Self::JSDocNonNullableType(e) => e.node_id,
            Self::JSDocUnknownType(e) => e.node_id,
        }
    }
}

impl<'a> GetChildren<'a> for TSAnyKeyword {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSAnyKeyword(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for TSStringKeyword {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSStringKeyword(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for TSBooleanKeyword {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSBooleanKeyword(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for TSNumberKeyword {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSNumberKeyword(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for TSNeverKeyword {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSNeverKeyword(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for TSIntrinsicKeyword {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSIntrinsicKeyword(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for TSUnknownKeyword {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSUnknownKeyword(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for TSNullKeyword {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSNullKeyword(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for TSUndefinedKeyword {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSUndefinedKeyword(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for TSVoidKeyword {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSVoidKeyword(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for TSSymbolKeyword {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSSymbolKeyword(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for TSThisType {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSThisType(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for TSObjectKeyword {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSObjectKeyword(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for TSBigIntKeyword {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSBigIntKeyword(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for TSTypeReference<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.type_name).to_ast_kind());
        if let Some(field) = &self.type_parameters {
            children.push(AstKind::TSTypeParameterInstantiation(field));
        }
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSTypeReference(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
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
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        match self {
            Self::IdentifierReference(e) => AstKind::IdentifierReference(e),
            Self::QualifiedName(e) => AstKind::TSQualifiedName(e),
        }
    }
    fn get_node_id(&'a self) -> u32 {
        match self {
            Self::IdentifierReference(e) => e.node_id,
            Self::QualifiedName(e) => e.node_id,
        }
    }
}

impl<'a> GetChildren<'a> for TSQualifiedName<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.left).to_ast_kind());
        children.push(AstKind::IdentifierName(&self.right));
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSQualifiedName(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for TSTypeParameterInstantiation<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        for item in &self.params {
            children.push((*item).to_ast_kind());
        }
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSTypeParameterInstantiation(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for TSTypeParameter<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::BindingIdentifier(&self.name));
        if let Some(field) = &self.constraint {
            children.push((*field).to_ast_kind());
        }
        if let Some(field) = &self.default {
            children.push((*field).to_ast_kind());
        }
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSTypeParameter(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
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
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSTypeParameterDeclaration(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for TSTypeAliasDeclaration<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::BindingIdentifier(&self.id));
        if let Some(field) = &self.type_parameters {
            children.push(AstKind::TSTypeParameterDeclaration(field));
        }
        children.push((*&self.type_annotation).to_ast_kind());
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSTypeAliasDeclaration(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for TSClassImplements<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.expression).to_ast_kind());
        if let Some(field) = &self.type_parameters {
            children.push(AstKind::TSTypeParameterInstantiation(field));
        }
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSClassImplements(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
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
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSInterfaceDeclaration(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for TSInterfaceBody<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        for item in &self.body {
            children.push((*item).to_ast_kind());
        }
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSInterfaceBody(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for TSPropertySignature<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.key).to_ast_kind());
        if let Some(field) = &self.type_annotation {
            children.push(AstKind::TSTypeAnnotation(field));
        }
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSPropertySignature(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
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
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        match self {
            Self::TSIndexSignature(e) => AstKind::TSIndexSignature(e),
            Self::TSPropertySignature(e) => AstKind::TSPropertySignature(e),
            Self::TSCallSignatureDeclaration(e) => AstKind::TSCallSignatureDeclaration(e),
            Self::TSConstructSignatureDeclaration(e) => AstKind::TSConstructSignatureDeclaration(e),
            Self::TSMethodSignature(e) => AstKind::TSMethodSignature(e),
        }
    }
    fn get_node_id(&'a self) -> u32 {
        match self {
            Self::TSIndexSignature(e) => e.node_id,
            Self::TSPropertySignature(e) => e.node_id,
            Self::TSCallSignatureDeclaration(e) => e.node_id,
            Self::TSConstructSignatureDeclaration(e) => e.node_id,
            Self::TSMethodSignature(e) => e.node_id,
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
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSIndexSignature(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
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
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSCallSignatureDeclaration(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for TSMethodSignature<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.key).to_ast_kind());
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
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSMethodSignature(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
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
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSConstructSignatureDeclaration(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for TSIndexSignatureName<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::TSTypeAnnotation(&*self.type_annotation));
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSIndexSignatureName(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for TSInterfaceHeritage<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.expression).to_ast_kind());
        if let Some(field) = &self.type_parameters {
            children.push(AstKind::TSTypeParameterInstantiation(field));
        }
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSInterfaceHeritage(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for TSTypePredicate<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.parameter_name).to_ast_kind());
        if let Some(field) = &self.type_annotation {
            children.push(AstKind::TSTypeAnnotation(field));
        }
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSTypePredicate(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
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
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        match self {
            Self::Identifier(e) => AstKind::IdentifierName(e),
            Self::This(e) => AstKind::TSThisType(e),
        }
    }
    fn get_node_id(&'a self) -> u32 {
        match self {
            Self::Identifier(e) => e.node_id,
            Self::This(e) => e.node_id,
        }
    }
}

impl<'a> GetChildren<'a> for TSModuleDeclaration<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.id).to_ast_kind());
        if let Some(field) = &self.body {
            children.push((*field).to_ast_kind());
        }
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSModuleDeclaration(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
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
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        match self {
            Self::Identifier(e) => AstKind::BindingIdentifier(e),
            Self::StringLiteral(e) => AstKind::StringLiteral(e),
        }
    }
    fn get_node_id(&'a self) -> u32 {
        match self {
            Self::Identifier(e) => e.node_id,
            Self::StringLiteral(e) => e.node_id,
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
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        match self {
            Self::TSModuleDeclaration(e) => AstKind::TSModuleDeclaration(e),
            Self::TSModuleBlock(e) => AstKind::TSModuleBlock(e),
        }
    }
    fn get_node_id(&'a self) -> u32 {
        match self {
            Self::TSModuleDeclaration(e) => e.node_id,
            Self::TSModuleBlock(e) => e.node_id,
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
            children.push((*item).to_ast_kind());
        }
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSModuleBlock(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for TSTypeLiteral<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        for item in &self.members {
            children.push((*item).to_ast_kind());
        }
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSTypeLiteral(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for TSInferType<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::TSTypeParameter(&*self.type_parameter));
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSInferType(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for TSTypeQuery<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.expr_name).to_ast_kind());
        if let Some(field) = &self.type_parameters {
            children.push(AstKind::TSTypeParameterInstantiation(field));
        }
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSTypeQuery(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
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
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        match self {
            Self::TSImportType(e) => AstKind::TSImportType(e),
            Self::IdentifierReference(e) => AstKind::IdentifierReference(e),
            Self::QualifiedName(e) => AstKind::TSQualifiedName(e),
        }
    }
    fn get_node_id(&'a self) -> u32 {
        match self {
            Self::TSImportType(e) => e.node_id,
            Self::IdentifierReference(e) => e.node_id,
            Self::QualifiedName(e) => e.node_id,
        }
    }
}

impl<'a> GetChildren<'a> for TSImportType<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.parameter).to_ast_kind());
        if let Some(field) = &self.qualifier {
            children.push((*field).to_ast_kind());
        }
        if let Some(field) = &self.attributes {
            children.push(AstKind::TSImportAttributes(field));
        }
        if let Some(field) = &self.type_parameters {
            children.push(AstKind::TSTypeParameterInstantiation(field));
        }
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSImportType(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
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
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSImportAttributes(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for TSImportAttribute<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.name).to_ast_kind());
        children.push((*&self.value).to_ast_kind());
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSImportAttribute(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
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
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        match self {
            Self::Identifier(e) => AstKind::IdentifierName(e),
            Self::StringLiteral(e) => AstKind::StringLiteral(e),
        }
    }
    fn get_node_id(&'a self) -> u32 {
        match self {
            Self::Identifier(e) => e.node_id,
            Self::StringLiteral(e) => e.node_id,
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
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSFunctionType(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
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
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSConstructorType(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for TSMappedType<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::TSTypeParameter(&*self.type_parameter));
        if let Some(field) = &self.name_type {
            children.push((*field).to_ast_kind());
        }
        if let Some(field) = &self.type_annotation {
            children.push((*field).to_ast_kind());
        }
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSMappedType(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for TSTemplateLiteralType<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        for item in &self.quasis {
            children.push(AstKind::TemplateElement(item));
        }
        for item in &self.types {
            children.push((*item).to_ast_kind());
        }
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSTemplateLiteralType(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for TSAsExpression<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.expression).to_ast_kind());
        children.push((*&self.type_annotation).to_ast_kind());
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSAsExpression(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for TSSatisfiesExpression<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.expression).to_ast_kind());
        children.push((*&self.type_annotation).to_ast_kind());
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSSatisfiesExpression(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for TSTypeAssertion<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.expression).to_ast_kind());
        children.push((*&self.type_annotation).to_ast_kind());
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSTypeAssertion(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for TSImportEqualsDeclaration<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::BindingIdentifier(&self.id));
        children.push((*&self.module_reference).to_ast_kind());
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSImportEqualsDeclaration(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
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
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        match self {
            Self::ExternalModuleReference(e) => AstKind::TSExternalModuleReference(e),
            Self::IdentifierReference(e) => AstKind::IdentifierReference(e),
            Self::QualifiedName(e) => AstKind::TSQualifiedName(e),
        }
    }
    fn get_node_id(&'a self) -> u32 {
        match self {
            Self::ExternalModuleReference(e) => e.node_id,
            Self::IdentifierReference(e) => e.node_id,
            Self::QualifiedName(e) => e.node_id,
        }
    }
}

impl<'a> GetChildren<'a> for TSExternalModuleReference<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::StringLiteral(&self.expression));
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSExternalModuleReference(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for TSNonNullExpression<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.expression).to_ast_kind());
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSNonNullExpression(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for Decorator<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.expression).to_ast_kind());
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::Decorator(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for TSExportAssignment<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.expression).to_ast_kind());
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSExportAssignment(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for TSNamespaceExportDeclaration<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::IdentifierName(&self.id));
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSNamespaceExportDeclaration(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for TSInstantiationExpression<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.expression).to_ast_kind());
        children.push(AstKind::TSTypeParameterInstantiation(&*self.type_parameters));
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::TSInstantiationExpression(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for JSDocNullableType<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.type_annotation).to_ast_kind());
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::JSDocNullableType(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for JSDocNonNullableType<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.type_annotation).to_ast_kind());
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::JSDocNonNullableType(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for JSDocUnknownType {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::JSDocUnknownType(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
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
            children.push((*item).to_ast_kind());
        }
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::JSXElement(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for JSXOpeningElement<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.name).to_ast_kind());
        for item in &self.attributes {
            children.push((*item).to_ast_kind());
        }
        if let Some(field) = &self.type_parameters {
            children.push(AstKind::TSTypeParameterInstantiation(field));
        }
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::JSXOpeningElement(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for JSXClosingElement<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.name).to_ast_kind());
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::JSXClosingElement(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for JSXFragment<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::JSXOpeningFragment(&self.opening_fragment));
        children.push(AstKind::JSXClosingFragment(&self.closing_fragment));
        for item in &self.children {
            children.push((*item).to_ast_kind());
        }
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::JSXFragment(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for JSXOpeningFragment {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::JSXOpeningFragment(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for JSXClosingFragment {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::JSXClosingFragment(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
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
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        match self {
            Self::Identifier(e) => AstKind::JSXIdentifier(e),
            Self::IdentifierReference(e) => AstKind::IdentifierReference(e),
            Self::NamespacedName(e) => AstKind::JSXNamespacedName(e),
            Self::MemberExpression(e) => AstKind::JSXMemberExpression(e),
            Self::ThisExpression(e) => AstKind::ThisExpression(e),
        }
    }
    fn get_node_id(&'a self) -> u32 {
        match self {
            Self::Identifier(e) => e.node_id,
            Self::IdentifierReference(e) => e.node_id,
            Self::NamespacedName(e) => e.node_id,
            Self::MemberExpression(e) => e.node_id,
            Self::ThisExpression(e) => e.node_id,
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
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::JSXNamespacedName(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for JSXMemberExpression<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.object).to_ast_kind());
        children.push(AstKind::JSXIdentifier(&self.property));
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::JSXMemberExpression(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
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
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        match self {
            Self::IdentifierReference(e) => AstKind::IdentifierReference(e),
            Self::MemberExpression(e) => AstKind::JSXMemberExpression(e),
            Self::ThisExpression(e) => AstKind::ThisExpression(e),
        }
    }
    fn get_node_id(&'a self) -> u32 {
        match self {
            Self::IdentifierReference(e) => e.node_id,
            Self::MemberExpression(e) => e.node_id,
            Self::ThisExpression(e) => e.node_id,
        }
    }
}

impl<'a> GetChildren<'a> for JSXExpressionContainer<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.expression).to_ast_kind());
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::JSXExpressionContainer(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
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
            Self::GeneralBinaryExpression(_) => vec![],
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
            Self::ElementAccessExpression(_) => vec![],
            Self::PropertyAccessExpression(_) => vec![],
            Self::PrivateFieldExpression(_) => vec![],
        }
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        match self {
            Self::EmptyExpression(e) => AstKind::JSXEmptyExpression(e),
            Self::BooleanLiteral(e) => AstKind::BooleanLiteral(e),
            Self::NullLiteral(e) => AstKind::NullLiteral(e),
            Self::NumericLiteral(e) => AstKind::NumericLiteral(e),
            Self::BigIntLiteral(e) => AstKind::BigIntLiteral(e),
            Self::RegExpLiteral(e) => AstKind::RegExpLiteral(e),
            Self::StringLiteral(e) => AstKind::StringLiteral(e),
            Self::TemplateLiteral(e) => AstKind::TemplateLiteral(e),
            Self::Identifier(e) => AstKind::IdentifierReference(e),
            Self::MetaProperty(e) => AstKind::MetaProperty(e),
            Self::Super(e) => AstKind::Super(e),
            Self::ArrayExpression(e) => AstKind::ArrayExpression(e),
            Self::ArrowFunctionExpression(e) => AstKind::ArrowFunctionExpression(e),
            Self::AssignmentExpression(e) => AstKind::AssignmentExpression(e),
            Self::AwaitExpression(e) => AstKind::AwaitExpression(e),
            Self::GeneralBinaryExpression(e) => AstKind::GeneralBinaryExpression(e),
            Self::CallExpression(e) => AstKind::CallExpression(e),
            Self::ChainExpression(e) => AstKind::ChainExpression(e),
            Self::ClassExpression(e) => AstKind::Class(e),
            Self::ConditionalExpression(e) => AstKind::ConditionalExpression(e),
            Self::FunctionExpression(e) => AstKind::Function(e),
            Self::ImportExpression(e) => AstKind::ImportExpression(e),
            Self::LogicalExpression(e) => AstKind::LogicalExpression(e),
            Self::NewExpression(e) => AstKind::NewExpression(e),
            Self::ObjectExpression(e) => AstKind::ObjectExpression(e),
            Self::ParenthesizedExpression(e) => AstKind::ParenthesizedExpression(e),
            Self::SequenceExpression(e) => AstKind::SequenceExpression(e),
            Self::TaggedTemplateExpression(e) => AstKind::TaggedTemplateExpression(e),
            Self::ThisExpression(e) => AstKind::ThisExpression(e),
            Self::UnaryExpression(e) => AstKind::UnaryExpression(e),
            Self::UpdateExpression(e) => AstKind::UpdateExpression(e),
            Self::YieldExpression(e) => AstKind::YieldExpression(e),
            Self::PrivateInExpression(e) => AstKind::PrivateInExpression(e),
            Self::JSXElement(e) => AstKind::JSXElement(e),
            Self::JSXFragment(e) => AstKind::JSXFragment(e),
            Self::TSAsExpression(e) => AstKind::TSAsExpression(e),
            Self::TSSatisfiesExpression(e) => AstKind::TSSatisfiesExpression(e),
            Self::TSTypeAssertion(e) => AstKind::TSTypeAssertion(e),
            Self::TSNonNullExpression(e) => AstKind::TSNonNullExpression(e),
            Self::TSInstantiationExpression(e) => AstKind::TSInstantiationExpression(e),
            Self::ElementAccessExpression(e) => AstKind::ElementAccessExpression(e),
            Self::PropertyAccessExpression(e) => AstKind::PropertyAccessExpression(e),
            Self::PrivateFieldExpression(e) => AstKind::PrivateFieldExpression(e),
        }
    }
    fn get_node_id(&'a self) -> u32 {
        match self {
            Self::EmptyExpression(e) => e.node_id,
            Self::BooleanLiteral(e) => e.node_id,
            Self::NullLiteral(e) => e.node_id,
            Self::NumericLiteral(e) => e.node_id,
            Self::BigIntLiteral(e) => e.node_id,
            Self::RegExpLiteral(e) => e.node_id,
            Self::StringLiteral(e) => e.node_id,
            Self::TemplateLiteral(e) => e.node_id,
            Self::Identifier(e) => e.node_id,
            Self::MetaProperty(e) => e.node_id,
            Self::Super(e) => e.node_id,
            Self::ArrayExpression(e) => e.node_id,
            Self::ArrowFunctionExpression(e) => e.node_id,
            Self::AssignmentExpression(e) => e.node_id,
            Self::AwaitExpression(e) => e.node_id,
            Self::GeneralBinaryExpression(e) => e.node_id,
            Self::CallExpression(e) => e.node_id,
            Self::ChainExpression(e) => e.node_id,
            Self::ClassExpression(e) => e.node_id,
            Self::ConditionalExpression(e) => e.node_id,
            Self::FunctionExpression(e) => e.node_id,
            Self::ImportExpression(e) => e.node_id,
            Self::LogicalExpression(e) => e.node_id,
            Self::NewExpression(e) => e.node_id,
            Self::ObjectExpression(e) => e.node_id,
            Self::ParenthesizedExpression(e) => e.node_id,
            Self::SequenceExpression(e) => e.node_id,
            Self::TaggedTemplateExpression(e) => e.node_id,
            Self::ThisExpression(e) => e.node_id,
            Self::UnaryExpression(e) => e.node_id,
            Self::UpdateExpression(e) => e.node_id,
            Self::YieldExpression(e) => e.node_id,
            Self::PrivateInExpression(e) => e.node_id,
            Self::JSXElement(e) => e.node_id,
            Self::JSXFragment(e) => e.node_id,
            Self::TSAsExpression(e) => e.node_id,
            Self::TSSatisfiesExpression(e) => e.node_id,
            Self::TSTypeAssertion(e) => e.node_id,
            Self::TSNonNullExpression(e) => e.node_id,
            Self::TSInstantiationExpression(e) => e.node_id,
            Self::ElementAccessExpression(e) => e.node_id,
            Self::PropertyAccessExpression(e) => e.node_id,
            Self::PrivateFieldExpression(e) => e.node_id,
        }
    }
}

impl<'a> GetChildren<'a> for JSXEmptyExpression {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::JSXEmptyExpression(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
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
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        match self {
            Self::Attribute(e) => AstKind::JSXAttribute(e),
            Self::SpreadAttribute(e) => AstKind::JSXSpreadAttribute(e),
        }
    }
    fn get_node_id(&'a self) -> u32 {
        match self {
            Self::Attribute(e) => e.node_id,
            Self::SpreadAttribute(e) => e.node_id,
        }
    }
}

impl<'a> GetChildren<'a> for JSXAttribute<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.name).to_ast_kind());
        if let Some(field) = &self.value {
            children.push((*field).to_ast_kind());
        }
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::JSXAttribute(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for JSXSpreadAttribute<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.argument).to_ast_kind());
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::JSXSpreadAttribute(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
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
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        match self {
            Self::Identifier(e) => AstKind::JSXIdentifier(e),
            Self::NamespacedName(e) => AstKind::JSXNamespacedName(e),
        }
    }
    fn get_node_id(&'a self) -> u32 {
        match self {
            Self::Identifier(e) => e.node_id,
            Self::NamespacedName(e) => e.node_id,
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
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        match self {
            Self::StringLiteral(e) => AstKind::StringLiteral(e),
            Self::ExpressionContainer(e) => AstKind::JSXExpressionContainer(e),
            Self::Element(e) => AstKind::JSXElement(e),
            Self::Fragment(e) => AstKind::JSXFragment(e),
        }
    }
    fn get_node_id(&'a self) -> u32 {
        match self {
            Self::StringLiteral(e) => e.node_id,
            Self::ExpressionContainer(e) => e.node_id,
            Self::Element(e) => e.node_id,
            Self::Fragment(e) => e.node_id,
        }
    }
}

impl<'a> GetChildren<'a> for JSXIdentifier<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::JSXIdentifier(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
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
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        match self {
            Self::Text(e) => AstKind::JSXText(e),
            Self::Element(e) => AstKind::JSXElement(e),
            Self::Fragment(e) => AstKind::JSXFragment(e),
            Self::ExpressionContainer(e) => AstKind::JSXExpressionContainer(e),
            Self::Spread(e) => AstKind::JSXSpreadChild(e),
        }
    }
    fn get_node_id(&'a self) -> u32 {
        match self {
            Self::Text(e) => e.node_id,
            Self::Element(e) => e.node_id,
            Self::Fragment(e) => e.node_id,
            Self::ExpressionContainer(e) => e.node_id,
            Self::Spread(e) => e.node_id,
        }
    }
}

impl<'a> GetChildren<'a> for JSXSpreadChild<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push((*&self.expression).to_ast_kind());
        children
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::JSXSpreadChild(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}

impl<'a> GetChildren<'a> for JSXText<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
    fn to_ast_kind(&'a self) -> AstKind<'a> {
        AstKind::JSXText(self)
    }
    fn get_node_id(&'a self) -> u32 {
        self.node_id
    }
}
