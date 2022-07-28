use super::span::Ident;
use super::punctuated::Punctuated;
use super::typed_variable::TypedVariables;
use super::ty::Type;
use super::stream::{Parse, ParseStream, Result};
use core::fmt::{Debug, Formatter, Result as FmtResult};
use std::fmt::Write;

#[derive(Clone)]
pub struct Signature {
    pub name: Ident,
    pub args: Punctuated <TypedVariables, ','>,
    pub return_ty: Option <Type>
}

impl Debug for Signature {
    fn fmt(&self, f: &mut Formatter <'_>) -> FmtResult {
        f.write_str("fn ")?;
        f.write_str(&self.name.name)?;
        f.write_char('(')?;
        self.args.debug(f)?;
        f.write_char(')')?;

        if let Some(ty) = &self.return_ty {
            f.write_str(" -> ")?;
            f.write_fmt(format_args!("{:?}", ty))?
        }

        Ok(())
    }
}

impl Parse for Signature {
    fn parse(stream: &mut ParseStream) -> Result <Self> {
        stream.keyword("fn")?;
        let name = Ident::parse(stream)?;
        let args = stream.embraced('(', ')')?;

        let return_ty = if stream.punct("->").is_ok() {
            Some(Type::parse(stream)?)
        } else {
            None
        };

        Ok(Self {
            name,
            args,
            return_ty
        })
    }
}
