use crate::parse::expr::Expr;
use crate::parse::signature::{Arg, Signature};
use crate::parse::ty::{Ptrs, Refs, Type};
use crate::span::Span;
use super::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum Affix {
    Prefix,
    Suffix
}

impl Parse for Affix {
    fn parse(stream: &mut ParseStream) -> Result <Self> {
        if stream.keyword("prefix").is_ok() {
            return Ok(Self::Prefix)
        }

        stream.keyword("suffix")?;
        Ok(Self::Suffix)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum LiteralType {
    /// "hi there"
    String,

    /// 'k'
    Char,

    /// 1.0 -4.864 0.865 -0.314
    Float,

    /// 27 2 5 -3 1 -115 0
    Int
}

#[derive(Debug, Clone)]
pub struct LiteralMacroStmtBody {
    pub affix: Affix,
    pub lit: DoubleQuotedString,
    pub ty: LiteralType,
    pub return_ty: Option <Type>,
    pub body: Expr
}

impl Parse for LiteralMacroStmtBody {
    fn parse(stream: &mut ParseStream) -> Result <Self> {
        let affix = Affix::parse(stream)?;
        let lit = stream.double_quoted_string()?;

        let (ty, return_ty) = {
            let sig = Signature::parse(stream)?;

            if sig.args.0.len() != 1 {
                return Err(ParseStreamError {
                    span: if sig.args.0.is_empty() {
                        Span::with_extra_column(lit.span().end, 1)
                    } else {
                        match &sig.args.0[1] {
                            Arg::Named(named) => named.span(),
                            Arg::Unnamed(unnamed) => unnamed.it.span
                        }
                    },
                    parsing_depth: stream.depth,
                    expected: String::from("only one argument for an affix operator"),
                    help: vec![]
                })
            }

            let ty = match &sig.args.0[0] {
                Arg::Named(typed) => {
                    if typed.names.len() != 1 {
                        return Err(ParseStreamError {
                            span: typed.span(),
                            parsing_depth: stream.depth,
                            expected: String::from("only one argument for an affix operator"),
                            help: vec![]
                        })
                    }

                    &typed.ty
                }
                Arg::Unnamed(unnamed) => {
                    if unnamed.times != 1 {
                        return Err(ParseStreamError {
                            span: unnamed.it.span,
                            parsing_depth: stream.depth,
                            expected: String::from("only one argument for an affix operator"),
                            help: vec![]
                        })
                    }

                    &unnamed.it
                }
            };

            let ty = if ty.refs == Refs::SINGLE && ty.ptrs == Ptrs::ZERO && ty.name.name == "str" {
                LiteralType::String
            } else if ty.is_pure() && ty.name.name == "char" {
                LiteralType::Char
            } else if ty.is_pure() && ty.name.name == "float" {
                LiteralType::Float
            } else if ty.is_pure() && ty.name.name == "int" {
                LiteralType::Int
            } else {
                return Err(ParseStreamError {
                    span: ty.span,
                    parsing_depth: stream.depth,
                    expected: String::from("one of &str, char, float, int"),
                    help: vec![]
                })
            };

            (ty, sig.return_ty)
        };

        let body = Expr::single_or_block(stream)?;

        Ok(Self {
            affix,
            lit,
            ty,
            return_ty,
            body
        })
    }
}
