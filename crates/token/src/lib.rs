use serde::Deserialize;
use serde::Serialize;
use strum::Display;

use mago_interner::StringIdentifier;
use mago_span::Span;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize, PartialOrd, Ord, Display)]
#[serde(tag = "type", content = "value")]
pub enum DocumentKind {
    Heredoc,
    Nowdoc,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize, PartialOrd, Ord, Display)]
#[serde(tag = "type", content = "value")]
pub enum Associativity {
    NonAssociative,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize, PartialOrd, Ord, Display)]
#[serde(tag = "type", content = "value")]
pub enum Precedence {
    Lowest,
    LowLogicalOr,
    LowLogicalXor,
    LowLogicalAnd,
    Print,
    Yield,
    YieldFrom,
    IncDec,
    KeyOr,
    KeyXor,
    KeyAnd,
    Assignment,
    ElvisOrConditional,
    NullCoalesce,
    Or,
    And,
    BitwiseOr,
    BitwiseXor,
    BitwiseAnd,
    Equality,
    Comparison,
    Concat,
    BitShift,
    AddSub,
    MulDivMod,
    Bang,
    Instanceof,
    Prefix,
    Pow,
    Clone,
    CallDim,
    New,
    ObjectAccess,
}

pub trait GetPrecedence {
    fn precedence(&self) -> Precedence;
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize, PartialOrd, Ord, Display)]
#[serde(tag = "type", content = "value")]
pub enum TokenKind {
    Whitespace,                  // ` `
    Eval,                        // `eval`
    Die,                         // `die`
    Self_,                       // `self`
    Parent,                      // `parent`
    Backtick,                    // `` ` ``
    DocumentStart(DocumentKind), // `<<<abc`, or `<<<'abc'`
    DocumentEnd,                 // `abc`
    From,                        // `from`
    Print,                       // `print`
    Dollar,                      // `$`
    HaltCompiler,                // `__halt_compiler`
    Readonly,                    // `readonly`
    Global,                      // `global`
    Abstract,                    // `abstract`
    Ampersand,                   // `&`
    AmpersandEqual,              // `&=`
    AmpersandAmpersand,          // `&&`
    AmpersandAmpersandEqual,     // `&&=`
    Array,                       // `array`
    ArrayCast,                   // `(array)`
    MinusGreaterThan,            // `->`
    QuestionMinusGreaterThan,    // `?->`
    At,                          // `@`
    As,                          // `as`
    Asterisk,                    // `*`
    HashLeftBracket,             // `#[`
    Bang,                        // `!`
    BangEqual,                   // `!=`
    LessThanGreaterThan,         // `<>`
    BangEqualEqual,              // `!==`
    LessThanEqualGreaterThan,    // `<=>`
    BoolCast,                    // `(bool)`
    BooleanCast,                 // `(boolean)`
    And,                         // `and`
    Or,                          // `or`
    Break,                       // `break`
    Callable,                    // `callable`
    Caret,                       // `^`
    CaretEqual,                  // `^=`
    Case,                        // `case`
    Catch,                       // `catch`
    Class,                       // `class`
    ClassConstant,               // `__CLASS__`
    TraitConstant,               // `__TRAIT__`
    FunctionConstant,            // `__FUNCTION__`
    MethodConstant,              // `__METHOD__`
    LineConstant,                // `__LINE__`
    FileConstant,                // `__FILE__`
    Clone,                       // `clone`
    MinusEqual,                  // `-=`
    CloseTag,                    // `?>`
    QuestionQuestion,            // `??`
    QuestionQuestionEqual,       // `??=`
    AsteriskEqual,               // `*=`
    Colon,                       // `:`
    Comma,                       // `,`
    SingleLineComment,           // `// comment`
    HashComment,                 // `# comment`
    MultiLineComment,            // `/* comment */`
    DocBlockComment,             // `/** comment */`
    Const,                       // `const`
    PartialLiteralString,        // `"string` or `'string`, missing closing quote
    LiteralString,               // `"string"` or `'string'`
    Continue,                    // `continue`
    Declare,                     // `declare`
    MinusMinus,                  // `--`
    Default,                     // `default`
    DirConstant,                 // `__DIR__`
    SlashEqual,                  // `/=`
    Do,                          // `do`
    DollarLeftBrace,             // `${`
    Dot,                         // `.`
    DotEqual,                    // `.=`
    EqualGreaterThan,            // `=>`
    DoubleCast,                  // `(double)`
    RealCast,                    // `(real)`
    FloatCast,                   // `(float)`
    ColonColon,                  // `::`
    EqualEqual,                  // `==`
    DoubleQuote,                 // `"`
    Else,                        // `else`
    Echo,                        // `echo`
    DotDotDot,                   // `...`
    ElseIf,                      // `elseif`
    Empty,                       // `empty`
    EndDeclare,                  // `enddeclare`
    EndFor,                      // `endfor`
    EndForeach,                  // `endforeach`
    EndIf,                       // `endif`
    EndSwitch,                   // `endswitch`
    EndWhile,                    // `endwhile`
    Enum,                        // `enum`
    Equal,                       // `=`
    Extends,                     // `extends`
    False,                       // `false`
    Final,                       // `final`
    Finally,                     // `finally`
    LiteralFloat,                // `1.0`
    Fn,                          // `fn`
    For,                         // `for`
    Foreach,                     // `foreach`
    FullyQualifiedIdentifier,    // `\Namespace\Class`
    Function,                    // `function`
    Goto,                        // `goto`
    GreaterThan,                 // `>`
    GreaterThanEqual,            // `>=`
    Identifier,                  // `name`
    If,                          // `if`
    Implements,                  // `implements`
    Include,                     // `include`
    IncludeOnce,                 // `include_once`
    PlusPlus,                    // `++`
    InlineText,                  // inline text outside of PHP tags, also referred to as "HTML"
    InlineShebang,               // `#!...`
    Instanceof,                  // `instanceof`
    Insteadof,                   // `insteadof`
    Exit,                        // `exit`
    Unset,                       // `unset`
    Isset,                       // `isset`
    List,                        // `list`
    LiteralInteger,              // `1`
    IntCast,                     // `(int)`
    IntegerCast,                 // `(integer)`
    Interface,                   // `interface`
    LeftBrace,                   // `{`
    LeftBracket,                 // `[`
    LeftParenthesis,             // `(`
    LeftShift,                   // `<<`
    LeftShiftEqual,              // `<<=`
    RightShift,                  // `>>`
    RightShiftEqual,             // `>>=`
    LessThan,                    // `<`
    LessThanEqual,               // `<=`
    Match,                       // `match`
    Minus,                       // `-`
    Namespace,                   // `namespace`
    NamespaceSeparator,          // `\`
    NamespaceConstant,           // `__NAMESPACE__`
    New,                         // `new`
    Null,                        // `null`
    ObjectCast,                  // `(object)`
    UnsetCast,                   // `(unset)`
    OpenTag,                     // `<?php`
    EchoTag,                     // `<?=`
    ShortOpenTag,                // `<?`
    Percent,                     // `%`
    PercentEqual,                // `%=`
    Pipe,                        // `|`
    PipeEqual,                   // `|=`
    Plus,                        // `+`
    PlusEqual,                   // `+=`
    AsteriskAsterisk,            // `**`
    AsteriskAsteriskEqual,       // `**=`
    Private,                     // `private`
    PrivateSet,                  // `private(set)`
    Protected,                   // `protected`
    ProtectedSet,                // `protected(set)`
    Public,                      // `public`
    PublicSet,                   // `public(set)`
    QualifiedIdentifier,         // `Namespace\Class`
    Question,                    // `?`
    QuestionColon,               // `?:`
    Require,                     // `require`
    RequireOnce,                 // `require_once`
    Return,                      // `return`
    RightBrace,                  // `}`
    RightBracket,                // `]`
    RightParenthesis,            // `)`
    Semicolon,                   // `;`
    Slash,                       // `/`
    Static,                      // `static`
    StringCast,                  // `(string)`
    BinaryCast,                  // `(binary)`
    StringPart,                  // `string` inside a double-quoted string, or a document string
    Switch,                      // `switch`
    Throw,                       // `throw`
    Trait,                       // `trait`
    EqualEqualEqual,             // `===`
    True,                        // `true`
    Try,                         // `try`
    Use,                         // `use`
    Var,                         // `var`
    Variable,                    // `$name`
    Yield,                       // `yield`
    While,                       // `while`
    Tilde,                       // `~`
    PipePipe,                    // `||`
    Xor,                         // `xor`
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub struct Token {
    pub kind: TokenKind,
    pub value: StringIdentifier,
    pub span: Span,
}

impl Precedence {
    #[inline(always)]
    pub fn infix(kind: &TokenKind) -> Precedence {
        match kind {
            T!["**"] => Precedence::Pow,
            T!["instanceof"] => Precedence::Instanceof,
            T!["*" | "/" | "%"] => Precedence::MulDivMod,
            T!["+" | "-"] => Precedence::AddSub,
            T!["<<"] | T![">>"] => Precedence::BitShift,
            T!["."] => Precedence::Concat,
            T!["<" | "<=" | ">" | ">="] => Precedence::Comparison,
            T!["==" | "!=" | "===" | "!==" | "<>" | "<=>"] => Precedence::Equality,
            T!["&"] => Precedence::BitwiseAnd,
            T!["^"] => Precedence::BitwiseXor,
            T!["|"] => Precedence::BitwiseOr,
            T!["&&"] => Precedence::And,
            T!["||"] => Precedence::Or,
            T!["??"] => Precedence::NullCoalesce,
            T!["?" | "?:"] => Precedence::ElvisOrConditional,
            T!["="
                | "+="
                | "-="
                | "*="
                | "**="
                | "/="
                | ".="
                | "&&="
                | "??="
                | "%="
                | "&="
                | "|="
                | "^="
                | "<<="
                | ">>="] => Precedence::Assignment,
            T!["yield"] => Precedence::Yield,
            T!["and"] => Precedence::KeyAnd,
            T!["or"] => Precedence::KeyOr,
            T!["xor"] => Precedence::KeyXor,
            _ => Precedence::Lowest,
        }
    }

    #[inline(always)]
    pub fn postfix(kind: &TokenKind) -> Self {
        match kind {
            T!["++" | "--"] => Self::Prefix,
            T!["(" | "["] => Self::CallDim,
            T!["->" | "?->" | "::"] => Self::ObjectAccess,
            _ => Self::Lowest,
        }
    }

    #[inline(always)]
    pub fn associativity(&self) -> Option<Associativity> {
        Some(match self {
            Self::Instanceof
            | Self::MulDivMod
            | Self::AddSub
            | Self::BitShift
            | Self::Concat
            | Self::BitwiseAnd
            | Self::BitwiseOr
            | Self::BitwiseXor
            | Self::And
            | Self::Or
            | Self::KeyAnd
            | Self::KeyOr
            | Self::KeyXor => Associativity::Left,
            Self::Pow | Self::NullCoalesce | Self::Assignment => Associativity::Right,
            Self::ElvisOrConditional | Self::Equality | Self::Comparison => Associativity::NonAssociative,
            _ => return None,
        })
    }
}

impl TokenKind {
    #[inline(always)]
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            TokenKind::Eval
                | TokenKind::Die
                | TokenKind::Empty
                | TokenKind::Isset
                | TokenKind::Unset
                | TokenKind::Exit
                | TokenKind::EndDeclare
                | TokenKind::EndSwitch
                | TokenKind::EndWhile
                | TokenKind::EndForeach
                | TokenKind::EndFor
                | TokenKind::EndIf
                | TokenKind::From
                | TokenKind::And
                | TokenKind::Or
                | TokenKind::Xor
                | TokenKind::Print
                | TokenKind::Readonly
                | TokenKind::Global
                | TokenKind::Match
                | TokenKind::Abstract
                | TokenKind::Array
                | TokenKind::As
                | TokenKind::Break
                | TokenKind::Case
                | TokenKind::Catch
                | TokenKind::Class
                | TokenKind::Clone
                | TokenKind::Continue
                | TokenKind::Const
                | TokenKind::Declare
                | TokenKind::Default
                | TokenKind::Do
                | TokenKind::Echo
                | TokenKind::ElseIf
                | TokenKind::Else
                | TokenKind::Enum
                | TokenKind::Extends
                | TokenKind::False
                | TokenKind::Finally
                | TokenKind::Final
                | TokenKind::Fn
                | TokenKind::Foreach
                | TokenKind::For
                | TokenKind::Function
                | TokenKind::Goto
                | TokenKind::If
                | TokenKind::IncludeOnce
                | TokenKind::Include
                | TokenKind::Implements
                | TokenKind::Interface
                | TokenKind::Instanceof
                | TokenKind::Namespace
                | TokenKind::New
                | TokenKind::Null
                | TokenKind::Private
                | TokenKind::PrivateSet
                | TokenKind::Protected
                | TokenKind::Public
                | TokenKind::RequireOnce
                | TokenKind::Require
                | TokenKind::Return
                | TokenKind::Static
                | TokenKind::Switch
                | TokenKind::Throw
                | TokenKind::Trait
                | TokenKind::True
                | TokenKind::Try
                | TokenKind::Use
                | TokenKind::Var
                | TokenKind::Yield
                | TokenKind::While
                | TokenKind::Insteadof
                | TokenKind::List
                | TokenKind::Self_
                | TokenKind::Parent
                | TokenKind::DirConstant
                | TokenKind::FileConstant
                | TokenKind::LineConstant
                | TokenKind::FunctionConstant
                | TokenKind::ClassConstant
                | TokenKind::MethodConstant
                | TokenKind::TraitConstant
                | TokenKind::NamespaceConstant
                | TokenKind::HaltCompiler
        )
    }

    #[inline(always)]
    pub fn is_infix(&self) -> bool {
        matches!(
            self,
            T!["**"
                | ">>="
                | "<<="
                | "^="
                | "&="
                | "|="
                | "%="
                | "**="
                | "and"
                | "or"
                | "xor"
                | "<=>"
                | "<<"
                | ">>"
                | "&"
                | "|"
                | "^"
                | "%"
                | "instanceof"
                | "*"
                | "/"
                | "+"
                | "-"
                | "."
                | "<"
                | ">"
                | "<="
                | ">="
                | "=="
                | "==="
                | "!="
                | "!=="
                | "<>"
                | "?"
                | "?:"
                | "&&"
                | "||"
                | "="
                | "+="
                | "-="
                | ".="
                | "??="
                | "/="
                | "*="
                | "??"]
        )
    }

    #[inline(always)]
    pub fn is_postfix(&self) -> bool {
        matches!(self, T!["++" | "--" | "(" | "[" | "->" | "?->" | "::"])
    }

    #[inline(always)]
    pub fn is_visibility_modifier(&self) -> bool {
        matches!(self, T!["public" | "protected" | "private" | "private(set)" | "protected(set)" | "public(set)"])
    }

    #[inline(always)]
    pub fn is_modifier(&self) -> bool {
        matches!(
            self,
            T!["public"
                | "protected"
                | "private"
                | "private(set)"
                | "protected(set)"
                | "public(set)"
                | "static"
                | "final"
                | "abstract"
                | "readonly"]
        )
    }

    #[inline(always)]
    pub fn is_identifier_maybe_soft_reserved(&self) -> bool {
        if let TokenKind::Identifier = self {
            true
        } else {
            self.is_soft_reserved_identifier()
        }
    }

    #[inline(always)]
    pub fn is_identifier_maybe_reserved(&self) -> bool {
        if let TokenKind::Identifier = self {
            true
        } else {
            self.is_reserved_identifier()
        }
    }

    #[inline(always)]
    pub fn is_soft_reserved_identifier(&self) -> bool {
        matches!(self, T!["parent" | "self" | "true" | "false" | "list" | "null" | "enum" | "from" | "readonly"],)
    }

    #[inline(always)]
    pub fn is_reserved_identifier(&self) -> bool {
        if self.is_soft_reserved_identifier() {
            return true;
        }

        matches!(
            self,
            T!["static"
                | "abstract"
                | "final"
                | "for"
                | "private"
                | "private(set)"
                | "protected"
                | "protected(set)"
                | "public"
                | "public(set)"
                | "include"
                | "include_once"
                | "eval"
                | "require"
                | "require_once"
                | "or"
                | "xor"
                | "and"
                | "instanceof"
                | "new"
                | "clone"
                | "exit"
                | "die"
                | "if"
                | "elseif"
                | "else"
                | "endif"
                | "echo"
                | "do"
                | "while"
                | "endwhile"
                | "endfor"
                | "foreach"
                | "endforeach"
                | "declare"
                | "enddeclare"
                | "as"
                | "try"
                | "catch"
                | "finally"
                | "throw"
                | "use"
                | "insteadof"
                | "global"
                | "var"
                | "unset"
                | "isset"
                | "empty"
                | "continue"
                | "goto"
                | "function"
                | "const"
                | "return"
                | "print"
                | "yield"
                | "list"
                | "switch"
                | "endswitch"
                | "case"
                | "default"
                | "break"
                | "array"
                | "callable"
                | "extends"
                | "implements"
                | "namespace"
                | "trait"
                | "interface"
                | "class"
                | "__CLASS__"
                | "__TRAIT__"
                | "__FUNCTION__"
                | "__METHOD__"
                | "__LINE__"
                | "__FILE__"
                | "__DIR__"
                | "__NAMESPACE__"
                | "__halt_compiler"
                | "fn"
                | "match"]
        )
    }

    #[inline(always)]
    pub fn is_literal(&self) -> bool {
        matches!(
            self,
            T!["true" | "false" | "null" | LiteralFloat | LiteralInteger | LiteralString | PartialLiteralString]
        )
    }

    #[inline(always)]
    pub fn is_magic_constant(&self) -> bool {
        matches!(
            self,
            T!["__CLASS__"
                | "__DIR__"
                | "__FILE__"
                | "__FUNCTION__"
                | "__LINE__"
                | "__METHOD__"
                | "__NAMESPACE__"
                | "__TRAIT__"]
        )
    }

    #[inline(always)]
    pub const fn is_cast(&self) -> bool {
        matches!(
            self,
            T!["(string)"
                | "(binary)"
                | "(int)"
                | "(integer)"
                | "(float)"
                | "(double)"
                | "(real)"
                | "(bool)"
                | "(boolean)"
                | "(array)"
                | "(object)"
                | "(unset)"]
        )
    }

    #[inline(always)]
    pub const fn is_unary_prefix(&self) -> bool {
        if self.is_cast() {
            return true;
        }

        matches!(self, T!["@" | "!" | "~" | "-" | "+" | "++" | "--" | "&"])
    }

    #[inline(always)]
    pub fn is_trivia(&self) -> bool {
        matches!(self, T![SingleLineComment | MultiLineComment | DocBlockComment | HashComment | Whitespace])
    }

    #[inline(always)]
    pub fn is_comment(&self) -> bool {
        matches!(self, T![SingleLineComment | MultiLineComment | DocBlockComment | HashComment])
    }

    #[inline(always)]
    pub fn is_construct(&self) -> bool {
        matches!(
            self,
            T!["isset"
                | "empty"
                | "eval"
                | "include"
                | "include_once"
                | "require"
                | "require_once"
                | "print"
                | "unset"
                | "exit"
                | "die"]
        )
    }
}

impl Token {
    pub fn new(kind: TokenKind, value: StringIdentifier, span: Span) -> Self {
        Self { kind, value, span }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?} {}", self.kind, self.span)
    }
}

#[macro_export]
macro_rules! T {
    ("eval") => {
        $crate::TokenKind::Eval
    };
    ("die") => {
        $crate::TokenKind::Die
    };
    ("self") => {
        $crate::TokenKind::Self_
    };
    ("parent") => {
        $crate::TokenKind::Parent
    };
    ("`") => {
        $crate::TokenKind::Backtick
    };
    ("<<<") => {
        $crate::TokenKind::DocumentStart(_)
    };
    (">>>") => {
        $crate::TokenKind::DocumentEnd
    };
    ("from") => {
        $crate::TokenKind::From
    };
    ("print") => {
        $crate::TokenKind::Print
    };
    ("$") => {
        $crate::TokenKind::Dollar
    };
    ("__halt_compiler") => {
        $crate::TokenKind::HaltCompiler
    };
    ("readonly") => {
        $crate::TokenKind::Readonly
    };
    ("global") => {
        $crate::TokenKind::Global
    };
    ("abstract") => {
        $crate::TokenKind::Abstract
    };
    ("&") => {
        $crate::TokenKind::Ampersand
    };
    ("&=") => {
        $crate::TokenKind::AmpersandEqual
    };
    ("&&") => {
        $crate::TokenKind::AmpersandAmpersand
    };
    ("&&=") => {
        $crate::TokenKind::AmpersandAmpersandEqual
    };
    ("array") => {
        $crate::TokenKind::Array
    };
    ("(array)") => {
        $crate::TokenKind::ArrayCast
    };
    ("->") => {
        $crate::TokenKind::MinusGreaterThan
    };
    ("?->") => {
        $crate::TokenKind::QuestionMinusGreaterThan
    };
    ("@") => {
        $crate::TokenKind::At
    };
    ("as") => {
        $crate::TokenKind::As
    };
    ("*") => {
        $crate::TokenKind::Asterisk
    };
    ("#[") => {
        $crate::TokenKind::HashLeftBracket
    };
    ("!") => {
        $crate::TokenKind::Bang
    };
    ("!=") => {
        $crate::TokenKind::BangEqual
    };
    ("<>") => {
        $crate::TokenKind::LessThanGreaterThan
    };
    ("!==") => {
        $crate::TokenKind::BangEqualEqual
    };
    ("<=>") => {
        $crate::TokenKind::LessThanEqualGreaterThan
    };
    ("(bool)") => {
        $crate::TokenKind::BoolCast
    };
    ("(boolean)") => {
        $crate::TokenKind::BooleanCast
    };
    ("and") => {
        $crate::TokenKind::And
    };
    ("or") => {
        $crate::TokenKind::Or
    };
    ("break") => {
        $crate::TokenKind::Break
    };
    ("callable") => {
        $crate::TokenKind::Callable
    };
    ("^") => {
        $crate::TokenKind::Caret
    };
    ("^=") => {
        $crate::TokenKind::CaretEqual
    };
    ("case") => {
        $crate::TokenKind::Case
    };
    ("catch") => {
        $crate::TokenKind::Catch
    };
    ("class") => {
        $crate::TokenKind::Class
    };
    ("__CLASS__") => {
        $crate::TokenKind::ClassConstant
    };
    ("__TRAIT__") => {
        $crate::TokenKind::TraitConstant
    };
    ("__FUNCTION__") => {
        $crate::TokenKind::FunctionConstant
    };
    ("__METHOD__") => {
        $crate::TokenKind::MethodConstant
    };
    ("__LINE__") => {
        $crate::TokenKind::LineConstant
    };
    ("__FILE__") => {
        $crate::TokenKind::FileConstant
    };
    ("clone") => {
        $crate::TokenKind::Clone
    };
    ("-=") => {
        $crate::TokenKind::MinusEqual
    };
    ("?>") => {
        $crate::TokenKind::CloseTag
    };
    ("??") => {
        $crate::TokenKind::QuestionQuestion
    };
    ("??=") => {
        $crate::TokenKind::QuestionQuestionEqual
    };
    ("*=") => {
        $crate::TokenKind::AsteriskEqual
    };
    (":") => {
        $crate::TokenKind::Colon
    };
    (",") => {
        $crate::TokenKind::Comma
    };
    ("// comment") => {
        $crate::TokenKind::SingleLineComment
    };
    ("# comment") => {
        $crate::TokenKind::HashComment
    };
    ("/* comment */") => {
        $crate::TokenKind::MultiLineComment
    };
    ("/** comment */") => {
        $crate::TokenKind::DocBlockComment
    };
    ("const") => {
        $crate::TokenKind::Const
    };
    ("continue") => {
        $crate::TokenKind::Continue
    };
    ("declare") => {
        $crate::TokenKind::Declare
    };
    ("--") => {
        $crate::TokenKind::MinusMinus
    };
    ("default") => {
        $crate::TokenKind::Default
    };
    ("__DIR__") => {
        $crate::TokenKind::DirConstant
    };
    ("/=") => {
        $crate::TokenKind::SlashEqual
    };
    ("do") => {
        $crate::TokenKind::Do
    };
    ("${") => {
        $crate::TokenKind::DollarLeftBrace
    };
    (".") => {
        $crate::TokenKind::Dot
    };
    (".=") => {
        $crate::TokenKind::DotEqual
    };
    ("=>") => {
        $crate::TokenKind::EqualGreaterThan
    };
    ("(double)") => {
        $crate::TokenKind::DoubleCast
    };
    ("(real)") => {
        $crate::TokenKind::RealCast
    };
    ("(float)") => {
        $crate::TokenKind::FloatCast
    };
    ("::") => {
        $crate::TokenKind::ColonColon
    };
    ("==") => {
        $crate::TokenKind::EqualEqual
    };
    ("\"") => {
        $crate::TokenKind::DoubleQuote
    };
    ("else") => {
        $crate::TokenKind::Else
    };
    ("echo") => {
        $crate::TokenKind::Echo
    };
    ("...") => {
        $crate::TokenKind::DotDotDot
    };
    ("elseif") => {
        $crate::TokenKind::ElseIf
    };
    ("empty") => {
        $crate::TokenKind::Empty
    };
    ("enddeclare") => {
        $crate::TokenKind::EndDeclare
    };
    ("endfor") => {
        $crate::TokenKind::EndFor
    };
    ("endforeach") => {
        $crate::TokenKind::EndForeach
    };
    ("endif") => {
        $crate::TokenKind::EndIf
    };
    ("endswitch") => {
        $crate::TokenKind::EndSwitch
    };
    ("endwhile") => {
        $crate::TokenKind::EndWhile
    };
    ("enum") => {
        $crate::TokenKind::Enum
    };
    ("=") => {
        $crate::TokenKind::Equal
    };
    ("extends") => {
        $crate::TokenKind::Extends
    };
    ("false") => {
        $crate::TokenKind::False
    };
    ("final") => {
        $crate::TokenKind::Final
    };
    ("finally") => {
        $crate::TokenKind::Finally
    };
    ("fn") => {
        $crate::TokenKind::Fn
    };
    ("for") => {
        $crate::TokenKind::For
    };
    ("foreach") => {
        $crate::TokenKind::Foreach
    };
    ("\\Fully\\Qualified\\Identifier") => {
        $crate::TokenKind::FullyQualifiedIdentifier
    };
    ("function") => {
        $crate::TokenKind::Function
    };
    ("goto") => {
        $crate::TokenKind::Goto
    };
    (">") => {
        $crate::TokenKind::GreaterThan
    };
    (">=") => {
        $crate::TokenKind::GreaterThanEqual
    };
    ("Identifier") => {
        $crate::TokenKind::Identifier
    };
    ("if") => {
        $crate::TokenKind::If
    };
    ("implements") => {
        $crate::TokenKind::Implements
    };
    ("include") => {
        $crate::TokenKind::Include
    };
    ("include_once") => {
        $crate::TokenKind::IncludeOnce
    };
    ("++") => {
        $crate::TokenKind::PlusPlus
    };
    ("instanceof") => {
        $crate::TokenKind::Instanceof
    };
    ("insteadof") => {
        $crate::TokenKind::Insteadof
    };
    ("exit") => {
        $crate::TokenKind::Exit
    };
    ("unset") => {
        $crate::TokenKind::Unset
    };
    ("isset") => {
        $crate::TokenKind::Isset
    };
    ("list") => {
        $crate::TokenKind::List
    };
    ("(int)") => {
        $crate::TokenKind::IntCast
    };
    ("(integer)") => {
        $crate::TokenKind::IntegerCast
    };
    ("interface") => {
        $crate::TokenKind::Interface
    };
    ("{") => {
        $crate::TokenKind::LeftBrace
    };
    ("[") => {
        $crate::TokenKind::LeftBracket
    };
    ("(") => {
        $crate::TokenKind::LeftParenthesis
    };
    (")") => {
        $crate::TokenKind::RightParenthesis
    };
    ("<<") => {
        $crate::TokenKind::LeftShift
    };
    ("<<=") => {
        $crate::TokenKind::LeftShiftEqual
    };
    (">>") => {
        $crate::TokenKind::RightShift
    };
    (">>=") => {
        $crate::TokenKind::RightShiftEqual
    };
    ("<") => {
        $crate::TokenKind::LessThan
    };
    ("<=") => {
        $crate::TokenKind::LessThanEqual
    };
    ("match") => {
        $crate::TokenKind::Match
    };
    ("-") => {
        $crate::TokenKind::Minus
    };
    ("namespace") => {
        $crate::TokenKind::Namespace
    };
    ("\\") => {
        $crate::TokenKind::NamespaceSeparator
    };
    ("__NAMESPACE__") => {
        $crate::TokenKind::NamespaceConstant
    };
    ("new") => {
        $crate::TokenKind::New
    };
    ("null") => {
        $crate::TokenKind::Null
    };
    ("(object)") => {
        $crate::TokenKind::ObjectCast
    };
    ("(unset)") => {
        $crate::TokenKind::UnsetCast
    };
    ("<?php") => {
        $crate::TokenKind::OpenTag
    };
    ("<?=") => {
        $crate::TokenKind::EchoTag
    };
    ("<?") => {
        $crate::TokenKind::ShortOpenTag
    };
    ("%") => {
        $crate::TokenKind::Percent
    };
    ("%=") => {
        $crate::TokenKind::PercentEqual
    };
    ("|") => {
        $crate::TokenKind::Pipe
    };
    ("|=") => {
        $crate::TokenKind::PipeEqual
    };
    ("+") => {
        $crate::TokenKind::Plus
    };
    ("+=") => {
        $crate::TokenKind::PlusEqual
    };
    ("**") => {
        $crate::TokenKind::AsteriskAsterisk
    };
    ("**=") => {
        $crate::TokenKind::AsteriskAsteriskEqual
    };
    ("private(set)") => {
        $crate::TokenKind::PrivateSet
    };
    ("private") => {
        $crate::TokenKind::Private
    };
    ("protected") => {
        $crate::TokenKind::Protected
    };
    ("protected(set)") => {
        $crate::TokenKind::ProtectedSet
    };
    ("public") => {
        $crate::TokenKind::Public
    };
    ("public(set)") => {
        $crate::TokenKind::PublicSet
    };
    ("Qualified\\Identifier") => {
        $crate::TokenKind::QualifiedIdentifier
    };
    ("?") => {
        $crate::TokenKind::Question
    };
    ("?:") => {
        $crate::TokenKind::QuestionColon
    };
    ("require") => {
        $crate::TokenKind::Require
    };
    ("require_once") => {
        $crate::TokenKind::RequireOnce
    };
    ("return") => {
        $crate::TokenKind::Return
    };
    ("}") => {
        $crate::TokenKind::RightBrace
    };
    ("]") => {
        $crate::TokenKind::RightBracket
    };
    (";") => {
        $crate::TokenKind::Semicolon
    };
    ("/") => {
        $crate::TokenKind::Slash
    };
    ("static") => {
        $crate::TokenKind::Static
    };
    ("(string)") => {
        $crate::TokenKind::StringCast
    };
    ("(binary)") => {
        $crate::TokenKind::BinaryCast
    };
    ("switch") => {
        $crate::TokenKind::Switch
    };
    ("throw") => {
        $crate::TokenKind::Throw
    };
    ("trait") => {
        $crate::TokenKind::Trait
    };
    ("===") => {
        $crate::TokenKind::EqualEqualEqual
    };
    ("true") => {
        $crate::TokenKind::True
    };
    ("try") => {
        $crate::TokenKind::Try
    };
    ("use") => {
        $crate::TokenKind::Use
    };
    ("var") => {
        $crate::TokenKind::Var
    };
    ("$variable") => {
        $crate::TokenKind::Variable
    };
    ("yield") => {
        $crate::TokenKind::Yield
    };
    ("while") => {
        $crate::TokenKind::While
    };
    ("~") => {
        $crate::TokenKind::Tilde
    };
    ("||") => {
        $crate::TokenKind::PipePipe
    };
    ("xor") => {
        $crate::TokenKind::Xor
    };
    ($name:ident) => {
        $crate::TokenKind::$name
    };
    ($first:tt | $($rest:tt)+) => {
        $crate::T![$first] | $crate::T![$($rest)+]
    };
    ($($kind:tt),+ $(,)?) => {
        &[$($crate::T![$kind]),+]
    };
}
