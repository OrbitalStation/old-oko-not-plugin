use super::span::Ident;
use super::stream::{Parse, ParseStream, Result};

#[derive(Debug, Copy, Clone)]
pub struct Ptr {
    pub len: u8,
    pub muts: u8
}

impl Parse for Ptr {
    fn parse(stream: &mut ParseStream) -> Result <Self> {
        let mut len = 0;
        let mut muts = 0b0000_0000;

        while stream.punct("*").is_ok() {
            if stream.keyword("mut").is_ok() {
                muts |= 1 << len;
            }

            len += 1
        }

        Ok(Self {
            len,
            muts
        })
    }
}

#[derive(Debug, Clone)]
pub struct Type {
    pub ptr: Ptr,
    pub name: Ident
}

impl Parse for Type {
    fn parse(stream: &mut ParseStream) -> Result <Self> {
        let ptr = Ptr::parse(stream)?;
        let name = Ident::parse(stream)?;

        Ok(Self {
            ptr,
            name
        })
    }
}
