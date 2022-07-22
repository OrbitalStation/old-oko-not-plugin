use super::stream::{Parse, ParseStream, Result};

#[derive(Debug, Clone)]
pub struct Punctuated <T: Parse, const P: char> (pub Vec <T>);

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
