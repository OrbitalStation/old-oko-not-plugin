use super::punctuated::Punctuated;
use super::typed_variable::TypedVariables;
use super::x_times::XTimes;
use super::ty::Type;
use super::stream::{Parse, ParseStream, ParseStreamError, Result};
use crate::span::Span;
use core::fmt::{Debug, Formatter, Result as FmtResult, Write};

#[derive(Clone)]
#[repr(u8)]
pub enum Arg {
    Named(TypedVariables),
    Unnamed(XTimes <Type, true>)
}

impl Debug for Arg {
    fn fmt(&self, f: &mut Formatter <'_>) -> FmtResult {
        match self {
            Self::Named(named) => named.fmt(f),
            Self::Unnamed(unnamed) => unnamed.fmt(f)
        }
    }
}

impl Parse for Arg {
    fn parse(stream: &mut ParseStream) -> Result <Self> {
        if let Ok(ok) = TypedVariables::parse(stream) {
            return Ok(Self::Named(ok))
        }

        if let Ok(ok) = XTimes::parse(stream) {
            return Ok(Self::Unnamed(ok))
        }

        Err(ParseStreamError {
            span: Span::with_extra_column(stream.cursor, 1),
            parsing_depth: stream.depth,
            expected: String::from("either named(x: T or x y <...>: T) or unnamed(T or T x N) parameters"),
            help: vec![]
        })
    }
}

#[derive(Clone)]
pub struct Signature {
    pub args: Punctuated <Arg, ',', true>,
    pub return_ty: Option <Type>
}

impl Debug for Signature {
    fn fmt(&self, f: &mut Formatter <'_>) -> FmtResult {
        f.write_char('(')?;
        self.args.debug(f)?;
        f.write_char(')')?;

        if let Some(ty) = &self.return_ty {
            f.write_str(" -> ")?;
            ty.fmt(f)?
        }

        Ok(())
    }
}

impl Parse for Signature {
    fn parse(stream: &mut ParseStream) -> Result <Self> {
        let args = stream.embraced('(', ')')?;

        let return_ty = if stream.punct("->").is_ok() {
            Some(Type::parse(stream)?)
        } else {
            None
        };

        Ok(Self {
            args,
            return_ty
        })
    }
}
