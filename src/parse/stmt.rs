use super::stream::{Parse, ParseStream, Result};
use super::span::{Ident, DoubleQuotedString};

macro_rules! stmt {
    ($( $field:ident )*) => {
        #[derive(Debug, Clone)]
        #[repr(u8)]
        pub enum Stmt {$(
            $field(Box <c_like_concat::concat!($field, Stmt)>)
        ),*}

        impl Parse for Stmt {
            fn parse(stream: &mut ParseStream) -> Result <Self> {
                let mut clone;

                $(
                    clone = stream.clone();
                    let c_like_concat::concat!($field, _error) = match c_like_concat::concat!($field, Stmt::parse(&mut clone)) {
                        Ok(x) => {
                            *stream = clone;
                            return Ok(Self::$field(Box::new(x)))
                        },
                        Err(err) => err
                    };
                )*

                Err([$( c_like_concat::concat!($field, _error) ),*]
                    .iter()
                    .max_by(|a, b| a.parsing_depth.0.cmp(&b.parsing_depth.0))
                    .unwrap()
                    .clone())
            }
        }
    };
}

stmt! {
    Entity
    Fn
}

#[derive(Debug, Clone)]
pub struct EntityStmt {
    pub name: Ident,
    pub components: Vec <Ident>
}

impl Parse for EntityStmt {
    fn parse(stream: &mut ParseStream) -> Result <Self> {
        stream.keyword("entity")?;
        let name = stream.ident()?;
        stream.punct("=")?;

        let mut components = vec![];

        loop {
            components.push(stream.ident()?);

            let mut clone = stream.clone();
            if let Ok(_) = clone.punct("+") {
                *stream = clone;
            } else if let Ok(_) = clone.punct(";") {
                *stream = clone;
                break
            }
        }

        Ok(Self {
            name,
            components
        })
    }
}

#[derive(Debug, Clone)]
pub struct FnStmt {
    pub name: Ident,
    pub literal: DoubleQuotedString
}

impl Parse for FnStmt {
    fn parse(stream: &mut ParseStream) -> Result <Self> {
        stream.keyword("fn")?;
        let name = stream.ident()?;
        stream.punct("=")?;
        let literal = stream.double_quoted_string()?;

        Ok(Self {
            name,
            literal
        })
    }
}
