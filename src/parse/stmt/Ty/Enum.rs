use super::*;
use super::super::super::ty::Type;

///
/// The field of the enum variant of the `ty` statement body.
///
/// `ty Option = None | Some i32`
///
/// `------------^^^^`
///
/// `-------------------^^^^^^^^`
///
#[derive(Debug, Clone)]
pub struct Field {
    pub name: Ident,
    pub attached_type: Option <Type>
}

impl Parse for Field {
    fn parse(stream: &mut ParseStream) -> Result <Self> {
        let name = Ident::parse(stream)?;
        let attached_type = Type::parse(stream).ok();

        Ok(Self {
            name,
            attached_type
        })
    }
}

///
/// The enum variant of the `ty` statement body.
///
/// `ty Option = None | Some i32`
///
/// `------------^^^^^^^^^^^^^^^`
///
#[derive(Debug, Clone)]
pub struct EnumTyStmtBody {
    pub fields: Punctuated <Field, '|'>
}

impl Parse for EnumTyStmtBody {
    fn parse(stream: &mut ParseStream) -> Result <Self> {
        let fields = Punctuated::parse(stream)?;
        stream.newline()?;

        Ok(Self {
            fields
        })
    }
}
