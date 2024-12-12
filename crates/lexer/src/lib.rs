use std::fmt::Debug;
use std::mem;

use mago_interner::ThreadedInterner;
use mago_span::Position;
use mago_span::Span;
use mago_token::DocumentKind;
use mago_token::Token;
use mago_token::TokenKind;

use crate::error::SyntaxError;
use crate::input::Input;
use crate::internal::macros::float_exponent;
use crate::internal::macros::float_separator;
use crate::internal::macros::number_separator;
use crate::internal::macros::number_sign;
use crate::internal::macros::part_of_identifier;
use crate::internal::macros::start_of_binary_number;
use crate::internal::macros::start_of_float_number;
use crate::internal::macros::start_of_hexadecimal_number;
use crate::internal::macros::start_of_identifier;
use crate::internal::macros::start_of_number;
use crate::internal::macros::start_of_octal_number;
use crate::internal::macros::start_of_octal_or_float_number;
use crate::internal::mode::HaltStage;
use crate::internal::mode::Interpolation;
use crate::internal::mode::LexerMode;
use crate::internal::utils::NumberKind;

pub mod error;
pub mod input;

mod internal;

/// The `Lexer` struct is responsible for tokenizing input source code into discrete tokens
/// based on PHP language syntax. It is designed to work with PHP code from version 7.0 up to 8.4.
///
/// The lexer reads through the provided input and processes it accordingly.
///
/// It identifies PHP-specific tokens, including operators, keywords, comments, strings, and other syntax elements,
/// and produces a sequence of [`Token`] objects that are used in further stages of compilation or interpretation.
///
/// The lexer is designed to be used in a streaming fashion, where it reads the input source code in chunks
/// and produces tokens incrementally. This allows for efficient processing of large source files and
/// minimizes memory usage.
#[derive(Debug)]
pub struct Lexer<'a, 'i> {
    interner: &'i ThreadedInterner,
    input: Input<'a>,
    mode: LexerMode<'a>,
    interpolating: bool,
}

impl<'a, 'i> Lexer<'a, 'i> {
    /// Creates a new `Lexer` instance.
    ///
    /// # Parameters
    ///
    /// - `interner`: The interner to use for string interning.
    /// - `input`: The input source code to tokenize.
    ///
    /// # Returns
    ///
    /// A new `Lexer` instance that reads from the provided byte slice.
    pub fn new(interner: &'i ThreadedInterner, input: Input<'a>) -> Lexer<'a, 'i> {
        Lexer { interner, input, mode: LexerMode::Inline, interpolating: false }
    }

    /// Creates a new `Lexer` instance for parsing a script block.
    ///
    /// # Parameters
    ///
    /// - `interner`: The interner to use for string interning.
    /// - `input`: The input source code to tokenize.
    ///
    /// # Returns
    ///
    /// A new `Lexer` instance that reads from the provided byte slice.
    pub fn scripting(interner: &'i ThreadedInterner, input: Input<'a>) -> Lexer<'a, 'i> {
        Lexer { interner, input, mode: LexerMode::Script, interpolating: false }
    }

    /// Check if the lexer has reached the end of the input.
    ///
    /// If this method returns `true`, the lexer will not produce any more tokens.
    pub fn has_reached_eof(&self) -> bool {
        self.input.has_reached_eof()
    }

    /// Get the current position of the lexer in the input source code.
    pub fn get_position(&self) -> Position {
        self.input.position()
    }

    /// Tokenizes the next input from the source code.
    ///
    /// This method reads from the input and produces the next [`Token`] based on the current [`LexerMode`].
    /// It handles various lexical elements such as inline text, script code, strings with interpolation,
    /// comments, and different PHP-specific constructs.
    ///
    /// # Returns
    ///
    /// - `Some(Ok(Token))` if a token was successfully parsed.
    /// - `Some(Err(SyntaxError))` if a syntax error occurred while parsing the next token.
    /// - `None` if the end of the input has been reached.
    ///
    /// # Examples
    ///
    /// ```
    /// use mago_interner::ThreadedInterner;
    /// use mago_lexer::Lexer;
    /// use mago_source::SourceIdentifier;
    /// use mago_lexer::input::Input;
    ///
    /// let interner = ThreadedInterner::new();
    ///
    /// let source = SourceIdentifier::dummy();
    /// let input = Input::new(source, b"<?php echo 'Hello, World!'; ?>");
    ///
    /// let mut lexer = Lexer::new(&interner, input);
    ///
    /// while let Some(result) = lexer.advance() {
    ///     match result {
    ///         Ok(token) => println!("Token: {:?}", token),
    ///         Err(error) => eprintln!("Syntax error: {:?}", error),
    ///     }
    /// }
    /// ```
    ///
    /// # Notes
    ///
    /// - It efficiently handles tokenization by consuming input based on patterns specific to PHP syntax.
    /// - The lexer supports complex features like string interpolation and different numeric formats.
    ///
    /// # Errors
    ///
    /// Returns `Some(Err(SyntaxError))` in cases such as:
    ///
    /// - Unrecognized tokens that do not match any known PHP syntax.
    /// - Unexpected tokens in a given context, such as an unexpected end of string.
    ///
    /// # Panics
    ///
    /// This method should not panic under normal operation. If it does, it indicates a bug in the lexer implementation.
    ///
    /// # See Also
    ///
    /// - [`Token`]: Represents a lexical token with its kind, value, and span.
    /// - [`SyntaxError`]: Represents errors that can occur during lexing.
    pub fn advance(&mut self) -> Option<Result<Token, SyntaxError>> {
        if self.input.has_reached_eof() {
            return None;
        }

        match self.mode {
            LexerMode::Inline => {
                let start = self.input.position();
                if self.input.is_at(b"<?", false) {
                    let (kind, buffer) = if self.input.is_at(b"<?php", true) {
                        (TokenKind::OpenTag, self.input.consume(5))
                    } else if self.input.is_at(b"<?=", false) {
                        (TokenKind::EchoTag, self.input.consume(3))
                    } else {
                        (TokenKind::ShortOpenTag, self.input.consume(2))
                    };

                    let end = self.input.position();
                    let tag = self.token(kind, buffer, start, end);

                    self.mode = LexerMode::Script;

                    return tag;
                }

                if self.input.is_at(b"#!", true) {
                    let buffer = self.input.consume_until_inclusive(b"\n", false);
                    let end = self.input.position();

                    self.token(TokenKind::InlineShebang, buffer, start, end)
                } else {
                    let buffer = self.input.consume_until(b"<?", false);
                    let end = self.input.position();

                    self.token(TokenKind::InlineText, buffer, start, end)
                }
            }
            LexerMode::Script => {
                let whitespaces = self.input.consume_whitespaces();
                if !whitespaces.is_empty() {
                    let start = self.input.position();
                    let buffer = whitespaces;
                    let end = self.input.position();

                    return self.token(TokenKind::Whitespace, buffer, start, end);
                }

                let mut document_label: &[u8] = &[];

                let (token_kind, len) = match self.input.read(3) {
                    [b'!', b'=', b'='] => (TokenKind::BangEqualEqual, 3),
                    [b'?', b'?', b'='] => (TokenKind::QuestionQuestionEqual, 3),
                    [b'?', b'-', b'>'] => (TokenKind::QuestionMinusGreaterThan, 3),
                    [b'=', b'=', b'='] => (TokenKind::EqualEqualEqual, 3),
                    [b'.', b'.', b'.'] => (TokenKind::DotDotDot, 3),
                    [b'<', b'=', b'>'] => (TokenKind::LessThanEqualGreaterThan, 3),
                    [b'<', b'<', b'='] => (TokenKind::LeftShiftEqual, 3),
                    [b'>', b'>', b'='] => (TokenKind::RightShiftEqual, 3),
                    [b'<', b'<', b'<'] if matches_start_of_heredoc_document(&self.input) => {
                        let (length, whitespaces, label_length) = read_start_of_heredoc_document(&self.input, false);

                        document_label = self.input.peek(3 + whitespaces, label_length);

                        (TokenKind::DocumentStart(DocumentKind::Heredoc), length)
                    }
                    [b'<', b'<', b'<'] if matches_start_of_double_quote_heredoc_document(&self.input) => {
                        let (length, whitespaces, label_length) = read_start_of_heredoc_document(&self.input, true);

                        document_label = self.input.peek(4 + whitespaces, label_length);

                        (TokenKind::DocumentStart(DocumentKind::Heredoc), length)
                    }
                    [b'<', b'<', b'<'] if matches_start_of_nowdoc_document(&self.input) => {
                        let (length, whitespaces, label_length) = read_start_of_nowdoc_document(&self.input);

                        document_label = self.input.peek(4 + whitespaces, label_length);

                        (TokenKind::DocumentStart(DocumentKind::Nowdoc), length)
                    }
                    [b'!', b'=', ..] => (TokenKind::BangEqual, 2),
                    [b'&', b'&', ..] => (TokenKind::AmpersandAmpersand, 2),
                    [b'&', b'=', ..] => (TokenKind::AmpersandEqual, 2),
                    [b'.', b'=', ..] => (TokenKind::DotEqual, 2),
                    [b'?', b'?', ..] => (TokenKind::QuestionQuestion, 2),
                    [b'?', b':', ..] => (TokenKind::QuestionColon, 2),
                    [b'?', b'>', ..] => (TokenKind::CloseTag, 2),
                    [b'=', b'>', ..] => (TokenKind::EqualGreaterThan, 2),
                    [b'=', b'=', ..] => (TokenKind::EqualEqual, 2),
                    [b'+', b'+', ..] => (TokenKind::PlusPlus, 2),
                    [b'+', b'=', ..] => (TokenKind::PlusEqual, 2),
                    [b'%', b'=', ..] => (TokenKind::PercentEqual, 2),
                    [b'-', b'-', ..] => (TokenKind::MinusMinus, 2),
                    [b'-', b'>', ..] => (TokenKind::MinusGreaterThan, 2),
                    [b'-', b'=', ..] => (TokenKind::MinusEqual, 2),
                    [b'<', b'<', ..] => (TokenKind::LeftShift, 2),
                    [b'<', b'=', ..] => (TokenKind::LessThanEqual, 2),
                    [b'<', b'>', ..] => (TokenKind::LessThanGreaterThan, 2),
                    [b'>', b'>', ..] => (TokenKind::RightShift, 2),
                    [b'>', b'=', ..] => (TokenKind::GreaterThanEqual, 2),
                    [b':', b':', ..] => (TokenKind::ColonColon, 2),
                    [b'#', b'[', ..] => (TokenKind::HashLeftBracket, 2),
                    [b'|', b'=', ..] => (TokenKind::PipeEqual, 2),
                    [b'|', b'|', ..] => (TokenKind::PipePipe, 2),
                    [b'/', b'=', ..] => (TokenKind::SlashEqual, 2),
                    [b'^', b'=', ..] => (TokenKind::CaretEqual, 2),
                    [b'*', b'*', ..] => (TokenKind::AsteriskAsterisk, 2),
                    [b'*', b'=', ..] => (TokenKind::AsteriskEqual, 2),
                    [b'/', b'/', ..] => {
                        let mut length = 2;
                        loop {
                            match self.input.peek(length, 3) {
                                [b'\n' | b'\r', ..] => {
                                    break;
                                }
                                [w, b'?', b'>'] if w.is_ascii_whitespace() => {
                                    break;
                                }
                                [b'?', b'>', ..] | [] => {
                                    break;
                                }
                                [_, ..] => {
                                    length += 1;
                                }
                            }
                        }

                        (TokenKind::SingleLineComment, length)
                    }
                    [b'/', b'*', asterisk] => {
                        let mut length = 2;
                        loop {
                            match self.input.peek(length, 2) {
                                [b'*', b'/'] => {
                                    length += 2;

                                    break;
                                }
                                [_, ..] => {
                                    length += 1;
                                }
                                [] => {
                                    break;
                                }
                            }
                        }

                        if asterisk == &b'*' {
                            (TokenKind::DocBlockComment, length)
                        } else {
                            (TokenKind::MultiLineComment, length)
                        }
                    }
                    [b'\\', start_of_identifier!(), ..] => {
                        let mut length = 2;
                        let mut last_was_slash = false;
                        loop {
                            match self.input.peek(length, 1) {
                                [start_of_identifier!(), ..] if last_was_slash => {
                                    length += 1;
                                    last_was_slash = false;
                                }
                                [part_of_identifier!(), ..] if !last_was_slash => {
                                    length += 1;
                                }
                                [b'\\', ..] => {
                                    if last_was_slash {
                                        length -= 1;

                                        break;
                                    }

                                    length += 1;
                                    last_was_slash = true;
                                }
                                _ => {
                                    break;
                                }
                            }
                        }

                        (TokenKind::FullyQualifiedIdentifier, length)
                    }
                    [b'$', start_of_identifier!(), ..] => {
                        let mut length = 2;
                        while let [part_of_identifier!(), ..] = self.input.peek(length, 1) {
                            length += 1;
                        }

                        (TokenKind::Variable, length)
                    }
                    [b'$', b'{', ..] => (TokenKind::DollarLeftBrace, 2),
                    [b'$', ..] => (TokenKind::Dollar, 1),
                    [b'@', ..] => (TokenKind::At, 1),
                    [b'!', ..] => (TokenKind::Bang, 1),
                    [b'&', ..] => (TokenKind::Ampersand, 1),
                    [b'?', ..] => (TokenKind::Question, 1),
                    [b'=', ..] => (TokenKind::Equal, 1),
                    [b'`', ..] => (TokenKind::Backtick, 1),
                    [b')', ..] => (TokenKind::RightParenthesis, 1),
                    [b';', ..] => (TokenKind::Semicolon, 1),
                    [b'+', ..] => (TokenKind::Plus, 1),
                    [b'%', ..] => (TokenKind::Percent, 1),
                    [b'-', ..] => (TokenKind::Minus, 1),
                    [b'<', ..] => (TokenKind::LessThan, 1),
                    [b'>', ..] => (TokenKind::GreaterThan, 1),
                    [b',', ..] => (TokenKind::Comma, 1),
                    [b'[', ..] => (TokenKind::LeftBracket, 1),
                    [b']', ..] => (TokenKind::RightBracket, 1),
                    [b'{', ..] => (TokenKind::LeftBrace, 1),
                    [b'}', ..] => (TokenKind::RightBrace, 1),
                    [b':', ..] => (TokenKind::Colon, 1),
                    [b'~', ..] => (TokenKind::Tilde, 1),
                    [b'|', ..] => (TokenKind::Pipe, 1),
                    [b'^', ..] => (TokenKind::Caret, 1),
                    [b'*', ..] => (TokenKind::Asterisk, 1),
                    [b'/', ..] => (TokenKind::Slash, 1),
                    [quote @ b'\'', ..] => read_literal_string(&self.input, quote),
                    [quote @ b'"', ..] if matches_literal_double_quote_string(&self.input) => {
                        read_literal_string(&self.input, quote)
                    }
                    [b'"', ..] => (TokenKind::DoubleQuote, 1),
                    [b'(', ..] => 'parenthesis: {
                        for (value, kind) in internal::consts::CAST_TYPES {
                            if let Some(length) = self.input.match_sequence_ignore_whitespace(value, true) {
                                break 'parenthesis (kind, length);
                            }
                        }

                        (TokenKind::LeftParenthesis, 1)
                    }
                    [b'#', ..] => {
                        let mut length = 1;
                        loop {
                            match self.input.peek(length, 3) {
                                [b'\n' | b'\r', ..] => {
                                    break;
                                }
                                [w, b'?', b'>'] if w.is_ascii_whitespace() => {
                                    break;
                                }
                                [b'?', b'>', ..] | [] => {
                                    break;
                                }
                                [_, ..] => {
                                    length += 1;
                                }
                            }
                        }

                        (TokenKind::HashComment, length)
                    }
                    [b'\\', ..] => (TokenKind::NamespaceSeparator, 1),
                    [start_of_identifier!(), ..] => 'identifier: {
                        let mut length = 1;
                        let mut ended_with_slash = false;
                        loop {
                            match self.input.peek(length, 2) {
                                [part_of_identifier!(), ..] => {
                                    length += 1;
                                }
                                [b'\\', start_of_identifier!(), ..] => {
                                    ended_with_slash = true;
                                    break;
                                }
                                // special case for `private(set)`
                                [b'(', ..] if length == 7 => {
                                    if self.input.is_at(b"private(set)", true) {
                                        break 'identifier (TokenKind::PrivateSet, 7 + 5);
                                    }

                                    break;
                                }
                                // special case for `public(set)`
                                [b'(', ..] if length == 6 => {
                                    if self.input.is_at(b"public(set)", true) {
                                        break 'identifier (TokenKind::PublicSet, 6 + 5);
                                    }

                                    break;
                                }
                                // special case for `protected(set)`
                                [b'(', ..] if length == 9 => {
                                    if self.input.is_at(b"protected(set)", true) {
                                        break 'identifier (TokenKind::ProtectedSet, 9 + 5);
                                    }

                                    break;
                                }
                                _ => {
                                    break;
                                }
                            }
                        }

                        if !ended_with_slash {
                            for (value, kind) in internal::consts::KEYWORD_TYPES {
                                if value.len() != length {
                                    continue;
                                }

                                if self.input.is_at(value, true) {
                                    break 'identifier (kind, value.len());
                                }
                            }
                        }

                        let mut slashes = 0;
                        let mut last_was_slash = false;
                        loop {
                            match self.input.peek(length, 1) {
                                [start_of_identifier!(), ..] if last_was_slash => {
                                    length += 1;
                                    last_was_slash = false;
                                }
                                [part_of_identifier!(), ..] if !last_was_slash => {
                                    length += 1;
                                }
                                [b'\\', ..] if !self.interpolating => {
                                    if !last_was_slash {
                                        length += 1;
                                        slashes += 1;
                                        last_was_slash = true;
                                    } else {
                                        length -= 1;
                                        slashes -= 1;
                                        last_was_slash = false;

                                        break;
                                    }
                                }
                                _ => {
                                    break;
                                }
                            }
                        }

                        if last_was_slash {
                            length -= 1;
                            slashes -= 1;
                        }

                        if slashes > 0 {
                            (TokenKind::QualifiedIdentifier, length)
                        } else {
                            (TokenKind::Identifier, length)
                        }
                    }
                    [b'.', start_of_number!(), ..] => {
                        let mut length = read_digits_of_base(&self.input, 2, 10);
                        if let float_exponent!() = self.input.peek(length, 1) {
                            length += 1;
                            if let number_sign!() = self.input.peek(length, 1) {
                                length += 1;
                            }

                            length = read_digits_of_base(&self.input, length, 10);
                        }

                        (TokenKind::LiteralFloat, length)
                    }
                    [start_of_number!(), ..] => 'number: {
                        let mut length = 1;

                        let (base, kind): (u8, NumberKind) = match self.input.read(2) {
                            start_of_binary_number!() => {
                                length += 1;

                                (2, NumberKind::Integer)
                            }
                            start_of_octal_number!() => {
                                length += 1;

                                (8, NumberKind::Integer)
                            }
                            start_of_hexadecimal_number!() => {
                                length += 1;

                                (16, NumberKind::Integer)
                            }
                            start_of_octal_or_float_number!() => (10, NumberKind::OctalOrFloat),
                            start_of_float_number!() => (10, NumberKind::Float),
                            _ => (10, NumberKind::IntegerOrFloat),
                        };

                        if kind != NumberKind::Float {
                            length = read_digits_of_base(&self.input, length, base);

                            if kind == NumberKind::Integer {
                                break 'number (TokenKind::LiteralInteger, length);
                            }
                        }

                        let is_float = matches!(self.input.peek(length, 3), float_separator!());

                        if !is_float {
                            break 'number (TokenKind::LiteralInteger, length);
                        }

                        if let [b'.'] = self.input.peek(length, 1) {
                            length += 1;
                            length = read_digits_of_base(&self.input, length, 10);
                        }

                        if let float_exponent!() = self.input.peek(length, 1) {
                            length += 1;
                            if let number_sign!() = self.input.peek(length, 1) {
                                length += 1;
                            }

                            length = read_digits_of_base(&self.input, length, 10);
                        }

                        (TokenKind::LiteralFloat, length)
                    }
                    [b'.', ..] => (TokenKind::Dot, 1),
                    [unknown_byte, ..] => {
                        return Some(Err(SyntaxError::UnrecognizedToken(*unknown_byte, self.input.position())));
                    }
                    [] => {
                        // we check for EOF before entering scripting section,
                        // so this should be unreachable.
                        unreachable!()
                    }
                };

                self.mode = match token_kind {
                    TokenKind::DoubleQuote => LexerMode::DoubleQuoteString(Interpolation::None),
                    TokenKind::Backtick => LexerMode::ShellExecuteString(Interpolation::None),
                    TokenKind::CloseTag => LexerMode::Inline,
                    TokenKind::HaltCompiler => LexerMode::Halt(HaltStage::LookingForLeftParenthesis),
                    TokenKind::DocumentStart(document_kind) => {
                        LexerMode::DocumentString(document_kind, document_label, Interpolation::None)
                    }
                    _ => LexerMode::Script,
                };

                let start = self.input.position();
                let buffer = self.input.consume(len);
                let end = self.input.position();

                self.token(token_kind, buffer, start, end)
            }
            LexerMode::DoubleQuoteString(interpolation) => match &interpolation {
                Interpolation::None => {
                    let start = self.input.position();

                    let mut length = 0;
                    let mut last_was_slash = false;
                    let mut token_kind = TokenKind::StringPart;
                    loop {
                        match self.input.peek(length, 2) {
                            [b'$', start_of_identifier!(), ..] if !last_was_slash => {
                                let until_offset = read_until_end_of_variable_interpolation(&self.input, length + 2);

                                self.mode =
                                    LexerMode::DoubleQuoteString(Interpolation::Until(start.offset + until_offset));

                                break;
                            }
                            [b'{', b'$', ..] | [b'$', b'{', ..] if !last_was_slash => {
                                let until_offset = read_until_end_of_brace_interpolation(&self.input, length + 2);

                                self.mode =
                                    LexerMode::DoubleQuoteString(Interpolation::Until(start.offset + until_offset));

                                break;
                            }
                            [b'\\', ..] => {
                                length += 1;

                                last_was_slash = !last_was_slash;
                            }
                            [b'"', ..] if !last_was_slash => {
                                if length == 0 {
                                    length += 1;
                                    token_kind = TokenKind::DoubleQuote;

                                    break;
                                }

                                break;
                            }
                            [_, ..] => {
                                length += 1;
                                last_was_slash = false;
                            }
                            [] => {
                                break;
                            }
                        }
                    }

                    let buffer = self.input.consume(length);
                    let end = self.input.position();

                    if TokenKind::DoubleQuote == token_kind {
                        self.mode = LexerMode::Script;
                    }

                    self.token(token_kind, buffer, start, end)
                }
                Interpolation::Until(offset) => {
                    self.interpolation(*offset, LexerMode::DoubleQuoteString(Interpolation::None))
                }
            },
            LexerMode::ShellExecuteString(interpolation) => match &interpolation {
                Interpolation::None => {
                    let start = self.input.position();

                    let mut length = 0;
                    let mut last_was_slash = false;
                    let mut token_kind = TokenKind::StringPart;
                    loop {
                        match self.input.peek(length, 2) {
                            [b'$', start_of_identifier!(), ..] if !last_was_slash => {
                                let until_offset = read_until_end_of_variable_interpolation(&self.input, length + 2);

                                self.mode =
                                    LexerMode::ShellExecuteString(Interpolation::Until(start.offset + until_offset));

                                break;
                            }
                            [b'{', b'$', ..] | [b'$', b'{', ..] if !last_was_slash => {
                                let until_offset = read_until_end_of_brace_interpolation(&self.input, length + 2);

                                self.mode =
                                    LexerMode::ShellExecuteString(Interpolation::Until(start.offset + until_offset));

                                break;
                            }
                            [b'\\', ..] => {
                                length += 1;
                                last_was_slash = true;
                            }
                            [b'`', ..] if !last_was_slash => {
                                if length == 0 {
                                    length += 1;
                                    token_kind = TokenKind::Backtick;

                                    break;
                                }

                                break;
                            }
                            [_, ..] => {
                                length += 1;
                                last_was_slash = false;
                            }
                            [] => {
                                break;
                            }
                        }
                    }

                    let buffer = self.input.consume(length);
                    let end = self.input.position();

                    if TokenKind::Backtick == token_kind {
                        self.mode = LexerMode::Script;
                    }

                    self.token(token_kind, buffer, start, end)
                }
                Interpolation::Until(offset) => {
                    self.interpolation(*offset, LexerMode::ShellExecuteString(Interpolation::None))
                }
            },
            LexerMode::DocumentString(kind, label, interpolation) => match &kind {
                DocumentKind::Heredoc => match &interpolation {
                    Interpolation::None => {
                        let start = self.input.position();

                        let mut length = 0;
                        let mut last_was_slash = false;
                        let mut only_whitespaces = true;
                        let mut token_kind = TokenKind::StringPart;
                        loop {
                            match self.input.peek(length, 2) {
                                [b'\n', ..] => {
                                    length += 1;

                                    break;
                                }
                                [byte, ..] if byte.is_ascii_whitespace() => {
                                    length += 1;
                                }
                                [b'$', start_of_identifier!(), ..] if !last_was_slash => {
                                    let until_offset =
                                        read_until_end_of_variable_interpolation(&self.input, length + 2);

                                    self.mode = LexerMode::DocumentString(
                                        kind,
                                        label,
                                        Interpolation::Until(start.offset + until_offset),
                                    );

                                    break;
                                }
                                [b'{', b'$', ..] | [b'$', b'{', ..] if !last_was_slash => {
                                    let until_offset = read_until_end_of_brace_interpolation(&self.input, length + 2);

                                    self.mode = LexerMode::DocumentString(
                                        kind,
                                        label,
                                        Interpolation::Until(start.offset + until_offset),
                                    );

                                    break;
                                }
                                [b'\\', ..] => {
                                    length += 1;
                                    last_was_slash = true;
                                    only_whitespaces = false;
                                }
                                [_, ..] => {
                                    if only_whitespaces && self.input.peek(length, label.len()) == label {
                                        length += label.len();
                                        token_kind = TokenKind::DocumentEnd;

                                        break;
                                    }

                                    length += 1;
                                    last_was_slash = false;
                                    only_whitespaces = false;
                                }
                                [] => {
                                    break;
                                }
                            }
                        }

                        let buffer = self.input.consume(length);
                        let end = self.input.position();

                        if TokenKind::DocumentEnd == token_kind {
                            self.mode = LexerMode::Script;
                        }

                        self.token(token_kind, buffer, start, end)
                    }
                    Interpolation::Until(offset) => {
                        self.interpolation(*offset, LexerMode::DocumentString(kind, label, Interpolation::None))
                    }
                },
                DocumentKind::Nowdoc => {
                    let start = self.input.position();

                    let mut length = 0;
                    let mut terminated = false;
                    let mut only_whitespaces = true;

                    loop {
                        match self.input.peek(length, 1) {
                            [b'\n', ..] => {
                                length += 1;

                                break;
                            }
                            [byte, ..] if byte.is_ascii_whitespace() => {
                                length += 1;
                            }
                            [_, ..] => {
                                if only_whitespaces && self.input.peek(length, label.len()) == label {
                                    length += label.len();
                                    terminated = true;

                                    break;
                                }

                                only_whitespaces = false;
                                length += 1;
                            }
                            [] => {
                                break;
                            }
                        }
                    }

                    let buffer = self.input.consume(length);
                    let end = self.input.position();

                    if terminated {
                        self.mode = LexerMode::Script;

                        return self.token(TokenKind::DocumentEnd, buffer, start, end);
                    }

                    self.token(TokenKind::StringPart, buffer, start, end)
                }
            },
            LexerMode::Halt(stage) => 'halt: {
                let start = self.input.position();
                if let HaltStage::End = stage {
                    let buffer = self.input.consume_remaining();
                    let end = self.input.position();

                    break 'halt self.token(TokenKind::InlineText, buffer, start, end);
                }

                let whitespaces = self.input.consume_whitespaces();
                if !whitespaces.is_empty() {
                    let end = self.input.position();

                    break 'halt self.token(TokenKind::Whitespace, whitespaces, start, end);
                }

                match &stage {
                    HaltStage::LookingForLeftParenthesis => {
                        if self.input.is_at(b"(", false) {
                            let buffer = self.input.consume(1);
                            let end = self.input.position();

                            self.mode = LexerMode::Halt(HaltStage::LookingForRightParenthesis);

                            self.token(TokenKind::LeftParenthesis, buffer, start, end)
                        } else {
                            Some(Err(SyntaxError::UnexpectedToken(self.input.read(1)[0], self.input.position())))
                        }
                    }
                    HaltStage::LookingForRightParenthesis => {
                        if self.input.is_at(b")", false) {
                            let buffer = self.input.consume(1);
                            let end = self.input.position();

                            self.mode = LexerMode::Halt(HaltStage::LookingForTerminator);

                            self.token(TokenKind::RightParenthesis, buffer, start, end)
                        } else {
                            Some(Err(SyntaxError::UnexpectedToken(self.input.read(1)[0], self.input.position())))
                        }
                    }
                    HaltStage::LookingForTerminator => {
                        if self.input.is_at(b";", false) {
                            let buffer = self.input.consume(1);
                            let end = self.input.position();

                            self.mode = LexerMode::Halt(HaltStage::End);

                            self.token(TokenKind::Semicolon, buffer, start, end)
                        } else if self.input.is_at(b"?>", false) {
                            let buffer = self.input.consume(2);
                            let end = self.input.position();

                            self.mode = LexerMode::Halt(HaltStage::End);

                            self.token(TokenKind::CloseTag, buffer, start, end)
                        } else {
                            return Some(Err(SyntaxError::UnexpectedToken(
                                self.input.read(1)[0],
                                self.input.position(),
                            )));
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    fn token(
        &mut self,
        kind: TokenKind,
        value: &[u8],
        from: Position,
        to: Position,
    ) -> Option<Result<Token, SyntaxError>> {
        Some(Ok(Token { kind, value: self.interner.intern(String::from_utf8_lossy(value)), span: Span::new(from, to) }))
    }

    fn interpolation(&mut self, until: usize, next_mode: LexerMode<'a>) -> Option<Result<Token, SyntaxError>> {
        let mut mode = LexerMode::Script;

        mem::swap(&mut self.mode, &mut mode);
        self.interpolating = true;

        let result = self.advance();

        mem::swap(&mut self.mode, &mut mode);
        self.interpolating = false;

        match result {
            Some(Ok(token)) if token.span.has_offset(until) => {
                self.mode = next_mode;
            }
            _ => {}
        }

        result
    }
}

fn matches_start_of_heredoc_document(input: &Input) -> bool {
    let mut length = 3;
    let mut whitespaces = 0;
    loop {
        if input.peek(length + whitespaces, 1).first().map(|t| t.is_ascii_whitespace()).unwrap_or(false) {
            whitespaces += 1;
        } else {
            break;
        }
    }

    length += whitespaces;

    if !matches!(input.peek(length, 1), [start_of_identifier!(), ..]) {
        return false;
    }

    length += 1;
    loop {
        match input.peek(length, 2) {
            [b'\n', ..] => {
                return true;
            }
            [part_of_identifier!(), ..] => {
                length += 1;
            }
            [..] => {
                return false;
            }
        }
    }
}

fn matches_start_of_double_quote_heredoc_document(input: &Input) -> bool {
    let mut length = 3;
    let mut whitespaces = 0;
    loop {
        if input.peek(length + whitespaces, 1).first().map(|t| t.is_ascii_whitespace()).unwrap_or(false) {
            whitespaces += 1;
        } else {
            break;
        }
    }

    length += whitespaces;

    if !matches!(input.peek(length, 1), [b'"', ..]) {
        return false;
    }

    length += 1;

    if !matches!(input.peek(length, 1), [start_of_identifier!(), ..]) {
        return false;
    }

    length += 1;

    let mut terminated = false;
    loop {
        match input.peek(length, 2) {
            [b'\n', ..] => {
                return terminated;
            }
            [part_of_identifier!(), ..] if !terminated => {
                length += 1;
            }
            [b'"', ..] if !terminated => {
                terminated = true;
                length += 1;
            }
            [..] => {
                return false;
            }
        }
    }
}

fn matches_start_of_nowdoc_document(input: &Input) -> bool {
    let mut length = 3;
    let mut whitespaces = 0;
    loop {
        if input.peek(length + whitespaces, 1).first().map(|t| t.is_ascii_whitespace()).unwrap_or(false) {
            whitespaces += 1;
        } else {
            break;
        }
    }

    length += whitespaces;

    if !matches!(input.peek(length, 1), [b'\'', ..]) {
        return false;
    }

    length += 1;

    if !matches!(input.peek(length, 1), [start_of_identifier!(), ..]) {
        return false;
    }

    length += 1;

    let mut terminated = false;
    loop {
        match input.peek(length, 2) {
            [b'\n', ..] => {
                return terminated;
            }
            [part_of_identifier!(), ..] if !terminated => {
                length += 1;
            }
            [b'\'', ..] if !terminated => {
                terminated = true;
                length += 1;
            }
            [..] => {
                return false;
            }
        }
    }
}

fn matches_literal_double_quote_string(input: &Input) -> bool {
    let mut length = 1;
    loop {
        match input.peek(length, 2) {
            [b'"', ..] => {
                return true;
            }
            [b'\\', ..] => {
                length += 2;
            }
            [b'$', start_of_identifier!() | b'{'] | [b'{', b'$'] => {
                return false;
            }
            [_, ..] => {
                length += 1;
            }
            [] => {
                return true;
            }
        }
    }
}

fn read_start_of_heredoc_document(input: &Input, double_quoted: bool) -> (usize, usize, usize) {
    let mut length = 3;
    let mut whitespaces = 0;
    loop {
        if input.peek(length + whitespaces, 1).first().map(|t| t.is_ascii_whitespace()).unwrap_or(false) {
            whitespaces += 1;
        } else {
            break;
        }
    }

    length += whitespaces + if double_quoted { 2 } else { 1 };
    let mut label_length = 1;
    let mut terminated = false;
    loop {
        match input.peek(length, 2) {
            [b'\n', ..] => {
                length += 1;

                return (length, whitespaces, label_length);
            }
            [part_of_identifier!(), ..] if !double_quoted || !terminated => {
                length += 1;
                label_length += 1;
            }
            [b'"', ..] if double_quoted && !terminated => {
                length += 1;
                terminated = true;
            }
            [..] => unreachable!(),
        }
    }
}

fn read_start_of_nowdoc_document(input: &Input) -> (usize, usize, usize) {
    let mut length = 3;
    let mut whitespaces = 0;
    loop {
        if input.peek(length + whitespaces, 1).first().map(|t| t.is_ascii_whitespace()).unwrap_or(false) {
            whitespaces += 1;
        } else {
            break;
        }
    }

    length += whitespaces + 2;
    let mut label_length = 1;
    let mut terminated = false;
    loop {
        match input.peek(length, 2) {
            [b'\n', ..] => {
                length += 1;

                return (length, whitespaces, label_length);
            }
            [part_of_identifier!(), ..] if !terminated => {
                length += 1;
                label_length += 1;
            }
            [b'\'', ..] if !terminated => {
                length += 1;
                terminated = true;
            }
            [..] => unreachable!(),
        }
    }
}

fn read_literal_string(input: &Input, quote: &u8) -> (TokenKind, usize) {
    let mut length = 1;
    let mut last_was_backslash = false;
    let mut partial = false;
    loop {
        match input.peek(length, 1) {
            [b'\\', ..] => {
                length += 1;
                last_was_backslash = !last_was_backslash;
            }
            [byte, ..] => {
                if byte == quote && !last_was_backslash {
                    length += 1;

                    break;
                }

                length += 1;
                last_was_backslash = false;
            }
            [] => {
                partial = true;

                break;
            }
        }
    }

    if partial {
        (TokenKind::PartialLiteralString, length)
    } else {
        (TokenKind::LiteralString, length)
    }
}

fn read_digits_of_base(input: &Input, offset: usize, base: u8) -> usize {
    if 16 == base {
        read_digits_with(input, offset, u8::is_ascii_hexdigit)
    } else {
        let max = b'0' + base;
        let range = b'0'..max;

        read_digits_with(input, offset, |byte| range.contains(byte))
    }
}

fn read_digits_with<F: Fn(&u8) -> bool>(input: &Input, offset: usize, is_digit: F) -> usize {
    let mut length = offset;
    loop {
        match input.peek(length, 2) {
            [b, ..] if is_digit(b) => {
                length += 1;
            }
            [number_separator!(), b] if is_digit(b) => {
                length += 2;
            }
            _ => {
                break;
            }
        }
    }

    length
}

fn read_until_end_of_variable_interpolation(input: &Input, from: usize) -> usize {
    let mut until_offset = from;
    loop {
        match input.peek(until_offset, 4) {
            [part_of_identifier!(), ..] => {
                until_offset += 1;
            }
            [b'[', ..] => {
                until_offset += 1;

                let mut nesting = 0;
                loop {
                    match input.peek(until_offset, 1) {
                        [b']', ..] => {
                            until_offset += 1;

                            if nesting == 0 {
                                break;
                            } else {
                                nesting -= 1;
                            }
                        }
                        [b'[', ..] => {
                            until_offset += 1;
                            nesting += 1;
                        }
                        [byte, ..] => {
                            if byte.is_ascii_whitespace() {
                                // we don't want to include whitespaces in here
                                // see: https://3v4l.org/7Cp3H
                                break;
                            }

                            until_offset += 1;
                        }
                        [] => {
                            break;
                        }
                    }
                }

                break;
            }
            [b'-', b'>', start_of_identifier!(), ..] => {
                until_offset += 3;
                loop {
                    if matches!(input.peek(until_offset, 1), [part_of_identifier!()]) {
                        until_offset += 1;
                    } else {
                        break;
                    }
                }

                break;
            }
            [b'?', b'-', b'>', start_of_identifier!(), ..] => {
                until_offset += 4;
                loop {
                    if matches!(input.peek(until_offset, 1), [part_of_identifier!()]) {
                        until_offset += 1;
                    } else {
                        break;
                    }
                }

                break;
            }
            [..] => {
                break;
            }
        }
    }

    until_offset
}

fn read_until_end_of_brace_interpolation(input: &Input, from: usize) -> usize {
    let mut until_offset = from;
    let mut nesting = 0;
    loop {
        match input.peek(until_offset, 1) {
            [b'}', ..] => {
                until_offset += 1;

                if nesting == 0 {
                    break;
                } else {
                    nesting -= 1;
                }
            }
            [b'{', ..] => {
                until_offset += 1;
                nesting += 1;
            }
            [_, ..] => {
                until_offset += 1;
            }
            [] => {
                break;
            }
        }
    }

    until_offset
}
