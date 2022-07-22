//!
//! This module parses the source code and transforms it into an AST
//! 

pub mod stream;
pub mod span;
pub mod stmt;
pub mod ty;
pub mod typed_variable;
pub mod punctuated;

#[macro_use]
pub mod define_parseable_enum;

use crate::error::Result;
use stream::{Parse, ParseStream};
use stmt::Stmt;

///
/// Transforms the source code into a sequence of statements
///
pub fn parse(filename: &str) -> Result <Vec <Stmt>> {
    let code = std::fs::read_to_string(filename).expect("failed to read file");
    let mut stream = ParseStream::new(&code);

    let mut vec = vec![];

    while !stream.is_empty() {
        let stmt = Result(Stmt::parse(&mut stream).map_err(|err| err.to_error(&code, filename.to_string())))?;

        stream.trim();

        vec.push(stmt)
    }

    Result(Ok(vec))
}
