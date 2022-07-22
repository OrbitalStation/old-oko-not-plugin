use super::*;

#[derive(Debug, Clone)]
pub struct CallExpr {
    pub fun: Ident,
    pub args: Punctuated <Expr, ','>
}

impl Parse for CallExpr {
    fn parse(stream: &mut ParseStream) -> Result <Self> {
        let fun = stream.ident()?;
        stream.punct("(")?;
        let args = Punctuated::parse(stream)?;
        stream.punct(")")?;

        Ok(Self {
            fun,
            args
        })
    }
}
