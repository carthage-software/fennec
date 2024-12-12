use mago_ast::ast::*;
use mago_ast::sequence::Sequence;
use mago_token::T;

use crate::error::ParseError;
use crate::internal::token_stream::TokenStream;
use crate::internal::utils;

pub fn parse_modifier_sequence(stream: &mut TokenStream<'_, '_>) -> Result<Sequence<Modifier>, ParseError> {
    let mut modifiers = Vec::new();
    while let Some(modifier) = parse_optional_modifier(stream)? {
        modifiers.push(modifier);
    }

    Ok(Sequence::new(modifiers))
}

pub fn parse_optional_read_visibility_modifier(
    stream: &mut TokenStream<'_, '_>,
) -> Result<Option<Modifier>, ParseError> {
    Ok(Some(match utils::maybe_peek(stream)?.map(|t| t.kind) {
        Some(T!["public"]) => Modifier::Public(utils::expect_any_keyword(stream)?),
        Some(T!["protected"]) => Modifier::Protected(utils::expect_any_keyword(stream)?),
        Some(T!["private"]) => Modifier::Private(utils::expect_any_keyword(stream)?),
        _ => return Ok(None),
    }))
}

pub fn parse_optional_modifier(stream: &mut TokenStream<'_, '_>) -> Result<Option<Modifier>, ParseError> {
    Ok(Some(match utils::maybe_peek(stream)?.map(|t| t.kind) {
        Some(T!["public"]) => Modifier::Public(utils::expect_any_keyword(stream)?),
        Some(T!["protected"]) => Modifier::Protected(utils::expect_any_keyword(stream)?),
        Some(T!["private"]) => Modifier::Private(utils::expect_any_keyword(stream)?),
        Some(T!["static"]) => Modifier::Static(utils::expect_any_keyword(stream)?),
        Some(T!["final"]) => Modifier::Final(utils::expect_any_keyword(stream)?),
        Some(T!["abstract"]) => Modifier::Abstract(utils::expect_any_keyword(stream)?),
        Some(T!["readonly"]) => Modifier::Readonly(utils::expect_any_keyword(stream)?),
        Some(T!["private(set)"]) => Modifier::PrivateSet(utils::expect_any_keyword(stream)?),
        Some(T!["protected(set)"]) => Modifier::ProtectedSet(utils::expect_any_keyword(stream)?),
        Some(T!["public(set)"]) => Modifier::PublicSet(utils::expect_any_keyword(stream)?),
        _ => return Ok(None),
    }))
}
