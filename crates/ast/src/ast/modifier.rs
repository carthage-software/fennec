use mago_interner::ThreadedInterner;
use serde::Deserialize;
use serde::Serialize;
use strum::Display;

use mago_span::HasSpan;
use mago_span::Span;

use crate::ast::keyword::Keyword;
use crate::sequence::Sequence;

/// Represents a modifier statement.
///
/// # Examples
///
/// ```php
/// final class Foo {}
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, PartialOrd, Ord, Display)]
#[serde(tag = "type", content = "value")]
pub enum Modifier {
    Static(Keyword),
    Final(Keyword),
    Abstract(Keyword),
    Readonly(Keyword),
    Public(Keyword),
    PublicSet(Keyword),
    Protected(Keyword),
    ProtectedSet(Keyword),
    Private(Keyword),
    PrivateSet(Keyword),
}

impl Modifier {
    pub fn get_keyword(&self) -> &Keyword {
        match self {
            Modifier::Static(k) => k,
            Modifier::Final(k) => k,
            Modifier::Abstract(k) => k,
            Modifier::Readonly(k) => k,
            Modifier::Public(k) => k,
            Modifier::PublicSet(k) => k,
            Modifier::Protected(k) => k,
            Modifier::ProtectedSet(k) => k,
            Modifier::Private(k) => k,
            Modifier::PrivateSet(k) => k,
        }
    }

    /// Returns `true` if the modifier is a visibility modifier.
    pub fn is_visibility(&self) -> bool {
        matches!(
            self,
            Modifier::Public(..) | Modifier::Protected(..) | Modifier::Private(..) | Modifier::PrivateSet(..)
        )
    }

    /// Returns `true` if the modifier is a read visibility modifier.
    pub fn is_read_visibility(&self) -> bool {
        matches!(self, Modifier::Public(..) | Modifier::Protected(..) | Modifier::Private(..))
    }

    /// Returns `true` if the modifier is a write visibility modifier.
    pub fn is_write_visibility(&self) -> bool {
        matches!(self, Modifier::PrivateSet(..) | Modifier::ProtectedSet(..) | Modifier::PublicSet(..))
    }

    pub fn as_str<'a>(&self, interner: &'a ThreadedInterner) -> &'a str {
        match self {
            Modifier::Static(k) => interner.lookup(&k.value),
            Modifier::Final(k) => interner.lookup(&k.value),
            Modifier::Abstract(k) => interner.lookup(&k.value),
            Modifier::Readonly(k) => interner.lookup(&k.value),
            Modifier::Public(k) => interner.lookup(&k.value),
            Modifier::Protected(k) => interner.lookup(&k.value),
            Modifier::Private(k) => interner.lookup(&k.value),
            Modifier::PrivateSet(k) => interner.lookup(&k.value),
            Modifier::ProtectedSet(k) => interner.lookup(&k.value),
            Modifier::PublicSet(k) => interner.lookup(&k.value),
        }
    }
}

impl HasSpan for Modifier {
    fn span(&self) -> Span {
        match self {
            Modifier::Static(value)
            | Modifier::Final(value)
            | Modifier::Abstract(value)
            | Modifier::Readonly(value)
            | Modifier::Public(value)
            | Modifier::Protected(value)
            | Modifier::Private(value)
            | Modifier::PrivateSet(value)
            | Modifier::ProtectedSet(value)
            | Modifier::PublicSet(value) => value.span(),
        }
    }
}

impl Sequence<Modifier> {
    /// Returns the first abstract modifier in the sequence, if any.
    pub fn get_static(&self) -> Option<&Modifier> {
        self.iter().find(|modifier| matches!(modifier, Modifier::Static(..)))
    }

    /// Returns `true` if the sequence contains a static modifier.
    pub fn contains_static(&self) -> bool {
        self.iter().any(|modifier| matches!(modifier, Modifier::Static(..)))
    }

    /// Return the first final modifier in the sequence, if any.
    pub fn get_final(&self) -> Option<&Modifier> {
        self.iter().find(|modifier| matches!(modifier, Modifier::Final(_)))
    }

    /// Returns `true` if the sequence contains a final modifier.
    pub fn contains_final(&self) -> bool {
        self.iter().any(|modifier| matches!(modifier, Modifier::Final(..)))
    }

    /// Returns the first abstract modifier in the sequence, if any.
    pub fn get_abstract(&self) -> Option<&Modifier> {
        self.iter().find(|modifier| matches!(modifier, Modifier::Abstract(..)))
    }

    /// Returns `true` if the sequence contains an abstract modifier.
    pub fn contains_abstract(&self) -> bool {
        self.iter().any(|modifier| matches!(modifier, Modifier::Abstract(..)))
    }

    /// Returns the first abstract modifier in the sequence, if any.
    pub fn get_readonly(&self) -> Option<&Modifier> {
        self.iter().find(|modifier| matches!(modifier, Modifier::Readonly(..)))
    }

    /// Returns `true` if the sequence contains a readonly modifier.
    pub fn contains_readonly(&self) -> bool {
        self.iter().any(|modifier| matches!(modifier, Modifier::Readonly(..)))
    }

    pub fn get_first_visibility(&self) -> Option<&Modifier> {
        self.iter().find(|modifier| {
            matches!(
                modifier,
                Modifier::Public(..)
                    | Modifier::Protected(..)
                    | Modifier::Private(..)
                    | Modifier::PrivateSet(..)
                    | Modifier::ProtectedSet(..)
                    | Modifier::PublicSet(..)
            )
        })
    }

    pub fn get_first_read_visibility(&self) -> Option<&Modifier> {
        self.iter()
            .find(|modifier| matches!(modifier, Modifier::Public(..) | Modifier::Protected(..) | Modifier::Private(..)))
    }

    pub fn get_first_write_visibility(&self) -> Option<&Modifier> {
        self.iter().find(|modifier| {
            matches!(modifier, Modifier::PrivateSet(..) | Modifier::ProtectedSet(..) | Modifier::PublicSet(..))
        })
    }

    /// Returns `true` if the sequence contains a visibility modifier for reading or writing.
    pub fn contains_visibility(&self) -> bool {
        self.iter().any(Modifier::is_visibility)
    }

    pub fn get_public(&self) -> Option<&Modifier> {
        self.iter().find(|modifier| matches!(modifier, Modifier::Public(..)))
    }

    /// Returns `true` if the sequence contains a public visibility modifier.
    pub fn contains_public(&self) -> bool {
        self.iter().any(|modifier| matches!(modifier, Modifier::Public(..)))
    }

    pub fn get_protected(&self) -> Option<&Modifier> {
        self.iter().find(|modifier| matches!(modifier, Modifier::Protected(..)))
    }

    /// Returns `true` if the sequence contains a protected visibility modifier.
    pub fn contains_protected(&self) -> bool {
        self.iter().any(|modifier| matches!(modifier, Modifier::Protected(..)))
    }

    pub fn get_private(&self) -> Option<&Modifier> {
        self.iter().find(|modifier| matches!(modifier, Modifier::Private(..)))
    }

    /// Returns `true` if the sequence contains a private visibility modifier.
    pub fn contains_private(&self) -> bool {
        self.iter().any(|modifier| matches!(modifier, Modifier::Private(..)))
    }

    pub fn get_private_set(&self) -> Option<&Modifier> {
        self.iter().find(|modifier| matches!(modifier, Modifier::PrivateSet(..)))
    }

    pub fn contains_private_set(&self) -> bool {
        self.iter().any(|modifier| matches!(modifier, Modifier::PrivateSet(..)))
    }

    pub fn get_protected_set(&self) -> Option<&Modifier> {
        self.iter().find(|modifier| matches!(modifier, Modifier::ProtectedSet(..)))
    }

    pub fn contains_protected_set(&self) -> bool {
        self.iter().any(|modifier| matches!(modifier, Modifier::ProtectedSet(..)))
    }

    pub fn get_public_set(&self) -> Option<&Modifier> {
        self.iter().find(|modifier| matches!(modifier, Modifier::PublicSet(..)))
    }

    pub fn contains_public_set(&self) -> bool {
        self.iter().any(|modifier| matches!(modifier, Modifier::PublicSet(..)))
    }
}
