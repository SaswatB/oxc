// Auto-generated code, DO NOT EDIT DIRECTLY!
// To edit this generated file you have to edit `tasks/ast_tools/src/derives/get_parent.rs`

#![allow(clippy::match_same_arms)]
#![allow(unused_variables)]

use crate::AstKind;
use crate::GetParent;
#[allow(dead_code)]
pub trait GetChildren<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>>;
}

use crate::ast::js::*;

use crate::ast::jsx::*;

use crate::ast::literal::*;

use crate::ast::ts::*;

impl<'a> GetParent<'a> for BooleanLiteral<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for BooleanLiteral<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetParent<'a> for NullLiteral<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for NullLiteral<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetParent<'a> for NumericLiteral<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for NumericLiteral<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetParent<'a> for StringLiteral<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for StringLiteral<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetParent<'a> for BigIntLiteral<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for BigIntLiteral<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetParent<'a> for RegExpLiteral<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for RegExpLiteral<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetParent<'a> for Program<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for Expression<'a> {
    fn get_parent(&self) -> Option<AstKind<'a>> {
        match self {
            Self::BooleanLiteral(it) => GetParent::get_parent(it.as_ref()),
            Self::NullLiteral(it) => GetParent::get_parent(it.as_ref()),
            Self::NumericLiteral(it) => GetParent::get_parent(it.as_ref()),
            Self::BigIntLiteral(it) => GetParent::get_parent(it.as_ref()),
            Self::RegExpLiteral(it) => GetParent::get_parent(it.as_ref()),
            Self::StringLiteral(it) => GetParent::get_parent(it.as_ref()),
            Self::TemplateLiteral(it) => GetParent::get_parent(it.as_ref()),
            Self::Identifier(it) => GetParent::get_parent(it.as_ref()),
            Self::MetaProperty(it) => GetParent::get_parent(it.as_ref()),
            Self::Super(it) => GetParent::get_parent(it.as_ref()),
            Self::ArrayExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ArrowFunctionExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::AssignmentExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::AwaitExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::BinaryExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::CallExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ChainExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ClassExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ConditionalExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::FunctionExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ImportExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::LogicalExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::NewExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ObjectExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ParenthesizedExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::SequenceExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::TaggedTemplateExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ThisExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::UnaryExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::UpdateExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::YieldExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::PrivateInExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::JSXElement(it) => GetParent::get_parent(it.as_ref()),
            Self::JSXFragment(it) => GetParent::get_parent(it.as_ref()),
            Self::TSAsExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::TSSatisfiesExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::TSTypeAssertion(it) => GetParent::get_parent(it.as_ref()),
            Self::TSNonNullExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::TSInstantiationExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ComputedMemberExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::StaticMemberExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::PrivateFieldExpression(it) => GetParent::get_parent(it.as_ref()),
        }
    }
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        match self {
            Self::BooleanLiteral(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::NullLiteral(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::NumericLiteral(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::BigIntLiteral(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::RegExpLiteral(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::StringLiteral(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TemplateLiteral(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::Identifier(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::MetaProperty(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::Super(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ArrayExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ArrowFunctionExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::AssignmentExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::AwaitExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::BinaryExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::CallExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ChainExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ClassExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ConditionalExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::FunctionExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ImportExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::LogicalExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::NewExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ObjectExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ParenthesizedExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::SequenceExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TaggedTemplateExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ThisExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::UnaryExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::UpdateExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::YieldExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::PrivateInExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::JSXElement(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::JSXFragment(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSAsExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSSatisfiesExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSTypeAssertion(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSNonNullExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSInstantiationExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ComputedMemberExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::StaticMemberExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::PrivateFieldExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
        }
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

impl<'a> GetParent<'a> for IdentifierName<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for IdentifierName<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetParent<'a> for IdentifierReference<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for IdentifierReference<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetParent<'a> for BindingIdentifier<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for BindingIdentifier<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetParent<'a> for LabelIdentifier<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for LabelIdentifier<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetParent<'a> for ThisExpression<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for ThisExpression<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetParent<'a> for ArrayExpression<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for ArrayExpressionElement<'a> {
    fn get_parent(&self) -> Option<AstKind<'a>> {
        match self {
            Self::SpreadElement(it) => GetParent::get_parent(it.as_ref()),
            Self::Elision(it) => GetParent::get_parent(it),
            Self::BooleanLiteral(it) => GetParent::get_parent(it.as_ref()),
            Self::NullLiteral(it) => GetParent::get_parent(it.as_ref()),
            Self::NumericLiteral(it) => GetParent::get_parent(it.as_ref()),
            Self::BigIntLiteral(it) => GetParent::get_parent(it.as_ref()),
            Self::RegExpLiteral(it) => GetParent::get_parent(it.as_ref()),
            Self::StringLiteral(it) => GetParent::get_parent(it.as_ref()),
            Self::TemplateLiteral(it) => GetParent::get_parent(it.as_ref()),
            Self::Identifier(it) => GetParent::get_parent(it.as_ref()),
            Self::MetaProperty(it) => GetParent::get_parent(it.as_ref()),
            Self::Super(it) => GetParent::get_parent(it.as_ref()),
            Self::ArrayExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ArrowFunctionExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::AssignmentExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::AwaitExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::BinaryExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::CallExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ChainExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ClassExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ConditionalExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::FunctionExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ImportExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::LogicalExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::NewExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ObjectExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ParenthesizedExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::SequenceExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::TaggedTemplateExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ThisExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::UnaryExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::UpdateExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::YieldExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::PrivateInExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::JSXElement(it) => GetParent::get_parent(it.as_ref()),
            Self::JSXFragment(it) => GetParent::get_parent(it.as_ref()),
            Self::TSAsExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::TSSatisfiesExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::TSTypeAssertion(it) => GetParent::get_parent(it.as_ref()),
            Self::TSNonNullExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::TSInstantiationExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ComputedMemberExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::StaticMemberExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::PrivateFieldExpression(it) => GetParent::get_parent(it.as_ref()),
        }
    }
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        match self {
            Self::SpreadElement(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::Elision(it) => GetParent::set_parent(it, new_parent),
            Self::BooleanLiteral(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::NullLiteral(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::NumericLiteral(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::BigIntLiteral(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::RegExpLiteral(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::StringLiteral(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TemplateLiteral(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::Identifier(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::MetaProperty(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::Super(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ArrayExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ArrowFunctionExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::AssignmentExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::AwaitExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::BinaryExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::CallExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ChainExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ClassExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ConditionalExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::FunctionExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ImportExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::LogicalExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::NewExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ObjectExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ParenthesizedExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::SequenceExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TaggedTemplateExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ThisExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::UnaryExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::UpdateExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::YieldExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::PrivateInExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::JSXElement(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::JSXFragment(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSAsExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSSatisfiesExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSTypeAssertion(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSNonNullExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSInstantiationExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ComputedMemberExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::StaticMemberExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::PrivateFieldExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
        }
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

impl<'a> GetParent<'a> for Elision<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for Elision<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetParent<'a> for ObjectExpression<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for ObjectPropertyKind<'a> {
    fn get_parent(&self) -> Option<AstKind<'a>> {
        match self {
            Self::ObjectProperty(it) => GetParent::get_parent(it.as_ref()),
            Self::SpreadProperty(it) => GetParent::get_parent(it.as_ref()),
        }
    }
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        match self {
            Self::ObjectProperty(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::SpreadProperty(it) => GetParent::set_parent(it.as_mut(), new_parent),
        }
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

impl<'a> GetParent<'a> for ObjectProperty<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for PropertyKey<'a> {
    fn get_parent(&self) -> Option<AstKind<'a>> {
        match self {
            Self::StaticIdentifier(it) => GetParent::get_parent(it.as_ref()),
            Self::PrivateIdentifier(it) => GetParent::get_parent(it.as_ref()),
            Self::BooleanLiteral(it) => GetParent::get_parent(it.as_ref()),
            Self::NullLiteral(it) => GetParent::get_parent(it.as_ref()),
            Self::NumericLiteral(it) => GetParent::get_parent(it.as_ref()),
            Self::BigIntLiteral(it) => GetParent::get_parent(it.as_ref()),
            Self::RegExpLiteral(it) => GetParent::get_parent(it.as_ref()),
            Self::StringLiteral(it) => GetParent::get_parent(it.as_ref()),
            Self::TemplateLiteral(it) => GetParent::get_parent(it.as_ref()),
            Self::Identifier(it) => GetParent::get_parent(it.as_ref()),
            Self::MetaProperty(it) => GetParent::get_parent(it.as_ref()),
            Self::Super(it) => GetParent::get_parent(it.as_ref()),
            Self::ArrayExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ArrowFunctionExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::AssignmentExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::AwaitExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::BinaryExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::CallExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ChainExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ClassExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ConditionalExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::FunctionExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ImportExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::LogicalExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::NewExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ObjectExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ParenthesizedExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::SequenceExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::TaggedTemplateExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ThisExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::UnaryExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::UpdateExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::YieldExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::PrivateInExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::JSXElement(it) => GetParent::get_parent(it.as_ref()),
            Self::JSXFragment(it) => GetParent::get_parent(it.as_ref()),
            Self::TSAsExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::TSSatisfiesExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::TSTypeAssertion(it) => GetParent::get_parent(it.as_ref()),
            Self::TSNonNullExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::TSInstantiationExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ComputedMemberExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::StaticMemberExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::PrivateFieldExpression(it) => GetParent::get_parent(it.as_ref()),
        }
    }
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        match self {
            Self::StaticIdentifier(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::PrivateIdentifier(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::BooleanLiteral(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::NullLiteral(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::NumericLiteral(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::BigIntLiteral(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::RegExpLiteral(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::StringLiteral(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TemplateLiteral(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::Identifier(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::MetaProperty(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::Super(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ArrayExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ArrowFunctionExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::AssignmentExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::AwaitExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::BinaryExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::CallExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ChainExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ClassExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ConditionalExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::FunctionExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ImportExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::LogicalExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::NewExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ObjectExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ParenthesizedExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::SequenceExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TaggedTemplateExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ThisExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::UnaryExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::UpdateExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::YieldExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::PrivateInExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::JSXElement(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::JSXFragment(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSAsExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSSatisfiesExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSTypeAssertion(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSNonNullExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSInstantiationExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ComputedMemberExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::StaticMemberExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::PrivateFieldExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
        }
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

impl<'a> GetParent<'a> for TemplateLiteral<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for TaggedTemplateExpression<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for TemplateElement<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for TemplateElement<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetParent<'a> for MemberExpression<'a> {
    fn get_parent(&self) -> Option<AstKind<'a>> {
        match self {
            Self::ComputedMemberExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::StaticMemberExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::PrivateFieldExpression(it) => GetParent::get_parent(it.as_ref()),
        }
    }
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        match self {
            Self::ComputedMemberExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::StaticMemberExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::PrivateFieldExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
        }
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

impl<'a> GetParent<'a> for ComputedMemberExpression<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for StaticMemberExpression<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for PrivateFieldExpression<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for CallExpression<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for NewExpression<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for MetaProperty<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for SpreadElement<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for SpreadElement<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_expression(&self.argument));
        children
    }
}

impl<'a> GetParent<'a> for Argument<'a> {
    fn get_parent(&self) -> Option<AstKind<'a>> {
        match self {
            Self::SpreadElement(it) => GetParent::get_parent(it.as_ref()),
            Self::BooleanLiteral(it) => GetParent::get_parent(it.as_ref()),
            Self::NullLiteral(it) => GetParent::get_parent(it.as_ref()),
            Self::NumericLiteral(it) => GetParent::get_parent(it.as_ref()),
            Self::BigIntLiteral(it) => GetParent::get_parent(it.as_ref()),
            Self::RegExpLiteral(it) => GetParent::get_parent(it.as_ref()),
            Self::StringLiteral(it) => GetParent::get_parent(it.as_ref()),
            Self::TemplateLiteral(it) => GetParent::get_parent(it.as_ref()),
            Self::Identifier(it) => GetParent::get_parent(it.as_ref()),
            Self::MetaProperty(it) => GetParent::get_parent(it.as_ref()),
            Self::Super(it) => GetParent::get_parent(it.as_ref()),
            Self::ArrayExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ArrowFunctionExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::AssignmentExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::AwaitExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::BinaryExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::CallExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ChainExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ClassExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ConditionalExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::FunctionExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ImportExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::LogicalExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::NewExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ObjectExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ParenthesizedExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::SequenceExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::TaggedTemplateExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ThisExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::UnaryExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::UpdateExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::YieldExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::PrivateInExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::JSXElement(it) => GetParent::get_parent(it.as_ref()),
            Self::JSXFragment(it) => GetParent::get_parent(it.as_ref()),
            Self::TSAsExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::TSSatisfiesExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::TSTypeAssertion(it) => GetParent::get_parent(it.as_ref()),
            Self::TSNonNullExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::TSInstantiationExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ComputedMemberExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::StaticMemberExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::PrivateFieldExpression(it) => GetParent::get_parent(it.as_ref()),
        }
    }
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        match self {
            Self::SpreadElement(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::BooleanLiteral(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::NullLiteral(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::NumericLiteral(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::BigIntLiteral(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::RegExpLiteral(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::StringLiteral(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TemplateLiteral(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::Identifier(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::MetaProperty(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::Super(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ArrayExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ArrowFunctionExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::AssignmentExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::AwaitExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::BinaryExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::CallExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ChainExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ClassExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ConditionalExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::FunctionExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ImportExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::LogicalExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::NewExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ObjectExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ParenthesizedExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::SequenceExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TaggedTemplateExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ThisExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::UnaryExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::UpdateExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::YieldExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::PrivateInExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::JSXElement(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::JSXFragment(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSAsExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSSatisfiesExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSTypeAssertion(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSNonNullExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSInstantiationExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ComputedMemberExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::StaticMemberExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::PrivateFieldExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
        }
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

impl<'a> GetParent<'a> for UpdateExpression<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for UpdateExpression<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::SimpleAssignmentTarget(&self.argument));
        children
    }
}

impl<'a> GetParent<'a> for UnaryExpression<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for UnaryExpression<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_expression(&self.argument));
        children
    }
}

impl<'a> GetParent<'a> for BinaryExpression<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for PrivateInExpression<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for LogicalExpression<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for ConditionalExpression<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for AssignmentExpression<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for AssignmentTarget<'a> {
    fn get_parent(&self) -> Option<AstKind<'a>> {
        match self {
            Self::AssignmentTargetIdentifier(it) => GetParent::get_parent(it.as_ref()),
            Self::TSAsExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::TSSatisfiesExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::TSNonNullExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::TSTypeAssertion(it) => GetParent::get_parent(it.as_ref()),
            Self::TSInstantiationExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ComputedMemberExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::StaticMemberExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::PrivateFieldExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ArrayAssignmentTarget(it) => GetParent::get_parent(it.as_ref()),
            Self::ObjectAssignmentTarget(it) => GetParent::get_parent(it.as_ref()),
        }
    }
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        match self {
            Self::AssignmentTargetIdentifier(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSAsExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSSatisfiesExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSNonNullExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSTypeAssertion(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSInstantiationExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ComputedMemberExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::StaticMemberExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::PrivateFieldExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ArrayAssignmentTarget(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ObjectAssignmentTarget(it) => GetParent::set_parent(it.as_mut(), new_parent),
        }
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

impl<'a> GetParent<'a> for SimpleAssignmentTarget<'a> {
    fn get_parent(&self) -> Option<AstKind<'a>> {
        match self {
            Self::AssignmentTargetIdentifier(it) => GetParent::get_parent(it.as_ref()),
            Self::TSAsExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::TSSatisfiesExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::TSNonNullExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::TSTypeAssertion(it) => GetParent::get_parent(it.as_ref()),
            Self::TSInstantiationExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ComputedMemberExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::StaticMemberExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::PrivateFieldExpression(it) => GetParent::get_parent(it.as_ref()),
        }
    }
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        match self {
            Self::AssignmentTargetIdentifier(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSAsExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSSatisfiesExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSNonNullExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSTypeAssertion(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSInstantiationExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ComputedMemberExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::StaticMemberExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::PrivateFieldExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
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

impl<'a> GetParent<'a> for AssignmentTargetPattern<'a> {
    fn get_parent(&self) -> Option<AstKind<'a>> {
        match self {
            Self::ArrayAssignmentTarget(it) => GetParent::get_parent(it.as_ref()),
            Self::ObjectAssignmentTarget(it) => GetParent::get_parent(it.as_ref()),
        }
    }
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        match self {
            Self::ArrayAssignmentTarget(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ObjectAssignmentTarget(it) => GetParent::set_parent(it.as_mut(), new_parent),
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

impl<'a> GetParent<'a> for ArrayAssignmentTarget<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for ObjectAssignmentTarget<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for AssignmentTargetRest<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for AssignmentTargetRest<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::AssignmentTarget(&self.target));
        children
    }
}

impl<'a> GetParent<'a> for AssignmentTargetMaybeDefault<'a> {
    fn get_parent(&self) -> Option<AstKind<'a>> {
        match self {
            Self::AssignmentTargetWithDefault(it) => GetParent::get_parent(it.as_ref()),
            Self::AssignmentTargetIdentifier(it) => GetParent::get_parent(it.as_ref()),
            Self::TSAsExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::TSSatisfiesExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::TSNonNullExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::TSTypeAssertion(it) => GetParent::get_parent(it.as_ref()),
            Self::TSInstantiationExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ComputedMemberExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::StaticMemberExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::PrivateFieldExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ArrayAssignmentTarget(it) => GetParent::get_parent(it.as_ref()),
            Self::ObjectAssignmentTarget(it) => GetParent::get_parent(it.as_ref()),
        }
    }
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        match self {
            Self::AssignmentTargetWithDefault(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::AssignmentTargetIdentifier(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSAsExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSSatisfiesExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSNonNullExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSTypeAssertion(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSInstantiationExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ComputedMemberExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::StaticMemberExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::PrivateFieldExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ArrayAssignmentTarget(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ObjectAssignmentTarget(it) => GetParent::set_parent(it.as_mut(), new_parent),
        }
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

impl<'a> GetParent<'a> for AssignmentTargetWithDefault<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for AssignmentTargetProperty<'a> {
    fn get_parent(&self) -> Option<AstKind<'a>> {
        match self {
            Self::AssignmentTargetPropertyIdentifier(it) => GetParent::get_parent(it.as_ref()),
            Self::AssignmentTargetPropertyProperty(it) => GetParent::get_parent(it.as_ref()),
        }
    }
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        match self {
            Self::AssignmentTargetPropertyIdentifier(it) => {
                GetParent::set_parent(it.as_mut(), new_parent)
            }
            Self::AssignmentTargetPropertyProperty(it) => {
                GetParent::set_parent(it.as_mut(), new_parent)
            }
        }
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

impl<'a> GetParent<'a> for AssignmentTargetPropertyIdentifier<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for AssignmentTargetPropertyProperty<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for SequenceExpression<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for Super<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for Super<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetParent<'a> for AwaitExpression<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for AwaitExpression<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_expression(&self.argument));
        children
    }
}

impl<'a> GetParent<'a> for ChainExpression<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for ChainExpression<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_chain_element(&self.expression));
        children
    }
}

impl<'a> GetParent<'a> for ChainElement<'a> {
    fn get_parent(&self) -> Option<AstKind<'a>> {
        match self {
            Self::CallExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::TSNonNullExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ComputedMemberExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::StaticMemberExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::PrivateFieldExpression(it) => GetParent::get_parent(it.as_ref()),
        }
    }
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        match self {
            Self::CallExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSNonNullExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ComputedMemberExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::StaticMemberExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::PrivateFieldExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
        }
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

impl<'a> GetParent<'a> for ParenthesizedExpression<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for ParenthesizedExpression<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_expression(&self.expression));
        children
    }
}

impl<'a> GetParent<'a> for Statement<'a> {
    fn get_parent(&self) -> Option<AstKind<'a>> {
        match self {
            Self::BlockStatement(it) => GetParent::get_parent(it.as_ref()),
            Self::BreakStatement(it) => GetParent::get_parent(it.as_ref()),
            Self::ContinueStatement(it) => GetParent::get_parent(it.as_ref()),
            Self::DebuggerStatement(it) => GetParent::get_parent(it.as_ref()),
            Self::DoWhileStatement(it) => GetParent::get_parent(it.as_ref()),
            Self::EmptyStatement(it) => GetParent::get_parent(it.as_ref()),
            Self::ExpressionStatement(it) => GetParent::get_parent(it.as_ref()),
            Self::ForInStatement(it) => GetParent::get_parent(it.as_ref()),
            Self::ForOfStatement(it) => GetParent::get_parent(it.as_ref()),
            Self::ForStatement(it) => GetParent::get_parent(it.as_ref()),
            Self::IfStatement(it) => GetParent::get_parent(it.as_ref()),
            Self::LabeledStatement(it) => GetParent::get_parent(it.as_ref()),
            Self::ReturnStatement(it) => GetParent::get_parent(it.as_ref()),
            Self::SwitchStatement(it) => GetParent::get_parent(it.as_ref()),
            Self::ThrowStatement(it) => GetParent::get_parent(it.as_ref()),
            Self::TryStatement(it) => GetParent::get_parent(it.as_ref()),
            Self::WhileStatement(it) => GetParent::get_parent(it.as_ref()),
            Self::WithStatement(it) => GetParent::get_parent(it.as_ref()),
            Self::VariableDeclaration(it) => GetParent::get_parent(it.as_ref()),
            Self::FunctionDeclaration(it) => GetParent::get_parent(it.as_ref()),
            Self::ClassDeclaration(it) => GetParent::get_parent(it.as_ref()),
            Self::TSTypeAliasDeclaration(it) => GetParent::get_parent(it.as_ref()),
            Self::TSInterfaceDeclaration(it) => GetParent::get_parent(it.as_ref()),
            Self::TSEnumDeclaration(it) => GetParent::get_parent(it.as_ref()),
            Self::TSModuleDeclaration(it) => GetParent::get_parent(it.as_ref()),
            Self::TSImportEqualsDeclaration(it) => GetParent::get_parent(it.as_ref()),
            Self::ImportDeclaration(it) => GetParent::get_parent(it.as_ref()),
            Self::ExportAllDeclaration(it) => GetParent::get_parent(it.as_ref()),
            Self::ExportDefaultDeclaration(it) => GetParent::get_parent(it.as_ref()),
            Self::ExportNamedDeclaration(it) => GetParent::get_parent(it.as_ref()),
            Self::TSExportAssignment(it) => GetParent::get_parent(it.as_ref()),
            Self::TSNamespaceExportDeclaration(it) => GetParent::get_parent(it.as_ref()),
        }
    }
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        match self {
            Self::BlockStatement(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::BreakStatement(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ContinueStatement(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::DebuggerStatement(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::DoWhileStatement(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::EmptyStatement(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ExpressionStatement(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ForInStatement(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ForOfStatement(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ForStatement(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::IfStatement(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::LabeledStatement(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ReturnStatement(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::SwitchStatement(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ThrowStatement(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TryStatement(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::WhileStatement(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::WithStatement(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::VariableDeclaration(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::FunctionDeclaration(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ClassDeclaration(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSTypeAliasDeclaration(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSInterfaceDeclaration(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSEnumDeclaration(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSModuleDeclaration(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSImportEqualsDeclaration(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ImportDeclaration(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ExportAllDeclaration(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ExportDefaultDeclaration(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ExportNamedDeclaration(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSExportAssignment(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSNamespaceExportDeclaration(it) => {
                GetParent::set_parent(it.as_mut(), new_parent)
            }
        }
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

impl<'a> GetParent<'a> for Directive<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for Directive<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::StringLiteral(&self.expression));
        children
    }
}

impl<'a> GetParent<'a> for Hashbang<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for Hashbang<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetParent<'a> for BlockStatement<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for Declaration<'a> {
    fn get_parent(&self) -> Option<AstKind<'a>> {
        match self {
            Self::VariableDeclaration(it) => GetParent::get_parent(it.as_ref()),
            Self::FunctionDeclaration(it) => GetParent::get_parent(it.as_ref()),
            Self::ClassDeclaration(it) => GetParent::get_parent(it.as_ref()),
            Self::TSTypeAliasDeclaration(it) => GetParent::get_parent(it.as_ref()),
            Self::TSInterfaceDeclaration(it) => GetParent::get_parent(it.as_ref()),
            Self::TSEnumDeclaration(it) => GetParent::get_parent(it.as_ref()),
            Self::TSModuleDeclaration(it) => GetParent::get_parent(it.as_ref()),
            Self::TSImportEqualsDeclaration(it) => GetParent::get_parent(it.as_ref()),
        }
    }
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        match self {
            Self::VariableDeclaration(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::FunctionDeclaration(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ClassDeclaration(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSTypeAliasDeclaration(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSInterfaceDeclaration(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSEnumDeclaration(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSModuleDeclaration(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSImportEqualsDeclaration(it) => GetParent::set_parent(it.as_mut(), new_parent),
        }
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

impl<'a> GetParent<'a> for VariableDeclaration<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for VariableDeclarator<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for EmptyStatement<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for EmptyStatement<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetParent<'a> for ExpressionStatement<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for ExpressionStatement<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_expression(&self.expression));
        children
    }
}

impl<'a> GetParent<'a> for IfStatement<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for DoWhileStatement<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for WhileStatement<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for ForStatement<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for ForStatementInit<'a> {
    fn get_parent(&self) -> Option<AstKind<'a>> {
        match self {
            Self::VariableDeclaration(it) => GetParent::get_parent(it.as_ref()),
            Self::BooleanLiteral(it) => GetParent::get_parent(it.as_ref()),
            Self::NullLiteral(it) => GetParent::get_parent(it.as_ref()),
            Self::NumericLiteral(it) => GetParent::get_parent(it.as_ref()),
            Self::BigIntLiteral(it) => GetParent::get_parent(it.as_ref()),
            Self::RegExpLiteral(it) => GetParent::get_parent(it.as_ref()),
            Self::StringLiteral(it) => GetParent::get_parent(it.as_ref()),
            Self::TemplateLiteral(it) => GetParent::get_parent(it.as_ref()),
            Self::Identifier(it) => GetParent::get_parent(it.as_ref()),
            Self::MetaProperty(it) => GetParent::get_parent(it.as_ref()),
            Self::Super(it) => GetParent::get_parent(it.as_ref()),
            Self::ArrayExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ArrowFunctionExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::AssignmentExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::AwaitExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::BinaryExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::CallExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ChainExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ClassExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ConditionalExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::FunctionExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ImportExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::LogicalExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::NewExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ObjectExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ParenthesizedExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::SequenceExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::TaggedTemplateExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ThisExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::UnaryExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::UpdateExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::YieldExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::PrivateInExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::JSXElement(it) => GetParent::get_parent(it.as_ref()),
            Self::JSXFragment(it) => GetParent::get_parent(it.as_ref()),
            Self::TSAsExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::TSSatisfiesExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::TSTypeAssertion(it) => GetParent::get_parent(it.as_ref()),
            Self::TSNonNullExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::TSInstantiationExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ComputedMemberExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::StaticMemberExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::PrivateFieldExpression(it) => GetParent::get_parent(it.as_ref()),
        }
    }
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        match self {
            Self::VariableDeclaration(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::BooleanLiteral(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::NullLiteral(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::NumericLiteral(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::BigIntLiteral(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::RegExpLiteral(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::StringLiteral(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TemplateLiteral(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::Identifier(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::MetaProperty(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::Super(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ArrayExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ArrowFunctionExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::AssignmentExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::AwaitExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::BinaryExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::CallExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ChainExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ClassExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ConditionalExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::FunctionExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ImportExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::LogicalExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::NewExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ObjectExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ParenthesizedExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::SequenceExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TaggedTemplateExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ThisExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::UnaryExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::UpdateExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::YieldExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::PrivateInExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::JSXElement(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::JSXFragment(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSAsExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSSatisfiesExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSTypeAssertion(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSNonNullExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSInstantiationExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ComputedMemberExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::StaticMemberExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::PrivateFieldExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
        }
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

impl<'a> GetParent<'a> for ForInStatement<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for ForStatementLeft<'a> {
    fn get_parent(&self) -> Option<AstKind<'a>> {
        match self {
            Self::VariableDeclaration(it) => GetParent::get_parent(it.as_ref()),
            Self::AssignmentTargetIdentifier(it) => GetParent::get_parent(it.as_ref()),
            Self::TSAsExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::TSSatisfiesExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::TSNonNullExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::TSTypeAssertion(it) => GetParent::get_parent(it.as_ref()),
            Self::TSInstantiationExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ComputedMemberExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::StaticMemberExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::PrivateFieldExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ArrayAssignmentTarget(it) => GetParent::get_parent(it.as_ref()),
            Self::ObjectAssignmentTarget(it) => GetParent::get_parent(it.as_ref()),
        }
    }
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        match self {
            Self::VariableDeclaration(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::AssignmentTargetIdentifier(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSAsExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSSatisfiesExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSNonNullExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSTypeAssertion(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSInstantiationExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ComputedMemberExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::StaticMemberExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::PrivateFieldExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ArrayAssignmentTarget(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ObjectAssignmentTarget(it) => GetParent::set_parent(it.as_mut(), new_parent),
        }
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

impl<'a> GetParent<'a> for ForOfStatement<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for ContinueStatement<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for BreakStatement<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for ReturnStatement<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for WithStatement<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for SwitchStatement<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for SwitchCase<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for LabeledStatement<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for ThrowStatement<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for ThrowStatement<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_expression(&self.argument));
        children
    }
}

impl<'a> GetParent<'a> for TryStatement<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for CatchClause<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for CatchParameter<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for CatchParameter<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::BindingPattern(&self.pattern));
        children
    }
}

impl<'a> GetParent<'a> for DebuggerStatement<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for DebuggerStatement<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetParent<'a> for BindingPattern<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for BindingPatternKind<'a> {
    fn get_parent(&self) -> Option<AstKind<'a>> {
        match self {
            Self::BindingIdentifier(it) => GetParent::get_parent(it.as_ref()),
            Self::ObjectPattern(it) => GetParent::get_parent(it.as_ref()),
            Self::ArrayPattern(it) => GetParent::get_parent(it.as_ref()),
            Self::AssignmentPattern(it) => GetParent::get_parent(it.as_ref()),
        }
    }
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        match self {
            Self::BindingIdentifier(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ObjectPattern(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ArrayPattern(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::AssignmentPattern(it) => GetParent::set_parent(it.as_mut(), new_parent),
        }
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

impl<'a> GetParent<'a> for AssignmentPattern<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for ObjectPattern<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for BindingProperty<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for ArrayPattern<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for BindingRestElement<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for BindingRestElement<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::BindingPattern(&self.argument));
        children
    }
}

impl<'a> GetParent<'a> for Function<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for FormalParameters<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for FormalParameter<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for FunctionBody<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for ArrowFunctionExpression<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for YieldExpression<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for Class<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for ClassBody<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for ClassElement<'a> {
    fn get_parent(&self) -> Option<AstKind<'a>> {
        match self {
            Self::StaticBlock(it) => GetParent::get_parent(it.as_ref()),
            Self::MethodDefinition(it) => GetParent::get_parent(it.as_ref()),
            Self::PropertyDefinition(it) => GetParent::get_parent(it.as_ref()),
            Self::AccessorProperty(it) => GetParent::get_parent(it.as_ref()),
            Self::TSIndexSignature(it) => GetParent::get_parent(it.as_ref()),
        }
    }
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        match self {
            Self::StaticBlock(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::MethodDefinition(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::PropertyDefinition(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::AccessorProperty(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSIndexSignature(it) => GetParent::set_parent(it.as_mut(), new_parent),
        }
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

impl<'a> GetParent<'a> for MethodDefinition<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for PropertyDefinition<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for PrivateIdentifier<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for PrivateIdentifier<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetParent<'a> for StaticBlock<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for ModuleDeclaration<'a> {
    fn get_parent(&self) -> Option<AstKind<'a>> {
        match self {
            Self::ImportDeclaration(it) => GetParent::get_parent(it.as_ref()),
            Self::ExportAllDeclaration(it) => GetParent::get_parent(it.as_ref()),
            Self::ExportDefaultDeclaration(it) => GetParent::get_parent(it.as_ref()),
            Self::ExportNamedDeclaration(it) => GetParent::get_parent(it.as_ref()),
            Self::TSExportAssignment(it) => GetParent::get_parent(it.as_ref()),
            Self::TSNamespaceExportDeclaration(it) => GetParent::get_parent(it.as_ref()),
        }
    }
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        match self {
            Self::ImportDeclaration(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ExportAllDeclaration(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ExportDefaultDeclaration(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ExportNamedDeclaration(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSExportAssignment(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSNamespaceExportDeclaration(it) => {
                GetParent::set_parent(it.as_mut(), new_parent)
            }
        }
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

impl<'a> GetParent<'a> for AccessorProperty<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for ImportExpression<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for ImportDeclaration<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for ImportDeclarationSpecifier<'a> {
    fn get_parent(&self) -> Option<AstKind<'a>> {
        match self {
            Self::ImportSpecifier(it) => GetParent::get_parent(it.as_ref()),
            Self::ImportDefaultSpecifier(it) => GetParent::get_parent(it.as_ref()),
            Self::ImportNamespaceSpecifier(it) => GetParent::get_parent(it.as_ref()),
        }
    }
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        match self {
            Self::ImportSpecifier(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ImportDefaultSpecifier(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ImportNamespaceSpecifier(it) => GetParent::set_parent(it.as_mut(), new_parent),
        }
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

impl<'a> GetParent<'a> for ImportSpecifier<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for ImportDefaultSpecifier<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for ImportDefaultSpecifier<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::BindingIdentifier(&self.local));
        children
    }
}

impl<'a> GetParent<'a> for ImportNamespaceSpecifier<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for ImportNamespaceSpecifier<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::BindingIdentifier(&self.local));
        children
    }
}

impl<'a> GetParent<'a> for WithClause<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for ImportAttribute<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for ImportAttributeKey<'a> {
    fn get_parent(&self) -> Option<AstKind<'a>> {
        match self {
            Self::Identifier(it) => GetParent::get_parent(it),
            Self::StringLiteral(it) => GetParent::get_parent(it),
        }
    }
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        match self {
            Self::Identifier(it) => GetParent::set_parent(it, new_parent),
            Self::StringLiteral(it) => GetParent::set_parent(it, new_parent),
        }
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

impl<'a> GetParent<'a> for ExportNamedDeclaration<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for ExportDefaultDeclaration<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for ExportAllDeclaration<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for ExportSpecifier<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for ExportDefaultDeclarationKind<'a> {
    fn get_parent(&self) -> Option<AstKind<'a>> {
        match self {
            Self::FunctionDeclaration(it) => GetParent::get_parent(it.as_ref()),
            Self::ClassDeclaration(it) => GetParent::get_parent(it.as_ref()),
            Self::TSInterfaceDeclaration(it) => GetParent::get_parent(it.as_ref()),
            Self::BooleanLiteral(it) => GetParent::get_parent(it.as_ref()),
            Self::NullLiteral(it) => GetParent::get_parent(it.as_ref()),
            Self::NumericLiteral(it) => GetParent::get_parent(it.as_ref()),
            Self::BigIntLiteral(it) => GetParent::get_parent(it.as_ref()),
            Self::RegExpLiteral(it) => GetParent::get_parent(it.as_ref()),
            Self::StringLiteral(it) => GetParent::get_parent(it.as_ref()),
            Self::TemplateLiteral(it) => GetParent::get_parent(it.as_ref()),
            Self::Identifier(it) => GetParent::get_parent(it.as_ref()),
            Self::MetaProperty(it) => GetParent::get_parent(it.as_ref()),
            Self::Super(it) => GetParent::get_parent(it.as_ref()),
            Self::ArrayExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ArrowFunctionExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::AssignmentExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::AwaitExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::BinaryExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::CallExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ChainExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ClassExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ConditionalExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::FunctionExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ImportExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::LogicalExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::NewExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ObjectExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ParenthesizedExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::SequenceExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::TaggedTemplateExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ThisExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::UnaryExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::UpdateExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::YieldExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::PrivateInExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::JSXElement(it) => GetParent::get_parent(it.as_ref()),
            Self::JSXFragment(it) => GetParent::get_parent(it.as_ref()),
            Self::TSAsExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::TSSatisfiesExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::TSTypeAssertion(it) => GetParent::get_parent(it.as_ref()),
            Self::TSNonNullExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::TSInstantiationExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ComputedMemberExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::StaticMemberExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::PrivateFieldExpression(it) => GetParent::get_parent(it.as_ref()),
        }
    }
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        match self {
            Self::FunctionDeclaration(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ClassDeclaration(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSInterfaceDeclaration(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::BooleanLiteral(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::NullLiteral(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::NumericLiteral(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::BigIntLiteral(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::RegExpLiteral(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::StringLiteral(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TemplateLiteral(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::Identifier(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::MetaProperty(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::Super(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ArrayExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ArrowFunctionExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::AssignmentExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::AwaitExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::BinaryExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::CallExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ChainExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ClassExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ConditionalExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::FunctionExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ImportExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::LogicalExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::NewExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ObjectExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ParenthesizedExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::SequenceExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TaggedTemplateExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ThisExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::UnaryExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::UpdateExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::YieldExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::PrivateInExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::JSXElement(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::JSXFragment(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSAsExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSSatisfiesExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSTypeAssertion(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSNonNullExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSInstantiationExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ComputedMemberExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::StaticMemberExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::PrivateFieldExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
        }
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

impl<'a> GetParent<'a> for ModuleExportName<'a> {
    fn get_parent(&self) -> Option<AstKind<'a>> {
        match self {
            Self::IdentifierName(it) => GetParent::get_parent(it),
            Self::IdentifierReference(it) => GetParent::get_parent(it),
            Self::StringLiteral(it) => GetParent::get_parent(it),
        }
    }
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        match self {
            Self::IdentifierName(it) => GetParent::set_parent(it, new_parent),
            Self::IdentifierReference(it) => GetParent::set_parent(it, new_parent),
            Self::StringLiteral(it) => GetParent::set_parent(it, new_parent),
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

impl<'a> GetParent<'a> for TSThisParameter<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for TSEnumDeclaration<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for TSEnumMember<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for TSEnumMemberName<'a> {
    fn get_parent(&self) -> Option<AstKind<'a>> {
        match self {
            Self::Identifier(it) => GetParent::get_parent(it.as_ref()),
            Self::String(it) => GetParent::get_parent(it.as_ref()),
        }
    }
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        match self {
            Self::Identifier(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::String(it) => GetParent::set_parent(it.as_mut(), new_parent),
        }
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

impl<'a> GetParent<'a> for TSTypeAnnotation<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for TSTypeAnnotation<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_ts_type(&self.type_annotation));
        children
    }
}

impl<'a> GetParent<'a> for TSLiteralType<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for TSLiteralType<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_ts_literal(&self.literal));
        children
    }
}

impl<'a> GetParent<'a> for TSLiteral<'a> {
    fn get_parent(&self) -> Option<AstKind<'a>> {
        match self {
            Self::BooleanLiteral(it) => GetParent::get_parent(it.as_ref()),
            Self::NullLiteral(it) => GetParent::get_parent(it.as_ref()),
            Self::NumericLiteral(it) => GetParent::get_parent(it.as_ref()),
            Self::BigIntLiteral(it) => GetParent::get_parent(it.as_ref()),
            Self::RegExpLiteral(it) => GetParent::get_parent(it.as_ref()),
            Self::StringLiteral(it) => GetParent::get_parent(it.as_ref()),
            Self::TemplateLiteral(it) => GetParent::get_parent(it.as_ref()),
            Self::UnaryExpression(it) => GetParent::get_parent(it.as_ref()),
        }
    }
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        match self {
            Self::BooleanLiteral(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::NullLiteral(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::NumericLiteral(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::BigIntLiteral(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::RegExpLiteral(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::StringLiteral(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TemplateLiteral(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::UnaryExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
        }
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

impl<'a> GetParent<'a> for TSType<'a> {
    fn get_parent(&self) -> Option<AstKind<'a>> {
        match self {
            Self::TSAnyKeyword(it) => GetParent::get_parent(it.as_ref()),
            Self::TSBigIntKeyword(it) => GetParent::get_parent(it.as_ref()),
            Self::TSBooleanKeyword(it) => GetParent::get_parent(it.as_ref()),
            Self::TSIntrinsicKeyword(it) => GetParent::get_parent(it.as_ref()),
            Self::TSNeverKeyword(it) => GetParent::get_parent(it.as_ref()),
            Self::TSNullKeyword(it) => GetParent::get_parent(it.as_ref()),
            Self::TSNumberKeyword(it) => GetParent::get_parent(it.as_ref()),
            Self::TSObjectKeyword(it) => GetParent::get_parent(it.as_ref()),
            Self::TSStringKeyword(it) => GetParent::get_parent(it.as_ref()),
            Self::TSSymbolKeyword(it) => GetParent::get_parent(it.as_ref()),
            Self::TSUndefinedKeyword(it) => GetParent::get_parent(it.as_ref()),
            Self::TSUnknownKeyword(it) => GetParent::get_parent(it.as_ref()),
            Self::TSVoidKeyword(it) => GetParent::get_parent(it.as_ref()),
            Self::TSArrayType(it) => GetParent::get_parent(it.as_ref()),
            Self::TSConditionalType(it) => GetParent::get_parent(it.as_ref()),
            Self::TSConstructorType(it) => GetParent::get_parent(it.as_ref()),
            Self::TSFunctionType(it) => GetParent::get_parent(it.as_ref()),
            Self::TSImportType(it) => GetParent::get_parent(it.as_ref()),
            Self::TSIndexedAccessType(it) => GetParent::get_parent(it.as_ref()),
            Self::TSInferType(it) => GetParent::get_parent(it.as_ref()),
            Self::TSIntersectionType(it) => GetParent::get_parent(it.as_ref()),
            Self::TSLiteralType(it) => GetParent::get_parent(it.as_ref()),
            Self::TSMappedType(it) => GetParent::get_parent(it.as_ref()),
            Self::TSNamedTupleMember(it) => GetParent::get_parent(it.as_ref()),
            Self::TSQualifiedName(it) => GetParent::get_parent(it.as_ref()),
            Self::TSTemplateLiteralType(it) => GetParent::get_parent(it.as_ref()),
            Self::TSThisType(it) => GetParent::get_parent(it.as_ref()),
            Self::TSTupleType(it) => GetParent::get_parent(it.as_ref()),
            Self::TSTypeLiteral(it) => GetParent::get_parent(it.as_ref()),
            Self::TSTypeOperatorType(it) => GetParent::get_parent(it.as_ref()),
            Self::TSTypePredicate(it) => GetParent::get_parent(it.as_ref()),
            Self::TSTypeQuery(it) => GetParent::get_parent(it.as_ref()),
            Self::TSTypeReference(it) => GetParent::get_parent(it.as_ref()),
            Self::TSUnionType(it) => GetParent::get_parent(it.as_ref()),
            Self::TSParenthesizedType(it) => GetParent::get_parent(it.as_ref()),
            Self::JSDocNullableType(it) => GetParent::get_parent(it.as_ref()),
            Self::JSDocNonNullableType(it) => GetParent::get_parent(it.as_ref()),
            Self::JSDocUnknownType(it) => GetParent::get_parent(it.as_ref()),
        }
    }
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        match self {
            Self::TSAnyKeyword(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSBigIntKeyword(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSBooleanKeyword(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSIntrinsicKeyword(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSNeverKeyword(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSNullKeyword(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSNumberKeyword(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSObjectKeyword(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSStringKeyword(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSSymbolKeyword(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSUndefinedKeyword(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSUnknownKeyword(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSVoidKeyword(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSArrayType(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSConditionalType(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSConstructorType(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSFunctionType(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSImportType(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSIndexedAccessType(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSInferType(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSIntersectionType(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSLiteralType(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSMappedType(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSNamedTupleMember(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSQualifiedName(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSTemplateLiteralType(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSThisType(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSTupleType(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSTypeLiteral(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSTypeOperatorType(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSTypePredicate(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSTypeQuery(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSTypeReference(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSUnionType(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSParenthesizedType(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::JSDocNullableType(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::JSDocNonNullableType(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::JSDocUnknownType(it) => GetParent::set_parent(it.as_mut(), new_parent),
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

impl<'a> GetParent<'a> for TSConditionalType<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for TSUnionType<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for TSIntersectionType<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for TSParenthesizedType<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for TSParenthesizedType<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_ts_type(&self.type_annotation));
        children
    }
}

impl<'a> GetParent<'a> for TSTypeOperator<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for TSTypeOperator<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_ts_type(&self.type_annotation));
        children
    }
}

impl<'a> GetParent<'a> for TSArrayType<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for TSArrayType<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_ts_type(&self.element_type));
        children
    }
}

impl<'a> GetParent<'a> for TSIndexedAccessType<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for TSTupleType<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for TSNamedTupleMember<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for TSOptionalType<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for TSOptionalType<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_ts_type(&self.type_annotation));
        children
    }
}

impl<'a> GetParent<'a> for TSRestType<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for TSRestType<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_ts_type(&self.type_annotation));
        children
    }
}

impl<'a> GetParent<'a> for TSTupleElement<'a> {
    fn get_parent(&self) -> Option<AstKind<'a>> {
        match self {
            Self::TSOptionalType(it) => GetParent::get_parent(it.as_ref()),
            Self::TSRestType(it) => GetParent::get_parent(it.as_ref()),
            Self::TSAnyKeyword(it) => GetParent::get_parent(it.as_ref()),
            Self::TSBigIntKeyword(it) => GetParent::get_parent(it.as_ref()),
            Self::TSBooleanKeyword(it) => GetParent::get_parent(it.as_ref()),
            Self::TSIntrinsicKeyword(it) => GetParent::get_parent(it.as_ref()),
            Self::TSNeverKeyword(it) => GetParent::get_parent(it.as_ref()),
            Self::TSNullKeyword(it) => GetParent::get_parent(it.as_ref()),
            Self::TSNumberKeyword(it) => GetParent::get_parent(it.as_ref()),
            Self::TSObjectKeyword(it) => GetParent::get_parent(it.as_ref()),
            Self::TSStringKeyword(it) => GetParent::get_parent(it.as_ref()),
            Self::TSSymbolKeyword(it) => GetParent::get_parent(it.as_ref()),
            Self::TSUndefinedKeyword(it) => GetParent::get_parent(it.as_ref()),
            Self::TSUnknownKeyword(it) => GetParent::get_parent(it.as_ref()),
            Self::TSVoidKeyword(it) => GetParent::get_parent(it.as_ref()),
            Self::TSArrayType(it) => GetParent::get_parent(it.as_ref()),
            Self::TSConditionalType(it) => GetParent::get_parent(it.as_ref()),
            Self::TSConstructorType(it) => GetParent::get_parent(it.as_ref()),
            Self::TSFunctionType(it) => GetParent::get_parent(it.as_ref()),
            Self::TSImportType(it) => GetParent::get_parent(it.as_ref()),
            Self::TSIndexedAccessType(it) => GetParent::get_parent(it.as_ref()),
            Self::TSInferType(it) => GetParent::get_parent(it.as_ref()),
            Self::TSIntersectionType(it) => GetParent::get_parent(it.as_ref()),
            Self::TSLiteralType(it) => GetParent::get_parent(it.as_ref()),
            Self::TSMappedType(it) => GetParent::get_parent(it.as_ref()),
            Self::TSNamedTupleMember(it) => GetParent::get_parent(it.as_ref()),
            Self::TSQualifiedName(it) => GetParent::get_parent(it.as_ref()),
            Self::TSTemplateLiteralType(it) => GetParent::get_parent(it.as_ref()),
            Self::TSThisType(it) => GetParent::get_parent(it.as_ref()),
            Self::TSTupleType(it) => GetParent::get_parent(it.as_ref()),
            Self::TSTypeLiteral(it) => GetParent::get_parent(it.as_ref()),
            Self::TSTypeOperatorType(it) => GetParent::get_parent(it.as_ref()),
            Self::TSTypePredicate(it) => GetParent::get_parent(it.as_ref()),
            Self::TSTypeQuery(it) => GetParent::get_parent(it.as_ref()),
            Self::TSTypeReference(it) => GetParent::get_parent(it.as_ref()),
            Self::TSUnionType(it) => GetParent::get_parent(it.as_ref()),
            Self::TSParenthesizedType(it) => GetParent::get_parent(it.as_ref()),
            Self::JSDocNullableType(it) => GetParent::get_parent(it.as_ref()),
            Self::JSDocNonNullableType(it) => GetParent::get_parent(it.as_ref()),
            Self::JSDocUnknownType(it) => GetParent::get_parent(it.as_ref()),
        }
    }
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        match self {
            Self::TSOptionalType(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSRestType(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSAnyKeyword(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSBigIntKeyword(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSBooleanKeyword(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSIntrinsicKeyword(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSNeverKeyword(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSNullKeyword(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSNumberKeyword(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSObjectKeyword(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSStringKeyword(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSSymbolKeyword(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSUndefinedKeyword(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSUnknownKeyword(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSVoidKeyword(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSArrayType(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSConditionalType(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSConstructorType(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSFunctionType(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSImportType(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSIndexedAccessType(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSInferType(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSIntersectionType(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSLiteralType(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSMappedType(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSNamedTupleMember(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSQualifiedName(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSTemplateLiteralType(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSThisType(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSTupleType(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSTypeLiteral(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSTypeOperatorType(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSTypePredicate(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSTypeQuery(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSTypeReference(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSUnionType(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSParenthesizedType(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::JSDocNullableType(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::JSDocNonNullableType(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::JSDocUnknownType(it) => GetParent::set_parent(it.as_mut(), new_parent),
        }
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

impl<'a> GetParent<'a> for TSAnyKeyword<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for TSAnyKeyword<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetParent<'a> for TSStringKeyword<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for TSStringKeyword<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetParent<'a> for TSBooleanKeyword<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for TSBooleanKeyword<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetParent<'a> for TSNumberKeyword<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for TSNumberKeyword<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetParent<'a> for TSNeverKeyword<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for TSNeverKeyword<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetParent<'a> for TSIntrinsicKeyword<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for TSIntrinsicKeyword<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetParent<'a> for TSUnknownKeyword<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for TSUnknownKeyword<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetParent<'a> for TSNullKeyword<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for TSNullKeyword<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetParent<'a> for TSUndefinedKeyword<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for TSUndefinedKeyword<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetParent<'a> for TSVoidKeyword<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for TSVoidKeyword<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetParent<'a> for TSSymbolKeyword<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for TSSymbolKeyword<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetParent<'a> for TSThisType<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for TSThisType<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetParent<'a> for TSObjectKeyword<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for TSObjectKeyword<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetParent<'a> for TSBigIntKeyword<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for TSBigIntKeyword<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetParent<'a> for TSTypeReference<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for TSTypeName<'a> {
    fn get_parent(&self) -> Option<AstKind<'a>> {
        match self {
            Self::IdentifierReference(it) => GetParent::get_parent(it.as_ref()),
            Self::QualifiedName(it) => GetParent::get_parent(it.as_ref()),
        }
    }
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        match self {
            Self::IdentifierReference(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::QualifiedName(it) => GetParent::set_parent(it.as_mut(), new_parent),
        }
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

impl<'a> GetParent<'a> for TSQualifiedName<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for TSTypeParameterInstantiation<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for TSTypeParameter<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for TSTypeParameterDeclaration<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for TSTypeAliasDeclaration<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for TSClassImplements<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for TSInterfaceDeclaration<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for TSInterfaceBody<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for TSPropertySignature<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for TSSignature<'a> {
    fn get_parent(&self) -> Option<AstKind<'a>> {
        match self {
            Self::TSIndexSignature(it) => GetParent::get_parent(it.as_ref()),
            Self::TSPropertySignature(it) => GetParent::get_parent(it.as_ref()),
            Self::TSCallSignatureDeclaration(it) => GetParent::get_parent(it.as_ref()),
            Self::TSConstructSignatureDeclaration(it) => GetParent::get_parent(it.as_ref()),
            Self::TSMethodSignature(it) => GetParent::get_parent(it.as_ref()),
        }
    }
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        match self {
            Self::TSIndexSignature(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSPropertySignature(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSCallSignatureDeclaration(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSConstructSignatureDeclaration(it) => {
                GetParent::set_parent(it.as_mut(), new_parent)
            }
            Self::TSMethodSignature(it) => GetParent::set_parent(it.as_mut(), new_parent),
        }
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

impl<'a> GetParent<'a> for TSIndexSignature<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for TSCallSignatureDeclaration<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for TSMethodSignature<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for TSConstructSignatureDeclaration<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for TSIndexSignatureName<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for TSIndexSignatureName<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::TSTypeAnnotation(&*self.type_annotation));
        children
    }
}

impl<'a> GetParent<'a> for TSInterfaceHeritage<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for TSTypePredicate<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for TSTypePredicateName<'a> {
    fn get_parent(&self) -> Option<AstKind<'a>> {
        match self {
            Self::Identifier(it) => GetParent::get_parent(it.as_ref()),
            Self::This(it) => GetParent::get_parent(it),
        }
    }
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        match self {
            Self::Identifier(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::This(it) => GetParent::set_parent(it, new_parent),
        }
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

impl<'a> GetParent<'a> for TSModuleDeclaration<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for TSModuleDeclarationName<'a> {
    fn get_parent(&self) -> Option<AstKind<'a>> {
        match self {
            Self::Identifier(it) => GetParent::get_parent(it),
            Self::StringLiteral(it) => GetParent::get_parent(it),
        }
    }
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        match self {
            Self::Identifier(it) => GetParent::set_parent(it, new_parent),
            Self::StringLiteral(it) => GetParent::set_parent(it, new_parent),
        }
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

impl<'a> GetParent<'a> for TSModuleDeclarationBody<'a> {
    fn get_parent(&self) -> Option<AstKind<'a>> {
        match self {
            Self::TSModuleDeclaration(it) => GetParent::get_parent(it.as_ref()),
            Self::TSModuleBlock(it) => GetParent::get_parent(it.as_ref()),
        }
    }
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        match self {
            Self::TSModuleDeclaration(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSModuleBlock(it) => GetParent::set_parent(it.as_mut(), new_parent),
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

impl<'a> GetParent<'a> for TSModuleBlock<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for TSTypeLiteral<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for TSInferType<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for TSInferType<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::TSTypeParameter(&*self.type_parameter));
        children
    }
}

impl<'a> GetParent<'a> for TSTypeQuery<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for TSTypeQueryExprName<'a> {
    fn get_parent(&self) -> Option<AstKind<'a>> {
        match self {
            Self::TSImportType(it) => GetParent::get_parent(it.as_ref()),
            Self::IdentifierReference(it) => GetParent::get_parent(it.as_ref()),
            Self::QualifiedName(it) => GetParent::get_parent(it.as_ref()),
        }
    }
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        match self {
            Self::TSImportType(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::IdentifierReference(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::QualifiedName(it) => GetParent::set_parent(it.as_mut(), new_parent),
        }
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

impl<'a> GetParent<'a> for TSImportType<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for TSImportAttributes<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for TSImportAttribute<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for TSImportAttributeName<'a> {
    fn get_parent(&self) -> Option<AstKind<'a>> {
        match self {
            Self::Identifier(it) => GetParent::get_parent(it),
            Self::StringLiteral(it) => GetParent::get_parent(it),
        }
    }
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        match self {
            Self::Identifier(it) => GetParent::set_parent(it, new_parent),
            Self::StringLiteral(it) => GetParent::set_parent(it, new_parent),
        }
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

impl<'a> GetParent<'a> for TSFunctionType<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for TSConstructorType<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for TSMappedType<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for TSTemplateLiteralType<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for TSAsExpression<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for TSSatisfiesExpression<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for TSTypeAssertion<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for TSImportEqualsDeclaration<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for TSModuleReference<'a> {
    fn get_parent(&self) -> Option<AstKind<'a>> {
        match self {
            Self::ExternalModuleReference(it) => GetParent::get_parent(it.as_ref()),
            Self::IdentifierReference(it) => GetParent::get_parent(it.as_ref()),
            Self::QualifiedName(it) => GetParent::get_parent(it.as_ref()),
        }
    }
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        match self {
            Self::ExternalModuleReference(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::IdentifierReference(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::QualifiedName(it) => GetParent::set_parent(it.as_mut(), new_parent),
        }
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

impl<'a> GetParent<'a> for TSExternalModuleReference<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for TSExternalModuleReference<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::StringLiteral(&self.expression));
        children
    }
}

impl<'a> GetParent<'a> for TSNonNullExpression<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for TSNonNullExpression<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_expression(&self.expression));
        children
    }
}

impl<'a> GetParent<'a> for Decorator<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for Decorator<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_expression(&self.expression));
        children
    }
}

impl<'a> GetParent<'a> for TSExportAssignment<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for TSExportAssignment<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_expression(&self.expression));
        children
    }
}

impl<'a> GetParent<'a> for TSNamespaceExportDeclaration<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for TSNamespaceExportDeclaration<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::IdentifierName(&self.id));
        children
    }
}

impl<'a> GetParent<'a> for TSInstantiationExpression<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for JSDocNullableType<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for JSDocNullableType<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_ts_type(&self.type_annotation));
        children
    }
}

impl<'a> GetParent<'a> for JSDocNonNullableType<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for JSDocNonNullableType<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_ts_type(&self.type_annotation));
        children
    }
}

impl<'a> GetParent<'a> for JSDocUnknownType<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for JSDocUnknownType<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetParent<'a> for JSXElement<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for JSXOpeningElement<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for JSXClosingElement<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for JSXClosingElement<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::JSXElementName(&self.name));
        children
    }
}

impl<'a> GetParent<'a> for JSXFragment<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for JSXOpeningFragment<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for JSXOpeningFragment<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetParent<'a> for JSXClosingFragment<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for JSXClosingFragment<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetParent<'a> for JSXElementName<'a> {
    fn get_parent(&self) -> Option<AstKind<'a>> {
        match self {
            Self::Identifier(it) => GetParent::get_parent(it.as_ref()),
            Self::IdentifierReference(it) => GetParent::get_parent(it.as_ref()),
            Self::NamespacedName(it) => GetParent::get_parent(it.as_ref()),
            Self::MemberExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ThisExpression(it) => GetParent::get_parent(it.as_ref()),
        }
    }
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        match self {
            Self::Identifier(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::IdentifierReference(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::NamespacedName(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::MemberExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ThisExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
        }
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

impl<'a> GetParent<'a> for JSXNamespacedName<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for JSXMemberExpression<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for JSXMemberExpressionObject<'a> {
    fn get_parent(&self) -> Option<AstKind<'a>> {
        match self {
            Self::IdentifierReference(it) => GetParent::get_parent(it.as_ref()),
            Self::MemberExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ThisExpression(it) => GetParent::get_parent(it.as_ref()),
        }
    }
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        match self {
            Self::IdentifierReference(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::MemberExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ThisExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
        }
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

impl<'a> GetParent<'a> for JSXExpressionContainer<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for JSXExpressionContainer<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_jsx_expression(&self.expression));
        children
    }
}

impl<'a> GetParent<'a> for JSXExpression<'a> {
    fn get_parent(&self) -> Option<AstKind<'a>> {
        match self {
            Self::EmptyExpression(it) => GetParent::get_parent(it),
            Self::BooleanLiteral(it) => GetParent::get_parent(it.as_ref()),
            Self::NullLiteral(it) => GetParent::get_parent(it.as_ref()),
            Self::NumericLiteral(it) => GetParent::get_parent(it.as_ref()),
            Self::BigIntLiteral(it) => GetParent::get_parent(it.as_ref()),
            Self::RegExpLiteral(it) => GetParent::get_parent(it.as_ref()),
            Self::StringLiteral(it) => GetParent::get_parent(it.as_ref()),
            Self::TemplateLiteral(it) => GetParent::get_parent(it.as_ref()),
            Self::Identifier(it) => GetParent::get_parent(it.as_ref()),
            Self::MetaProperty(it) => GetParent::get_parent(it.as_ref()),
            Self::Super(it) => GetParent::get_parent(it.as_ref()),
            Self::ArrayExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ArrowFunctionExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::AssignmentExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::AwaitExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::BinaryExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::CallExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ChainExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ClassExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ConditionalExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::FunctionExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ImportExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::LogicalExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::NewExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ObjectExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ParenthesizedExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::SequenceExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::TaggedTemplateExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ThisExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::UnaryExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::UpdateExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::YieldExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::PrivateInExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::JSXElement(it) => GetParent::get_parent(it.as_ref()),
            Self::JSXFragment(it) => GetParent::get_parent(it.as_ref()),
            Self::TSAsExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::TSSatisfiesExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::TSTypeAssertion(it) => GetParent::get_parent(it.as_ref()),
            Self::TSNonNullExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::TSInstantiationExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::ComputedMemberExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::StaticMemberExpression(it) => GetParent::get_parent(it.as_ref()),
            Self::PrivateFieldExpression(it) => GetParent::get_parent(it.as_ref()),
        }
    }
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        match self {
            Self::EmptyExpression(it) => GetParent::set_parent(it, new_parent),
            Self::BooleanLiteral(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::NullLiteral(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::NumericLiteral(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::BigIntLiteral(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::RegExpLiteral(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::StringLiteral(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TemplateLiteral(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::Identifier(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::MetaProperty(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::Super(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ArrayExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ArrowFunctionExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::AssignmentExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::AwaitExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::BinaryExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::CallExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ChainExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ClassExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ConditionalExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::FunctionExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ImportExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::LogicalExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::NewExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ObjectExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ParenthesizedExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::SequenceExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TaggedTemplateExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ThisExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::UnaryExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::UpdateExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::YieldExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::PrivateInExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::JSXElement(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::JSXFragment(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSAsExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSSatisfiesExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSTypeAssertion(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSNonNullExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::TSInstantiationExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ComputedMemberExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::StaticMemberExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::PrivateFieldExpression(it) => GetParent::set_parent(it.as_mut(), new_parent),
        }
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

impl<'a> GetParent<'a> for JSXEmptyExpression<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for JSXEmptyExpression<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetParent<'a> for JSXAttributeItem<'a> {
    fn get_parent(&self) -> Option<AstKind<'a>> {
        match self {
            Self::Attribute(it) => GetParent::get_parent(it.as_ref()),
            Self::SpreadAttribute(it) => GetParent::get_parent(it.as_ref()),
        }
    }
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        match self {
            Self::Attribute(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::SpreadAttribute(it) => GetParent::set_parent(it.as_mut(), new_parent),
        }
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

impl<'a> GetParent<'a> for JSXAttribute<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
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

impl<'a> GetParent<'a> for JSXSpreadAttribute<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for JSXSpreadAttribute<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_expression(&self.argument));
        children
    }
}

impl<'a> GetParent<'a> for JSXAttributeName<'a> {
    fn get_parent(&self) -> Option<AstKind<'a>> {
        match self {
            Self::Identifier(it) => GetParent::get_parent(it.as_ref()),
            Self::NamespacedName(it) => GetParent::get_parent(it.as_ref()),
        }
    }
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        match self {
            Self::Identifier(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::NamespacedName(it) => GetParent::set_parent(it.as_mut(), new_parent),
        }
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

impl<'a> GetParent<'a> for JSXAttributeValue<'a> {
    fn get_parent(&self) -> Option<AstKind<'a>> {
        match self {
            Self::StringLiteral(it) => GetParent::get_parent(it.as_ref()),
            Self::ExpressionContainer(it) => GetParent::get_parent(it.as_ref()),
            Self::Element(it) => GetParent::get_parent(it.as_ref()),
            Self::Fragment(it) => GetParent::get_parent(it.as_ref()),
        }
    }
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        match self {
            Self::StringLiteral(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ExpressionContainer(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::Element(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::Fragment(it) => GetParent::set_parent(it.as_mut(), new_parent),
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

impl<'a> GetParent<'a> for JSXIdentifier<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for JSXIdentifier<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}

impl<'a> GetParent<'a> for JSXChild<'a> {
    fn get_parent(&self) -> Option<AstKind<'a>> {
        match self {
            Self::Text(it) => GetParent::get_parent(it.as_ref()),
            Self::Element(it) => GetParent::get_parent(it.as_ref()),
            Self::Fragment(it) => GetParent::get_parent(it.as_ref()),
            Self::ExpressionContainer(it) => GetParent::get_parent(it.as_ref()),
            Self::Spread(it) => GetParent::get_parent(it.as_ref()),
        }
    }
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        match self {
            Self::Text(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::Element(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::Fragment(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::ExpressionContainer(it) => GetParent::set_parent(it.as_mut(), new_parent),
            Self::Spread(it) => GetParent::set_parent(it.as_mut(), new_parent),
        }
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

impl<'a> GetParent<'a> for JSXSpreadChild<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for JSXSpreadChild<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        let mut children = Vec::new();
        children.push(AstKind::from_expression(&self.expression));
        children
    }
}

impl<'a> GetParent<'a> for JSXText<'a> {
    #[inline]
    fn get_parent(&self) -> Option<AstKind<'a>> {
        self.parent
    }
    #[inline]
    fn set_parent(&mut self, new_parent: AstKind<'a>) {
        self.parent = Some(new_parent);
    }
}
impl<'a> GetChildren<'a> for JSXText<'a> {
    fn get_children(&'a self) -> Vec<AstKind<'a>> {
        vec![]
    }
}
