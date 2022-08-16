use super::*;

#[derive(Debug, Clone)]
pub struct BlockExpr {
    pub expressions: Vec <Expr>
}

impl Parse for BlockExpr {
    fn parse(stream: &mut ParseStream) -> Result <Self> {
        stream.punct("{")?;
        let expressions = stream.parse_until_error()?;
        stream.punct("}")?;

        Ok(Self {
            expressions
        })
    }
}
