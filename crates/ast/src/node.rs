use std::fmt::Debug;

use serde::Deserialize;
use serde::Serialize;
use strum::Display;

use mago_span::HasSpan;
use mago_span::Span;

use crate::ast::*;
use crate::Program;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize, PartialOrd, Ord, Display)]
#[serde(tag = "type", content = "value")]
pub enum NodeKind {
    Program,
    Access,
    ClassConstantAccess,
    NullSafePropertyAccess,
    PropertyAccess,
    StaticPropertyAccess,
    Argument,
    ArgumentList,
    NamedArgument,
    PositionalArgument,
    Array,
    ArrayAccess,
    ArrayAppend,
    ArrayElement,
    KeyValueArrayElement,
    LegacyArray,
    List,
    MissingArrayElement,
    ValueArrayElement,
    VariadicArrayElement,
    Attribute,
    AttributeList,
    Block,
    Call,
    FunctionCall,
    MethodCall,
    NullSafeMethodCall,
    StaticMethodCall,
    ClassLikeConstant,
    ClassLikeConstantItem,
    EnumCase,
    EnumCaseBackedItem,
    EnumCaseItem,
    EnumCaseUnitItem,
    Extends,
    Implements,
    ClassLikeConstantSelector,
    ClassLikeMember,
    ClassLikeMemberExpressionSelector,
    ClassLikeMemberSelector,
    Method,
    MethodAbstractBody,
    MethodBody,
    HookedProperty,
    PlainProperty,
    Property,
    PropertyAbstractItem,
    PropertyConcreteItem,
    PropertyHook,
    PropertyHookAbstractBody,
    PropertyHookBody,
    PropertyHookConcreteBody,
    PropertyHookConcreteExpressionBody,
    PropertyHookList,
    PropertyItem,
    TraitUse,
    TraitUseAbsoluteMethodReference,
    TraitUseAbstractSpecification,
    TraitUseAdaptation,
    TraitUseAliasAdaptation,
    TraitUseConcreteSpecification,
    TraitUseMethodReference,
    TraitUsePrecedenceAdaptation,
    TraitUseSpecification,
    AnonymousClass,
    Class,
    Enum,
    EnumBackingTypeHint,
    Interface,
    Trait,
    Clone,
    ClosureCreation,
    FunctionClosureCreation,
    MethodClosureCreation,
    StaticMethodClosureCreation,
    Constant,
    ConstantItem,
    Construct,
    DieConstruct,
    EmptyConstruct,
    EvalConstruct,
    ExitConstruct,
    IncludeConstruct,
    IncludeOnceConstruct,
    IssetConstruct,
    PrintConstruct,
    RequireConstruct,
    RequireOnceConstruct,
    If,
    IfBody,
    IfColonDelimitedBody,
    IfColonDelimitedBodyElseClause,
    IfColonDelimitedBodyElseIfClause,
    IfStatementBody,
    IfStatementBodyElseClause,
    IfStatementBodyElseIfClause,
    Match,
    MatchArm,
    MatchDefaultArm,
    MatchExpressionArm,
    Switch,
    SwitchBody,
    SwitchBraceDelimitedBody,
    SwitchCase,
    SwitchCaseSeparator,
    SwitchColonDelimitedBody,
    SwitchDefaultCase,
    SwitchExpressionCase,
    Declare,
    DeclareBody,
    DeclareColonDelimitedBody,
    DeclareItem,
    Echo,
    Expression,
    Binary,
    BinaryOperator,
    UnaryPrefix,
    UnaryPrefixOperator,
    UnaryPostfix,
    UnaryPostfixOperator,
    Parenthesized,
    ArrowFunction,
    Closure,
    ClosureUseClause,
    ClosureUseClauseVariable,
    Function,
    FunctionLikeParameter,
    FunctionLikeParameterDefaultValue,
    FunctionLikeParameterList,
    FunctionLikeReturnTypeHint,
    Global,
    Goto,
    Label,
    HaltCompiler,
    FullyQualifiedIdentifier,
    Identifier,
    LocalIdentifier,
    QualifiedIdentifier,
    Inline,
    Instantiation,
    Keyword,
    Literal,
    LiteralFloat,
    LiteralInteger,
    LiteralString,
    MagicConstant,
    Modifier,
    Namespace,
    NamespaceBody,
    NamespaceImplicitBody,
    Assignment,
    AssignmentOperator,
    Conditional,
    DoWhile,
    Foreach,
    ForeachBody,
    ForeachColonDelimitedBody,
    ForeachKeyValueTarget,
    ForeachTarget,
    ForeachValueTarget,
    For,
    ForBody,
    ForColonDelimitedBody,
    While,
    WhileBody,
    WhileColonDelimitedBody,
    Break,
    Continue,
    Return,
    Static,
    StaticAbstractItem,
    StaticConcreteItem,
    StaticItem,
    Try,
    TryCatchClause,
    TryFinallyClause,
    MaybeTypedUseItem,
    MixedUseItemList,
    TypedUseItemList,
    TypedUseItemSequence,
    Use,
    UseItem,
    UseItemAlias,
    UseItemSequence,
    UseItems,
    UseType,
    Yield,
    YieldFrom,
    YieldPair,
    YieldValue,
    Statement,
    ExpressionStatement,
    BracedExpressionStringPart,
    DocumentString,
    InterpolatedString,
    LiteralStringPart,
    ShellExecuteString,
    CompositeString,
    StringPart,
    ClosingTag,
    EchoOpeningTag,
    FullOpeningTag,
    OpeningTag,
    ShortOpeningTag,
    Terminator,
    Throw,
    Hint,
    IntersectionHint,
    NullableHint,
    ParenthesizedHint,
    UnionHint,
    Unset,
    DirectVariable,
    IndirectVariable,
    NestedVariable,
    Variable,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, PartialOrd, Ord, Display)]
#[serde(tag = "type", content = "value")]
pub enum Node<'a> {
    Program(&'a Program),
    Access(&'a Access),
    ClassConstantAccess(&'a ClassConstantAccess),
    NullSafePropertyAccess(&'a NullSafePropertyAccess),
    PropertyAccess(&'a PropertyAccess),
    StaticPropertyAccess(&'a StaticPropertyAccess),
    Argument(&'a Argument),
    ArgumentList(&'a ArgumentList),
    NamedArgument(&'a NamedArgument),
    PositionalArgument(&'a PositionalArgument),
    Array(&'a Array),
    ArrayAccess(&'a ArrayAccess),
    ArrayAppend(&'a ArrayAppend),
    ArrayElement(&'a ArrayElement),
    KeyValueArrayElement(&'a KeyValueArrayElement),
    LegacyArray(&'a LegacyArray),
    List(&'a List),
    MissingArrayElement(&'a MissingArrayElement),
    ValueArrayElement(&'a ValueArrayElement),
    VariadicArrayElement(&'a VariadicArrayElement),
    Attribute(&'a Attribute),
    AttributeList(&'a AttributeList),
    Block(&'a Block),
    Call(&'a Call),
    FunctionCall(&'a FunctionCall),
    MethodCall(&'a MethodCall),
    NullSafeMethodCall(&'a NullSafeMethodCall),
    StaticMethodCall(&'a StaticMethodCall),
    ClassLikeConstant(&'a ClassLikeConstant),
    ClassLikeConstantItem(&'a ClassLikeConstantItem),
    EnumCase(&'a EnumCase),
    EnumCaseBackedItem(&'a EnumCaseBackedItem),
    EnumCaseItem(&'a EnumCaseItem),
    EnumCaseUnitItem(&'a EnumCaseUnitItem),
    Extends(&'a Extends),
    Implements(&'a Implements),
    ClassLikeConstantSelector(&'a ClassLikeConstantSelector),
    ClassLikeMember(&'a ClassLikeMember),
    ClassLikeMemberExpressionSelector(&'a ClassLikeMemberExpressionSelector),
    ClassLikeMemberSelector(&'a ClassLikeMemberSelector),
    Method(&'a Method),
    MethodAbstractBody(&'a MethodAbstractBody),
    MethodBody(&'a MethodBody),
    HookedProperty(&'a HookedProperty),
    PlainProperty(&'a PlainProperty),
    Property(&'a Property),
    PropertyAbstractItem(&'a PropertyAbstractItem),
    PropertyConcreteItem(&'a PropertyConcreteItem),
    PropertyHook(&'a PropertyHook),
    PropertyHookAbstractBody(&'a PropertyHookAbstractBody),
    PropertyHookBody(&'a PropertyHookBody),
    PropertyHookConcreteBody(&'a PropertyHookConcreteBody),
    PropertyHookConcreteExpressionBody(&'a PropertyHookConcreteExpressionBody),
    PropertyHookList(&'a PropertyHookList),
    PropertyItem(&'a PropertyItem),
    TraitUse(&'a TraitUse),
    TraitUseAbsoluteMethodReference(&'a TraitUseAbsoluteMethodReference),
    TraitUseAbstractSpecification(&'a TraitUseAbstractSpecification),
    TraitUseAdaptation(&'a TraitUseAdaptation),
    TraitUseAliasAdaptation(&'a TraitUseAliasAdaptation),
    TraitUseConcreteSpecification(&'a TraitUseConcreteSpecification),
    TraitUseMethodReference(&'a TraitUseMethodReference),
    TraitUsePrecedenceAdaptation(&'a TraitUsePrecedenceAdaptation),
    TraitUseSpecification(&'a TraitUseSpecification),
    AnonymousClass(&'a AnonymousClass),
    Class(&'a Class),
    Enum(&'a Enum),
    EnumBackingTypeHint(&'a EnumBackingTypeHint),
    Interface(&'a Interface),
    Trait(&'a Trait),
    Clone(&'a Clone),
    ClosureCreation(&'a ClosureCreation),
    FunctionClosureCreation(&'a FunctionClosureCreation),
    MethodClosureCreation(&'a MethodClosureCreation),
    StaticMethodClosureCreation(&'a StaticMethodClosureCreation),
    Constant(&'a Constant),
    ConstantItem(&'a ConstantItem),
    Construct(&'a Construct),
    DieConstruct(&'a DieConstruct),
    EmptyConstruct(&'a EmptyConstruct),
    EvalConstruct(&'a EvalConstruct),
    ExitConstruct(&'a ExitConstruct),
    IncludeConstruct(&'a IncludeConstruct),
    IncludeOnceConstruct(&'a IncludeOnceConstruct),
    IssetConstruct(&'a IssetConstruct),
    PrintConstruct(&'a PrintConstruct),
    RequireConstruct(&'a RequireConstruct),
    RequireOnceConstruct(&'a RequireOnceConstruct),
    If(&'a If),
    IfBody(&'a IfBody),
    IfColonDelimitedBody(&'a IfColonDelimitedBody),
    IfColonDelimitedBodyElseClause(&'a IfColonDelimitedBodyElseClause),
    IfColonDelimitedBodyElseIfClause(&'a IfColonDelimitedBodyElseIfClause),
    IfStatementBody(&'a IfStatementBody),
    IfStatementBodyElseClause(&'a IfStatementBodyElseClause),
    IfStatementBodyElseIfClause(&'a IfStatementBodyElseIfClause),
    Match(&'a Match),
    MatchArm(&'a MatchArm),
    MatchDefaultArm(&'a MatchDefaultArm),
    MatchExpressionArm(&'a MatchExpressionArm),
    Switch(&'a Switch),
    SwitchBody(&'a SwitchBody),
    SwitchBraceDelimitedBody(&'a SwitchBraceDelimitedBody),
    SwitchCase(&'a SwitchCase),
    SwitchCaseSeparator(&'a SwitchCaseSeparator),
    SwitchColonDelimitedBody(&'a SwitchColonDelimitedBody),
    SwitchDefaultCase(&'a SwitchDefaultCase),
    SwitchExpressionCase(&'a SwitchExpressionCase),
    Declare(&'a Declare),
    DeclareBody(&'a DeclareBody),
    DeclareColonDelimitedBody(&'a DeclareColonDelimitedBody),
    DeclareItem(&'a DeclareItem),
    Echo(&'a Echo),
    Expression(&'a Expression),
    Binary(&'a Binary),
    BinaryOperator(&'a BinaryOperator),
    UnaryPrefix(&'a UnaryPrefix),
    UnaryPrefixOperator(&'a UnaryPrefixOperator),
    UnaryPostfix(&'a UnaryPostfix),
    UnaryPostfixOperator(&'a UnaryPostfixOperator),
    Parenthesized(&'a Parenthesized),
    ArrowFunction(&'a ArrowFunction),
    Closure(&'a Closure),
    ClosureUseClause(&'a ClosureUseClause),
    ClosureUseClauseVariable(&'a ClosureUseClauseVariable),
    Function(&'a Function),
    FunctionLikeParameter(&'a FunctionLikeParameter),
    FunctionLikeParameterDefaultValue(&'a FunctionLikeParameterDefaultValue),
    FunctionLikeParameterList(&'a FunctionLikeParameterList),
    FunctionLikeReturnTypeHint(&'a FunctionLikeReturnTypeHint),
    Global(&'a Global),
    Goto(&'a Goto),
    Label(&'a Label),
    HaltCompiler(&'a HaltCompiler),
    FullyQualifiedIdentifier(&'a FullyQualifiedIdentifier),
    Identifier(&'a Identifier),
    LocalIdentifier(&'a LocalIdentifier),
    QualifiedIdentifier(&'a QualifiedIdentifier),
    Inline(&'a Inline),
    Instantiation(&'a Instantiation),
    Keyword(&'a Keyword),
    Literal(&'a Literal),
    LiteralFloat(&'a LiteralFloat),
    LiteralInteger(&'a LiteralInteger),
    LiteralString(&'a LiteralString),
    MagicConstant(&'a MagicConstant),
    Modifier(&'a Modifier),
    Namespace(&'a Namespace),
    NamespaceBody(&'a NamespaceBody),
    NamespaceImplicitBody(&'a NamespaceImplicitBody),
    AssignmentOperation(&'a Assignment),
    AssignmentOperator(&'a AssignmentOperator),
    Conditional(&'a Conditional),
    DoWhile(&'a DoWhile),
    Foreach(&'a Foreach),
    ForeachBody(&'a ForeachBody),
    ForeachColonDelimitedBody(&'a ForeachColonDelimitedBody),
    ForeachKeyValueTarget(&'a ForeachKeyValueTarget),
    ForeachTarget(&'a ForeachTarget),
    ForeachValueTarget(&'a ForeachValueTarget),
    For(&'a For),
    ForBody(&'a ForBody),
    ForColonDelimitedBody(&'a ForColonDelimitedBody),
    While(&'a While),
    WhileBody(&'a WhileBody),
    WhileColonDelimitedBody(&'a WhileColonDelimitedBody),
    Break(&'a Break),
    Continue(&'a Continue),
    Return(&'a Return),
    Static(&'a Static),
    StaticAbstractItem(&'a StaticAbstractItem),
    StaticConcreteItem(&'a StaticConcreteItem),
    StaticItem(&'a StaticItem),
    Try(&'a Try),
    TryCatchClause(&'a TryCatchClause),
    TryFinallyClause(&'a TryFinallyClause),
    MaybeTypedUseItem(&'a MaybeTypedUseItem),
    MixedUseItemList(&'a MixedUseItemList),
    TypedUseItemList(&'a TypedUseItemList),
    TypedUseItemSequence(&'a TypedUseItemSequence),
    Use(&'a Use),
    UseItem(&'a UseItem),
    UseItemAlias(&'a UseItemAlias),
    UseItemSequence(&'a UseItemSequence),
    UseItems(&'a UseItems),
    UseType(&'a UseType),
    Yield(&'a Yield),
    YieldFrom(&'a YieldFrom),
    YieldPair(&'a YieldPair),
    YieldValue(&'a YieldValue),
    Statement(&'a Statement),
    ExpressionStatement(&'a ExpressionStatement),
    BracedExpressionStringPart(&'a BracedExpressionStringPart),
    DocumentString(&'a DocumentString),
    InterpolatedString(&'a InterpolatedString),
    LiteralStringPart(&'a LiteralStringPart),
    ShellExecuteString(&'a ShellExecuteString),
    CompositeString(&'a CompositeString),
    StringPart(&'a StringPart),
    ClosingTag(&'a ClosingTag),
    EchoOpeningTag(&'a EchoOpeningTag),
    FullOpeningTag(&'a FullOpeningTag),
    OpeningTag(&'a OpeningTag),
    ShortOpeningTag(&'a ShortOpeningTag),
    Terminator(&'a Terminator),
    Throw(&'a Throw),
    Hint(&'a Hint),
    IntersectionHint(&'a IntersectionHint),
    NullableHint(&'a NullableHint),
    ParenthesizedHint(&'a ParenthesizedHint),
    UnionHint(&'a UnionHint),
    Unset(&'a Unset),
    DirectVariable(&'a DirectVariable),
    IndirectVariable(&'a IndirectVariable),
    NestedVariable(&'a NestedVariable),
    Variable(&'a Variable),
}

impl<'a> Node<'a> {
    #[inline]
    pub fn filter_map<F, T>(&self, f: F) -> Vec<T>
    where
        F: Fn(&Node<'a>) -> Option<T>,
    {
        self.filter_map_internal(&f)
    }

    #[inline]
    fn filter_map_internal<F, T>(&self, f: &F) -> Vec<T>
    where
        F: Fn(&Node<'a>) -> Option<T>,
    {
        let mut result = vec![];
        for child in self.children() {
            result.extend(child.filter_map_internal(f));
        }

        if let Some(child) = f(self) {
            result.push(child);
        }

        result
    }

    #[inline]
    pub const fn is_declaration(&self) -> bool {
        matches!(
            self,
            Self::Class(_) | Self::Interface(_) | Self::Trait(_) | Self::Enum(_) | Self::Function(_) | Self::Method(_)
        )
    }

    #[inline]
    pub const fn is_statement(&self) -> bool {
        matches!(
            self,
            Self::Statement(_)
                | Self::OpeningTag(_)
                | Self::EchoOpeningTag(_)
                | Self::FullOpeningTag(_)
                | Self::ShortOpeningTag(_)
                | Self::ClosingTag(_)
                | Self::Inline(_)
                | Self::Namespace(_)
                | Self::Use(_)
                | Self::Class(_)
                | Self::Interface(_)
                | Self::Trait(_)
                | Self::Enum(_)
                | Self::Block(_)
                | Self::Constant(_)
                | Self::Function(_)
                | Self::Declare(_)
                | Self::Goto(_)
                | Self::Label(_)
                | Self::Try(_)
                | Self::Foreach(_)
                | Self::For(_)
                | Self::While(_)
                | Self::DoWhile(_)
                | Self::Continue(_)
                | Self::Break(_)
                | Self::Switch(_)
                | Self::If(_)
                | Self::Return(_)
                | Self::ExpressionStatement(_)
                | Self::Echo(_)
                | Self::Global(_)
                | Self::Static(_)
                | Self::HaltCompiler(_)
                | Self::Unset(_)
        )
    }

    #[inline]
    pub const fn kind(&self) -> NodeKind {
        match &self {
            Self::Program(_) => NodeKind::Program,
            Self::Access(_) => NodeKind::Access,
            Self::ClassConstantAccess(_) => NodeKind::ClassConstantAccess,
            Self::NullSafePropertyAccess(_) => NodeKind::NullSafePropertyAccess,
            Self::PropertyAccess(_) => NodeKind::PropertyAccess,
            Self::StaticPropertyAccess(_) => NodeKind::StaticPropertyAccess,
            Self::Argument(_) => NodeKind::Argument,
            Self::ArgumentList(_) => NodeKind::ArgumentList,
            Self::NamedArgument(_) => NodeKind::NamedArgument,
            Self::PositionalArgument(_) => NodeKind::PositionalArgument,
            Self::Array(_) => NodeKind::Array,
            Self::ArrayAccess(_) => NodeKind::ArrayAccess,
            Self::ArrayAppend(_) => NodeKind::ArrayAppend,
            Self::ArrayElement(_) => NodeKind::ArrayElement,
            Self::KeyValueArrayElement(_) => NodeKind::KeyValueArrayElement,
            Self::LegacyArray(_) => NodeKind::LegacyArray,
            Self::List(_) => NodeKind::List,
            Self::MissingArrayElement(_) => NodeKind::MissingArrayElement,
            Self::ValueArrayElement(_) => NodeKind::ValueArrayElement,
            Self::VariadicArrayElement(_) => NodeKind::VariadicArrayElement,
            Self::Attribute(_) => NodeKind::Attribute,
            Self::AttributeList(_) => NodeKind::AttributeList,
            Self::Block(_) => NodeKind::Block,
            Self::Call(_) => NodeKind::Call,
            Self::FunctionCall(_) => NodeKind::FunctionCall,
            Self::MethodCall(_) => NodeKind::MethodCall,
            Self::NullSafeMethodCall(_) => NodeKind::NullSafeMethodCall,
            Self::StaticMethodCall(_) => NodeKind::StaticMethodCall,
            Self::ClassLikeConstant(_) => NodeKind::ClassLikeConstant,
            Self::ClassLikeConstantItem(_) => NodeKind::ClassLikeConstantItem,
            Self::EnumCase(_) => NodeKind::EnumCase,
            Self::EnumCaseBackedItem(_) => NodeKind::EnumCaseBackedItem,
            Self::EnumCaseItem(_) => NodeKind::EnumCaseItem,
            Self::EnumCaseUnitItem(_) => NodeKind::EnumCaseUnitItem,
            Self::Extends(_) => NodeKind::Extends,
            Self::Implements(_) => NodeKind::Implements,
            Self::ClassLikeConstantSelector(_) => NodeKind::ClassLikeConstantSelector,
            Self::ClassLikeMember(_) => NodeKind::ClassLikeMember,
            Self::ClassLikeMemberExpressionSelector(_) => NodeKind::ClassLikeMemberExpressionSelector,
            Self::ClassLikeMemberSelector(_) => NodeKind::ClassLikeMemberSelector,
            Self::Method(_) => NodeKind::Method,
            Self::MethodAbstractBody(_) => NodeKind::MethodAbstractBody,
            Self::MethodBody(_) => NodeKind::MethodBody,
            Self::HookedProperty(_) => NodeKind::HookedProperty,
            Self::PlainProperty(_) => NodeKind::PlainProperty,
            Self::Property(_) => NodeKind::Property,
            Self::PropertyAbstractItem(_) => NodeKind::PropertyAbstractItem,
            Self::PropertyConcreteItem(_) => NodeKind::PropertyConcreteItem,
            Self::PropertyHook(_) => NodeKind::PropertyHook,
            Self::PropertyHookAbstractBody(_) => NodeKind::PropertyHookAbstractBody,
            Self::PropertyHookBody(_) => NodeKind::PropertyHookBody,
            Self::PropertyHookConcreteBody(_) => NodeKind::PropertyHookConcreteBody,
            Self::PropertyHookConcreteExpressionBody(_) => NodeKind::PropertyHookConcreteExpressionBody,
            Self::PropertyHookList(_) => NodeKind::PropertyHookList,
            Self::PropertyItem(_) => NodeKind::PropertyItem,
            Self::TraitUse(_) => NodeKind::TraitUse,
            Self::TraitUseAbsoluteMethodReference(_) => NodeKind::TraitUseAbsoluteMethodReference,
            Self::TraitUseAbstractSpecification(_) => NodeKind::TraitUseAbstractSpecification,
            Self::TraitUseAdaptation(_) => NodeKind::TraitUseAdaptation,
            Self::TraitUseAliasAdaptation(_) => NodeKind::TraitUseAliasAdaptation,
            Self::TraitUseConcreteSpecification(_) => NodeKind::TraitUseConcreteSpecification,
            Self::TraitUseMethodReference(_) => NodeKind::TraitUseMethodReference,
            Self::TraitUsePrecedenceAdaptation(_) => NodeKind::TraitUsePrecedenceAdaptation,
            Self::TraitUseSpecification(_) => NodeKind::TraitUseSpecification,
            Self::AnonymousClass(_) => NodeKind::AnonymousClass,
            Self::Class(_) => NodeKind::Class,
            Self::Enum(_) => NodeKind::Enum,
            Self::EnumBackingTypeHint(_) => NodeKind::EnumBackingTypeHint,
            Self::Interface(_) => NodeKind::Interface,
            Self::Trait(_) => NodeKind::Trait,
            Self::Clone(_) => NodeKind::Clone,
            Self::ClosureCreation(_) => NodeKind::ClosureCreation,
            Self::FunctionClosureCreation(_) => NodeKind::FunctionClosureCreation,
            Self::MethodClosureCreation(_) => NodeKind::MethodClosureCreation,
            Self::StaticMethodClosureCreation(_) => NodeKind::StaticMethodClosureCreation,
            Self::Constant(_) => NodeKind::Constant,
            Self::ConstantItem(_) => NodeKind::ConstantItem,
            Self::Construct(_) => NodeKind::Construct,
            Self::DieConstruct(_) => NodeKind::DieConstruct,
            Self::EmptyConstruct(_) => NodeKind::EmptyConstruct,
            Self::EvalConstruct(_) => NodeKind::EvalConstruct,
            Self::ExitConstruct(_) => NodeKind::ExitConstruct,
            Self::IncludeConstruct(_) => NodeKind::IncludeConstruct,
            Self::IncludeOnceConstruct(_) => NodeKind::IncludeOnceConstruct,
            Self::IssetConstruct(_) => NodeKind::IssetConstruct,
            Self::PrintConstruct(_) => NodeKind::PrintConstruct,
            Self::RequireConstruct(_) => NodeKind::RequireConstruct,
            Self::RequireOnceConstruct(_) => NodeKind::RequireOnceConstruct,
            Self::If(_) => NodeKind::If,
            Self::IfBody(_) => NodeKind::IfBody,
            Self::IfColonDelimitedBody(_) => NodeKind::IfColonDelimitedBody,
            Self::IfColonDelimitedBodyElseClause(_) => NodeKind::IfColonDelimitedBodyElseClause,
            Self::IfColonDelimitedBodyElseIfClause(_) => NodeKind::IfColonDelimitedBodyElseIfClause,
            Self::IfStatementBody(_) => NodeKind::IfStatementBody,
            Self::IfStatementBodyElseClause(_) => NodeKind::IfStatementBodyElseClause,
            Self::IfStatementBodyElseIfClause(_) => NodeKind::IfStatementBodyElseIfClause,
            Self::Match(_) => NodeKind::Match,
            Self::MatchArm(_) => NodeKind::MatchArm,
            Self::MatchDefaultArm(_) => NodeKind::MatchDefaultArm,
            Self::MatchExpressionArm(_) => NodeKind::MatchExpressionArm,
            Self::Switch(_) => NodeKind::Switch,
            Self::SwitchBody(_) => NodeKind::SwitchBody,
            Self::SwitchBraceDelimitedBody(_) => NodeKind::SwitchBraceDelimitedBody,
            Self::SwitchCase(_) => NodeKind::SwitchCase,
            Self::SwitchCaseSeparator(_) => NodeKind::SwitchCaseSeparator,
            Self::SwitchColonDelimitedBody(_) => NodeKind::SwitchColonDelimitedBody,
            Self::SwitchDefaultCase(_) => NodeKind::SwitchDefaultCase,
            Self::SwitchExpressionCase(_) => NodeKind::SwitchExpressionCase,
            Self::Declare(_) => NodeKind::Declare,
            Self::DeclareBody(_) => NodeKind::DeclareBody,
            Self::DeclareColonDelimitedBody(_) => NodeKind::DeclareColonDelimitedBody,
            Self::DeclareItem(_) => NodeKind::DeclareItem,
            Self::Echo(_) => NodeKind::Echo,
            Self::Expression(_) => NodeKind::Expression,
            Self::Binary(_) => NodeKind::Binary,
            Self::BinaryOperator(_) => NodeKind::BinaryOperator,
            Self::UnaryPrefix(_) => NodeKind::UnaryPrefix,
            Self::UnaryPrefixOperator(_) => NodeKind::UnaryPrefixOperator,
            Self::UnaryPostfix(_) => NodeKind::UnaryPostfix,
            Self::UnaryPostfixOperator(_) => NodeKind::UnaryPostfixOperator,
            Self::Parenthesized(_) => NodeKind::Parenthesized,
            Self::ArrowFunction(_) => NodeKind::ArrowFunction,
            Self::Closure(_) => NodeKind::Closure,
            Self::ClosureUseClause(_) => NodeKind::ClosureUseClause,
            Self::ClosureUseClauseVariable(_) => NodeKind::ClosureUseClauseVariable,
            Self::Function(_) => NodeKind::Function,
            Self::FunctionLikeParameter(_) => NodeKind::FunctionLikeParameter,
            Self::FunctionLikeParameterDefaultValue(_) => NodeKind::FunctionLikeParameterDefaultValue,
            Self::FunctionLikeParameterList(_) => NodeKind::FunctionLikeParameterList,
            Self::FunctionLikeReturnTypeHint(_) => NodeKind::FunctionLikeReturnTypeHint,
            Self::Global(_) => NodeKind::Global,
            Self::Goto(_) => NodeKind::Goto,
            Self::Label(_) => NodeKind::Label,
            Self::HaltCompiler(_) => NodeKind::HaltCompiler,
            Self::FullyQualifiedIdentifier(_) => NodeKind::FullyQualifiedIdentifier,
            Self::Identifier(_) => NodeKind::Identifier,
            Self::LocalIdentifier(_) => NodeKind::LocalIdentifier,
            Self::QualifiedIdentifier(_) => NodeKind::QualifiedIdentifier,
            Self::Inline(_) => NodeKind::Inline,
            Self::Instantiation(_) => NodeKind::Instantiation,
            Self::Keyword(_) => NodeKind::Keyword,
            Self::Literal(_) => NodeKind::Literal,
            Self::LiteralFloat(_) => NodeKind::LiteralFloat,
            Self::LiteralInteger(_) => NodeKind::LiteralInteger,
            Self::LiteralString(_) => NodeKind::LiteralString,
            Self::MagicConstant(_) => NodeKind::MagicConstant,
            Self::Modifier(_) => NodeKind::Modifier,
            Self::Namespace(_) => NodeKind::Namespace,
            Self::NamespaceBody(_) => NodeKind::NamespaceBody,
            Self::NamespaceImplicitBody(_) => NodeKind::NamespaceImplicitBody,
            Self::AssignmentOperation(_) => NodeKind::Assignment,
            Self::AssignmentOperator(_) => NodeKind::AssignmentOperator,
            Self::Conditional(_) => NodeKind::Conditional,
            Self::DoWhile(_) => NodeKind::DoWhile,
            Self::Foreach(_) => NodeKind::Foreach,
            Self::ForeachBody(_) => NodeKind::ForeachBody,
            Self::ForeachColonDelimitedBody(_) => NodeKind::ForeachColonDelimitedBody,
            Self::ForeachKeyValueTarget(_) => NodeKind::ForeachKeyValueTarget,
            Self::ForeachTarget(_) => NodeKind::ForeachTarget,
            Self::ForeachValueTarget(_) => NodeKind::ForeachValueTarget,
            Self::For(_) => NodeKind::For,
            Self::ForBody(_) => NodeKind::ForBody,
            Self::ForColonDelimitedBody(_) => NodeKind::ForColonDelimitedBody,
            Self::While(_) => NodeKind::While,
            Self::WhileBody(_) => NodeKind::WhileBody,
            Self::WhileColonDelimitedBody(_) => NodeKind::WhileColonDelimitedBody,
            Self::Break(_) => NodeKind::Break,
            Self::Continue(_) => NodeKind::Continue,
            Self::Return(_) => NodeKind::Return,
            Self::Static(_) => NodeKind::Static,
            Self::StaticAbstractItem(_) => NodeKind::StaticAbstractItem,
            Self::StaticConcreteItem(_) => NodeKind::StaticConcreteItem,
            Self::StaticItem(_) => NodeKind::StaticItem,
            Self::Try(_) => NodeKind::Try,
            Self::TryCatchClause(_) => NodeKind::TryCatchClause,
            Self::TryFinallyClause(_) => NodeKind::TryFinallyClause,
            Self::MaybeTypedUseItem(_) => NodeKind::MaybeTypedUseItem,
            Self::MixedUseItemList(_) => NodeKind::MixedUseItemList,
            Self::TypedUseItemList(_) => NodeKind::TypedUseItemList,
            Self::TypedUseItemSequence(_) => NodeKind::TypedUseItemSequence,
            Self::Use(_) => NodeKind::Use,
            Self::UseItem(_) => NodeKind::UseItem,
            Self::UseItemAlias(_) => NodeKind::UseItemAlias,
            Self::UseItemSequence(_) => NodeKind::UseItemSequence,
            Self::UseItems(_) => NodeKind::UseItems,
            Self::UseType(_) => NodeKind::UseType,
            Self::Yield(_) => NodeKind::Yield,
            Self::YieldFrom(_) => NodeKind::YieldFrom,
            Self::YieldPair(_) => NodeKind::YieldPair,
            Self::YieldValue(_) => NodeKind::YieldValue,
            Self::Statement(_) => NodeKind::Statement,
            Self::ExpressionStatement(_) => NodeKind::ExpressionStatement,
            Self::BracedExpressionStringPart(_) => NodeKind::BracedExpressionStringPart,
            Self::DocumentString(_) => NodeKind::DocumentString,
            Self::InterpolatedString(_) => NodeKind::InterpolatedString,
            Self::LiteralStringPart(_) => NodeKind::LiteralStringPart,
            Self::ShellExecuteString(_) => NodeKind::ShellExecuteString,
            Self::CompositeString(_) => NodeKind::CompositeString,
            Self::StringPart(_) => NodeKind::StringPart,
            Self::ClosingTag(_) => NodeKind::ClosingTag,
            Self::EchoOpeningTag(_) => NodeKind::EchoOpeningTag,
            Self::FullOpeningTag(_) => NodeKind::FullOpeningTag,
            Self::OpeningTag(_) => NodeKind::OpeningTag,
            Self::ShortOpeningTag(_) => NodeKind::ShortOpeningTag,
            Self::Terminator(_) => NodeKind::Terminator,
            Self::Throw(_) => NodeKind::Throw,
            Self::Hint(_) => NodeKind::Hint,
            Self::IntersectionHint(_) => NodeKind::IntersectionHint,
            Self::NullableHint(_) => NodeKind::NullableHint,
            Self::ParenthesizedHint(_) => NodeKind::ParenthesizedHint,
            Self::UnionHint(_) => NodeKind::UnionHint,
            Self::Unset(_) => NodeKind::Unset,
            Self::DirectVariable(_) => NodeKind::DirectVariable,
            Self::IndirectVariable(_) => NodeKind::IndirectVariable,
            Self::NestedVariable(_) => NodeKind::NestedVariable,
            Self::Variable(_) => NodeKind::Variable,
        }
    }

    #[inline]
    pub fn children(&self) -> Vec<Node<'a>> {
        match &self {
            Node::Program(node) => node.statements.iter().map(Node::Statement).collect(),
            Node::Access(node) => match &node {
                Access::Property(node) => vec![Node::PropertyAccess(node)],
                Access::NullSafeProperty(node) => vec![Node::NullSafePropertyAccess(node)],
                Access::StaticProperty(node) => vec![Node::StaticPropertyAccess(node)],
                Access::ClassConstant(node) => vec![Node::ClassConstantAccess(node)],
            },
            Node::ClassConstantAccess(node) => {
                vec![Node::Expression(&node.class), Node::ClassLikeConstantSelector(&node.constant)]
            }
            Node::NullSafePropertyAccess(node) => {
                vec![Node::Expression(&node.object), Node::ClassLikeMemberSelector(&node.property)]
            }
            Node::PropertyAccess(node) => {
                vec![Node::Expression(&node.object), Node::ClassLikeMemberSelector(&node.property)]
            }
            Node::StaticPropertyAccess(node) => {
                vec![Node::Expression(&node.class), Node::Variable(&node.property)]
            }
            Node::Argument(node) => match &node {
                Argument::Named(node) => vec![Node::NamedArgument(node)],
                Argument::Positional(node) => vec![Node::PositionalArgument(node)],
            },
            Node::ArgumentList(node) => node.arguments.iter().map(Node::Argument).collect(),
            Node::NamedArgument(node) => {
                vec![Node::LocalIdentifier(&node.name), Node::Expression(&node.value)]
            }
            Node::PositionalArgument(node) => vec![Node::Expression(&node.value)],
            Node::Array(node) => node.elements.iter().map(Node::ArrayElement).collect(),
            Node::ArrayAccess(node) => {
                vec![Node::Expression(&node.array), Node::Expression(&node.index)]
            }
            Node::ArrayAppend(node) => {
                vec![Node::Expression(&node.array)]
            }
            Node::ArrayElement(node) => match &node {
                ArrayElement::KeyValue(node) => vec![Node::KeyValueArrayElement(node)],
                ArrayElement::Missing(node) => vec![Node::MissingArrayElement(node)],
                ArrayElement::Value(node) => vec![Node::ValueArrayElement(node)],
                ArrayElement::Variadic(node) => vec![Node::VariadicArrayElement(node)],
            },
            Node::KeyValueArrayElement(node) => {
                vec![Node::Expression(&node.key), Node::Expression(&node.value)]
            }
            Node::LegacyArray(node) => node.elements.iter().map(Node::ArrayElement).collect(),
            Node::List(node) => node.elements.iter().map(Node::ArrayElement).collect(),
            Node::MissingArrayElement(_) => vec![],
            Node::ValueArrayElement(node) => vec![Node::Expression(&node.value)],
            Node::VariadicArrayElement(node) => vec![Node::Expression(&node.value)],
            Node::Attribute(node) => {
                let mut children = vec![Node::Identifier(&node.name)];
                if let Some(arguments) = &node.arguments {
                    children.push(Node::ArgumentList(arguments));
                }

                children
            }
            Node::AttributeList(node) => node.attributes.iter().map(Node::Attribute).collect(),
            Node::Block(node) => node.statements.iter().map(Node::Statement).collect(),
            Node::Call(node) => match node {
                Call::Function(node) => vec![Node::FunctionCall(node)],
                Call::Method(node) => vec![Node::MethodCall(node)],
                Call::NullSafeMethod(node) => vec![Node::NullSafeMethodCall(node)],
                Call::StaticMethod(node) => vec![Node::StaticMethodCall(node)],
            },
            Node::FunctionCall(node) => {
                vec![Node::Expression(&node.function), Node::ArgumentList(&node.arguments)]
            }
            Node::MethodCall(node) => {
                vec![
                    Node::Expression(&node.object),
                    Node::ClassLikeMemberSelector(&node.method),
                    Node::ArgumentList(&node.arguments),
                ]
            }
            Node::NullSafeMethodCall(node) => {
                vec![
                    Node::Expression(&node.object),
                    Node::ClassLikeMemberSelector(&node.method),
                    Node::ArgumentList(&node.arguments),
                ]
            }
            Node::StaticMethodCall(node) => {
                vec![
                    Node::Expression(&node.class),
                    Node::ClassLikeMemberSelector(&node.method),
                    Node::ArgumentList(&node.arguments),
                ]
            }
            Node::ClassLikeConstant(node) => {
                let mut children: Vec<_> = node.attributes.iter().map(Node::AttributeList).collect();

                children.extend(node.modifiers.iter().map(Node::Modifier));
                children.push(Node::Keyword(&node.r#const));
                if let Some(hint) = &node.hint {
                    children.push(Node::Hint(hint));
                }

                children.extend(node.items.iter().map(Node::ClassLikeConstantItem));
                children.push(Node::Terminator(&node.terminator));

                children
            }
            Node::ClassLikeConstantItem(node) => {
                vec![Node::LocalIdentifier(&node.name), Node::Expression(&node.value)]
            }
            Node::EnumCase(node) => {
                let mut children: Vec<_> = node.attributes.iter().map(Node::AttributeList).collect();

                children.push(Node::Keyword(&node.case));
                children.push(Node::EnumCaseItem(&node.item));
                children.push(Node::Terminator(&node.terminator));

                children
            }
            Node::EnumCaseBackedItem(node) => {
                vec![Node::LocalIdentifier(&node.name), Node::Expression(&node.value)]
            }
            Node::EnumCaseItem(node) => match &node {
                EnumCaseItem::Backed(node) => vec![Node::EnumCaseBackedItem(node)],
                EnumCaseItem::Unit(node) => vec![Node::EnumCaseUnitItem(node)],
            },
            Node::EnumCaseUnitItem(node) => vec![Node::LocalIdentifier(&node.name)],
            Node::Extends(node) => {
                let mut children = vec![];

                children.push(Node::Keyword(&node.extends));
                children.extend(node.types.iter().map(Node::Identifier));

                children
            }
            Node::Implements(node) => {
                let mut children = vec![];

                children.push(Node::Keyword(&node.implements));
                children.extend(node.types.iter().map(Node::Identifier));

                children
            }
            Node::ClassLikeConstantSelector(node) => match node {
                ClassLikeConstantSelector::Identifier(node) => vec![Node::LocalIdentifier(node)],
                ClassLikeConstantSelector::Expression(node) => {
                    vec![Node::ClassLikeMemberExpressionSelector(node)]
                }
            },
            Node::ClassLikeMember(node) => match node {
                ClassLikeMember::TraitUse(node) => vec![Node::TraitUse(node)],
                ClassLikeMember::Constant(node) => vec![Node::ClassLikeConstant(node)],
                ClassLikeMember::Property(node) => vec![Node::Property(node)],
                ClassLikeMember::EnumCase(node) => vec![Node::EnumCase(node)],
                ClassLikeMember::Method(node) => vec![Node::Method(node)],
            },
            Node::ClassLikeMemberExpressionSelector(node) => vec![Node::Expression(&node.expression)],
            Node::ClassLikeMemberSelector(node) => match node {
                ClassLikeMemberSelector::Identifier(node) => vec![Node::LocalIdentifier(node)],
                ClassLikeMemberSelector::Variable(node) => vec![Node::Variable(node)],
                ClassLikeMemberSelector::Expression(node) => vec![Node::ClassLikeMemberExpressionSelector(node)],
            },
            Node::Method(node) => {
                let mut children: Vec<Node> = vec![];

                children.extend(node.attributes.iter().map(Node::AttributeList));
                children.extend(node.modifiers.iter().map(Node::Modifier));
                children.push(Node::Keyword(&node.function));
                children.push(Node::LocalIdentifier(&node.name));
                children.push(Node::FunctionLikeParameterList(&node.parameters));
                children.extend(node.return_type_hint.iter().map(Node::FunctionLikeReturnTypeHint));
                children.push(Node::MethodBody(&node.body));

                children
            }
            Node::MethodAbstractBody(_) => vec![],
            Node::MethodBody(node) => match node {
                MethodBody::Abstract(node) => vec![Node::MethodAbstractBody(node)],
                MethodBody::Concrete(node) => vec![Node::Block(node)],
            },
            Node::HookedProperty(node) => {
                let mut children: Vec<Node> = vec![];

                children.extend(node.attributes.iter().map(Node::AttributeList));
                children.extend(node.var.iter().map(Node::Keyword));
                children.extend(node.modifiers.iter().map(Node::Modifier));
                children.extend(node.hint.iter().map(Node::Hint));
                children.push(Node::PropertyItem(&node.item));
                children.push(Node::PropertyHookList(&node.hooks));

                children
            }
            Node::PlainProperty(node) => {
                let mut children: Vec<Node> = vec![];

                children.extend(node.attributes.iter().map(Node::AttributeList));
                children.extend(node.var.iter().map(Node::Keyword));
                children.extend(node.modifiers.iter().map(Node::Modifier));
                children.extend(node.hint.iter().map(Node::Hint));
                children.extend(node.items.iter().map(Node::PropertyItem));

                children
            }
            Node::Property(node) => match node {
                Property::Plain(node) => vec![Node::PlainProperty(node)],
                Property::Hooked(node) => vec![Node::HookedProperty(node)],
            },
            Node::PropertyAbstractItem(node) => {
                vec![Node::DirectVariable(&node.variable)]
            }
            Node::PropertyConcreteItem(node) => {
                vec![Node::DirectVariable(&node.variable), Node::Expression(&node.value)]
            }
            Node::PropertyHook(node) => {
                let mut children: Vec<Node> = vec![];

                children.extend(node.attributes.iter().map(Node::AttributeList));
                children.extend(node.modifiers.iter().map(Node::Modifier));
                children.push(Node::LocalIdentifier(&node.name));
                children.extend(node.parameters.iter().map(Node::FunctionLikeParameterList));
                children.push(Node::PropertyHookBody(&node.body));

                children
            }
            Node::PropertyHookAbstractBody(_) => {
                vec![]
            }
            Node::PropertyHookBody(node) => vec![match node {
                PropertyHookBody::Abstract(node) => Node::PropertyHookAbstractBody(node),
                PropertyHookBody::Concrete(node) => Node::PropertyHookConcreteBody(node),
            }],
            Node::PropertyHookConcreteBody(node) => vec![match node {
                PropertyHookConcreteBody::Expression(node) => Node::PropertyHookConcreteExpressionBody(node),
                PropertyHookConcreteBody::Block(node) => Node::Block(node),
            }],
            Node::PropertyHookConcreteExpressionBody(node) => vec![Node::Expression(&node.expression)],
            Node::PropertyHookList(node) => node.hooks.iter().map(Node::PropertyHook).collect(),
            Node::PropertyItem(node) => match node {
                PropertyItem::Abstract(node) => vec![Node::PropertyAbstractItem(node)],
                PropertyItem::Concrete(node) => vec![Node::PropertyConcreteItem(node)],
            },
            Node::TraitUse(node) => {
                let mut children: Vec<Node> = vec![];

                children.push(Node::Keyword(&node.r#use));
                children.extend(node.trait_names.iter().map(Node::Identifier));
                children.push(Node::TraitUseSpecification(&node.specification));

                children
            }
            Node::TraitUseAbsoluteMethodReference(node) => {
                vec![Node::Identifier(&node.trait_name), Node::LocalIdentifier(&node.method_name)]
            }
            Node::TraitUseAbstractSpecification(node) => vec![Node::Terminator(&node.0)],
            Node::TraitUseAdaptation(node) => match node {
                TraitUseAdaptation::Precedence(adaptation) => {
                    let mut children = vec![
                        Node::TraitUseAbsoluteMethodReference(&adaptation.method_reference),
                        Node::Keyword(&adaptation.insteadof),
                    ];

                    children.extend(adaptation.trait_names.iter().map(Node::Identifier));
                    children.push(Node::Terminator(&adaptation.terminator));

                    children
                }
                TraitUseAdaptation::Alias(adaptation) => {
                    let mut children = vec![
                        Node::TraitUseMethodReference(&adaptation.method_reference),
                        Node::Keyword(&adaptation.r#as),
                    ];

                    if let Some(visibility) = &adaptation.visibility {
                        children.push(Node::Modifier(visibility));
                    }

                    if let Some(alias) = &adaptation.alias {
                        children.push(Node::LocalIdentifier(alias));
                    }

                    children.push(Node::Terminator(&adaptation.terminator));
                    children
                }
            },
            Node::TraitUseAliasAdaptation(node) => {
                let mut children =
                    vec![Node::TraitUseMethodReference(&node.method_reference), Node::Keyword(&node.r#as)];

                if let Some(visibility) = &node.visibility {
                    children.push(Node::Modifier(visibility));
                }

                if let Some(alias) = &node.alias {
                    children.push(Node::LocalIdentifier(alias));
                }

                children.push(Node::Terminator(&node.terminator));
                children
            }
            Node::TraitUseConcreteSpecification(node) => {
                node.adaptations.iter().map(Node::TraitUseAdaptation).collect()
            }
            Node::TraitUseMethodReference(node) => match node {
                TraitUseMethodReference::Identifier(identifier) => {
                    vec![Node::LocalIdentifier(identifier)]
                }
                TraitUseMethodReference::Absolute(reference) => {
                    vec![Node::TraitUseAbsoluteMethodReference(reference)]
                }
            },
            Node::TraitUsePrecedenceAdaptation(node) => {
                let mut children =
                    vec![Node::TraitUseAbsoluteMethodReference(&node.method_reference), Node::Keyword(&node.insteadof)];

                children.extend(node.trait_names.iter().map(Node::Identifier));
                children.push(Node::Terminator(&node.terminator));

                children
            }
            Node::TraitUseSpecification(node) => match node {
                TraitUseSpecification::Abstract(specification) => {
                    vec![Node::TraitUseAbstractSpecification(specification)]
                }
                TraitUseSpecification::Concrete(specification) => {
                    vec![Node::TraitUseConcreteSpecification(specification)]
                }
            },
            Node::AnonymousClass(node) => {
                let mut children = vec![Node::Keyword(&node.new)];
                children.extend(node.attributes.iter().map(Node::AttributeList));
                children.extend(node.modifiers.iter().map(Node::Modifier));
                children.push(Node::Keyword(&node.class));
                if let Some(arguments) = &node.arguments {
                    children.push(Node::ArgumentList(arguments));
                }
                children.extend(node.extends.iter().map(Node::Extends));
                children.extend(node.implements.iter().map(Node::Implements));
                children.extend(node.members.iter().map(Node::ClassLikeMember));

                children
            }
            Node::Class(node) => {
                let mut children = vec![];
                children.extend(node.attributes.iter().map(Node::AttributeList));
                children.extend(node.modifiers.iter().map(Node::Modifier));
                children.push(Node::Keyword(&node.class));
                children.push(Node::LocalIdentifier(&node.name));
                children.extend(node.extends.iter().map(Node::Extends));
                children.extend(node.implements.iter().map(Node::Implements));
                children.extend(node.members.iter().map(Node::ClassLikeMember));

                children
            }
            Node::Enum(node) => {
                let mut children = vec![];
                children.extend(node.attributes.iter().map(Node::AttributeList));
                children.push(Node::Keyword(&node.r#enum));
                children.push(Node::LocalIdentifier(&node.name));
                children.extend(node.backing_type_hint.iter().map(Node::EnumBackingTypeHint));
                children.extend(node.implements.iter().map(Node::Implements));
                children.extend(node.members.iter().map(Node::ClassLikeMember));

                children
            }
            Node::EnumBackingTypeHint(node) => {
                vec![Node::Hint(&node.hint)]
            }
            Node::Interface(node) => {
                let mut children = vec![];
                children.extend(node.attributes.iter().map(Node::AttributeList));
                children.push(Node::Keyword(&node.interface));
                children.push(Node::LocalIdentifier(&node.name));
                children.extend(node.extends.iter().map(Node::Extends));
                children.extend(node.members.iter().map(Node::ClassLikeMember));

                children
            }
            Node::Trait(node) => {
                let mut children = vec![];
                children.extend(node.attributes.iter().map(Node::AttributeList));
                children.push(Node::Keyword(&node.r#trait));
                children.push(Node::LocalIdentifier(&node.name));
                children.extend(node.members.iter().map(Node::ClassLikeMember));

                children
            }
            Node::Clone(node) => {
                vec![Node::Keyword(&node.clone), Node::Expression(&node.object)]
            }
            Node::ClosureCreation(node) => vec![match node {
                ClosureCreation::Function(node) => Node::FunctionClosureCreation(node),
                ClosureCreation::Method(node) => Node::MethodClosureCreation(node),
                ClosureCreation::StaticMethod(node) => Node::StaticMethodClosureCreation(node),
            }],
            Node::FunctionClosureCreation(node) => vec![Node::Expression(&node.function)],
            Node::MethodClosureCreation(node) => {
                vec![Node::Expression(&node.object), Node::ClassLikeMemberSelector(&node.method)]
            }
            Node::StaticMethodClosureCreation(node) => {
                vec![Node::Expression(&node.class), Node::ClassLikeMemberSelector(&node.method)]
            }
            Node::Constant(node) => {
                let mut children = vec![Node::Keyword(&node.r#const)];

                children.extend(node.items.iter().map(Node::ConstantItem));
                children.push(Node::Terminator(&node.terminator));

                children
            }
            Node::ConstantItem(node) => {
                vec![Node::LocalIdentifier(&node.name), Node::Expression(&node.value)]
            }
            Node::Construct(node) => vec![match node {
                Construct::Isset(node) => Node::IssetConstruct(node),
                Construct::Empty(node) => Node::EmptyConstruct(node),
                Construct::Eval(node) => Node::EvalConstruct(node),
                Construct::Include(node) => Node::IncludeConstruct(node),
                Construct::IncludeOnce(node) => Node::IncludeOnceConstruct(node),
                Construct::Require(node) => Node::RequireConstruct(node),
                Construct::RequireOnce(node) => Node::RequireOnceConstruct(node),
                Construct::Print(node) => Node::PrintConstruct(node),
                Construct::Exit(node) => Node::ExitConstruct(node),
                Construct::Die(node) => Node::DieConstruct(node),
            }],
            Node::IssetConstruct(node) => {
                let mut children = vec![Node::Keyword(&node.isset)];
                children.extend(node.values.iter().map(Node::Expression));

                children
            }
            Node::EmptyConstruct(node) => {
                vec![Node::Keyword(&node.empty), Node::Expression(&node.value)]
            }
            Node::EvalConstruct(node) => {
                vec![Node::Keyword(&node.eval), Node::Expression(&node.value)]
            }
            Node::IncludeConstruct(node) => {
                vec![Node::Keyword(&node.include), Node::Expression(&node.value)]
            }
            Node::IncludeOnceConstruct(node) => vec![Node::Keyword(&node.include_once), Node::Expression(&node.value)],
            Node::RequireConstruct(node) => {
                vec![Node::Keyword(&node.require), Node::Expression(&node.value)]
            }
            Node::RequireOnceConstruct(node) => vec![Node::Keyword(&node.require_once), Node::Expression(&node.value)],
            Node::PrintConstruct(node) => {
                vec![Node::Keyword(&node.print), Node::Expression(&node.value)]
            }
            Node::ExitConstruct(node) => {
                let mut children = vec![Node::Keyword(&node.exit)];
                if let Some(arguments) = &node.arguments {
                    children.push(Node::ArgumentList(arguments));
                }
                children
            }
            Node::DieConstruct(node) => {
                let mut children = vec![Node::Keyword(&node.die)];
                if let Some(arguments) = &node.arguments {
                    children.push(Node::ArgumentList(arguments));
                }
                children
            }
            Node::If(node) => {
                vec![Node::Keyword(&node.r#if), Node::Expression(&node.condition), Node::IfBody(&node.body)]
            }
            Node::IfBody(node) => match node {
                IfBody::Statement(statement_body) => vec![Node::IfStatementBody(statement_body)],
                IfBody::ColonDelimited(colon_body) => vec![Node::IfColonDelimitedBody(colon_body)],
            },
            Node::IfStatementBody(node) => {
                let mut children = vec![Node::Statement(&node.statement)];

                children.extend(node.else_if_clauses.iter().map(Node::IfStatementBodyElseIfClause));
                if let Some(else_clause) = &node.else_clause {
                    children.push(Node::IfStatementBodyElseClause(else_clause));
                }

                children
            }
            Node::IfStatementBodyElseIfClause(node) => {
                vec![Node::Keyword(&node.elseif), Node::Expression(&node.condition), Node::Statement(&node.statement)]
            }
            Node::IfStatementBodyElseClause(node) => {
                vec![Node::Keyword(&node.r#else), Node::Statement(&node.statement)]
            }
            Node::IfColonDelimitedBody(node) => {
                let mut children = node.statements.iter().map(Node::Statement).collect::<Vec<_>>();

                children.extend(node.else_if_clauses.iter().map(Node::IfColonDelimitedBodyElseIfClause));

                if let Some(else_clause) = &node.else_clause {
                    children.push(Node::IfColonDelimitedBodyElseClause(else_clause));
                }

                children.push(Node::Keyword(&node.endif));
                children.push(Node::Terminator(&node.terminator));

                children
            }
            Node::IfColonDelimitedBodyElseIfClause(node) => {
                let mut children = vec![Node::Keyword(&node.elseif), Node::Expression(&node.condition)];
                children.extend(node.statements.iter().map(Node::Statement));

                children
            }
            Node::IfColonDelimitedBodyElseClause(node) => {
                let mut children = vec![Node::Keyword(&node.r#else)];

                children.extend(node.statements.iter().map(Node::Statement));

                children
            }
            Node::Match(node) => {
                let mut children = vec![Node::Keyword(&node.r#match), Node::Expression(&node.expression)];
                children.extend(node.arms.iter().map(Node::MatchArm));

                children
            }
            Node::MatchArm(node) => match node {
                MatchArm::Expression(expr_arm) => vec![Node::MatchExpressionArm(expr_arm)],
                MatchArm::Default(default_arm) => vec![Node::MatchDefaultArm(default_arm)],
            },
            Node::MatchExpressionArm(node) => {
                let mut children = vec![];

                children.extend(node.conditions.iter().map(Node::Expression));
                children.push(Node::Expression(&node.expression));

                children
            }
            Node::MatchDefaultArm(node) => {
                vec![Node::Keyword(&node.default), Node::Expression(&node.expression)]
            }
            Node::Switch(node) => {
                vec![Node::Keyword(&node.switch), Node::Expression(&node.expression), Node::SwitchBody(&node.body)]
            }
            Node::SwitchBody(node) => match node {
                SwitchBody::BraceDelimited(body) => vec![Node::SwitchBraceDelimitedBody(body)],
                SwitchBody::ColonDelimited(body) => vec![Node::SwitchColonDelimitedBody(body)],
            },
            Node::SwitchBraceDelimitedBody(node) => {
                let mut children = vec![];

                if let Some(terminator) = &node.optional_terminator {
                    children.push(Node::Terminator(terminator));
                }

                children.extend(node.cases.iter().map(Node::SwitchCase));

                children
            }
            Node::SwitchColonDelimitedBody(node) => {
                let mut children = vec![];

                if let Some(terminator) = &node.optional_terminator {
                    children.push(Node::Terminator(terminator));
                }

                children.extend(node.cases.iter().map(Node::SwitchCase));
                children.push(Node::Keyword(&node.end_switch));
                children.push(Node::Terminator(&node.terminator));

                children
            }
            Node::SwitchCase(node) => match node {
                SwitchCase::Expression(expression_case) => vec![Node::SwitchExpressionCase(expression_case)],
                SwitchCase::Default(default_case) => vec![Node::SwitchDefaultCase(default_case)],
            },
            Node::SwitchExpressionCase(node) => {
                let mut children = vec![
                    Node::Keyword(&node.case),
                    Node::Expression(&node.expression),
                    Node::SwitchCaseSeparator(&node.separator),
                ];

                children.extend(node.statements.iter().map(Node::Statement));

                children
            }
            Node::SwitchDefaultCase(node) => {
                let mut children = vec![Node::Keyword(&node.default), Node::SwitchCaseSeparator(&node.separator)];
                children.extend(node.statements.iter().map(Node::Statement));

                children
            }
            Node::SwitchCaseSeparator(_) => vec![],
            Node::Declare(node) => {
                let mut children = vec![Node::Keyword(&node.declare)];

                children.extend(node.items.iter().map(Node::DeclareItem));
                children.push(Node::DeclareBody(&node.body));

                children
            }
            Node::DeclareBody(node) => match node {
                DeclareBody::Statement(statement) => vec![Node::Statement(statement)],
                DeclareBody::ColonDelimited(body) => vec![Node::DeclareColonDelimitedBody(body)],
            },
            Node::DeclareColonDelimitedBody(node) => {
                let mut children = node.statements.iter().map(Node::Statement).collect::<Vec<_>>();

                children.push(Node::Keyword(&node.end_declare));
                children.push(Node::Terminator(&node.terminator));

                children
            }
            Node::DeclareItem(node) => {
                vec![Node::LocalIdentifier(&node.name), Node::Expression(&node.value)]
            }
            Node::Echo(node) => {
                let mut children = vec![Node::Keyword(&node.echo)];
                children.extend(node.values.iter().map(Node::Expression));
                children.push(Node::Terminator(&node.terminator));

                children
            }
            Node::Parenthesized(node) => vec![Node::Expression(&node.expression)],
            Node::Expression(node) => vec![match node {
                Expression::Binary(node) => Node::Binary(node),
                Expression::UnaryPrefix(node) => Node::UnaryPrefix(node),
                Expression::UnaryPostfix(node) => Node::UnaryPostfix(node),
                Expression::Parenthesized(node) => Node::Parenthesized(node),
                Expression::Literal(node) => Node::Literal(node),
                Expression::CompositeString(node) => Node::CompositeString(node),
                Expression::AssignmentOperation(node) => Node::AssignmentOperation(node),
                Expression::Conditional(node) => Node::Conditional(node),
                Expression::Array(node) => Node::Array(node),
                Expression::LegacyArray(node) => Node::LegacyArray(node),
                Expression::List(node) => Node::List(node),
                Expression::ArrayAccess(node) => Node::ArrayAccess(node),
                Expression::ArrayAppend(node) => Node::ArrayAppend(node),
                Expression::AnonymousClass(node) => Node::AnonymousClass(node),
                Expression::Closure(node) => Node::Closure(node),
                Expression::ArrowFunction(node) => Node::ArrowFunction(node),
                Expression::Variable(node) => Node::Variable(node),
                Expression::Identifier(node) => Node::Identifier(node),
                Expression::Match(node) => Node::Match(node),
                Expression::Yield(node) => Node::Yield(node),
                Expression::Construct(node) => Node::Construct(node),
                Expression::Throw(node) => Node::Throw(node),
                Expression::Clone(node) => Node::Clone(node),
                Expression::Call(node) => Node::Call(node),
                Expression::Access(node) => Node::Access(node),
                Expression::ClosureCreation(node) => Node::ClosureCreation(node),
                Expression::Parent(node) => Node::Keyword(node),
                Expression::Static(node) => Node::Keyword(node),
                Expression::Self_(node) => Node::Keyword(node),
                Expression::Instantiation(node) => Node::Instantiation(node),
                Expression::MagicConstant(node) => Node::MagicConstant(node),
            }],
            Node::Binary(node) => {
                vec![Node::Expression(&node.lhs), Node::BinaryOperator(&node.operator), Node::Expression(&node.rhs)]
            }
            Node::BinaryOperator(operator) => match operator {
                BinaryOperator::Addition(_) => vec![],
                BinaryOperator::Subtraction(_) => vec![],
                BinaryOperator::Multiplication(_) => vec![],
                BinaryOperator::Division(_) => vec![],
                BinaryOperator::Modulo(_) => vec![],
                BinaryOperator::Exponentiation(_) => vec![],
                BinaryOperator::BitwiseAnd(_) => vec![],
                BinaryOperator::BitwiseOr(_) => vec![],
                BinaryOperator::BitwiseXor(_) => vec![],
                BinaryOperator::LeftShift(_) => vec![],
                BinaryOperator::RightShift(_) => vec![],
                BinaryOperator::NullCoalesce(_) => vec![],
                BinaryOperator::Equal(_) => vec![],
                BinaryOperator::NotEqual(_) => vec![],
                BinaryOperator::Identical(_) => vec![],
                BinaryOperator::NotIdentical(_) => vec![],
                BinaryOperator::AngledNotEqual(_) => vec![],
                BinaryOperator::LessThan(_) => vec![],
                BinaryOperator::LessThanOrEqual(_) => vec![],
                BinaryOperator::GreaterThan(_) => vec![],
                BinaryOperator::GreaterThanOrEqual(_) => vec![],
                BinaryOperator::Spaceship(_) => vec![],
                BinaryOperator::StringConcat(_) => vec![],
                BinaryOperator::And(_) => vec![],
                BinaryOperator::Or(_) => vec![],
                BinaryOperator::Elvis(_) => vec![],
                BinaryOperator::Instanceof(keyword) => vec![Node::Keyword(keyword)],
                BinaryOperator::LowAnd(keyword) => vec![Node::Keyword(keyword)],
                BinaryOperator::LowOr(keyword) => vec![Node::Keyword(keyword)],
                BinaryOperator::LowXor(keyword) => vec![Node::Keyword(keyword)],
            },
            Node::UnaryPrefix(node) => {
                vec![Node::UnaryPrefixOperator(&node.operator), Node::Expression(&node.operand)]
            }
            Node::UnaryPrefixOperator(operator) => match operator {
                UnaryPrefixOperator::ErrorControl(_) => vec![],
                UnaryPrefixOperator::Reference(_) => vec![],
                UnaryPrefixOperator::ArrayCast(_, _) => vec![],
                UnaryPrefixOperator::BoolCast(_, _) => vec![],
                UnaryPrefixOperator::BooleanCast(_, _) => vec![],
                UnaryPrefixOperator::DoubleCast(_, _) => vec![],
                UnaryPrefixOperator::RealCast(_, _) => vec![],
                UnaryPrefixOperator::FloatCast(_, _) => vec![],
                UnaryPrefixOperator::IntCast(_, _) => vec![],
                UnaryPrefixOperator::IntegerCast(_, _) => vec![],
                UnaryPrefixOperator::ObjectCast(_, _) => vec![],
                UnaryPrefixOperator::UnsetCast(_, _) => vec![],
                UnaryPrefixOperator::StringCast(_, _) => vec![],
                UnaryPrefixOperator::BinaryCast(_, _) => vec![],
                UnaryPrefixOperator::BitwiseNot(_) => vec![],
                UnaryPrefixOperator::Not(_) => vec![],
                UnaryPrefixOperator::PreIncrement(_) => vec![],
                UnaryPrefixOperator::PreDecrement(_) => vec![],
                UnaryPrefixOperator::Plus(_) => vec![],
                UnaryPrefixOperator::Negation(_) => vec![],
            },
            Node::UnaryPostfix(node) => {
                vec![Node::Expression(&node.operand), Node::UnaryPostfixOperator(&node.operator)]
            }
            Node::UnaryPostfixOperator(operator) => match operator {
                UnaryPostfixOperator::PostIncrement(_) => vec![],
                UnaryPostfixOperator::PostDecrement(_) => vec![],
            },
            Node::ArrowFunction(node) => {
                let mut children = vec![];

                children.extend(node.attributes.iter().map(Node::AttributeList));
                if let Some(r#static) = &node.r#static {
                    children.push(Node::Keyword(r#static));
                }
                children.push(Node::Keyword(&node.r#fn));
                children.push(Node::FunctionLikeParameterList(&node.parameters));
                if let Some(return_type_hint) = &node.return_type_hint {
                    children.push(Node::FunctionLikeReturnTypeHint(return_type_hint));
                }
                children.push(Node::Expression(&node.expression));

                children
            }
            Node::Closure(node) => {
                let mut children = vec![];

                children.extend(node.attributes.iter().map(Node::AttributeList));
                children.push(Node::Keyword(&node.function));
                children.push(Node::FunctionLikeParameterList(&node.parameters));
                if let Some(use_clause) = &node.use_clause {
                    children.push(Node::ClosureUseClause(use_clause));
                }
                if let Some(return_type_hint) = &node.return_type_hint {
                    children.push(Node::FunctionLikeReturnTypeHint(return_type_hint));
                }
                children.push(Node::Block(&node.body));

                children
            }
            Node::ClosureUseClause(node) => {
                let mut children = vec![Node::Keyword(&node.r#use)];
                children.extend(node.variables.iter().map(Node::ClosureUseClauseVariable));

                children
            }
            Node::ClosureUseClauseVariable(node) => vec![Node::DirectVariable(&node.variable)],
            Node::Function(node) => {
                let mut children = vec![];

                children.extend(node.attributes.iter().map(Node::AttributeList));
                children.push(Node::Keyword(&node.function));
                children.push(Node::LocalIdentifier(&node.name));
                children.push(Node::FunctionLikeParameterList(&node.parameters));
                if let Some(return_type_hint) = &node.return_type_hint {
                    children.push(Node::FunctionLikeReturnTypeHint(return_type_hint));
                }

                children.push(Node::Block(&node.body));

                children
            }
            Node::FunctionLikeParameterList(node) => node.parameters.iter().map(Node::FunctionLikeParameter).collect(),
            Node::FunctionLikeParameter(node) => {
                let mut children = vec![];

                children.extend(node.attributes.iter().map(Node::AttributeList));
                children.extend(node.modifiers.iter().map(Node::Modifier));
                if let Some(hint) = &node.hint {
                    children.push(Node::Hint(hint));
                }
                children.push(Node::DirectVariable(&node.variable));
                if let Some(default_value) = &node.default_value {
                    children.push(Node::FunctionLikeParameterDefaultValue(default_value));
                }

                if let Some(hooks) = &node.hooks {
                    children.push(Node::PropertyHookList(hooks));
                }

                children
            }
            Node::FunctionLikeParameterDefaultValue(node) => vec![Node::Expression(&node.value)],
            Node::FunctionLikeReturnTypeHint(hint) => vec![Node::Hint(&hint.hint)],
            Node::Global(node) => {
                let mut children: Vec<Node> = vec![];

                children.push(Node::Keyword(&node.r#global));
                children.extend(node.variables.iter().map(Node::Variable));

                children
            }
            Node::Goto(node) => {
                vec![Node::Keyword(&node.r#goto), Node::LocalIdentifier(&node.label)]
            }
            Node::Label(node) => {
                vec![Node::LocalIdentifier(&node.name)]
            }
            Node::HaltCompiler(node) => {
                vec![Node::Keyword(&node.halt_compiler)]
            }
            Node::FullyQualifiedIdentifier(_) => vec![],
            Node::Identifier(node) => vec![match node {
                Identifier::Local(node) => Node::LocalIdentifier(node),
                Identifier::Qualified(node) => Node::QualifiedIdentifier(node),
                Identifier::FullyQualified(node) => Node::FullyQualifiedIdentifier(node),
            }],
            Node::LocalIdentifier(_) => vec![],
            Node::QualifiedIdentifier(_) => vec![],
            Node::Inline(_) => vec![],
            Node::Instantiation(node) => {
                let mut children = vec![Node::Keyword(&node.new), Node::Expression(&node.class)];

                if let Some(arguments) = &node.arguments {
                    children.push(Node::ArgumentList(arguments));
                }

                children
            }
            Node::Keyword(_) => vec![],
            Node::Literal(node) => vec![match node {
                Literal::Float(node) => Node::LiteralFloat(node),
                Literal::Integer(node) => Node::LiteralInteger(node),
                Literal::String(node) => Node::LiteralString(node),
                Literal::True(node) => Node::Keyword(node),
                Literal::False(node) => Node::Keyword(node),
                Literal::Null(node) => Node::Keyword(node),
            }],
            Node::LiteralFloat(_) => vec![],
            Node::LiteralInteger(_) => vec![],
            Node::LiteralString(_) => vec![],
            Node::MagicConstant(node) => vec![match node {
                MagicConstant::Class(node) => Node::LocalIdentifier(node),
                MagicConstant::Directory(node) => Node::LocalIdentifier(node),
                MagicConstant::File(node) => Node::LocalIdentifier(node),
                MagicConstant::Function(node) => Node::LocalIdentifier(node),
                MagicConstant::Line(node) => Node::LocalIdentifier(node),
                MagicConstant::Method(node) => Node::LocalIdentifier(node),
                MagicConstant::Namespace(node) => Node::LocalIdentifier(node),
                MagicConstant::Trait(node) => Node::LocalIdentifier(node),
                MagicConstant::Property(node) => Node::LocalIdentifier(node),
            }],
            Node::Modifier(node) => vec![match node {
                Modifier::Abstract(node) => Node::Keyword(node),
                Modifier::Final(node) => Node::Keyword(node),
                Modifier::Private(node) => Node::Keyword(node),
                Modifier::Protected(node) => Node::Keyword(node),
                Modifier::Public(node) => Node::Keyword(node),
                Modifier::Static(node) => Node::Keyword(node),
                Modifier::Readonly(node) => Node::Keyword(node),
                Modifier::PrivateSet(node) => Node::Keyword(node),
                Modifier::ProtectedSet(node) => Node::Keyword(node),
                Modifier::PublicSet(node) => Node::Keyword(node),
            }],
            Node::Namespace(node) => {
                let mut children = vec![Node::Keyword(&node.r#namespace)];

                if let Some(name) = &node.name {
                    children.push(Node::Identifier(name));
                }

                children.push(Node::NamespaceBody(&node.body));

                children
            }
            Node::NamespaceBody(node) => {
                vec![match node {
                    NamespaceBody::BraceDelimited(node) => Node::Block(node),
                    NamespaceBody::Implicit(node) => Node::NamespaceImplicitBody(node),
                }]
            }
            Node::NamespaceImplicitBody(node) => {
                let mut children = vec![Node::Terminator(&node.terminator)];

                children.extend(node.statements.iter().map(Node::Statement));

                children
            }
            Node::AssignmentOperation(node) => {
                vec![Node::Expression(&node.lhs), Node::AssignmentOperator(&node.operator), Node::Expression(&node.rhs)]
            }
            Node::AssignmentOperator(_) => vec![],
            Node::Conditional(node) => {
                let mut children = vec![Node::Expression(&node.condition)];

                if let Some(then) = &node.then {
                    children.push(Node::Expression(then));
                }

                children.push(Node::Expression(&node.r#else));

                children
            }
            Node::DoWhile(node) => vec![
                Node::Keyword(&node.r#do),
                Node::Statement(&node.statement),
                Node::Keyword(&node.r#while),
                Node::Expression(&node.condition),
                Node::Terminator(&node.terminator),
            ],
            Node::Foreach(node) => vec![
                Node::Keyword(&node.r#foreach),
                Node::Expression(&node.expression),
                Node::Keyword(&node.r#as),
                Node::ForeachTarget(&node.target),
                Node::ForeachBody(&node.body),
            ],
            Node::ForeachBody(node) => vec![match node {
                ForeachBody::Statement(node) => Node::Statement(node),
                ForeachBody::ColonDelimited(node) => Node::ForeachColonDelimitedBody(node),
            }],
            Node::ForeachColonDelimitedBody(node) => {
                let mut children = node.statements.iter().map(Node::Statement).collect::<Vec<_>>();

                children.push(Node::Keyword(&node.end_foreach));
                children.push(Node::Terminator(&node.terminator));

                children
            }
            Node::ForeachKeyValueTarget(node) => {
                vec![Node::Expression(&node.key), Node::Expression(&node.value)]
            }
            Node::ForeachTarget(node) => vec![match node {
                ForeachTarget::KeyValue(node) => Node::ForeachKeyValueTarget(node),
                ForeachTarget::Value(node) => Node::ForeachValueTarget(node),
            }],
            Node::ForeachValueTarget(node) => vec![Node::Expression(&node.value)],
            Node::For(node) => {
                let mut children = vec![Node::Keyword(&node.r#for)];

                children.extend(node.initializations.iter().map(Node::Expression));
                children.extend(node.conditions.iter().map(Node::Expression));
                children.extend(node.increments.iter().map(Node::Expression));
                children.push(Node::ForBody(&node.body));

                children
            }
            Node::ForBody(node) => match node {
                ForBody::Statement(statement) => vec![Node::Statement(statement)],
                ForBody::ColonDelimited(body) => vec![Node::ForColonDelimitedBody(body)],
            },
            Node::ForColonDelimitedBody(node) => {
                let mut children = vec![];

                children.extend(node.statements.iter().map(Node::Statement));
                children.push(Node::Keyword(&node.end_for));
                children.push(Node::Terminator(&node.terminator));

                children
            }
            Node::While(node) => {
                let mut children = vec![Node::Keyword(&node.r#while)];

                children.push(Node::Expression(&node.condition));
                children.push(Node::WhileBody(&node.body));

                children
            }
            Node::WhileBody(node) => match node {
                WhileBody::Statement(statement) => vec![Node::Statement(statement)],
                WhileBody::ColonDelimited(body) => vec![Node::WhileColonDelimitedBody(body)],
            },
            Node::WhileColonDelimitedBody(node) => {
                let mut children = vec![];

                children.extend(node.statements.iter().map(Node::Statement));
                children.push(Node::Keyword(&node.end_while));
                children.push(Node::Terminator(&node.terminator));

                children
            }
            Node::Break(node) => {
                let mut children = vec![Node::Keyword(&node.r#break)];

                if let Some(level) = &node.level {
                    children.push(Node::Expression(level));
                }

                children.push(Node::Terminator(&node.terminator));

                children
            }
            Node::Continue(node) => {
                let mut children = vec![Node::Keyword(&node.r#continue)];

                if let Some(level) = &node.level {
                    children.push(Node::Expression(level));
                }

                children.push(Node::Terminator(&node.terminator));

                children
            }
            Node::Return(node) => {
                let mut children = vec![Node::Keyword(&node.r#return)];

                if let Some(value) = &node.value {
                    children.push(Node::Expression(value));
                }

                children.push(Node::Terminator(&node.terminator));

                children
            }
            Node::Static(node) => {
                let mut children = vec![Node::Keyword(&node.r#static)];

                children.extend(node.items.iter().map(Node::StaticItem));
                children.push(Node::Terminator(&node.terminator));

                children
            }
            Node::StaticItem(node) => vec![match node {
                StaticItem::Abstract(item) => Node::StaticAbstractItem(item),
                StaticItem::Concrete(item) => Node::StaticConcreteItem(item),
            }],
            Node::StaticAbstractItem(node) => {
                vec![Node::DirectVariable(&node.variable)]
            }
            Node::StaticConcreteItem(node) => {
                vec![Node::DirectVariable(&node.variable), Node::Expression(&node.value)]
            }
            Node::Try(node) => {
                let mut children = vec![];

                children.push(Node::Keyword(&node.r#try));
                children.push(Node::Block(&node.block));
                children.extend(node.catch_clauses.iter().map(Node::TryCatchClause));
                if let Some(finally) = &node.finally_clause {
                    children.push(Node::TryFinallyClause(finally));
                }

                children
            }
            Node::TryCatchClause(node) => {
                let mut children = vec![Node::Keyword(&node.r#catch)];

                children.push(Node::Hint(&node.hint));
                if let Some(variable) = &node.variable {
                    children.push(Node::DirectVariable(variable));
                }
                children.push(Node::Block(&node.block));

                children
            }
            Node::TryFinallyClause(node) => {
                vec![Node::Keyword(&node.r#finally), Node::Block(&node.block)]
            }
            Node::MaybeTypedUseItem(node) => {
                let mut children = vec![];
                if let Some(r#type) = &node.r#type {
                    children.push(Node::UseType(r#type));
                }

                children.push(Node::UseItem(&node.item));

                children
            }
            Node::MixedUseItemList(node) => {
                let mut children = vec![Node::Identifier(&node.namespace)];

                children.extend(node.items.iter().map(Node::MaybeTypedUseItem));

                children
            }
            Node::TypedUseItemList(node) => {
                let mut children = vec![Node::UseType(&node.r#type), Node::Identifier(&node.namespace)];

                children.extend(node.items.iter().map(Node::UseItem));

                children
            }
            Node::TypedUseItemSequence(node) => {
                let mut children = vec![Node::UseType(&node.r#type)];

                children.extend(node.items.iter().map(Node::UseItem));
                children
            }
            Node::Use(node) => {
                vec![Node::Keyword(&node.r#use), Node::UseItems(&node.items), Node::Terminator(&node.terminator)]
            }
            Node::UseItem(node) => {
                let mut result = vec![Node::Identifier(&node.name)];

                if let Some(alias) = &node.alias {
                    result.push(Node::UseItemAlias(alias));
                }

                result
            }
            Node::UseItemAlias(node) => {
                vec![Node::Keyword(&node.r#as), Node::LocalIdentifier(&node.identifier)]
            }
            Node::UseItemSequence(node) => node.items.iter().map(Node::UseItem).collect(),
            Node::UseItems(node) => vec![match node {
                UseItems::Sequence(node) => Node::UseItemSequence(node),
                UseItems::TypedList(node) => Node::TypedUseItemList(node),
                UseItems::MixedList(node) => Node::MixedUseItemList(node),
                UseItems::TypedSequence(node) => Node::TypedUseItemSequence(node),
            }],
            Node::UseType(node) => vec![match node {
                UseType::Const(node) => Node::Keyword(node),
                UseType::Function(node) => Node::Keyword(node),
            }],
            Node::Yield(node) => vec![match node {
                Yield::Value(node) => Node::YieldValue(node),
                Yield::Pair(node) => Node::YieldPair(node),
                Yield::From(node) => Node::YieldFrom(node),
            }],
            Node::YieldFrom(node) => {
                vec![Node::Keyword(&node.r#yield), Node::Keyword(&node.from), Node::Expression(&node.iterator)]
            }
            Node::YieldPair(node) => {
                vec![Node::Keyword(&node.r#yield), Node::Expression(&node.key), Node::Expression(&node.value)]
            }
            Node::YieldValue(node) => match &node.value {
                Some(value) => vec![Node::Keyword(&node.r#yield), Node::Expression(value)],
                None => vec![Node::Keyword(&node.r#yield)],
            },
            Node::Statement(node) => match &node {
                Statement::OpeningTag(node) => vec![Node::OpeningTag(node)],
                Statement::ClosingTag(node) => vec![Node::ClosingTag(node)],
                Statement::Inline(node) => vec![Node::Inline(node)],
                Statement::Namespace(node) => vec![Node::Namespace(node)],
                Statement::Use(node) => vec![Node::Use(node)],
                Statement::Class(node) => vec![Node::Class(node)],
                Statement::Interface(node) => vec![Node::Interface(node)],
                Statement::Trait(node) => vec![Node::Trait(node)],
                Statement::Enum(node) => vec![Node::Enum(node)],
                Statement::Block(node) => vec![Node::Block(node)],
                Statement::Constant(node) => vec![Node::Constant(node)],
                Statement::Function(node) => vec![Node::Function(node)],
                Statement::Declare(node) => vec![Node::Declare(node)],
                Statement::Goto(node) => vec![Node::Goto(node)],
                Statement::Label(node) => vec![Node::Label(node)],
                Statement::Try(node) => vec![Node::Try(node)],
                Statement::Foreach(node) => vec![Node::Foreach(node)],
                Statement::For(node) => vec![Node::For(node)],
                Statement::While(node) => vec![Node::While(node)],
                Statement::DoWhile(node) => vec![Node::DoWhile(node)],
                Statement::Continue(node) => vec![Node::Continue(node)],
                Statement::Break(node) => vec![Node::Break(node)],
                Statement::Switch(node) => vec![Node::Switch(node)],
                Statement::If(node) => vec![Node::If(node)],
                Statement::Return(node) => vec![Node::Return(node)],
                Statement::Expression(node) => vec![Node::ExpressionStatement(node)],
                Statement::Echo(node) => vec![Node::Echo(node)],
                Statement::Global(node) => vec![Node::Global(node)],
                Statement::Static(node) => vec![Node::Static(node)],
                Statement::HaltCompiler(node) => vec![Node::HaltCompiler(node)],
                Statement::Unset(node) => vec![Node::Unset(node)],
                Statement::Noop(_) => vec![],
            },
            Node::ExpressionStatement(node) => {
                vec![Node::Expression(&node.expression), Node::Terminator(&node.terminator)]
            }
            Node::BracedExpressionStringPart(node) => vec![Node::Expression(&node.expression)],
            Node::DocumentString(node) => node.parts.iter().map(Node::StringPart).collect(),
            Node::InterpolatedString(node) => node.parts.iter().map(Node::StringPart).collect(),
            Node::LiteralStringPart(_) => vec![],
            Node::ShellExecuteString(node) => node.parts.iter().map(Node::StringPart).collect(),
            Node::CompositeString(node) => vec![match node {
                CompositeString::ShellExecute(node) => Node::ShellExecuteString(node),
                CompositeString::Interpolated(node) => Node::InterpolatedString(node),
                CompositeString::Document(node) => Node::DocumentString(node),
            }],
            Node::StringPart(node) => vec![match node {
                StringPart::Literal(node) => Node::LiteralStringPart(node),
                StringPart::Expression(node) => Node::Expression(node),
                StringPart::BracedExpression(node) => Node::BracedExpressionStringPart(node),
            }],
            Node::ClosingTag(_) => vec![],
            Node::EchoOpeningTag(_) => vec![],
            Node::FullOpeningTag(_) => vec![],
            Node::OpeningTag(node) => match node {
                OpeningTag::Full(node) => vec![Node::FullOpeningTag(node)],
                OpeningTag::Short(node) => vec![Node::ShortOpeningTag(node)],
                OpeningTag::Echo(node) => vec![Node::EchoOpeningTag(node)],
            },
            Node::ShortOpeningTag(_) => vec![],
            Node::Terminator(node) => match node {
                Terminator::Semicolon(_) => vec![],
                Terminator::ClosingTag(closing_tag) => vec![Node::ClosingTag(closing_tag)],
                Terminator::TagPair(closing_tag, opening_tag) => {
                    vec![Node::ClosingTag(closing_tag), Node::OpeningTag(opening_tag)]
                }
            },
            Node::Throw(node) => vec![Node::Keyword(&node.throw), Node::Expression(&node.exception)],
            Node::Hint(node) => match &node {
                Hint::Identifier(identifier) => vec![Node::Identifier(identifier)],
                Hint::Parenthesized(parenthesized_hint) => vec![Node::ParenthesizedHint(parenthesized_hint)],
                Hint::Nullable(nullable_hint) => vec![Node::NullableHint(nullable_hint)],
                Hint::Union(union_hint) => vec![Node::UnionHint(union_hint)],
                Hint::Intersection(intersection_hint) => vec![Node::IntersectionHint(intersection_hint)],
                Hint::Null(keyword)
                | Hint::True(keyword)
                | Hint::False(keyword)
                | Hint::Array(keyword)
                | Hint::Callable(keyword)
                | Hint::Static(keyword)
                | Hint::Self_(keyword)
                | Hint::Parent(keyword) => vec![Node::Keyword(keyword)],
                Hint::Void(local_identifier)
                | Hint::Never(local_identifier)
                | Hint::Float(local_identifier)
                | Hint::Bool(local_identifier)
                | Hint::Integer(local_identifier)
                | Hint::String(local_identifier)
                | Hint::Object(local_identifier)
                | Hint::Mixed(local_identifier)
                | Hint::Iterable(local_identifier) => vec![Node::LocalIdentifier(local_identifier)],
            },
            Node::IntersectionHint(node) => vec![Node::Hint(&node.left), Node::Hint(&node.right)],
            Node::NullableHint(node) => vec![Node::Hint(&node.hint)],
            Node::ParenthesizedHint(node) => vec![Node::Hint(&node.hint)],
            Node::UnionHint(node) => vec![Node::Hint(&node.left), Node::Hint(&node.right)],
            Node::Unset(node) => {
                let mut children = vec![Node::Keyword(&node.unset)];
                children.extend(node.values.iter().map(Node::Expression));
                children.push(Node::Terminator(&node.terminator));

                children
            }
            Node::DirectVariable(_) => vec![],
            Node::IndirectVariable(node) => vec![Node::Expression(&node.expression)],
            Node::NestedVariable(node) => {
                vec![Node::Variable(&node.variable)]
            }
            Node::Variable(node) => match node {
                Variable::Direct(node) => vec![Node::DirectVariable(node)],
                Variable::Indirect(node) => vec![Node::IndirectVariable(node)],
                Variable::Nested(node) => vec![Node::NestedVariable(node)],
            },
        }
    }
}

impl HasSpan for Node<'_> {
    fn span(&self) -> Span {
        match self {
            Self::Program(node) => node.span(),
            Self::Access(node) => node.span(),
            Self::ClassConstantAccess(node) => node.span(),
            Self::NullSafePropertyAccess(node) => node.span(),
            Self::PropertyAccess(node) => node.span(),
            Self::StaticPropertyAccess(node) => node.span(),
            Self::Argument(node) => node.span(),
            Self::ArgumentList(node) => node.span(),
            Self::NamedArgument(node) => node.span(),
            Self::PositionalArgument(node) => node.span(),
            Self::Array(node) => node.span(),
            Self::ArrayAccess(node) => node.span(),
            Self::ArrayAppend(node) => node.span(),
            Self::ArrayElement(node) => node.span(),
            Self::KeyValueArrayElement(node) => node.span(),
            Self::LegacyArray(node) => node.span(),
            Self::List(node) => node.span(),
            Self::MissingArrayElement(node) => node.span(),
            Self::ValueArrayElement(node) => node.span(),
            Self::VariadicArrayElement(node) => node.span(),
            Self::Attribute(node) => node.span(),
            Self::AttributeList(node) => node.span(),
            Self::Block(node) => node.span(),
            Self::Call(node) => node.span(),
            Self::FunctionCall(node) => node.span(),
            Self::MethodCall(node) => node.span(),
            Self::NullSafeMethodCall(node) => node.span(),
            Self::StaticMethodCall(node) => node.span(),
            Self::ClassLikeConstant(node) => node.span(),
            Self::ClassLikeConstantItem(node) => node.span(),
            Self::EnumCase(node) => node.span(),
            Self::EnumCaseBackedItem(node) => node.span(),
            Self::EnumCaseItem(node) => node.span(),
            Self::EnumCaseUnitItem(node) => node.span(),
            Self::Extends(node) => node.span(),
            Self::Implements(node) => node.span(),
            Self::ClassLikeConstantSelector(node) => node.span(),
            Self::ClassLikeMember(node) => node.span(),
            Self::ClassLikeMemberExpressionSelector(node) => node.span(),
            Self::ClassLikeMemberSelector(node) => node.span(),
            Self::Method(node) => node.span(),
            Self::MethodAbstractBody(node) => node.span(),
            Self::MethodBody(node) => node.span(),
            Self::HookedProperty(node) => node.span(),
            Self::PlainProperty(node) => node.span(),
            Self::Property(node) => node.span(),
            Self::PropertyAbstractItem(node) => node.span(),
            Self::PropertyConcreteItem(node) => node.span(),
            Self::PropertyHook(node) => node.span(),
            Self::PropertyHookAbstractBody(node) => node.span(),
            Self::PropertyHookBody(node) => node.span(),
            Self::PropertyHookConcreteBody(node) => node.span(),
            Self::PropertyHookConcreteExpressionBody(node) => node.span(),
            Self::PropertyHookList(node) => node.span(),
            Self::PropertyItem(node) => node.span(),
            Self::TraitUse(node) => node.span(),
            Self::TraitUseAbsoluteMethodReference(node) => node.span(),
            Self::TraitUseAbstractSpecification(node) => node.span(),
            Self::TraitUseAdaptation(node) => node.span(),
            Self::TraitUseAliasAdaptation(node) => node.span(),
            Self::TraitUseConcreteSpecification(node) => node.span(),
            Self::TraitUseMethodReference(node) => node.span(),
            Self::TraitUsePrecedenceAdaptation(node) => node.span(),
            Self::TraitUseSpecification(node) => node.span(),
            Self::AnonymousClass(node) => node.span(),
            Self::Class(node) => node.span(),
            Self::Enum(node) => node.span(),
            Self::EnumBackingTypeHint(node) => node.span(),
            Self::Interface(node) => node.span(),
            Self::Trait(node) => node.span(),
            Self::Clone(node) => node.span(),
            Self::ClosureCreation(node) => node.span(),
            Self::FunctionClosureCreation(node) => node.span(),
            Self::MethodClosureCreation(node) => node.span(),
            Self::StaticMethodClosureCreation(node) => node.span(),
            Self::Constant(node) => node.span(),
            Self::ConstantItem(node) => node.span(),
            Self::Construct(node) => node.span(),
            Self::DieConstruct(node) => node.span(),
            Self::EmptyConstruct(node) => node.span(),
            Self::EvalConstruct(node) => node.span(),
            Self::ExitConstruct(node) => node.span(),
            Self::IncludeConstruct(node) => node.span(),
            Self::IncludeOnceConstruct(node) => node.span(),
            Self::IssetConstruct(node) => node.span(),
            Self::PrintConstruct(node) => node.span(),
            Self::RequireConstruct(node) => node.span(),
            Self::RequireOnceConstruct(node) => node.span(),
            Self::If(node) => node.span(),
            Self::IfBody(node) => node.span(),
            Self::IfColonDelimitedBody(node) => node.span(),
            Self::IfColonDelimitedBodyElseClause(node) => node.span(),
            Self::IfColonDelimitedBodyElseIfClause(node) => node.span(),
            Self::IfStatementBody(node) => node.span(),
            Self::IfStatementBodyElseClause(node) => node.span(),
            Self::IfStatementBodyElseIfClause(node) => node.span(),
            Self::Match(node) => node.span(),
            Self::MatchArm(node) => node.span(),
            Self::MatchDefaultArm(node) => node.span(),
            Self::MatchExpressionArm(node) => node.span(),
            Self::Switch(node) => node.span(),
            Self::SwitchBody(node) => node.span(),
            Self::SwitchBraceDelimitedBody(node) => node.span(),
            Self::SwitchCase(node) => node.span(),
            Self::SwitchCaseSeparator(node) => node.span(),
            Self::SwitchColonDelimitedBody(node) => node.span(),
            Self::SwitchDefaultCase(node) => node.span(),
            Self::SwitchExpressionCase(node) => node.span(),
            Self::Declare(node) => node.span(),
            Self::DeclareBody(node) => node.span(),
            Self::DeclareColonDelimitedBody(node) => node.span(),
            Self::DeclareItem(node) => node.span(),
            Self::Echo(node) => node.span(),
            Self::Expression(node) => node.span(),
            Self::Binary(node) => node.span(),
            Self::BinaryOperator(node) => node.span(),
            Self::UnaryPrefix(node) => node.span(),
            Self::UnaryPrefixOperator(node) => node.span(),
            Self::UnaryPostfix(node) => node.span(),
            Self::UnaryPostfixOperator(node) => node.span(),
            Self::Parenthesized(node) => node.span(),
            Self::ArrowFunction(node) => node.span(),
            Self::Closure(node) => node.span(),
            Self::ClosureUseClause(node) => node.span(),
            Self::ClosureUseClauseVariable(node) => node.span(),
            Self::Function(node) => node.span(),
            Self::FunctionLikeParameter(node) => node.span(),
            Self::FunctionLikeParameterDefaultValue(node) => node.span(),
            Self::FunctionLikeParameterList(node) => node.span(),
            Self::FunctionLikeReturnTypeHint(node) => node.span(),
            Self::Global(node) => node.span(),
            Self::Goto(node) => node.span(),
            Self::Label(node) => node.span(),
            Self::HaltCompiler(node) => node.span(),
            Self::FullyQualifiedIdentifier(node) => node.span(),
            Self::Identifier(node) => node.span(),
            Self::LocalIdentifier(node) => node.span(),
            Self::QualifiedIdentifier(node) => node.span(),
            Self::Inline(node) => node.span(),
            Self::Instantiation(node) => node.span(),
            Self::Keyword(node) => node.span(),
            Self::Literal(node) => node.span(),
            Self::LiteralFloat(node) => node.span(),
            Self::LiteralInteger(node) => node.span(),
            Self::LiteralString(node) => node.span(),
            Self::MagicConstant(node) => node.span(),
            Self::Modifier(node) => node.span(),
            Self::Namespace(node) => node.span(),
            Self::NamespaceBody(node) => node.span(),
            Self::NamespaceImplicitBody(node) => node.span(),
            Self::AssignmentOperation(node) => node.span(),
            Self::AssignmentOperator(node) => node.span(),
            Self::Conditional(node) => node.span(),
            Self::DoWhile(node) => node.span(),
            Self::Foreach(node) => node.span(),
            Self::ForeachBody(node) => node.span(),
            Self::ForeachColonDelimitedBody(node) => node.span(),
            Self::ForeachKeyValueTarget(node) => node.span(),
            Self::ForeachTarget(node) => node.span(),
            Self::ForeachValueTarget(node) => node.span(),
            Self::For(node) => node.span(),
            Self::ForBody(node) => node.span(),
            Self::ForColonDelimitedBody(node) => node.span(),
            Self::While(node) => node.span(),
            Self::WhileBody(node) => node.span(),
            Self::WhileColonDelimitedBody(node) => node.span(),
            Self::Break(node) => node.span(),
            Self::Continue(node) => node.span(),
            Self::Return(node) => node.span(),
            Self::Static(node) => node.span(),
            Self::StaticAbstractItem(node) => node.span(),
            Self::StaticConcreteItem(node) => node.span(),
            Self::StaticItem(node) => node.span(),
            Self::Try(node) => node.span(),
            Self::TryCatchClause(node) => node.span(),
            Self::TryFinallyClause(node) => node.span(),
            Self::MaybeTypedUseItem(node) => node.span(),
            Self::MixedUseItemList(node) => node.span(),
            Self::TypedUseItemList(node) => node.span(),
            Self::TypedUseItemSequence(node) => node.span(),
            Self::Use(node) => node.span(),
            Self::UseItem(node) => node.span(),
            Self::UseItemAlias(node) => node.span(),
            Self::UseItemSequence(node) => node.span(),
            Self::UseItems(node) => node.span(),
            Self::UseType(node) => node.span(),
            Self::Yield(node) => node.span(),
            Self::YieldFrom(node) => node.span(),
            Self::YieldPair(node) => node.span(),
            Self::YieldValue(node) => node.span(),
            Self::Statement(node) => node.span(),
            Self::ExpressionStatement(node) => node.span(),
            Self::BracedExpressionStringPart(node) => node.span(),
            Self::DocumentString(node) => node.span(),
            Self::InterpolatedString(node) => node.span(),
            Self::LiteralStringPart(node) => node.span(),
            Self::ShellExecuteString(node) => node.span(),
            Self::CompositeString(node) => node.span(),
            Self::StringPart(node) => node.span(),
            Self::ClosingTag(node) => node.span(),
            Self::EchoOpeningTag(node) => node.span(),
            Self::FullOpeningTag(node) => node.span(),
            Self::OpeningTag(node) => node.span(),
            Self::ShortOpeningTag(node) => node.span(),
            Self::Terminator(node) => node.span(),
            Self::Throw(node) => node.span(),
            Self::Hint(node) => node.span(),
            Self::IntersectionHint(node) => node.span(),
            Self::NullableHint(node) => node.span(),
            Self::ParenthesizedHint(node) => node.span(),
            Self::UnionHint(node) => node.span(),
            Self::Unset(node) => node.span(),
            Self::DirectVariable(node) => node.span(),
            Self::IndirectVariable(node) => node.span(),
            Self::NestedVariable(node) => node.span(),
            Self::Variable(node) => node.span(),
        }
    }
}
