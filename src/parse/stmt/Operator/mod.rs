crate::define_parseable_enum! {
    NAME = OperatorStmtBody

    FIELDS:

    Literal
    /* Binary */
    /* Unary */
}

#[derive(Debug, Clone)]
pub struct OperatorStmt {
    pub body: OperatorStmtBody
}

impl Parse for OperatorStmt {
    fn parse(stream: &mut ParseStream) -> Result <Self> {
        stream.keyword("operator")?;
        let body = OperatorStmtBody::parse(stream)?;

        Ok(Self {
            body
        })
    }
}
