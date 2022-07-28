use super::span::Ident;
use super::stream::{Parse, ParseStream, Result, ParseStreamError};
use crate::span::Span;
use core::fmt::{Debug, Formatter, Result as FmtResult};
use std::fmt::Write;

#[derive(Copy, Clone)]
pub struct Ptr {
    pub len: u8,
    pub muts: u8
}

impl Debug for Ptr {
    fn fmt(&self, f: &mut Formatter <'_>) -> FmtResult {
        for i in 0..self.len {
            f.write_char('*')?;

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

impl Parse for Ptr {
    fn parse(stream: &mut ParseStream) -> Result <Self> {
        let mut len = 0;
        let mut muts = 0b0000_0000;

        while stream.punct("*").is_ok() {
            if len >= u8::BITS as u8 {
                return Err(ParseStreamError {
                    span: Span::with_shrunk_column(stream.cursor, 1),
                    parsing_depth: stream.depth,
                    expected: String::from("not an extra `*` token"),
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

#[derive(Debug, Clone)]
pub struct Type {
    pub ptr: Ptr,
    pub name: Ident
}

impl Parse for Type {
    fn parse(stream: &mut ParseStream) -> Result <Self> {
        let ptr = Ptr::parse(stream)?;
        let name = Ident::parse(stream)?;

        Ok(Self {
            ptr,
            name
        })
    }
}
