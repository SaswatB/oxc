// Auto-generated code, DO NOT EDIT DIRECTLY!
// To edit this generated file you have to edit `tasks/ast_tools/src/derives/get_parent.rs`

#![allow(clippy::match_same_arms)]

use crate::AstKind;
use crate::GetParent;

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
