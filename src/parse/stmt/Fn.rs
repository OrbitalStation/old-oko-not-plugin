use crate::parse::signature::Signature;
use crate::parse::expr::{Expr, BlockExpr};
use super::*;

#[derive(Debug, Clone)]
pub struct FnStmt {
    pub sig: Signature,
    pub body: Expr
}

impl Parse for FnStmt {
    fn parse(stream: &mut ParseStream) -> Result <Self> {
        let sig = Signature::parse(stream)?;

        let body = if stream.punct("=").is_ok() {
            Expr::parse(stream)?
        } else {
            Expr::Block(Box::new(BlockExpr::parse(stream)?))
        };

        Ok(Self {
            sig,
            body
        })
    }
}
