use super::span::{Ident, DoubleQuotedString};
use crate::span::{CursorPosition, Span};
use super::punctuated::Punctuated;
use crate::error::Error;

pub type Result <T> = core::result::Result <T, ParseStreamError>;

pub trait Parse: Sized {
    fn parse(stream: &mut ParseStream) -> Result <Self>;
}

///
/// Just a convenience newtype
///
/// Indicates how many characters have been parsed up to the present moment
///
#[derive(Debug, Copy, Clone)]
pub struct ParsingDepth(pub usize);

pub const LINE_SEPARATOR: char = '\n';

pub const ESCAPING_SIGN: char = '\\';

#[derive(Clone)]
pub struct ParseStream <'a> {
    pub code: &'a str,
    pub cursor: CursorPosition,
    pub depth: ParsingDepth
}

#[derive(Clone, Debug)]
pub struct ParseStreamError {
    ///
    /// The span of the problematic place in the code
    ///
    pub span: Span,

    ///
    /// How many symbols were parsed before the failure
    ///
    pub parsing_depth: ParsingDepth,

    ///
    /// What was expected at this place
    ///
    pub expected: String,

    ///
    /// Optional help(s) that might be useful to user
    ///
    pub help: Vec <String>
}

impl ParseStreamError {
    pub fn to_error(self, code: &str, filename: String) -> Error {
        let spanned = self.span.apply(code);

        Error {
            span: self.span.eof_or(code),
            message: format!("unexpected token `{spanned}`"),
            spanned,
            clarifying: format!("expected {}", self.expected),
            help: self.help,
            filename,
            code: code.to_string()
        }
    }

    pub fn with_custom_expected(mut self, expected: String) -> Self {
        self.expected = expected;
        self
    }
}

impl <'a> ParseStream <'a> {
    pub const fn new(code: &'a str) -> Self {
        Self {
            code,
            cursor: CursorPosition::default(),
            depth: ParsingDepth(0)
        }
    }

    pub fn number_u8(&mut self) -> Result <u8> {
        self.trim();

        let non_numeric = self.code.find(|char: char| !char.is_numeric()).unwrap_or(self.code.len());
        return if let Ok(ok) = self.code[..non_numeric].parse::<u8>() {
            self.offset_by(ok.to_string().len());
            Ok(ok)
        } else {
            Err(ParseStreamError {
                span: Span::with_extra_column(self.cursor, 1),
                parsing_depth: self.depth,
                expected: String::from("an `u8` number"),
                help: vec![]
            })
        }

    }

    pub fn newline(&mut self) -> Result <()> {
        let newline = self.code.find(|char: char| char == LINE_SEPARATOR).ok_or_else(|| ParseStreamError {
            span: Span::with_extra_column(self.cursor, self.code.len()),
            parsing_depth: self.depth,
            expected: String::from("newline"),
            help: vec![]
        })?;

        if let Some(wrong) = self.code[..newline].find(|char: char| !char.is_whitespace()) {
            Err(ParseStreamError {
                span: Span::with_extra_column(self.cursor.extend_column_by(wrong), 1),
                parsing_depth: self.depth,
                expected: String::from("newline"),
                help: vec![]
            })
        } else {
            self.offset_by(newline);
            Ok(())
        }
    }

    pub fn one_or_more <T: Parse> (&mut self) -> Result <Vec <T>> {
        self.trim();

        let mut result = vec![T::parse(self)?];

        while let Ok(x) = T::parse(self) {
            result.push(x)
        }

        Ok(result)
    }

    pub fn parse_until_error <T: Parse> (&mut self) -> Result <Vec <T>> {
        self.trim();

        let mut result = vec![];

        while let Ok(x) = T::parse(self) {
            result.push(x)
        }

        Ok(result)
    }

    pub fn is_empty(&self) -> bool {
        self.code.is_empty()
    }

    // pub fn embraced_in_figures_or_single <T: Parse, const P: char, const IS_ZERO_ALLOWED: bool> (&mut self) -> Result <Punctuated <T, P, IS_ZERO_ALLOWED>> {
    //     self.trim();
    //
    //     if self.punct("=").is_ok() {
    //         return Ok(Punctuated(vec![T::parse(self)?]))
    //     }
    //
    //     self.embraced_in_figures()
    // }

    // pub fn embraced_in_figures <T: Parse, const P: char, const IS_ZERO_ALLOWED: bool> (&mut self) -> Result <Punctuated <T, P, IS_ZERO_ALLOWED>> {
    //     self.embraced::<T, P, IS_ZERO_ALLOWED>('{', '}')
    // }

    pub fn embraced <T: Parse, const P: char, const IS_ZERO_ALLOWED: bool> (&mut self, open_delim: char, close_delim: char) -> Result <Punctuated <T, P, IS_ZERO_ALLOWED>> {
        self.trim();

        if self.code.starts_with(open_delim) {
            let mut clone = self.clone();

            clone.offset_by(open_delim.len_utf8());

            let punctuated = Punctuated::parse(&mut clone)?;

            clone.trim();

            return if clone.code.starts_with(close_delim) {
                clone.offset_by(close_delim.len_utf8());
                *self = clone;
                Ok(punctuated)
            } else {
                Err(ParseStreamError {
                    span: Span::EOF,
                    parsing_depth: self.depth,
                    expected: format!("the second closing delimiter - `{close_delim}`"),
                    help: vec![]
                })
            }

        }

        Err(ParseStreamError {
            span: Span::with_extra_column(self.cursor, 1),
            parsing_depth: self.depth,
            expected: format!("`{open_delim} ... {close_delim}` block"),
            help: vec![]
        })
    }

    pub fn double_quoted_string(&mut self) -> Result <DoubleQuotedString> {
        self.enclosed('"').map(|(value, start)| DoubleQuotedString::new(value, start))
    }

    pub fn enclosed(&mut self, delimiter: char) -> Result <(String, CursorPosition)> {
        self.trim();

        if self.code.starts_with(delimiter) {
            let mut clone = self.clone();

            clone.offset_by(delimiter.len_utf8());

            let mut previous_was_escaping_sign = false;

            let mut offset = 0;

            for char in clone.code.chars() {
                if char == ESCAPING_SIGN {
                    previous_was_escaping_sign = true
                } else {
                    if char == delimiter && !previous_was_escaping_sign {
                        let string = clone.code[..offset].to_string();

                        clone.offset_by(offset + delimiter.len_utf8());

                        let start = self.cursor;

                        *self = clone;

                        return Ok((string, start))
                    }

                    previous_was_escaping_sign = false;
                }

                offset += char.len_utf8()
            }

            return Err(ParseStreamError {
                span: Span::EOF,
                parsing_depth: ParsingDepth(clone.depth.0 + clone.code.len()),
                expected: format!("the second closing delimiter - `{delimiter}`"),
                help: vec![]
            })
        }

        Err(ParseStreamError {
            span: Span::with_extra_column(self.cursor, 1),
            parsing_depth: self.depth,
            expected: format!("`{delimiter}`-enclosed string"),
            help: vec![]
        })
    }

    pub fn next_punct(&mut self) -> Result <(&str, Span)> {
        macro_rules! puncts {
            ($self:ident, $( $punct:literal )*) => {$(
                if $self.code.starts_with($punct) {
                    let span = Span::with_extra_column(self.cursor, $punct.len());

                    $self.offset_by($punct.len());
                    return Ok(($punct, span))
                }
            )*};
        }

        self.trim();

        puncts! {
            self,

            "=="
            "="

            ":"
            ";"

            "."
            ","

            "("
            ")"

            "["
            "]"

            "{"
            "}"

            "<"
            ">"

            "|"

            "->"

            "+"
            "-"
            "*"
            "/"

            "!"
        }

        Err(ParseStreamError {
            span: Span::with_extra_column(self.cursor, 1),
            parsing_depth: self.depth,
            expected: String::from("a punctuation token"),
            help: vec![]
        })
    }

    ///
    /// Tries to parse a punctuation token from the stream
    ///
    pub fn punct(&mut self, punct: &str) -> Result <()> {
        let mut clone = self.clone();
        match clone.next_punct() {
            Ok((parsed, _)) if parsed == punct => {
                *self = clone;
                Ok(())
            },
            Ok((_, span)) => Err(ParseStreamError {
                span,
                parsing_depth: self.depth,
                expected: format!("`{punct}`"),
                help: vec![]
            }),
            Err(err) => Err(err.with_custom_expected(format!("`{punct}`")))
        }
    }

    ///
    /// Tries to parse a keyword `keyword` from the stream.
    ///
    /// Behaves similar to [`Ident::parse`]
    ///
    pub fn keyword(&mut self, keyword: &str) -> Result <()> {
        let mut clone = self.clone();
        match Ident::parse(&mut clone) {
            Ok(Ident { name, .. }) if name == keyword => {
                *self = clone;
                Ok(())
            },
            Ok(ident) => Err(ParseStreamError {
                span: ident.span(),
                parsing_depth: self.depth,
                expected: format!("keyword `{keyword}`"),
                help: vec![]
            }),
            Err(err) => Err(err.with_custom_expected(format!("keyword `{keyword}`")))
        }
    }

    pub fn trim(&mut self) {
        self.offset_by(self.code.find(|char: char| !char.is_whitespace()).unwrap_or(self.code.len()));
    }

    pub fn offset_by(&mut self, offset: usize) {
        for char in self.code[..offset].chars() {
            if char == LINE_SEPARATOR {
                self.cursor.line += 1;
                self.cursor.column = 1;
            } else {
                self.cursor.column += 1
            }
        }

        self.code = &self.code[offset..];

        self.depth.0 += offset;
    }
}
