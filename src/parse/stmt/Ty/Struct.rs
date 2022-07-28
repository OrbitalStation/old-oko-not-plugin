use super::*;
use super::super::super::typed_variable::TypedVariables;

///
/// The struct variant of the `ty` statement body.
///
/// `ty Vec2 = x: T + y: T`
///
/// `----------^^^^^^^^^^^`
///
/// `ty Vec2 = x y: T`
///
/// `----------^^^^^^`
///
#[derive(Debug, Clone)]
pub struct StructTyStmtBody {
    pub fields: Punctuated <TypedVariables, '+'>
}

impl Parse for StructTyStmtBody {
    fn parse(stream: &mut ParseStream) -> Result <Self> {
        let fields = Punctuated::parse(stream)?;

        Ok(Self {
            fields
        })
    }
}
