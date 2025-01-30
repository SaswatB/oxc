// Auto-generated code, DO NOT EDIT DIRECTLY!
// To edit this generated file you have to edit `tasks/ast_tools/src/generators/ast_kind.rs`

#![allow(missing_docs)]
// FIXME (in ast_tools/src/generators/ast_kind.rs)

use std::ptr;

use oxc_span::{GetSpan, Span};

use crate::ast::*;
use crate::GetParent;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AstType {
    BooleanLiteral = 0,
    NullLiteral = 1,
    NumericLiteral = 2,
    StringLiteral = 3,
    BigIntLiteral = 4,
    RegExpLiteral = 5,
    Program = 6,
    IdentifierName = 7,
    IdentifierReference = 8,
    BindingIdentifier = 9,
    LabelIdentifier = 10,
    ThisExpression = 11,
    ArrayExpression = 12,
    ArrayExpressionElement = 13,
    Elision = 14,
    ObjectExpression = 15,
    ObjectProperty = 16,
    PropertyKey = 17,
    TemplateLiteral = 18,
    TaggedTemplateExpression = 19,
    MemberExpression = 20,
    CallExpression = 21,
    NewExpression = 22,
    MetaProperty = 23,
    SpreadElement = 24,
    Argument = 25,
    UpdateExpression = 26,
    UnaryExpression = 27,
    BinaryExpression = 28,
    PrivateInExpression = 29,
    LogicalExpression = 30,
    ConditionalExpression = 31,
    AssignmentExpression = 32,
    AssignmentTarget = 33,
    SimpleAssignmentTarget = 34,
    AssignmentTargetPattern = 35,
    ArrayAssignmentTarget = 36,
    ObjectAssignmentTarget = 37,
    AssignmentTargetWithDefault = 38,
    SequenceExpression = 39,
    Super = 40,
    AwaitExpression = 41,
    ChainExpression = 42,
    ParenthesizedExpression = 43,
    Directive = 44,
    Hashbang = 45,
    BlockStatement = 46,
    VariableDeclaration = 47,
    VariableDeclarator = 48,
    EmptyStatement = 49,
    ExpressionStatement = 50,
    IfStatement = 51,
    DoWhileStatement = 52,
    WhileStatement = 53,
    ForStatement = 54,
    ForStatementInit = 55,
    ForInStatement = 56,
    ForOfStatement = 57,
    ContinueStatement = 58,
    BreakStatement = 59,
    ReturnStatement = 60,
    WithStatement = 61,
    SwitchStatement = 62,
    SwitchCase = 63,
    LabeledStatement = 64,
    ThrowStatement = 65,
    TryStatement = 66,
    CatchClause = 67,
    CatchParameter = 68,
    DebuggerStatement = 69,
    AssignmentPattern = 70,
    ObjectPattern = 71,
    ArrayPattern = 72,
    BindingRestElement = 73,
    Function = 74,
    FormalParameters = 75,
    FormalParameter = 76,
    FunctionBody = 77,
    ArrowFunctionExpression = 78,
    YieldExpression = 79,
    Class = 80,
    ClassBody = 81,
    MethodDefinition = 82,
    PropertyDefinition = 83,
    PrivateIdentifier = 84,
    StaticBlock = 85,
    ModuleDeclaration = 86,
    ImportExpression = 87,
    ImportDeclaration = 88,
    ImportSpecifier = 89,
    ImportDefaultSpecifier = 90,
    ImportNamespaceSpecifier = 91,
    ExportNamedDeclaration = 92,
    ExportDefaultDeclaration = 93,
    ExportAllDeclaration = 94,
    ExportSpecifier = 95,
    TSThisParameter = 96,
    TSEnumDeclaration = 97,
    TSEnumMember = 98,
    TSTypeAnnotation = 99,
    TSLiteralType = 100,
    TSConditionalType = 101,
    TSUnionType = 102,
    TSIntersectionType = 103,
    TSParenthesizedType = 104,
    TSIndexedAccessType = 105,
    TSNamedTupleMember = 106,
    TSAnyKeyword = 107,
    TSStringKeyword = 108,
    TSBooleanKeyword = 109,
    TSNumberKeyword = 110,
    TSNeverKeyword = 111,
    TSIntrinsicKeyword = 112,
    TSUnknownKeyword = 113,
    TSNullKeyword = 114,
    TSUndefinedKeyword = 115,
    TSVoidKeyword = 116,
    TSSymbolKeyword = 117,
    TSThisType = 118,
    TSObjectKeyword = 119,
    TSBigIntKeyword = 120,
    TSTypeReference = 121,
    TSTypeName = 122,
    TSQualifiedName = 123,
    TSTypeParameterInstantiation = 124,
    TSTypeParameter = 125,
    TSTypeParameterDeclaration = 126,
    TSTypeAliasDeclaration = 127,
    TSClassImplements = 128,
    TSInterfaceDeclaration = 129,
    TSPropertySignature = 130,
    TSMethodSignature = 131,
    TSConstructSignatureDeclaration = 132,
    TSInterfaceHeritage = 133,
    TSModuleDeclaration = 134,
    TSModuleBlock = 135,
    TSTypeLiteral = 136,
    TSInferType = 137,
    TSTypeQuery = 138,
    TSImportType = 139,
    TSMappedType = 140,
    TSTemplateLiteralType = 141,
    TSAsExpression = 142,
    TSSatisfiesExpression = 143,
    TSTypeAssertion = 144,
    TSImportEqualsDeclaration = 145,
    TSModuleReference = 146,
    TSExternalModuleReference = 147,
    TSNonNullExpression = 148,
    Decorator = 149,
    TSExportAssignment = 150,
    TSInstantiationExpression = 151,
    JSXElement = 152,
    JSXOpeningElement = 153,
    JSXClosingElement = 154,
    JSXFragment = 155,
    JSXElementName = 156,
    JSXNamespacedName = 157,
    JSXMemberExpression = 158,
    JSXMemberExpressionObject = 159,
    JSXExpressionContainer = 160,
    JSXAttributeItem = 161,
    JSXSpreadAttribute = 162,
    JSXIdentifier = 163,
    JSXText = 164,
}

/// Untyped AST Node Kind
#[derive(Debug, Clone, Copy)]
#[repr(C, u8)]
pub enum AstKind<'a> {
    BooleanLiteral(&'a BooleanLiteral<'a>) = AstType::BooleanLiteral as u8,
    NullLiteral(&'a NullLiteral<'a>) = AstType::NullLiteral as u8,
    NumericLiteral(&'a NumericLiteral<'a>) = AstType::NumericLiteral as u8,
    StringLiteral(&'a StringLiteral<'a>) = AstType::StringLiteral as u8,
    BigIntLiteral(&'a BigIntLiteral<'a>) = AstType::BigIntLiteral as u8,
    RegExpLiteral(&'a RegExpLiteral<'a>) = AstType::RegExpLiteral as u8,
    Program(&'a Program<'a>) = AstType::Program as u8,
    IdentifierName(&'a IdentifierName<'a>) = AstType::IdentifierName as u8,
    IdentifierReference(&'a IdentifierReference<'a>) = AstType::IdentifierReference as u8,
    BindingIdentifier(&'a BindingIdentifier<'a>) = AstType::BindingIdentifier as u8,
    LabelIdentifier(&'a LabelIdentifier<'a>) = AstType::LabelIdentifier as u8,
    ThisExpression(&'a ThisExpression<'a>) = AstType::ThisExpression as u8,
    ArrayExpression(&'a ArrayExpression<'a>) = AstType::ArrayExpression as u8,
    ArrayExpressionElement(&'a ArrayExpressionElement<'a>) = AstType::ArrayExpressionElement as u8,
    Elision(&'a Elision<'a>) = AstType::Elision as u8,
    ObjectExpression(&'a ObjectExpression<'a>) = AstType::ObjectExpression as u8,
    ObjectProperty(&'a ObjectProperty<'a>) = AstType::ObjectProperty as u8,
    PropertyKey(&'a PropertyKey<'a>) = AstType::PropertyKey as u8,
    TemplateLiteral(&'a TemplateLiteral<'a>) = AstType::TemplateLiteral as u8,
    TaggedTemplateExpression(&'a TaggedTemplateExpression<'a>) =
        AstType::TaggedTemplateExpression as u8,
    MemberExpression(&'a MemberExpression<'a>) = AstType::MemberExpression as u8,
    CallExpression(&'a CallExpression<'a>) = AstType::CallExpression as u8,
    NewExpression(&'a NewExpression<'a>) = AstType::NewExpression as u8,
    MetaProperty(&'a MetaProperty<'a>) = AstType::MetaProperty as u8,
    SpreadElement(&'a SpreadElement<'a>) = AstType::SpreadElement as u8,
    Argument(&'a Argument<'a>) = AstType::Argument as u8,
    UpdateExpression(&'a UpdateExpression<'a>) = AstType::UpdateExpression as u8,
    UnaryExpression(&'a UnaryExpression<'a>) = AstType::UnaryExpression as u8,
    BinaryExpression(&'a BinaryExpression<'a>) = AstType::BinaryExpression as u8,
    PrivateInExpression(&'a PrivateInExpression<'a>) = AstType::PrivateInExpression as u8,
    LogicalExpression(&'a LogicalExpression<'a>) = AstType::LogicalExpression as u8,
    ConditionalExpression(&'a ConditionalExpression<'a>) = AstType::ConditionalExpression as u8,
    AssignmentExpression(&'a AssignmentExpression<'a>) = AstType::AssignmentExpression as u8,
    AssignmentTarget(&'a AssignmentTarget<'a>) = AstType::AssignmentTarget as u8,
    SimpleAssignmentTarget(&'a SimpleAssignmentTarget<'a>) = AstType::SimpleAssignmentTarget as u8,
    AssignmentTargetPattern(&'a AssignmentTargetPattern<'a>) =
        AstType::AssignmentTargetPattern as u8,
    ArrayAssignmentTarget(&'a ArrayAssignmentTarget<'a>) = AstType::ArrayAssignmentTarget as u8,
    ObjectAssignmentTarget(&'a ObjectAssignmentTarget<'a>) = AstType::ObjectAssignmentTarget as u8,
    AssignmentTargetWithDefault(&'a AssignmentTargetWithDefault<'a>) =
        AstType::AssignmentTargetWithDefault as u8,
    SequenceExpression(&'a SequenceExpression<'a>) = AstType::SequenceExpression as u8,
    Super(&'a Super<'a>) = AstType::Super as u8,
    AwaitExpression(&'a AwaitExpression<'a>) = AstType::AwaitExpression as u8,
    ChainExpression(&'a ChainExpression<'a>) = AstType::ChainExpression as u8,
    ParenthesizedExpression(&'a ParenthesizedExpression<'a>) =
        AstType::ParenthesizedExpression as u8,
    Directive(&'a Directive<'a>) = AstType::Directive as u8,
    Hashbang(&'a Hashbang<'a>) = AstType::Hashbang as u8,
    BlockStatement(&'a BlockStatement<'a>) = AstType::BlockStatement as u8,
    VariableDeclaration(&'a VariableDeclaration<'a>) = AstType::VariableDeclaration as u8,
    VariableDeclarator(&'a VariableDeclarator<'a>) = AstType::VariableDeclarator as u8,
    EmptyStatement(&'a EmptyStatement<'a>) = AstType::EmptyStatement as u8,
    ExpressionStatement(&'a ExpressionStatement<'a>) = AstType::ExpressionStatement as u8,
    IfStatement(&'a IfStatement<'a>) = AstType::IfStatement as u8,
    DoWhileStatement(&'a DoWhileStatement<'a>) = AstType::DoWhileStatement as u8,
    WhileStatement(&'a WhileStatement<'a>) = AstType::WhileStatement as u8,
    ForStatement(&'a ForStatement<'a>) = AstType::ForStatement as u8,
    ForStatementInit(&'a ForStatementInit<'a>) = AstType::ForStatementInit as u8,
    ForInStatement(&'a ForInStatement<'a>) = AstType::ForInStatement as u8,
    ForOfStatement(&'a ForOfStatement<'a>) = AstType::ForOfStatement as u8,
    ContinueStatement(&'a ContinueStatement<'a>) = AstType::ContinueStatement as u8,
    BreakStatement(&'a BreakStatement<'a>) = AstType::BreakStatement as u8,
    ReturnStatement(&'a ReturnStatement<'a>) = AstType::ReturnStatement as u8,
    WithStatement(&'a WithStatement<'a>) = AstType::WithStatement as u8,
    SwitchStatement(&'a SwitchStatement<'a>) = AstType::SwitchStatement as u8,
    SwitchCase(&'a SwitchCase<'a>) = AstType::SwitchCase as u8,
    LabeledStatement(&'a LabeledStatement<'a>) = AstType::LabeledStatement as u8,
    ThrowStatement(&'a ThrowStatement<'a>) = AstType::ThrowStatement as u8,
    TryStatement(&'a TryStatement<'a>) = AstType::TryStatement as u8,
    CatchClause(&'a CatchClause<'a>) = AstType::CatchClause as u8,
    CatchParameter(&'a CatchParameter<'a>) = AstType::CatchParameter as u8,
    DebuggerStatement(&'a DebuggerStatement<'a>) = AstType::DebuggerStatement as u8,
    AssignmentPattern(&'a AssignmentPattern<'a>) = AstType::AssignmentPattern as u8,
    ObjectPattern(&'a ObjectPattern<'a>) = AstType::ObjectPattern as u8,
    ArrayPattern(&'a ArrayPattern<'a>) = AstType::ArrayPattern as u8,
    BindingRestElement(&'a BindingRestElement<'a>) = AstType::BindingRestElement as u8,
    Function(&'a Function<'a>) = AstType::Function as u8,
    FormalParameters(&'a FormalParameters<'a>) = AstType::FormalParameters as u8,
    FormalParameter(&'a FormalParameter<'a>) = AstType::FormalParameter as u8,
    FunctionBody(&'a FunctionBody<'a>) = AstType::FunctionBody as u8,
    ArrowFunctionExpression(&'a ArrowFunctionExpression<'a>) =
        AstType::ArrowFunctionExpression as u8,
    YieldExpression(&'a YieldExpression<'a>) = AstType::YieldExpression as u8,
    Class(&'a Class<'a>) = AstType::Class as u8,
    ClassBody(&'a ClassBody<'a>) = AstType::ClassBody as u8,
    MethodDefinition(&'a MethodDefinition<'a>) = AstType::MethodDefinition as u8,
    PropertyDefinition(&'a PropertyDefinition<'a>) = AstType::PropertyDefinition as u8,
    PrivateIdentifier(&'a PrivateIdentifier<'a>) = AstType::PrivateIdentifier as u8,
    StaticBlock(&'a StaticBlock<'a>) = AstType::StaticBlock as u8,
    ModuleDeclaration(&'a ModuleDeclaration<'a>) = AstType::ModuleDeclaration as u8,
    ImportExpression(&'a ImportExpression<'a>) = AstType::ImportExpression as u8,
    ImportDeclaration(&'a ImportDeclaration<'a>) = AstType::ImportDeclaration as u8,
    ImportSpecifier(&'a ImportSpecifier<'a>) = AstType::ImportSpecifier as u8,
    ImportDefaultSpecifier(&'a ImportDefaultSpecifier<'a>) = AstType::ImportDefaultSpecifier as u8,
    ImportNamespaceSpecifier(&'a ImportNamespaceSpecifier<'a>) =
        AstType::ImportNamespaceSpecifier as u8,
    ExportNamedDeclaration(&'a ExportNamedDeclaration<'a>) = AstType::ExportNamedDeclaration as u8,
    ExportDefaultDeclaration(&'a ExportDefaultDeclaration<'a>) =
        AstType::ExportDefaultDeclaration as u8,
    ExportAllDeclaration(&'a ExportAllDeclaration<'a>) = AstType::ExportAllDeclaration as u8,
    ExportSpecifier(&'a ExportSpecifier<'a>) = AstType::ExportSpecifier as u8,
    TSThisParameter(&'a TSThisParameter<'a>) = AstType::TSThisParameter as u8,
    TSEnumDeclaration(&'a TSEnumDeclaration<'a>) = AstType::TSEnumDeclaration as u8,
    TSEnumMember(&'a TSEnumMember<'a>) = AstType::TSEnumMember as u8,
    TSTypeAnnotation(&'a TSTypeAnnotation<'a>) = AstType::TSTypeAnnotation as u8,
    TSLiteralType(&'a TSLiteralType<'a>) = AstType::TSLiteralType as u8,
    TSConditionalType(&'a TSConditionalType<'a>) = AstType::TSConditionalType as u8,
    TSUnionType(&'a TSUnionType<'a>) = AstType::TSUnionType as u8,
    TSIntersectionType(&'a TSIntersectionType<'a>) = AstType::TSIntersectionType as u8,
    TSParenthesizedType(&'a TSParenthesizedType<'a>) = AstType::TSParenthesizedType as u8,
    TSIndexedAccessType(&'a TSIndexedAccessType<'a>) = AstType::TSIndexedAccessType as u8,
    TSNamedTupleMember(&'a TSNamedTupleMember<'a>) = AstType::TSNamedTupleMember as u8,
    TSAnyKeyword(&'a TSAnyKeyword<'a>) = AstType::TSAnyKeyword as u8,
    TSStringKeyword(&'a TSStringKeyword<'a>) = AstType::TSStringKeyword as u8,
    TSBooleanKeyword(&'a TSBooleanKeyword<'a>) = AstType::TSBooleanKeyword as u8,
    TSNumberKeyword(&'a TSNumberKeyword<'a>) = AstType::TSNumberKeyword as u8,
    TSNeverKeyword(&'a TSNeverKeyword<'a>) = AstType::TSNeverKeyword as u8,
    TSIntrinsicKeyword(&'a TSIntrinsicKeyword<'a>) = AstType::TSIntrinsicKeyword as u8,
    TSUnknownKeyword(&'a TSUnknownKeyword<'a>) = AstType::TSUnknownKeyword as u8,
    TSNullKeyword(&'a TSNullKeyword<'a>) = AstType::TSNullKeyword as u8,
    TSUndefinedKeyword(&'a TSUndefinedKeyword<'a>) = AstType::TSUndefinedKeyword as u8,
    TSVoidKeyword(&'a TSVoidKeyword<'a>) = AstType::TSVoidKeyword as u8,
    TSSymbolKeyword(&'a TSSymbolKeyword<'a>) = AstType::TSSymbolKeyword as u8,
    TSThisType(&'a TSThisType<'a>) = AstType::TSThisType as u8,
    TSObjectKeyword(&'a TSObjectKeyword<'a>) = AstType::TSObjectKeyword as u8,
    TSBigIntKeyword(&'a TSBigIntKeyword<'a>) = AstType::TSBigIntKeyword as u8,
    TSTypeReference(&'a TSTypeReference<'a>) = AstType::TSTypeReference as u8,
    TSTypeName(&'a TSTypeName<'a>) = AstType::TSTypeName as u8,
    TSQualifiedName(&'a TSQualifiedName<'a>) = AstType::TSQualifiedName as u8,
    TSTypeParameterInstantiation(&'a TSTypeParameterInstantiation<'a>) =
        AstType::TSTypeParameterInstantiation as u8,
    TSTypeParameter(&'a TSTypeParameter<'a>) = AstType::TSTypeParameter as u8,
    TSTypeParameterDeclaration(&'a TSTypeParameterDeclaration<'a>) =
        AstType::TSTypeParameterDeclaration as u8,
    TSTypeAliasDeclaration(&'a TSTypeAliasDeclaration<'a>) = AstType::TSTypeAliasDeclaration as u8,
    TSClassImplements(&'a TSClassImplements<'a>) = AstType::TSClassImplements as u8,
    TSInterfaceDeclaration(&'a TSInterfaceDeclaration<'a>) = AstType::TSInterfaceDeclaration as u8,
    TSPropertySignature(&'a TSPropertySignature<'a>) = AstType::TSPropertySignature as u8,
    TSMethodSignature(&'a TSMethodSignature<'a>) = AstType::TSMethodSignature as u8,
    TSConstructSignatureDeclaration(&'a TSConstructSignatureDeclaration<'a>) =
        AstType::TSConstructSignatureDeclaration as u8,
    TSInterfaceHeritage(&'a TSInterfaceHeritage<'a>) = AstType::TSInterfaceHeritage as u8,
    TSModuleDeclaration(&'a TSModuleDeclaration<'a>) = AstType::TSModuleDeclaration as u8,
    TSModuleBlock(&'a TSModuleBlock<'a>) = AstType::TSModuleBlock as u8,
    TSTypeLiteral(&'a TSTypeLiteral<'a>) = AstType::TSTypeLiteral as u8,
    TSInferType(&'a TSInferType<'a>) = AstType::TSInferType as u8,
    TSTypeQuery(&'a TSTypeQuery<'a>) = AstType::TSTypeQuery as u8,
    TSImportType(&'a TSImportType<'a>) = AstType::TSImportType as u8,
    TSMappedType(&'a TSMappedType<'a>) = AstType::TSMappedType as u8,
    TSTemplateLiteralType(&'a TSTemplateLiteralType<'a>) = AstType::TSTemplateLiteralType as u8,
    TSAsExpression(&'a TSAsExpression<'a>) = AstType::TSAsExpression as u8,
    TSSatisfiesExpression(&'a TSSatisfiesExpression<'a>) = AstType::TSSatisfiesExpression as u8,
    TSTypeAssertion(&'a TSTypeAssertion<'a>) = AstType::TSTypeAssertion as u8,
    TSImportEqualsDeclaration(&'a TSImportEqualsDeclaration<'a>) =
        AstType::TSImportEqualsDeclaration as u8,
    TSModuleReference(&'a TSModuleReference<'a>) = AstType::TSModuleReference as u8,
    TSExternalModuleReference(&'a TSExternalModuleReference<'a>) =
        AstType::TSExternalModuleReference as u8,
    TSNonNullExpression(&'a TSNonNullExpression<'a>) = AstType::TSNonNullExpression as u8,
    Decorator(&'a Decorator<'a>) = AstType::Decorator as u8,
    TSExportAssignment(&'a TSExportAssignment<'a>) = AstType::TSExportAssignment as u8,
    TSInstantiationExpression(&'a TSInstantiationExpression<'a>) =
        AstType::TSInstantiationExpression as u8,
    JSXElement(&'a JSXElement<'a>) = AstType::JSXElement as u8,
    JSXOpeningElement(&'a JSXOpeningElement<'a>) = AstType::JSXOpeningElement as u8,
    JSXClosingElement(&'a JSXClosingElement<'a>) = AstType::JSXClosingElement as u8,
    JSXFragment(&'a JSXFragment<'a>) = AstType::JSXFragment as u8,
    JSXElementName(&'a JSXElementName<'a>) = AstType::JSXElementName as u8,
    JSXNamespacedName(&'a JSXNamespacedName<'a>) = AstType::JSXNamespacedName as u8,
    JSXMemberExpression(&'a JSXMemberExpression<'a>) = AstType::JSXMemberExpression as u8,
    JSXMemberExpressionObject(&'a JSXMemberExpressionObject<'a>) =
        AstType::JSXMemberExpressionObject as u8,
    JSXExpressionContainer(&'a JSXExpressionContainer<'a>) = AstType::JSXExpressionContainer as u8,
    JSXAttributeItem(&'a JSXAttributeItem<'a>) = AstType::JSXAttributeItem as u8,
    JSXSpreadAttribute(&'a JSXSpreadAttribute<'a>) = AstType::JSXSpreadAttribute as u8,
    JSXIdentifier(&'a JSXIdentifier<'a>) = AstType::JSXIdentifier as u8,
    JSXText(&'a JSXText<'a>) = AstType::JSXText as u8,
}

impl AstKind<'_> {
    /// Get the [`AstType`] of an [`AstKind`].
    #[inline]
    pub fn ty(&self) -> AstType {
        // SAFETY: `AstKind` is `#[repr(C, u8)]`, so discriminant is stored in first byte,
        // and it's valid to read it.
        // `AstType` is also `#[repr(u8)]` and `AstKind` and `AstType` both have the same
        // discriminants, so it's valid to read `AstKind`'s discriminant as `AstType`.
        unsafe { *ptr::from_ref(self).cast::<AstType>().as_ref().unwrap_unchecked() }
    }
}

impl GetSpan for AstKind<'_> {
    #[allow(clippy::match_same_arms)]
    fn span(&self) -> Span {
        match self {
            Self::BooleanLiteral(it) => it.span(),
            Self::NullLiteral(it) => it.span(),
            Self::NumericLiteral(it) => it.span(),
            Self::StringLiteral(it) => it.span(),
            Self::BigIntLiteral(it) => it.span(),
            Self::RegExpLiteral(it) => it.span(),
            Self::Program(it) => it.span(),
            Self::IdentifierName(it) => it.span(),
            Self::IdentifierReference(it) => it.span(),
            Self::BindingIdentifier(it) => it.span(),
            Self::LabelIdentifier(it) => it.span(),
            Self::ThisExpression(it) => it.span(),
            Self::ArrayExpression(it) => it.span(),
            Self::ArrayExpressionElement(it) => it.span(),
            Self::Elision(it) => it.span(),
            Self::ObjectExpression(it) => it.span(),
            Self::ObjectProperty(it) => it.span(),
            Self::PropertyKey(it) => it.span(),
            Self::TemplateLiteral(it) => it.span(),
            Self::TaggedTemplateExpression(it) => it.span(),
            Self::MemberExpression(it) => it.span(),
            Self::CallExpression(it) => it.span(),
            Self::NewExpression(it) => it.span(),
            Self::MetaProperty(it) => it.span(),
            Self::SpreadElement(it) => it.span(),
            Self::Argument(it) => it.span(),
            Self::UpdateExpression(it) => it.span(),
            Self::UnaryExpression(it) => it.span(),
            Self::BinaryExpression(it) => it.span(),
            Self::PrivateInExpression(it) => it.span(),
            Self::LogicalExpression(it) => it.span(),
            Self::ConditionalExpression(it) => it.span(),
            Self::AssignmentExpression(it) => it.span(),
            Self::AssignmentTarget(it) => it.span(),
            Self::SimpleAssignmentTarget(it) => it.span(),
            Self::AssignmentTargetPattern(it) => it.span(),
            Self::ArrayAssignmentTarget(it) => it.span(),
            Self::ObjectAssignmentTarget(it) => it.span(),
            Self::AssignmentTargetWithDefault(it) => it.span(),
            Self::SequenceExpression(it) => it.span(),
            Self::Super(it) => it.span(),
            Self::AwaitExpression(it) => it.span(),
            Self::ChainExpression(it) => it.span(),
            Self::ParenthesizedExpression(it) => it.span(),
            Self::Directive(it) => it.span(),
            Self::Hashbang(it) => it.span(),
            Self::BlockStatement(it) => it.span(),
            Self::VariableDeclaration(it) => it.span(),
            Self::VariableDeclarator(it) => it.span(),
            Self::EmptyStatement(it) => it.span(),
            Self::ExpressionStatement(it) => it.span(),
            Self::IfStatement(it) => it.span(),
            Self::DoWhileStatement(it) => it.span(),
            Self::WhileStatement(it) => it.span(),
            Self::ForStatement(it) => it.span(),
            Self::ForStatementInit(it) => it.span(),
            Self::ForInStatement(it) => it.span(),
            Self::ForOfStatement(it) => it.span(),
            Self::ContinueStatement(it) => it.span(),
            Self::BreakStatement(it) => it.span(),
            Self::ReturnStatement(it) => it.span(),
            Self::WithStatement(it) => it.span(),
            Self::SwitchStatement(it) => it.span(),
            Self::SwitchCase(it) => it.span(),
            Self::LabeledStatement(it) => it.span(),
            Self::ThrowStatement(it) => it.span(),
            Self::TryStatement(it) => it.span(),
            Self::CatchClause(it) => it.span(),
            Self::CatchParameter(it) => it.span(),
            Self::DebuggerStatement(it) => it.span(),
            Self::AssignmentPattern(it) => it.span(),
            Self::ObjectPattern(it) => it.span(),
            Self::ArrayPattern(it) => it.span(),
            Self::BindingRestElement(it) => it.span(),
            Self::Function(it) => it.span(),
            Self::FormalParameters(it) => it.span(),
            Self::FormalParameter(it) => it.span(),
            Self::FunctionBody(it) => it.span(),
            Self::ArrowFunctionExpression(it) => it.span(),
            Self::YieldExpression(it) => it.span(),
            Self::Class(it) => it.span(),
            Self::ClassBody(it) => it.span(),
            Self::MethodDefinition(it) => it.span(),
            Self::PropertyDefinition(it) => it.span(),
            Self::PrivateIdentifier(it) => it.span(),
            Self::StaticBlock(it) => it.span(),
            Self::ModuleDeclaration(it) => it.span(),
            Self::ImportExpression(it) => it.span(),
            Self::ImportDeclaration(it) => it.span(),
            Self::ImportSpecifier(it) => it.span(),
            Self::ImportDefaultSpecifier(it) => it.span(),
            Self::ImportNamespaceSpecifier(it) => it.span(),
            Self::ExportNamedDeclaration(it) => it.span(),
            Self::ExportDefaultDeclaration(it) => it.span(),
            Self::ExportAllDeclaration(it) => it.span(),
            Self::ExportSpecifier(it) => it.span(),
            Self::TSThisParameter(it) => it.span(),
            Self::TSEnumDeclaration(it) => it.span(),
            Self::TSEnumMember(it) => it.span(),
            Self::TSTypeAnnotation(it) => it.span(),
            Self::TSLiteralType(it) => it.span(),
            Self::TSConditionalType(it) => it.span(),
            Self::TSUnionType(it) => it.span(),
            Self::TSIntersectionType(it) => it.span(),
            Self::TSParenthesizedType(it) => it.span(),
            Self::TSIndexedAccessType(it) => it.span(),
            Self::TSNamedTupleMember(it) => it.span(),
            Self::TSAnyKeyword(it) => it.span(),
            Self::TSStringKeyword(it) => it.span(),
            Self::TSBooleanKeyword(it) => it.span(),
            Self::TSNumberKeyword(it) => it.span(),
            Self::TSNeverKeyword(it) => it.span(),
            Self::TSIntrinsicKeyword(it) => it.span(),
            Self::TSUnknownKeyword(it) => it.span(),
            Self::TSNullKeyword(it) => it.span(),
            Self::TSUndefinedKeyword(it) => it.span(),
            Self::TSVoidKeyword(it) => it.span(),
            Self::TSSymbolKeyword(it) => it.span(),
            Self::TSThisType(it) => it.span(),
            Self::TSObjectKeyword(it) => it.span(),
            Self::TSBigIntKeyword(it) => it.span(),
            Self::TSTypeReference(it) => it.span(),
            Self::TSTypeName(it) => it.span(),
            Self::TSQualifiedName(it) => it.span(),
            Self::TSTypeParameterInstantiation(it) => it.span(),
            Self::TSTypeParameter(it) => it.span(),
            Self::TSTypeParameterDeclaration(it) => it.span(),
            Self::TSTypeAliasDeclaration(it) => it.span(),
            Self::TSClassImplements(it) => it.span(),
            Self::TSInterfaceDeclaration(it) => it.span(),
            Self::TSPropertySignature(it) => it.span(),
            Self::TSMethodSignature(it) => it.span(),
            Self::TSConstructSignatureDeclaration(it) => it.span(),
            Self::TSInterfaceHeritage(it) => it.span(),
            Self::TSModuleDeclaration(it) => it.span(),
            Self::TSModuleBlock(it) => it.span(),
            Self::TSTypeLiteral(it) => it.span(),
            Self::TSInferType(it) => it.span(),
            Self::TSTypeQuery(it) => it.span(),
            Self::TSImportType(it) => it.span(),
            Self::TSMappedType(it) => it.span(),
            Self::TSTemplateLiteralType(it) => it.span(),
            Self::TSAsExpression(it) => it.span(),
            Self::TSSatisfiesExpression(it) => it.span(),
            Self::TSTypeAssertion(it) => it.span(),
            Self::TSImportEqualsDeclaration(it) => it.span(),
            Self::TSModuleReference(it) => it.span(),
            Self::TSExternalModuleReference(it) => it.span(),
            Self::TSNonNullExpression(it) => it.span(),
            Self::Decorator(it) => it.span(),
            Self::TSExportAssignment(it) => it.span(),
            Self::TSInstantiationExpression(it) => it.span(),
            Self::JSXElement(it) => it.span(),
            Self::JSXOpeningElement(it) => it.span(),
            Self::JSXClosingElement(it) => it.span(),
            Self::JSXFragment(it) => it.span(),
            Self::JSXElementName(it) => it.span(),
            Self::JSXNamespacedName(it) => it.span(),
            Self::JSXMemberExpression(it) => it.span(),
            Self::JSXMemberExpressionObject(it) => it.span(),
            Self::JSXExpressionContainer(it) => it.span(),
            Self::JSXAttributeItem(it) => it.span(),
            Self::JSXSpreadAttribute(it) => it.span(),
            Self::JSXIdentifier(it) => it.span(),
            Self::JSXText(it) => it.span(),
        }
    }
}

impl<'a> AstKind<'a> {
    pub fn get_parent(&self) -> Option<AstKind<'a>> {
        match self {
            Self::BooleanLiteral(it) => it.get_parent(),
            Self::NullLiteral(it) => it.get_parent(),
            Self::NumericLiteral(it) => it.get_parent(),
            Self::StringLiteral(it) => it.get_parent(),
            Self::BigIntLiteral(it) => it.get_parent(),
            Self::RegExpLiteral(it) => it.get_parent(),
            Self::Program(it) => it.get_parent(),
            Self::IdentifierName(it) => it.get_parent(),
            Self::IdentifierReference(it) => it.get_parent(),
            Self::BindingIdentifier(it) => it.get_parent(),
            Self::LabelIdentifier(it) => it.get_parent(),
            Self::ThisExpression(it) => it.get_parent(),
            Self::ArrayExpression(it) => it.get_parent(),
            Self::ArrayExpressionElement(it) => it.get_parent(),
            Self::Elision(it) => it.get_parent(),
            Self::ObjectExpression(it) => it.get_parent(),
            Self::ObjectProperty(it) => it.get_parent(),
            Self::PropertyKey(it) => it.get_parent(),
            Self::TemplateLiteral(it) => it.get_parent(),
            Self::TaggedTemplateExpression(it) => it.get_parent(),
            Self::MemberExpression(it) => it.get_parent(),
            Self::CallExpression(it) => it.get_parent(),
            Self::NewExpression(it) => it.get_parent(),
            Self::MetaProperty(it) => it.get_parent(),
            Self::SpreadElement(it) => it.get_parent(),
            Self::Argument(it) => it.get_parent(),
            Self::UpdateExpression(it) => it.get_parent(),
            Self::UnaryExpression(it) => it.get_parent(),
            Self::BinaryExpression(it) => it.get_parent(),
            Self::PrivateInExpression(it) => it.get_parent(),
            Self::LogicalExpression(it) => it.get_parent(),
            Self::ConditionalExpression(it) => it.get_parent(),
            Self::AssignmentExpression(it) => it.get_parent(),
            Self::AssignmentTarget(it) => it.get_parent(),
            Self::SimpleAssignmentTarget(it) => it.get_parent(),
            Self::AssignmentTargetPattern(it) => it.get_parent(),
            Self::ArrayAssignmentTarget(it) => it.get_parent(),
            Self::ObjectAssignmentTarget(it) => it.get_parent(),
            Self::AssignmentTargetWithDefault(it) => it.get_parent(),
            Self::SequenceExpression(it) => it.get_parent(),
            Self::Super(it) => it.get_parent(),
            Self::AwaitExpression(it) => it.get_parent(),
            Self::ChainExpression(it) => it.get_parent(),
            Self::ParenthesizedExpression(it) => it.get_parent(),
            Self::Directive(it) => it.get_parent(),
            Self::Hashbang(it) => it.get_parent(),
            Self::BlockStatement(it) => it.get_parent(),
            Self::VariableDeclaration(it) => it.get_parent(),
            Self::VariableDeclarator(it) => it.get_parent(),
            Self::EmptyStatement(it) => it.get_parent(),
            Self::ExpressionStatement(it) => it.get_parent(),
            Self::IfStatement(it) => it.get_parent(),
            Self::DoWhileStatement(it) => it.get_parent(),
            Self::WhileStatement(it) => it.get_parent(),
            Self::ForStatement(it) => it.get_parent(),
            Self::ForStatementInit(it) => it.get_parent(),
            Self::ForInStatement(it) => it.get_parent(),
            Self::ForOfStatement(it) => it.get_parent(),
            Self::ContinueStatement(it) => it.get_parent(),
            Self::BreakStatement(it) => it.get_parent(),
            Self::ReturnStatement(it) => it.get_parent(),
            Self::WithStatement(it) => it.get_parent(),
            Self::SwitchStatement(it) => it.get_parent(),
            Self::SwitchCase(it) => it.get_parent(),
            Self::LabeledStatement(it) => it.get_parent(),
            Self::ThrowStatement(it) => it.get_parent(),
            Self::TryStatement(it) => it.get_parent(),
            Self::CatchClause(it) => it.get_parent(),
            Self::CatchParameter(it) => it.get_parent(),
            Self::DebuggerStatement(it) => it.get_parent(),
            Self::AssignmentPattern(it) => it.get_parent(),
            Self::ObjectPattern(it) => it.get_parent(),
            Self::ArrayPattern(it) => it.get_parent(),
            Self::BindingRestElement(it) => it.get_parent(),
            Self::Function(it) => it.get_parent(),
            Self::FormalParameters(it) => it.get_parent(),
            Self::FormalParameter(it) => it.get_parent(),
            Self::FunctionBody(it) => it.get_parent(),
            Self::ArrowFunctionExpression(it) => it.get_parent(),
            Self::YieldExpression(it) => it.get_parent(),
            Self::Class(it) => it.get_parent(),
            Self::ClassBody(it) => it.get_parent(),
            Self::MethodDefinition(it) => it.get_parent(),
            Self::PropertyDefinition(it) => it.get_parent(),
            Self::PrivateIdentifier(it) => it.get_parent(),
            Self::StaticBlock(it) => it.get_parent(),
            Self::ModuleDeclaration(it) => it.get_parent(),
            Self::ImportExpression(it) => it.get_parent(),
            Self::ImportDeclaration(it) => it.get_parent(),
            Self::ImportSpecifier(it) => it.get_parent(),
            Self::ImportDefaultSpecifier(it) => it.get_parent(),
            Self::ImportNamespaceSpecifier(it) => it.get_parent(),
            Self::ExportNamedDeclaration(it) => it.get_parent(),
            Self::ExportDefaultDeclaration(it) => it.get_parent(),
            Self::ExportAllDeclaration(it) => it.get_parent(),
            Self::ExportSpecifier(it) => it.get_parent(),
            Self::TSThisParameter(it) => it.get_parent(),
            Self::TSEnumDeclaration(it) => it.get_parent(),
            Self::TSEnumMember(it) => it.get_parent(),
            Self::TSTypeAnnotation(it) => it.get_parent(),
            Self::TSLiteralType(it) => it.get_parent(),
            Self::TSConditionalType(it) => it.get_parent(),
            Self::TSUnionType(it) => it.get_parent(),
            Self::TSIntersectionType(it) => it.get_parent(),
            Self::TSParenthesizedType(it) => it.get_parent(),
            Self::TSIndexedAccessType(it) => it.get_parent(),
            Self::TSNamedTupleMember(it) => it.get_parent(),
            Self::TSAnyKeyword(it) => it.get_parent(),
            Self::TSStringKeyword(it) => it.get_parent(),
            Self::TSBooleanKeyword(it) => it.get_parent(),
            Self::TSNumberKeyword(it) => it.get_parent(),
            Self::TSNeverKeyword(it) => it.get_parent(),
            Self::TSIntrinsicKeyword(it) => it.get_parent(),
            Self::TSUnknownKeyword(it) => it.get_parent(),
            Self::TSNullKeyword(it) => it.get_parent(),
            Self::TSUndefinedKeyword(it) => it.get_parent(),
            Self::TSVoidKeyword(it) => it.get_parent(),
            Self::TSSymbolKeyword(it) => it.get_parent(),
            Self::TSThisType(it) => it.get_parent(),
            Self::TSObjectKeyword(it) => it.get_parent(),
            Self::TSBigIntKeyword(it) => it.get_parent(),
            Self::TSTypeReference(it) => it.get_parent(),
            Self::TSTypeName(it) => it.get_parent(),
            Self::TSQualifiedName(it) => it.get_parent(),
            Self::TSTypeParameterInstantiation(it) => it.get_parent(),
            Self::TSTypeParameter(it) => it.get_parent(),
            Self::TSTypeParameterDeclaration(it) => it.get_parent(),
            Self::TSTypeAliasDeclaration(it) => it.get_parent(),
            Self::TSClassImplements(it) => it.get_parent(),
            Self::TSInterfaceDeclaration(it) => it.get_parent(),
            Self::TSPropertySignature(it) => it.get_parent(),
            Self::TSMethodSignature(it) => it.get_parent(),
            Self::TSConstructSignatureDeclaration(it) => it.get_parent(),
            Self::TSInterfaceHeritage(it) => it.get_parent(),
            Self::TSModuleDeclaration(it) => it.get_parent(),
            Self::TSModuleBlock(it) => it.get_parent(),
            Self::TSTypeLiteral(it) => it.get_parent(),
            Self::TSInferType(it) => it.get_parent(),
            Self::TSTypeQuery(it) => it.get_parent(),
            Self::TSImportType(it) => it.get_parent(),
            Self::TSMappedType(it) => it.get_parent(),
            Self::TSTemplateLiteralType(it) => it.get_parent(),
            Self::TSAsExpression(it) => it.get_parent(),
            Self::TSSatisfiesExpression(it) => it.get_parent(),
            Self::TSTypeAssertion(it) => it.get_parent(),
            Self::TSImportEqualsDeclaration(it) => it.get_parent(),
            Self::TSModuleReference(it) => it.get_parent(),
            Self::TSExternalModuleReference(it) => it.get_parent(),
            Self::TSNonNullExpression(it) => it.get_parent(),
            Self::Decorator(it) => it.get_parent(),
            Self::TSExportAssignment(it) => it.get_parent(),
            Self::TSInstantiationExpression(it) => it.get_parent(),
            Self::JSXElement(it) => it.get_parent(),
            Self::JSXOpeningElement(it) => it.get_parent(),
            Self::JSXClosingElement(it) => it.get_parent(),
            Self::JSXFragment(it) => it.get_parent(),
            Self::JSXElementName(it) => it.get_parent(),
            Self::JSXNamespacedName(it) => it.get_parent(),
            Self::JSXMemberExpression(it) => it.get_parent(),
            Self::JSXMemberExpressionObject(it) => it.get_parent(),
            Self::JSXExpressionContainer(it) => it.get_parent(),
            Self::JSXAttributeItem(it) => it.get_parent(),
            Self::JSXSpreadAttribute(it) => it.get_parent(),
            Self::JSXIdentifier(it) => it.get_parent(),
            Self::JSXText(it) => it.get_parent(),
        }
    }
    pub fn set_parent(&mut self, new_parent: AstKind<'a>) {
        match self {
            Self::BooleanLiteral(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut BooleanLiteral) };
                it_mut.set_parent(new_parent)
            }
            Self::NullLiteral(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut NullLiteral) };
                it_mut.set_parent(new_parent)
            }
            Self::NumericLiteral(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut NumericLiteral) };
                it_mut.set_parent(new_parent)
            }
            Self::StringLiteral(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut StringLiteral) };
                it_mut.set_parent(new_parent)
            }
            Self::BigIntLiteral(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut BigIntLiteral) };
                it_mut.set_parent(new_parent)
            }
            Self::RegExpLiteral(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut RegExpLiteral) };
                it_mut.set_parent(new_parent)
            }
            Self::Program(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut Program) };
                it_mut.set_parent(new_parent)
            }
            Self::IdentifierName(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut IdentifierName) };
                it_mut.set_parent(new_parent)
            }
            Self::IdentifierReference(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut IdentifierReference) };
                it_mut.set_parent(new_parent)
            }
            Self::BindingIdentifier(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut BindingIdentifier) };
                it_mut.set_parent(new_parent)
            }
            Self::LabelIdentifier(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut LabelIdentifier) };
                it_mut.set_parent(new_parent)
            }
            Self::ThisExpression(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut ThisExpression) };
                it_mut.set_parent(new_parent)
            }
            Self::ArrayExpression(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut ArrayExpression) };
                it_mut.set_parent(new_parent)
            }
            Self::ArrayExpressionElement(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut ArrayExpressionElement) };
                it_mut.set_parent(new_parent)
            }
            Self::Elision(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut Elision) };
                it_mut.set_parent(new_parent)
            }
            Self::ObjectExpression(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut ObjectExpression) };
                it_mut.set_parent(new_parent)
            }
            Self::ObjectProperty(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut ObjectProperty) };
                it_mut.set_parent(new_parent)
            }
            Self::PropertyKey(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut PropertyKey) };
                it_mut.set_parent(new_parent)
            }
            Self::TemplateLiteral(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut TemplateLiteral) };
                it_mut.set_parent(new_parent)
            }
            Self::TaggedTemplateExpression(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut TaggedTemplateExpression) };
                it_mut.set_parent(new_parent)
            }
            Self::MemberExpression(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut MemberExpression) };
                it_mut.set_parent(new_parent)
            }
            Self::CallExpression(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut CallExpression) };
                it_mut.set_parent(new_parent)
            }
            Self::NewExpression(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut NewExpression) };
                it_mut.set_parent(new_parent)
            }
            Self::MetaProperty(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut MetaProperty) };
                it_mut.set_parent(new_parent)
            }
            Self::SpreadElement(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut SpreadElement) };
                it_mut.set_parent(new_parent)
            }
            Self::Argument(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut Argument) };
                it_mut.set_parent(new_parent)
            }
            Self::UpdateExpression(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut UpdateExpression) };
                it_mut.set_parent(new_parent)
            }
            Self::UnaryExpression(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut UnaryExpression) };
                it_mut.set_parent(new_parent)
            }
            Self::BinaryExpression(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut BinaryExpression) };
                it_mut.set_parent(new_parent)
            }
            Self::PrivateInExpression(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut PrivateInExpression) };
                it_mut.set_parent(new_parent)
            }
            Self::LogicalExpression(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut LogicalExpression) };
                it_mut.set_parent(new_parent)
            }
            Self::ConditionalExpression(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut ConditionalExpression) };
                it_mut.set_parent(new_parent)
            }
            Self::AssignmentExpression(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut AssignmentExpression) };
                it_mut.set_parent(new_parent)
            }
            Self::AssignmentTarget(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut AssignmentTarget) };
                it_mut.set_parent(new_parent)
            }
            Self::SimpleAssignmentTarget(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut SimpleAssignmentTarget) };
                it_mut.set_parent(new_parent)
            }
            Self::AssignmentTargetPattern(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut AssignmentTargetPattern) };
                it_mut.set_parent(new_parent)
            }
            Self::ArrayAssignmentTarget(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut ArrayAssignmentTarget) };
                it_mut.set_parent(new_parent)
            }
            Self::ObjectAssignmentTarget(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut ObjectAssignmentTarget) };
                it_mut.set_parent(new_parent)
            }
            Self::AssignmentTargetWithDefault(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut AssignmentTargetWithDefault) };
                it_mut.set_parent(new_parent)
            }
            Self::SequenceExpression(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut SequenceExpression) };
                it_mut.set_parent(new_parent)
            }
            Self::Super(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut Super) };
                it_mut.set_parent(new_parent)
            }
            Self::AwaitExpression(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut AwaitExpression) };
                it_mut.set_parent(new_parent)
            }
            Self::ChainExpression(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut ChainExpression) };
                it_mut.set_parent(new_parent)
            }
            Self::ParenthesizedExpression(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut ParenthesizedExpression) };
                it_mut.set_parent(new_parent)
            }
            Self::Directive(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut Directive) };
                it_mut.set_parent(new_parent)
            }
            Self::Hashbang(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut Hashbang) };
                it_mut.set_parent(new_parent)
            }
            Self::BlockStatement(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut BlockStatement) };
                it_mut.set_parent(new_parent)
            }
            Self::VariableDeclaration(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut VariableDeclaration) };
                it_mut.set_parent(new_parent)
            }
            Self::VariableDeclarator(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut VariableDeclarator) };
                it_mut.set_parent(new_parent)
            }
            Self::EmptyStatement(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut EmptyStatement) };
                it_mut.set_parent(new_parent)
            }
            Self::ExpressionStatement(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut ExpressionStatement) };
                it_mut.set_parent(new_parent)
            }
            Self::IfStatement(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut IfStatement) };
                it_mut.set_parent(new_parent)
            }
            Self::DoWhileStatement(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut DoWhileStatement) };
                it_mut.set_parent(new_parent)
            }
            Self::WhileStatement(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut WhileStatement) };
                it_mut.set_parent(new_parent)
            }
            Self::ForStatement(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut ForStatement) };
                it_mut.set_parent(new_parent)
            }
            Self::ForStatementInit(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut ForStatementInit) };
                it_mut.set_parent(new_parent)
            }
            Self::ForInStatement(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut ForInStatement) };
                it_mut.set_parent(new_parent)
            }
            Self::ForOfStatement(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut ForOfStatement) };
                it_mut.set_parent(new_parent)
            }
            Self::ContinueStatement(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut ContinueStatement) };
                it_mut.set_parent(new_parent)
            }
            Self::BreakStatement(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut BreakStatement) };
                it_mut.set_parent(new_parent)
            }
            Self::ReturnStatement(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut ReturnStatement) };
                it_mut.set_parent(new_parent)
            }
            Self::WithStatement(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut WithStatement) };
                it_mut.set_parent(new_parent)
            }
            Self::SwitchStatement(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut SwitchStatement) };
                it_mut.set_parent(new_parent)
            }
            Self::SwitchCase(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut SwitchCase) };
                it_mut.set_parent(new_parent)
            }
            Self::LabeledStatement(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut LabeledStatement) };
                it_mut.set_parent(new_parent)
            }
            Self::ThrowStatement(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut ThrowStatement) };
                it_mut.set_parent(new_parent)
            }
            Self::TryStatement(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut TryStatement) };
                it_mut.set_parent(new_parent)
            }
            Self::CatchClause(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut CatchClause) };
                it_mut.set_parent(new_parent)
            }
            Self::CatchParameter(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut CatchParameter) };
                it_mut.set_parent(new_parent)
            }
            Self::DebuggerStatement(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut DebuggerStatement) };
                it_mut.set_parent(new_parent)
            }
            Self::AssignmentPattern(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut AssignmentPattern) };
                it_mut.set_parent(new_parent)
            }
            Self::ObjectPattern(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut ObjectPattern) };
                it_mut.set_parent(new_parent)
            }
            Self::ArrayPattern(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut ArrayPattern) };
                it_mut.set_parent(new_parent)
            }
            Self::BindingRestElement(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut BindingRestElement) };
                it_mut.set_parent(new_parent)
            }
            Self::Function(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut Function) };
                it_mut.set_parent(new_parent)
            }
            Self::FormalParameters(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut FormalParameters) };
                it_mut.set_parent(new_parent)
            }
            Self::FormalParameter(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut FormalParameter) };
                it_mut.set_parent(new_parent)
            }
            Self::FunctionBody(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut FunctionBody) };
                it_mut.set_parent(new_parent)
            }
            Self::ArrowFunctionExpression(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut ArrowFunctionExpression) };
                it_mut.set_parent(new_parent)
            }
            Self::YieldExpression(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut YieldExpression) };
                it_mut.set_parent(new_parent)
            }
            Self::Class(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut Class) };
                it_mut.set_parent(new_parent)
            }
            Self::ClassBody(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut ClassBody) };
                it_mut.set_parent(new_parent)
            }
            Self::MethodDefinition(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut MethodDefinition) };
                it_mut.set_parent(new_parent)
            }
            Self::PropertyDefinition(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut PropertyDefinition) };
                it_mut.set_parent(new_parent)
            }
            Self::PrivateIdentifier(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut PrivateIdentifier) };
                it_mut.set_parent(new_parent)
            }
            Self::StaticBlock(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut StaticBlock) };
                it_mut.set_parent(new_parent)
            }
            Self::ModuleDeclaration(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut ModuleDeclaration) };
                it_mut.set_parent(new_parent)
            }
            Self::ImportExpression(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut ImportExpression) };
                it_mut.set_parent(new_parent)
            }
            Self::ImportDeclaration(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut ImportDeclaration) };
                it_mut.set_parent(new_parent)
            }
            Self::ImportSpecifier(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut ImportSpecifier) };
                it_mut.set_parent(new_parent)
            }
            Self::ImportDefaultSpecifier(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut ImportDefaultSpecifier) };
                it_mut.set_parent(new_parent)
            }
            Self::ImportNamespaceSpecifier(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut ImportNamespaceSpecifier) };
                it_mut.set_parent(new_parent)
            }
            Self::ExportNamedDeclaration(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut ExportNamedDeclaration) };
                it_mut.set_parent(new_parent)
            }
            Self::ExportDefaultDeclaration(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut ExportDefaultDeclaration) };
                it_mut.set_parent(new_parent)
            }
            Self::ExportAllDeclaration(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut ExportAllDeclaration) };
                it_mut.set_parent(new_parent)
            }
            Self::ExportSpecifier(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut ExportSpecifier) };
                it_mut.set_parent(new_parent)
            }
            Self::TSThisParameter(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut TSThisParameter) };
                it_mut.set_parent(new_parent)
            }
            Self::TSEnumDeclaration(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut TSEnumDeclaration) };
                it_mut.set_parent(new_parent)
            }
            Self::TSEnumMember(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut TSEnumMember) };
                it_mut.set_parent(new_parent)
            }
            Self::TSTypeAnnotation(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut TSTypeAnnotation) };
                it_mut.set_parent(new_parent)
            }
            Self::TSLiteralType(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut TSLiteralType) };
                it_mut.set_parent(new_parent)
            }
            Self::TSConditionalType(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut TSConditionalType) };
                it_mut.set_parent(new_parent)
            }
            Self::TSUnionType(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut TSUnionType) };
                it_mut.set_parent(new_parent)
            }
            Self::TSIntersectionType(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut TSIntersectionType) };
                it_mut.set_parent(new_parent)
            }
            Self::TSParenthesizedType(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut TSParenthesizedType) };
                it_mut.set_parent(new_parent)
            }
            Self::TSIndexedAccessType(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut TSIndexedAccessType) };
                it_mut.set_parent(new_parent)
            }
            Self::TSNamedTupleMember(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut TSNamedTupleMember) };
                it_mut.set_parent(new_parent)
            }
            Self::TSAnyKeyword(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut TSAnyKeyword) };
                it_mut.set_parent(new_parent)
            }
            Self::TSStringKeyword(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut TSStringKeyword) };
                it_mut.set_parent(new_parent)
            }
            Self::TSBooleanKeyword(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut TSBooleanKeyword) };
                it_mut.set_parent(new_parent)
            }
            Self::TSNumberKeyword(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut TSNumberKeyword) };
                it_mut.set_parent(new_parent)
            }
            Self::TSNeverKeyword(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut TSNeverKeyword) };
                it_mut.set_parent(new_parent)
            }
            Self::TSIntrinsicKeyword(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut TSIntrinsicKeyword) };
                it_mut.set_parent(new_parent)
            }
            Self::TSUnknownKeyword(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut TSUnknownKeyword) };
                it_mut.set_parent(new_parent)
            }
            Self::TSNullKeyword(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut TSNullKeyword) };
                it_mut.set_parent(new_parent)
            }
            Self::TSUndefinedKeyword(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut TSUndefinedKeyword) };
                it_mut.set_parent(new_parent)
            }
            Self::TSVoidKeyword(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut TSVoidKeyword) };
                it_mut.set_parent(new_parent)
            }
            Self::TSSymbolKeyword(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut TSSymbolKeyword) };
                it_mut.set_parent(new_parent)
            }
            Self::TSThisType(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut TSThisType) };
                it_mut.set_parent(new_parent)
            }
            Self::TSObjectKeyword(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut TSObjectKeyword) };
                it_mut.set_parent(new_parent)
            }
            Self::TSBigIntKeyword(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut TSBigIntKeyword) };
                it_mut.set_parent(new_parent)
            }
            Self::TSTypeReference(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut TSTypeReference) };
                it_mut.set_parent(new_parent)
            }
            Self::TSTypeName(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut TSTypeName) };
                it_mut.set_parent(new_parent)
            }
            Self::TSQualifiedName(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut TSQualifiedName) };
                it_mut.set_parent(new_parent)
            }
            Self::TSTypeParameterInstantiation(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut TSTypeParameterInstantiation) };
                it_mut.set_parent(new_parent)
            }
            Self::TSTypeParameter(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut TSTypeParameter) };
                it_mut.set_parent(new_parent)
            }
            Self::TSTypeParameterDeclaration(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut TSTypeParameterDeclaration) };
                it_mut.set_parent(new_parent)
            }
            Self::TSTypeAliasDeclaration(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut TSTypeAliasDeclaration) };
                it_mut.set_parent(new_parent)
            }
            Self::TSClassImplements(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut TSClassImplements) };
                it_mut.set_parent(new_parent)
            }
            Self::TSInterfaceDeclaration(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut TSInterfaceDeclaration) };
                it_mut.set_parent(new_parent)
            }
            Self::TSPropertySignature(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut TSPropertySignature) };
                it_mut.set_parent(new_parent)
            }
            Self::TSMethodSignature(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut TSMethodSignature) };
                it_mut.set_parent(new_parent)
            }
            Self::TSConstructSignatureDeclaration(it) => {
                let it_mut =
                    unsafe { &mut *(it as *const _ as *mut TSConstructSignatureDeclaration) };
                it_mut.set_parent(new_parent)
            }
            Self::TSInterfaceHeritage(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut TSInterfaceHeritage) };
                it_mut.set_parent(new_parent)
            }
            Self::TSModuleDeclaration(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut TSModuleDeclaration) };
                it_mut.set_parent(new_parent)
            }
            Self::TSModuleBlock(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut TSModuleBlock) };
                it_mut.set_parent(new_parent)
            }
            Self::TSTypeLiteral(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut TSTypeLiteral) };
                it_mut.set_parent(new_parent)
            }
            Self::TSInferType(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut TSInferType) };
                it_mut.set_parent(new_parent)
            }
            Self::TSTypeQuery(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut TSTypeQuery) };
                it_mut.set_parent(new_parent)
            }
            Self::TSImportType(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut TSImportType) };
                it_mut.set_parent(new_parent)
            }
            Self::TSMappedType(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut TSMappedType) };
                it_mut.set_parent(new_parent)
            }
            Self::TSTemplateLiteralType(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut TSTemplateLiteralType) };
                it_mut.set_parent(new_parent)
            }
            Self::TSAsExpression(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut TSAsExpression) };
                it_mut.set_parent(new_parent)
            }
            Self::TSSatisfiesExpression(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut TSSatisfiesExpression) };
                it_mut.set_parent(new_parent)
            }
            Self::TSTypeAssertion(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut TSTypeAssertion) };
                it_mut.set_parent(new_parent)
            }
            Self::TSImportEqualsDeclaration(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut TSImportEqualsDeclaration) };
                it_mut.set_parent(new_parent)
            }
            Self::TSModuleReference(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut TSModuleReference) };
                it_mut.set_parent(new_parent)
            }
            Self::TSExternalModuleReference(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut TSExternalModuleReference) };
                it_mut.set_parent(new_parent)
            }
            Self::TSNonNullExpression(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut TSNonNullExpression) };
                it_mut.set_parent(new_parent)
            }
            Self::Decorator(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut Decorator) };
                it_mut.set_parent(new_parent)
            }
            Self::TSExportAssignment(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut TSExportAssignment) };
                it_mut.set_parent(new_parent)
            }
            Self::TSInstantiationExpression(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut TSInstantiationExpression) };
                it_mut.set_parent(new_parent)
            }
            Self::JSXElement(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut JSXElement) };
                it_mut.set_parent(new_parent)
            }
            Self::JSXOpeningElement(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut JSXOpeningElement) };
                it_mut.set_parent(new_parent)
            }
            Self::JSXClosingElement(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut JSXClosingElement) };
                it_mut.set_parent(new_parent)
            }
            Self::JSXFragment(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut JSXFragment) };
                it_mut.set_parent(new_parent)
            }
            Self::JSXElementName(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut JSXElementName) };
                it_mut.set_parent(new_parent)
            }
            Self::JSXNamespacedName(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut JSXNamespacedName) };
                it_mut.set_parent(new_parent)
            }
            Self::JSXMemberExpression(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut JSXMemberExpression) };
                it_mut.set_parent(new_parent)
            }
            Self::JSXMemberExpressionObject(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut JSXMemberExpressionObject) };
                it_mut.set_parent(new_parent)
            }
            Self::JSXExpressionContainer(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut JSXExpressionContainer) };
                it_mut.set_parent(new_parent)
            }
            Self::JSXAttributeItem(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut JSXAttributeItem) };
                it_mut.set_parent(new_parent)
            }
            Self::JSXSpreadAttribute(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut JSXSpreadAttribute) };
                it_mut.set_parent(new_parent)
            }
            Self::JSXIdentifier(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut JSXIdentifier) };
                it_mut.set_parent(new_parent)
            }
            Self::JSXText(it) => {
                let it_mut = unsafe { &mut *(it as *const _ as *mut JSXText) };
                it_mut.set_parent(new_parent)
            }
        }
    }
}

impl<'a> AstKind<'a> {
    #[inline]
    pub fn as_boolean_literal(self) -> Option<&'a BooleanLiteral<'a>> {
        if let Self::BooleanLiteral(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_null_literal(self) -> Option<&'a NullLiteral<'a>> {
        if let Self::NullLiteral(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_numeric_literal(self) -> Option<&'a NumericLiteral<'a>> {
        if let Self::NumericLiteral(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_string_literal(self) -> Option<&'a StringLiteral<'a>> {
        if let Self::StringLiteral(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_big_int_literal(self) -> Option<&'a BigIntLiteral<'a>> {
        if let Self::BigIntLiteral(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_reg_exp_literal(self) -> Option<&'a RegExpLiteral<'a>> {
        if let Self::RegExpLiteral(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_program(self) -> Option<&'a Program<'a>> {
        if let Self::Program(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_identifier_name(self) -> Option<&'a IdentifierName<'a>> {
        if let Self::IdentifierName(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_identifier_reference(self) -> Option<&'a IdentifierReference<'a>> {
        if let Self::IdentifierReference(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_binding_identifier(self) -> Option<&'a BindingIdentifier<'a>> {
        if let Self::BindingIdentifier(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_label_identifier(self) -> Option<&'a LabelIdentifier<'a>> {
        if let Self::LabelIdentifier(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_this_expression(self) -> Option<&'a ThisExpression<'a>> {
        if let Self::ThisExpression(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_array_expression(self) -> Option<&'a ArrayExpression<'a>> {
        if let Self::ArrayExpression(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_array_expression_element(self) -> Option<&'a ArrayExpressionElement<'a>> {
        if let Self::ArrayExpressionElement(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_elision(self) -> Option<&'a Elision<'a>> {
        if let Self::Elision(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_object_expression(self) -> Option<&'a ObjectExpression<'a>> {
        if let Self::ObjectExpression(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_object_property(self) -> Option<&'a ObjectProperty<'a>> {
        if let Self::ObjectProperty(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_property_key(self) -> Option<&'a PropertyKey<'a>> {
        if let Self::PropertyKey(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_template_literal(self) -> Option<&'a TemplateLiteral<'a>> {
        if let Self::TemplateLiteral(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_tagged_template_expression(self) -> Option<&'a TaggedTemplateExpression<'a>> {
        if let Self::TaggedTemplateExpression(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_member_expression(self) -> Option<&'a MemberExpression<'a>> {
        if let Self::MemberExpression(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_call_expression(self) -> Option<&'a CallExpression<'a>> {
        if let Self::CallExpression(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_new_expression(self) -> Option<&'a NewExpression<'a>> {
        if let Self::NewExpression(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_meta_property(self) -> Option<&'a MetaProperty<'a>> {
        if let Self::MetaProperty(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_spread_element(self) -> Option<&'a SpreadElement<'a>> {
        if let Self::SpreadElement(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_argument(self) -> Option<&'a Argument<'a>> {
        if let Self::Argument(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_update_expression(self) -> Option<&'a UpdateExpression<'a>> {
        if let Self::UpdateExpression(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_unary_expression(self) -> Option<&'a UnaryExpression<'a>> {
        if let Self::UnaryExpression(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_binary_expression(self) -> Option<&'a BinaryExpression<'a>> {
        if let Self::BinaryExpression(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_private_in_expression(self) -> Option<&'a PrivateInExpression<'a>> {
        if let Self::PrivateInExpression(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_logical_expression(self) -> Option<&'a LogicalExpression<'a>> {
        if let Self::LogicalExpression(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_conditional_expression(self) -> Option<&'a ConditionalExpression<'a>> {
        if let Self::ConditionalExpression(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_assignment_expression(self) -> Option<&'a AssignmentExpression<'a>> {
        if let Self::AssignmentExpression(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_assignment_target(self) -> Option<&'a AssignmentTarget<'a>> {
        if let Self::AssignmentTarget(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_simple_assignment_target(self) -> Option<&'a SimpleAssignmentTarget<'a>> {
        if let Self::SimpleAssignmentTarget(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_assignment_target_pattern(self) -> Option<&'a AssignmentTargetPattern<'a>> {
        if let Self::AssignmentTargetPattern(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_array_assignment_target(self) -> Option<&'a ArrayAssignmentTarget<'a>> {
        if let Self::ArrayAssignmentTarget(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_object_assignment_target(self) -> Option<&'a ObjectAssignmentTarget<'a>> {
        if let Self::ObjectAssignmentTarget(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_assignment_target_with_default(self) -> Option<&'a AssignmentTargetWithDefault<'a>> {
        if let Self::AssignmentTargetWithDefault(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_sequence_expression(self) -> Option<&'a SequenceExpression<'a>> {
        if let Self::SequenceExpression(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_super(self) -> Option<&'a Super<'a>> {
        if let Self::Super(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_await_expression(self) -> Option<&'a AwaitExpression<'a>> {
        if let Self::AwaitExpression(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_chain_expression(self) -> Option<&'a ChainExpression<'a>> {
        if let Self::ChainExpression(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_parenthesized_expression(self) -> Option<&'a ParenthesizedExpression<'a>> {
        if let Self::ParenthesizedExpression(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_directive(self) -> Option<&'a Directive<'a>> {
        if let Self::Directive(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_hashbang(self) -> Option<&'a Hashbang<'a>> {
        if let Self::Hashbang(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_block_statement(self) -> Option<&'a BlockStatement<'a>> {
        if let Self::BlockStatement(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_variable_declaration(self) -> Option<&'a VariableDeclaration<'a>> {
        if let Self::VariableDeclaration(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_variable_declarator(self) -> Option<&'a VariableDeclarator<'a>> {
        if let Self::VariableDeclarator(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_empty_statement(self) -> Option<&'a EmptyStatement<'a>> {
        if let Self::EmptyStatement(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_expression_statement(self) -> Option<&'a ExpressionStatement<'a>> {
        if let Self::ExpressionStatement(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_if_statement(self) -> Option<&'a IfStatement<'a>> {
        if let Self::IfStatement(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_do_while_statement(self) -> Option<&'a DoWhileStatement<'a>> {
        if let Self::DoWhileStatement(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_while_statement(self) -> Option<&'a WhileStatement<'a>> {
        if let Self::WhileStatement(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_for_statement(self) -> Option<&'a ForStatement<'a>> {
        if let Self::ForStatement(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_for_statement_init(self) -> Option<&'a ForStatementInit<'a>> {
        if let Self::ForStatementInit(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_for_in_statement(self) -> Option<&'a ForInStatement<'a>> {
        if let Self::ForInStatement(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_for_of_statement(self) -> Option<&'a ForOfStatement<'a>> {
        if let Self::ForOfStatement(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_continue_statement(self) -> Option<&'a ContinueStatement<'a>> {
        if let Self::ContinueStatement(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_break_statement(self) -> Option<&'a BreakStatement<'a>> {
        if let Self::BreakStatement(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_return_statement(self) -> Option<&'a ReturnStatement<'a>> {
        if let Self::ReturnStatement(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_with_statement(self) -> Option<&'a WithStatement<'a>> {
        if let Self::WithStatement(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_switch_statement(self) -> Option<&'a SwitchStatement<'a>> {
        if let Self::SwitchStatement(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_switch_case(self) -> Option<&'a SwitchCase<'a>> {
        if let Self::SwitchCase(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_labeled_statement(self) -> Option<&'a LabeledStatement<'a>> {
        if let Self::LabeledStatement(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_throw_statement(self) -> Option<&'a ThrowStatement<'a>> {
        if let Self::ThrowStatement(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_try_statement(self) -> Option<&'a TryStatement<'a>> {
        if let Self::TryStatement(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_catch_clause(self) -> Option<&'a CatchClause<'a>> {
        if let Self::CatchClause(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_catch_parameter(self) -> Option<&'a CatchParameter<'a>> {
        if let Self::CatchParameter(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_debugger_statement(self) -> Option<&'a DebuggerStatement<'a>> {
        if let Self::DebuggerStatement(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_assignment_pattern(self) -> Option<&'a AssignmentPattern<'a>> {
        if let Self::AssignmentPattern(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_object_pattern(self) -> Option<&'a ObjectPattern<'a>> {
        if let Self::ObjectPattern(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_array_pattern(self) -> Option<&'a ArrayPattern<'a>> {
        if let Self::ArrayPattern(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_binding_rest_element(self) -> Option<&'a BindingRestElement<'a>> {
        if let Self::BindingRestElement(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_function(self) -> Option<&'a Function<'a>> {
        if let Self::Function(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_formal_parameters(self) -> Option<&'a FormalParameters<'a>> {
        if let Self::FormalParameters(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_formal_parameter(self) -> Option<&'a FormalParameter<'a>> {
        if let Self::FormalParameter(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_function_body(self) -> Option<&'a FunctionBody<'a>> {
        if let Self::FunctionBody(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_arrow_function_expression(self) -> Option<&'a ArrowFunctionExpression<'a>> {
        if let Self::ArrowFunctionExpression(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_yield_expression(self) -> Option<&'a YieldExpression<'a>> {
        if let Self::YieldExpression(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_class(self) -> Option<&'a Class<'a>> {
        if let Self::Class(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_class_body(self) -> Option<&'a ClassBody<'a>> {
        if let Self::ClassBody(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_method_definition(self) -> Option<&'a MethodDefinition<'a>> {
        if let Self::MethodDefinition(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_property_definition(self) -> Option<&'a PropertyDefinition<'a>> {
        if let Self::PropertyDefinition(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_private_identifier(self) -> Option<&'a PrivateIdentifier<'a>> {
        if let Self::PrivateIdentifier(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_static_block(self) -> Option<&'a StaticBlock<'a>> {
        if let Self::StaticBlock(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_module_declaration(self) -> Option<&'a ModuleDeclaration<'a>> {
        if let Self::ModuleDeclaration(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_import_expression(self) -> Option<&'a ImportExpression<'a>> {
        if let Self::ImportExpression(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_import_declaration(self) -> Option<&'a ImportDeclaration<'a>> {
        if let Self::ImportDeclaration(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_import_specifier(self) -> Option<&'a ImportSpecifier<'a>> {
        if let Self::ImportSpecifier(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_import_default_specifier(self) -> Option<&'a ImportDefaultSpecifier<'a>> {
        if let Self::ImportDefaultSpecifier(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_import_namespace_specifier(self) -> Option<&'a ImportNamespaceSpecifier<'a>> {
        if let Self::ImportNamespaceSpecifier(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_export_named_declaration(self) -> Option<&'a ExportNamedDeclaration<'a>> {
        if let Self::ExportNamedDeclaration(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_export_default_declaration(self) -> Option<&'a ExportDefaultDeclaration<'a>> {
        if let Self::ExportDefaultDeclaration(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_export_all_declaration(self) -> Option<&'a ExportAllDeclaration<'a>> {
        if let Self::ExportAllDeclaration(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_export_specifier(self) -> Option<&'a ExportSpecifier<'a>> {
        if let Self::ExportSpecifier(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_ts_this_parameter(self) -> Option<&'a TSThisParameter<'a>> {
        if let Self::TSThisParameter(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_ts_enum_declaration(self) -> Option<&'a TSEnumDeclaration<'a>> {
        if let Self::TSEnumDeclaration(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_ts_enum_member(self) -> Option<&'a TSEnumMember<'a>> {
        if let Self::TSEnumMember(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_ts_type_annotation(self) -> Option<&'a TSTypeAnnotation<'a>> {
        if let Self::TSTypeAnnotation(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_ts_literal_type(self) -> Option<&'a TSLiteralType<'a>> {
        if let Self::TSLiteralType(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_ts_conditional_type(self) -> Option<&'a TSConditionalType<'a>> {
        if let Self::TSConditionalType(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_ts_union_type(self) -> Option<&'a TSUnionType<'a>> {
        if let Self::TSUnionType(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_ts_intersection_type(self) -> Option<&'a TSIntersectionType<'a>> {
        if let Self::TSIntersectionType(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_ts_parenthesized_type(self) -> Option<&'a TSParenthesizedType<'a>> {
        if let Self::TSParenthesizedType(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_ts_indexed_access_type(self) -> Option<&'a TSIndexedAccessType<'a>> {
        if let Self::TSIndexedAccessType(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_ts_named_tuple_member(self) -> Option<&'a TSNamedTupleMember<'a>> {
        if let Self::TSNamedTupleMember(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_ts_any_keyword(self) -> Option<&'a TSAnyKeyword<'a>> {
        if let Self::TSAnyKeyword(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_ts_string_keyword(self) -> Option<&'a TSStringKeyword<'a>> {
        if let Self::TSStringKeyword(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_ts_boolean_keyword(self) -> Option<&'a TSBooleanKeyword<'a>> {
        if let Self::TSBooleanKeyword(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_ts_number_keyword(self) -> Option<&'a TSNumberKeyword<'a>> {
        if let Self::TSNumberKeyword(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_ts_never_keyword(self) -> Option<&'a TSNeverKeyword<'a>> {
        if let Self::TSNeverKeyword(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_ts_intrinsic_keyword(self) -> Option<&'a TSIntrinsicKeyword<'a>> {
        if let Self::TSIntrinsicKeyword(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_ts_unknown_keyword(self) -> Option<&'a TSUnknownKeyword<'a>> {
        if let Self::TSUnknownKeyword(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_ts_null_keyword(self) -> Option<&'a TSNullKeyword<'a>> {
        if let Self::TSNullKeyword(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_ts_undefined_keyword(self) -> Option<&'a TSUndefinedKeyword<'a>> {
        if let Self::TSUndefinedKeyword(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_ts_void_keyword(self) -> Option<&'a TSVoidKeyword<'a>> {
        if let Self::TSVoidKeyword(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_ts_symbol_keyword(self) -> Option<&'a TSSymbolKeyword<'a>> {
        if let Self::TSSymbolKeyword(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_ts_this_type(self) -> Option<&'a TSThisType<'a>> {
        if let Self::TSThisType(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_ts_object_keyword(self) -> Option<&'a TSObjectKeyword<'a>> {
        if let Self::TSObjectKeyword(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_ts_big_int_keyword(self) -> Option<&'a TSBigIntKeyword<'a>> {
        if let Self::TSBigIntKeyword(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_ts_type_reference(self) -> Option<&'a TSTypeReference<'a>> {
        if let Self::TSTypeReference(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_ts_type_name(self) -> Option<&'a TSTypeName<'a>> {
        if let Self::TSTypeName(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_ts_qualified_name(self) -> Option<&'a TSQualifiedName<'a>> {
        if let Self::TSQualifiedName(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_ts_type_parameter_instantiation(
        self,
    ) -> Option<&'a TSTypeParameterInstantiation<'a>> {
        if let Self::TSTypeParameterInstantiation(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_ts_type_parameter(self) -> Option<&'a TSTypeParameter<'a>> {
        if let Self::TSTypeParameter(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_ts_type_parameter_declaration(self) -> Option<&'a TSTypeParameterDeclaration<'a>> {
        if let Self::TSTypeParameterDeclaration(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_ts_type_alias_declaration(self) -> Option<&'a TSTypeAliasDeclaration<'a>> {
        if let Self::TSTypeAliasDeclaration(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_ts_class_implements(self) -> Option<&'a TSClassImplements<'a>> {
        if let Self::TSClassImplements(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_ts_interface_declaration(self) -> Option<&'a TSInterfaceDeclaration<'a>> {
        if let Self::TSInterfaceDeclaration(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_ts_property_signature(self) -> Option<&'a TSPropertySignature<'a>> {
        if let Self::TSPropertySignature(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_ts_method_signature(self) -> Option<&'a TSMethodSignature<'a>> {
        if let Self::TSMethodSignature(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_ts_construct_signature_declaration(
        self,
    ) -> Option<&'a TSConstructSignatureDeclaration<'a>> {
        if let Self::TSConstructSignatureDeclaration(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_ts_interface_heritage(self) -> Option<&'a TSInterfaceHeritage<'a>> {
        if let Self::TSInterfaceHeritage(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_ts_module_declaration(self) -> Option<&'a TSModuleDeclaration<'a>> {
        if let Self::TSModuleDeclaration(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_ts_module_block(self) -> Option<&'a TSModuleBlock<'a>> {
        if let Self::TSModuleBlock(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_ts_type_literal(self) -> Option<&'a TSTypeLiteral<'a>> {
        if let Self::TSTypeLiteral(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_ts_infer_type(self) -> Option<&'a TSInferType<'a>> {
        if let Self::TSInferType(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_ts_type_query(self) -> Option<&'a TSTypeQuery<'a>> {
        if let Self::TSTypeQuery(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_ts_import_type(self) -> Option<&'a TSImportType<'a>> {
        if let Self::TSImportType(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_ts_mapped_type(self) -> Option<&'a TSMappedType<'a>> {
        if let Self::TSMappedType(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_ts_template_literal_type(self) -> Option<&'a TSTemplateLiteralType<'a>> {
        if let Self::TSTemplateLiteralType(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_ts_as_expression(self) -> Option<&'a TSAsExpression<'a>> {
        if let Self::TSAsExpression(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_ts_satisfies_expression(self) -> Option<&'a TSSatisfiesExpression<'a>> {
        if let Self::TSSatisfiesExpression(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_ts_type_assertion(self) -> Option<&'a TSTypeAssertion<'a>> {
        if let Self::TSTypeAssertion(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_ts_import_equals_declaration(self) -> Option<&'a TSImportEqualsDeclaration<'a>> {
        if let Self::TSImportEqualsDeclaration(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_ts_module_reference(self) -> Option<&'a TSModuleReference<'a>> {
        if let Self::TSModuleReference(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_ts_external_module_reference(self) -> Option<&'a TSExternalModuleReference<'a>> {
        if let Self::TSExternalModuleReference(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_ts_non_null_expression(self) -> Option<&'a TSNonNullExpression<'a>> {
        if let Self::TSNonNullExpression(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_decorator(self) -> Option<&'a Decorator<'a>> {
        if let Self::Decorator(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_ts_export_assignment(self) -> Option<&'a TSExportAssignment<'a>> {
        if let Self::TSExportAssignment(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_ts_instantiation_expression(self) -> Option<&'a TSInstantiationExpression<'a>> {
        if let Self::TSInstantiationExpression(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_jsx_element(self) -> Option<&'a JSXElement<'a>> {
        if let Self::JSXElement(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_jsx_opening_element(self) -> Option<&'a JSXOpeningElement<'a>> {
        if let Self::JSXOpeningElement(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_jsx_closing_element(self) -> Option<&'a JSXClosingElement<'a>> {
        if let Self::JSXClosingElement(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_jsx_fragment(self) -> Option<&'a JSXFragment<'a>> {
        if let Self::JSXFragment(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_jsx_element_name(self) -> Option<&'a JSXElementName<'a>> {
        if let Self::JSXElementName(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_jsx_namespaced_name(self) -> Option<&'a JSXNamespacedName<'a>> {
        if let Self::JSXNamespacedName(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_jsx_member_expression(self) -> Option<&'a JSXMemberExpression<'a>> {
        if let Self::JSXMemberExpression(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_jsx_member_expression_object(self) -> Option<&'a JSXMemberExpressionObject<'a>> {
        if let Self::JSXMemberExpressionObject(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_jsx_expression_container(self) -> Option<&'a JSXExpressionContainer<'a>> {
        if let Self::JSXExpressionContainer(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_jsx_attribute_item(self) -> Option<&'a JSXAttributeItem<'a>> {
        if let Self::JSXAttributeItem(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_jsx_spread_attribute(self) -> Option<&'a JSXSpreadAttribute<'a>> {
        if let Self::JSXSpreadAttribute(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_jsx_identifier(self) -> Option<&'a JSXIdentifier<'a>> {
        if let Self::JSXIdentifier(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_jsx_text(self) -> Option<&'a JSXText<'a>> {
        if let Self::JSXText(v) = self {
            Some(v)
        } else {
            None
        }
    }
}
