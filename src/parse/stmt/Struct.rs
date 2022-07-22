use super::*;
use super::super::typed_variable::TypedVariable;
use super::super::punctuated::Punctuated;

#[derive(Debug, Clone)]
pub struct StructStmt {
    pub name: Ident,
    pub fields: Punctuated <TypedVariable, ','>
}

impl Parse for StructStmt {
    fn parse(stream: &mut ParseStream) -> Result <Self> {
        stream.keyword("struct")?;
        let name = stream.ident()?;
        let fields = stream.embraced_in_figures_or_single()?;

        Ok(Self {
            name,
            fields
        })
    }
}
