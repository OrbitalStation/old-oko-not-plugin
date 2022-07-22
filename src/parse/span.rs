use core::fmt::{Debug, Result, Formatter};
use crate::span::{Span, CursorPosition};

///
/// A string value with a span attached to it
///
#[derive(Clone)]
pub struct Ident {
    pub name: String,
    start: CursorPosition
}

impl Debug for Ident {
    fn fmt(&self, f: &mut Formatter <'_>) -> Result {
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
    fn fmt(&self, f: &mut Formatter <'_>) -> Result {
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
