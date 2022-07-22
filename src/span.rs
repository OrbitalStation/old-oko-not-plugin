use core::fmt::{Debug, Result, Formatter};

///
/// The position of an item in file
///
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CursorPosition {
    ///
    /// Starts from 1
    ///
    pub line: usize,

    ///
    /// Starts from 1
    ///
    pub column: usize
}

impl Debug for CursorPosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_fmt(format_args!("{}:{}", self.line, self.column))
    }
}

impl CursorPosition {
    pub const EOF: CursorPosition = CursorPosition {
        line: 0,
        column: 0
    };

    pub const fn default() -> Self {
        Self {
            line: 1,
            column: 1
        }
    }

    pub const fn extend_column_by(self, len: usize) -> Self {
        Self {
            line: self.line,
            column: self.column + len
        }
    }
}

/// The precise position of an item in the file
#[derive(Copy, Clone)]
pub struct Span {
    ///
    /// The pointer to first symbol of an item
    ///
    pub start: CursorPosition,

    ///
    /// The pointer to the symbol *after* the last symbol of an item
    ///
    pub end: CursorPosition
}

impl Debug for Span {
    fn fmt(&self, f: &mut Formatter <'_>) -> Result {
        f.write_fmt(format_args!("Span({:?}..{:?})", self.start, self.end))
    }
}

impl Span {
    pub const EOF: Span = Span {
        start: CursorPosition::EOF,
        end: CursorPosition::EOF
    };

    pub const fn with_extra_column(cursor: CursorPosition, extra_column: usize) -> Self {
        Self {
            start: cursor,
            end: cursor.extend_column_by(extra_column)
        }
    }

    pub fn apply(self, code: &str) -> String {
        if self.start == CursorPosition::EOF {
            return String::from("<EOF>")
        }

        let mut lines = code
            .lines()
            .skip(self.start.line - 1)
            .take(self.end.line - self.start.line + 1)
            .collect::<Vec <_>>();

        lines[0] = &lines[0][self.start.column - 1..];

        let last = lines.last_mut().expect("no lines in span");
        *last = &last[..(self.end.column - 1).min(last.len())];
        if let Some(idx) = last.find(char::is_whitespace) {
            *last = &last[..idx]
        }

        lines.join("\n")
    }
}
