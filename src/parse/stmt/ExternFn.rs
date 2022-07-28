use super::*;
use super::super::signature::Signature;
use crate::span::Span;
use core::fmt::{Debug, Write, Formatter, Result as FmtResult};

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum FFILanguage {
    C
}

impl Debug for FFILanguage {
    fn fmt(&self, f: &mut Formatter <'_>) -> FmtResult {
        f.write_str(match self {
            Self::C => "clang"
        })
    }
}

impl Parse for FFILanguage {
    fn parse(stream: &mut ParseStream) -> Result <Self> {
        if stream.keyword("clang").is_ok() {
            return Ok(Self::C)
        }

        Err(ParseStreamError {
            span: Span::with_extra_column(stream.cursor, 1),
            parsing_depth: stream.depth,
            expected: String::from("`clang`"),
            help: vec![]
        })
    }
}

#[derive(Clone)]
pub struct ExternFnStmt {
    pub lang: FFILanguage,
    pub sig: Signature
}

impl Debug for ExternFnStmt {
    fn fmt(&self, f: &mut Formatter <'_>) -> FmtResult {
        f.write_str("extern ")?;
        f.write_fmt(format_args!("{:?}", self.lang))?;
        f.write_char(' ')?;
        f.write_fmt(format_args!("{:?}", self.sig))?;

        Ok(())
    }
}

impl Parse for ExternFnStmt {
    fn parse(stream: &mut ParseStream) -> Result <Self> {
        stream.keyword("extern")?;
        let lang = FFILanguage::parse(stream)?;
        let sig = Signature::parse(stream)?;

        Ok(Self {
            lang,
            sig
        })
    }
}
