use super::span::{Ident, DoubleQuotedString};
use crate::span::{CursorPosition, Span};
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
    code: &'a str,
    cursor: CursorPosition,
    depth: ParsingDepth
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
        let applied = self.span.apply(code);

        Error {
            span: self.span,
            message: format!("unexpected token `{applied}`"),
            lines: applied,
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

    pub fn is_empty(&self) -> bool {
        self.code.is_empty()
    }

    pub fn double_quoted_string(&mut self) -> Result <DoubleQuotedString> {
        self.enclosed('"').map(|(value, start)| DoubleQuotedString::new(value, start))
    }

    pub fn enclosed(&mut self, delimiter: char) -> Result <(String, CursorPosition)> {
        self.trim();

        if self.code.starts_with(delimiter) {
            let start = self.cursor;

            self.offset_by(delimiter.len_utf8());

            let mut previous_was_escaping_sign = false;

            let mut offset = 0;

            for char in self.code.chars() {
                if char == ESCAPING_SIGN {
                    previous_was_escaping_sign = true
                } else {
                    if char == delimiter && !previous_was_escaping_sign {
                        let string = self.code[..offset].to_string();

                        self.offset_by(offset + delimiter.len_utf8());

                        return Ok((string, start))
                    }

                    previous_was_escaping_sign = false;
                }

                offset += char.len_utf8()
            }

            self.offset_by(self.code.len() - 1);

            return Err(ParseStreamError {
                span: Span::EOF,
                parsing_depth: self.depth,
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

    ///
    /// Tries to parse a punctuation token from the stream
    ///
    pub fn punct(&mut self, punct: &str) -> Result <()> {
        self.trim();

        let non_punct_symbol = self.code.find(|char: char| !char.is_ascii_punctuation()).unwrap_or(self.code.len());

        if non_punct_symbol == 0 {
            Err(ParseStreamError {
                span: Span::with_extra_column(self.cursor, 1),
                parsing_depth: self.depth,
                expected: format!("`{punct}`"),
                help: vec![]
            })
        } else {
            let parsed = &self.code[..non_punct_symbol];

            if punct == parsed {
                self.offset_by(non_punct_symbol);
                Ok(())
            } else {
                Err(ParseStreamError {
                    span: Span::with_extra_column(self.cursor, parsed.len()),
                    parsing_depth: self.depth,
                    expected: format!("`{punct}`"),
                    help: vec![]
                })
            }
        }
    }

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
    pub fn ident(&mut self) -> Result <Ident> {
        self.trim();

        let first_char_len = match self.code.chars().next() {
            // First character is alphabetic
            Some(char) if char.is_alphabetic() => char.len_utf8(),

            // Faced non-alphabetic symbol; error
            Some(_) => return Err(ParseStreamError {
                span: Span::with_extra_column(self.cursor, 1),
                parsing_depth: self.depth,
                expected: String::from("ident"),
                help: vec![]
            }),

            // Unexpected EOF in place of an ident
            None => return Err(ParseStreamError {
                span: Span::EOF,
                parsing_depth: self.depth,
                expected: String::from("ident"),
                help: vec![]
            })
        };


        let end = self.code[first_char_len..].find(|char: char| !char.is_alphanumeric()).unwrap_or(self.code.len() - 1) + 1;

        let ident = self.code[..end].to_string();

        let cursor = self.cursor;

        self.offset_by(ident.len());

        Ok(Ident::new(ident, cursor))
    }

    ///
    /// Tries to parse a keyword `keyword` from the stream.
    ///
    /// Behaves similar to [`ParseStream::ident`]
    ///
    pub fn keyword(&mut self, keyword: &str) -> Result <()> {
        match self.ident() {
            Ok(Ident { name, .. }) if name == keyword => Ok(()),
            Ok(ident) => Err(ParseStreamError {
                span: ident.span(),
                parsing_depth: self.depth,
                expected: format!("keyword `{keyword}`"),
                help: vec![]
            }),
            Err(err) => Err(err.with_custom_expected(format!("keyword `{keyword}`")))
        }
    }
}

impl <'a> ParseStream <'a> {
    pub fn trim(&mut self) {
        self.offset_by(self.code.find(|char: char| !char.is_whitespace()).unwrap_or(self.code.len()));
    }

    fn offset_by(&mut self, offset: usize) {
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
