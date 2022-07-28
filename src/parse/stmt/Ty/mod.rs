crate::define_parseable_enum! {
    ///
    /// The `ty` statement body.
    ///
    /// `ty Option = None | Some i32`
    ///
    /// `------------^^^^^^^^^^^^^^^`
    ///
    /// `ty Vec2 = x: T + y: T`
    ///
    /// `----------^^^^^^^^^^^`
    ///

    NAME = TyStmtBody

    FIELDS:

    Struct
    Enum
}

///
/// The `ty` statement.
///
/// `ty Option = None | Some i32`
///
/// `^^^^^^^^^^^^^^^^^^^^^^^^^^^`
///
#[derive(Debug, Clone)]
pub struct TyStmt {
    pub name: Ident,
    pub body: TyStmtBody
}

impl Parse for TyStmt {
    fn parse(stream: &mut ParseStream) -> Result <Self> {
        stream.keyword("ty")?;
        let name = Ident::parse(stream)?;
        stream.punct("=")?;
        let body = TyStmtBody::parse(stream)?;

        Ok(Self {
            name,
            body
        })
    }
}
