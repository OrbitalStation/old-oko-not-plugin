use super::span::Ident;
use super::stream::{Parse, ParseStream, Result, ParseStreamError};
use crate::span::Span;
use core::fmt::{Debug, Formatter, Result as FmtResult, Write};

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Muts <const SYMBOL: char> {
    pub len: u8,
    pub muts: u8
}

impl <const SYMBOL: char> Muts <SYMBOL> {
    pub const ZERO: Muts <SYMBOL> = Muts {
        len: 0,
        muts: 0
    };

    pub const SINGLE: Muts <SYMBOL> = Muts {
        len: 1,
        muts: 0
    };
}

impl <const SYMBOL: char> Debug for Muts <SYMBOL> {
    fn fmt(&self, f: &mut Formatter <'_>) -> FmtResult {
        for i in 0..self.len {
            f.write_char(SYMBOL)?;

            let is_mutable = ((self.muts >> i) & 1) != 0;

            if is_mutable {
                f.write_str("mut")?;

                if i + 1 != self.len {
                    f.write_char(' ')?
                }
            }
        }

        Ok(())
    }
}

impl <const SYMBOL: char> Parse for Muts <SYMBOL> {
    fn parse(stream: &mut ParseStream) -> Result <Self> {
        let mut len = 0;
        let mut muts = 0b0000_0000;

        let mut buf = [0; 4];
        let symbol = SYMBOL.encode_utf8(&mut buf);

        while stream.punct(symbol).is_ok() {
            if len >= u8::BITS as u8 {
                return Err(ParseStreamError {
                    span: Span::with_shrunk_column(stream.cursor, 1),
                    parsing_depth: stream.depth,
                    expected: format!("not an extra `{SYMBOL}` token"),
                    help: vec![]
                })
            }

            if stream.keyword("mut").is_ok() {
                muts |= 1 << len;
            }

            len += 1
        }

        Ok(Self {
            len,
            muts
        })
    }
}

pub type Ptrs = Muts <'*'>;

pub type Refs = Muts <'&'>;

#[derive(Clone)]
pub struct Type {
    pub refs: Refs,
    pub ptrs: Ptrs,
    pub name: Ident,
    pub span: Span
}

impl Type {
    pub fn is_pure(&self) -> bool {
        self.refs == Refs::ZERO && self.ptrs == Ptrs::ZERO
    }
}

impl Debug for Type {
    fn fmt(&self, f: &mut Formatter <'_>) -> FmtResult {
        self.refs.fmt(f)?;
        self.ptrs.fmt(f)?;
        f.write_str(&self.name.name)?;

        Ok(())
    }
}

impl Parse for Type {
    fn parse(stream: &mut ParseStream) -> Result <Self> {
        let start = stream.cursor;

        let refs = Refs::parse(stream)?;
        let ptrs = Ptrs::parse(stream)?;
        let name = Ident::parse(stream)?;
        let span = Span {
            start,
            end: stream.cursor
        };

        Ok(Self {
            refs,
            ptrs,
            name,
            span
        })
    }
}
