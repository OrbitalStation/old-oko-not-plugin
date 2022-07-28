use core::fmt::{Debug, Result as FmtResult, Formatter};
use crate::span::{Span, CursorPosition};
use super::stream::{Parse, ParseStream, Result as StreamResult, ParseStreamError};

///
/// A string value with a span attached to it
///
#[derive(Clone)]
pub struct Ident {
    pub name: String,
    start: CursorPosition
}

impl Parse for Ident {
    ///
    /// Tries to parse an identifier from the stream.
    ///
    /// It is assumed that the word is parsed successfully
    ///   if the current stream cursor points at the `word` itself
    ///   (excluding all the whitespaces before) and there are no
    ///   alphanumerical symbols *after*
    ///
    /// # Examples
    ///
    /// The `ident` = `human`
    ///
    /// This could be parsed successfully:
    ///
    /// `human is an animal`
    ///
    /// And this too:
    ///
    /// `       human is an animal even with whitespaces before it`
    ///
    /// But this could *not*..:
    ///
    /// `humans must die`
    ///
    /// ...because of the `s` after the word
    ///
    fn parse(stream: &mut ParseStream) -> StreamResult <Self> {
        stream.trim();

        let first_char_len = match stream.code.chars().next() {
            // First character is alphabetic
            Some(char) if char.is_alphabetic() => char.len_utf8(),

            // Faced non-alphabetic symbol; error
            Some(_) => return Err(ParseStreamError {
                span: Span::with_extra_column(stream.cursor, 1),
                parsing_depth: stream.depth,
                expected: String::from("ident"),
                help: vec![]
            }),

            // Unexpected EOF in place of an ident
            None => return Err(ParseStreamError {
                span: Span::EOF,
                parsing_depth: stream.depth,
                expected: String::from("ident"),
                help: vec![]
            })
        };


        let end = stream.code[first_char_len..].find(|char: char| !char.is_alphanumeric()).unwrap_or(stream.code.len() - 1) + 1;

        let ident = stream.code[..end].to_string();

        let cursor = stream.cursor;

        stream.offset_by(ident.len());

        Ok(Self::new(ident, cursor))
    }
}

impl Debug for Ident {
    fn fmt(&self, f: &mut Formatter <'_>) -> FmtResult {
        f.debug_struct("Ident")
            .field("name", &self.name)
            .field("span", &self.span())
            .finish()
    }
}

impl Ident {
    pub const fn new(name: String, start: CursorPosition) -> Self {
        Self {
            name,
            start
        }
    }

    pub fn span(&self) -> Span {
        Span {
            start: self.start,
            end: self.start.extend_column_by(self.name.len())
        }
    }
}

///
/// A `"`-enclosed string
///
#[derive(Clone)]
pub struct DoubleQuotedString {
    pub value: String,
    start: CursorPosition
}

impl Debug for DoubleQuotedString {
    fn fmt(&self, f: &mut Formatter <'_>) -> FmtResult {
        f.debug_struct("DoubleQuotedString")
            .field("value", &self.value)
            .field("span", &self.span())
            .finish()
    }
}

impl DoubleQuotedString {
    pub const fn new(value: String, start: CursorPosition) -> Self {
        Self {
            value,
            start
        }
    }

    pub fn span(&self) -> Span {
        Span {
            start: self.start,
            // `+2` is extra symbols for `"`s
            end: self.start.extend_column_by(self.value.len() + 2)
        }
    }
}
