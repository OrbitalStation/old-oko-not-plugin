use super::*;
use super::super::punctuated::Punctuated;

///
/// entity `<name>` = `<components>`+ ;
///
/// # Examples
///
/// `entity Player = Health + Position + PlayerControlled;`
///
#[derive(Debug, Clone)]
pub struct EntityStmt {
    pub name: Ident,
    pub components: Punctuated <Ident, '+'>
}

impl Parse for EntityStmt {
    fn parse(stream: &mut ParseStream) -> Result <Self> {
        stream.keyword("entity")?;
        let name = stream.ident()?;
        stream.punct("=")?;
        let components = Punctuated::parse(stream)?;
        stream.punct(";")?;

        Ok(Self {
            name,
            components
        })
    }
}
