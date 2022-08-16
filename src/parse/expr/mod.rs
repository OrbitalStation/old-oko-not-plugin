crate::define_parseable_enum! {
    NAME = Expr

    FIELDS:

    Call
    Block
}

impl Expr {
    pub fn single_or_block(stream: &mut ParseStream) -> Result <Self> {
        Ok(if stream.punct("=").is_ok() {
            Expr::parse(stream)?
        } else {
            Expr::Block(Box::new(BlockExpr::parse(stream)?))
        })
    }
}
