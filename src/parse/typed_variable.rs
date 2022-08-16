use super::span::Ident;
use super::ty::Type;
use super::stream::{Parse, ParseStream, Result};
use core::fmt::{Debug, Formatter, Result as FmtResult};
use std::fmt::Write;

///
/// A variable name(s) + type
///
/// `variable: SomeType`
///
/// `^^^^^^^^^^^^^^^^^^`
///
/// Multiple(both `x` and `y` have type `T`)
///
/// `x y: T`
///
/// `^^^^^^`
///
#[derive(Clone)]
pub struct TypedVariables {
    pub names: Vec <Ident>,
    pub ty: Type
}

impl Debug for TypedVariables {
    fn fmt(&self, f: &mut Formatter <'_>) -> FmtResult {
        for ident in self.names.iter().rev().skip(1).rev() {
            f.write_str(&ident.name)?;
            f.write_char(' ')?
        }
        f.write_str(&self.names.last().unwrap().name)?;

        f.write_str(": ")?;
        f.write_fmt(format_args!("{:?}", self.ty))?;

        Ok(())
    }
}

impl Parse for TypedVariables {
    fn parse(stream: &mut ParseStream) -> Result <Self> {
        let mut clone = stream.clone();

        let names = clone.one_or_more()?;
        clone.punct(":")?;
        let ty = Type::parse(&mut clone)?;
        *stream = clone;

        Ok(Self {
            names,
            ty
        })
    }
}
