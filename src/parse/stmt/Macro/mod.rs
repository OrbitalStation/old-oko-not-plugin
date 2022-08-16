crate::define_parseable_enum! {
    NAME = MacroStmtBody

    FIELDS:

    Literal
}

#[derive(Debug, Clone)]
pub struct MacroStmt {
    pub body: MacroStmtBody
}

impl Parse for MacroStmt {
    fn parse(stream: &mut ParseStream) -> Result <Self> {
        stream.keyword("macro")?;
        let body = MacroStmtBody::parse(stream)?;

        Ok(Self {
            body
        })
    }
}
