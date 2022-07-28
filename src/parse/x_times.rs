use super::stream::{Parse, ParseStream, Result};
use core::fmt::{Debug, Formatter, Result as FmtResult};

#[derive(Clone)]
pub struct XTimes <T: Parse, const IS_NO_X_ALLOWED: bool = false> {
    pub it: T,
    pub times: u8
}

impl <T: Parse + Debug, const IS_NO_X_ALLOWED: bool> Debug for XTimes <T, IS_NO_X_ALLOWED> {
    fn fmt(&self, f: &mut Formatter <'_>) -> FmtResult {
        self.it.fmt(f)?;

        if self.times != 1 {
            f.write_str(" x ")?;
            self.times.fmt(f)?;
        }

        Ok(())
    }
}

impl <T: Parse, const IS_NO_X_ALLOWED: bool> Parse for XTimes <T, IS_NO_X_ALLOWED> {
    fn parse(stream: &mut ParseStream) -> Result <Self> {
        let it = T::parse(stream)?;
        let times = match stream.keyword("x") {
            Ok(_) => stream.number_u8()?,
            Err(err) => if IS_NO_X_ALLOWED {
                1
            } else {
                return Err(err)
            }
        };

        Ok(Self {
            it,
            times
        })
    }
}
