use super::span::Ident;
use super::ty::Type;
use super::stream::{Parse, ParseStream, Result};

#[derive(Debug, Clone)]
pub struct TypedVariable {
    pub name: Ident,
    pub ty: Type
}

impl Parse for TypedVariable {
    fn parse(stream: &mut ParseStream) -> Result <Self> {
        let name = stream.ident()?;
        stream.punct(":")?;
        let ty = Type::parse(stream)?;
        
        Ok(Self {
            name,
            ty
        })
    }
}
