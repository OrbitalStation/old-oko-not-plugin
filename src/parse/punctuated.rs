use super::stream::{Parse, ParseStream, Result};
use core::fmt::{Debug, Write, Formatter, Result as FmtResult};

#[derive(Debug, Clone)]
pub struct Punctuated <T: Parse, const P: char> (pub Vec <T>);

impl <T: Parse + Debug, const P: char> Punctuated <T, P> {
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

impl <T: Parse, const P: char> Parse for Punctuated <T, P> {
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
                Err(err) => if vec.is_empty() {
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
