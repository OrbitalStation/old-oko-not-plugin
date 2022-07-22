#[macro_export]
macro_rules! define_parseable_enum {
    (
        NAME = $name:ident

        FIELDS: $( $field:ident )*
    ) => {
        use crate::parse::span::*;
        use crate::parse::stream::*;

        $(
            #[allow(non_snake_case)]
            mod $field;
            pub use self::$field::*;
        )*

        #[derive(Debug, Clone)]
        #[repr(u8)]
        pub enum $name {$(
            $field(Box <c_like_concat::concat!($field, $name)>)
        ),*}

        impl Parse for $name {
            fn parse(stream: &mut ParseStream) -> Result <Self> {
                let mut clone;

                $(
                    clone = stream.clone();
                    let c_like_concat::concat!($field, _error) = match c_like_concat::concat!($field, $name::parse(&mut clone)) {
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
