use super::*;
use super::super::super::typed_variable::TypedVariable;

///
/// The field or set of same-typed fields of the struct variant of the `ty` statement body.
///
/// `ty Vec2 = x: T + y: T`
///
/// `----------^^^^`
///
/// `-----------------^^^^`
///
#[derive(Debug, Clone)]
#[repr(u8)]
pub enum Fields {
    Single(TypedVariable)
}

impl Parse for Fields {
    fn parse(stream: &mut ParseStream) -> Result <Self> {
        Ok(Self::Single(TypedVariable::parse(stream)?))
    }
}

///
/// The struct variant of the `ty` statement body.
///
/// `ty Vec2 = x: T + y: T`
///
/// `----------^^^^^^^^^^^`
///
#[derive(Debug, Clone)]
pub struct StructTyStmtBody {
    pub fields: Punctuated <Fields, '+'>
}

impl Parse for StructTyStmtBody {
    fn parse(stream: &mut ParseStream) -> Result <Self> {
        let fields = Punctuated::parse(stream)?;

        Ok(Self {
            fields
        })
    }
}
