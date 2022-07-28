use super::*;

#[derive(Debug, Clone)]
pub struct FnStmt {
    pub name: Ident,
    pub literal: DoubleQuotedString
}

impl Parse for FnStmt {
    fn parse(stream: &mut ParseStream) -> Result <Self> {
        stream.keyword("fn")?;
        let name = Ident::parse(stream)?;
        stream.punct("=")?;
        let literal = stream.double_quoted_string()?;

        Ok(Self {
            name,
            literal
        })
    }
}
