use crate::parse::signature::Signature;
use super::*;

#[derive(Debug, Clone)]
pub struct FnStmt {
    pub sig: Signature
}

impl Parse for FnStmt {
    fn parse(stream: &mut ParseStream) -> Result <Self> {
        let sig = Signature::parse(stream)?;

        Ok(Self {
            sig
        })
    }
}
