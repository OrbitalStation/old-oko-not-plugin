use super::span::Ident;
use super::stream::{Parse, ParseStream, Result};

#[derive(Debug, Clone)]
pub struct Type {
    pub name: Ident
}

impl Parse for Type {
    fn parse(stream: &mut ParseStream) -> Result <Self> {
        let name = stream.ident()?;

        Ok(Self {
            name
        })
    }
}
