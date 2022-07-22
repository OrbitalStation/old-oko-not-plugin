use super::stream::{Parse, ParseStream, Result};
use super::span::{Ident, DoubleQuotedString};

macro_rules! stmt {
    ($( $field:ident )*) => {
        $(
            #[allow(non_snake_case)]
            mod $field;
            pub use self::$field::*;
        )*

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
    Struct
}
