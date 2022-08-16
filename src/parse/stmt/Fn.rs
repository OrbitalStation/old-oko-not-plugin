use core::fmt::{Debug, Formatter, Result as FmtResult};
use std::fmt::Write;
use crate::parse::signature::Signature;
use crate::parse::expr::Expr;
use super::*;

#[derive(Clone)]
pub struct FnStmt {
    pub name: Ident,
    pub sig: Signature,
    pub body: Expr
}

impl Debug for FnStmt {
    fn fmt(&self, f: &mut Formatter <'_>) -> FmtResult {
        f.write_str("fn ")?;
        f.write_str(&self.name.name)?;
        self.sig.fmt(f)?;
        f.write_char(' ')?;
        self.body.fmt(f)?;

        Ok(())
    }
}

impl Parse for FnStmt {
    fn parse(stream: &mut ParseStream) -> Result <Self> {
        stream.keyword("fn")?;
        let name = Ident::parse(stream)?;
        let sig = Signature::parse(stream)?;
        let body = Expr::single_or_block(stream)?;

        Ok(Self {
            name,
            sig,
            body
        })
    }
}
