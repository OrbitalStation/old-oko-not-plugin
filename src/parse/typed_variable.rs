use super::span::Ident;
use super::ty::Type;
use super::stream::{Parse, ParseStream, Result};

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
#[derive(Debug, Clone)]
pub struct TypedVariables {
    pub name: Vec <Ident>,
    pub ty: Type
}

impl Parse for TypedVariables {
    fn parse(stream: &mut ParseStream) -> Result <Self> {
        let name = stream.one_or_more()?;
        stream.punct(":")?;
        let ty = Type::parse(stream)?;
        
        Ok(Self {
            name,
            ty
        })
    }
}
