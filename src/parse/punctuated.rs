use super::stream::{Parse, ParseStream, Result};
use core::fmt::{Debug, Write, Formatter, Result as FmtResult};

///
/// Describes a sequence of elements separated by some token
///
/// # Examples
///
/// A sequence of strings separated by commas:
///
/// `"John", "Keit", "Arkady"`
///
/// `vvvvvvvvvvvvvvvvvvvvvvvv`
///
/// `Punctuated <&str, ','>`
///
/// # Generics
///
/// `T` -- the element type(`&str` in the example above)
/// `P` -- the separator type(`,` in the example above)
/// `IS_ZERO_ALLOWED` -- whether or not zero-elements sequences are allowed
///
#[derive(Debug, Clone)]
pub struct Punctuated <T: Parse, const P: char, const IS_ZERO_ALLOWED: bool = false> (pub Vec <T>);

impl <T: Parse + Debug, const P: char, const IS_ZERO_ALLOWED: bool> Punctuated <T, P, IS_ZERO_ALLOWED> {
    pub fn debug(&self, fmt: &mut Formatter <'_>) -> FmtResult {
        for elem in self.0.iter().rev().skip(1).rev() {
            fmt.write_fmt(format_args!("{elem:?}"))?;
            fmt.write_char(P)?;
            fmt.write_char(' ')?;
        }

        if let Some(elem) = self.0.last() {
            fmt.write_fmt(format_args!("{elem:?}"))?
        }

        Ok(())
    }
}

impl <T: Parse, const P: char, const IS_ZERO_ALLOWED: bool> Parse for Punctuated <T, P, IS_ZERO_ALLOWED> {
    fn parse(stream: &mut ParseStream) -> Result <Self> {
        let mut vec = vec![];

        let mut tmp = [0u8; 4];
        let p = P.encode_utf8(&mut tmp);

        loop {
            let mut clone = stream.clone();

            let value = match T::parse(&mut clone) {
                Ok(ok) => {
                    *stream = clone;
                    ok
                },
                Err(err) => if vec.is_empty() && !IS_ZERO_ALLOWED {
                    let expected = format!("at least one {}", err.expected);
                    return Err(err.with_custom_expected(expected))
                } else {
                    break
                }
            };

            vec.push(value);

            if stream.punct(p).is_err() { break }

        }

        Ok(Self(vec))
    }
}
