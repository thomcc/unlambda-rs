//! Evaluation API. By far ths most useful part of this crate.
use super::*;
use std::{fs::File, io::Cursor};
fn eval_imp(e: P<Expr>, input: Input<'_>, o: Output) -> Result<Output, Error> {
    let mut ctx = match input {
        Input::Str(s) => Ctx::new(Cursor::new(s.to_owned()), o),
        Input::File(s) => Ctx::new(File::open(s)?, o),
        Input::Stdin => Ctx::new(std::io::stdin(), o),
    };
    ctx.execute(e)?;
    Ok(ctx.stdout)
}

pub fn eval_to_vec<'a, I: 'a + Into<Input<'a>>>(
    source: I,
    input: Input<'_>,
) -> Result<Vec<u8>, Error> {
    match eval_imp(source.into().parse()?, input, Vec::with_capacity(32).into())? {
        Output::Buffer(b) => Ok(b),
        _ => unreachable!(),
    }
}

pub fn eval_to_stdout<'a, I: 'a + Into<Input<'a>>>(
    source: I,
    input: Input<'_>,
) -> Result<(), Error> {
    eval_imp(source.into().parse()?, input, std::io::stdout().into()).map(drop)
}

pub fn eval_to_string<'a, I: 'a + Into<Input<'a>>>(
    source: I,
    input: Input<'_>,
) -> Result<String, Error> {
    match String::from_utf8(eval_to_vec(source, input)?) {
        Ok(s) => Ok(s),
        Err(e) => Ok(String::from_utf8_lossy(e.as_bytes()).to_string()),
    }
}

/// Evaluation error
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    Io(std::io::Error),
    Parse(ParseError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(io) => io.fmt(f),
            Self::Parse(pe) => pe.fmt(f),
        }
    }
}

impl std::error::Error for Error {
    #[cold]
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
            Self::Parse(pe) => Some(pe),
        }
    }
}

impl From<std::io::Error> for Error {
    #[cold]
    fn from(src: std::io::Error) -> Self {
        Self::Io(src)
    }
}

impl From<ParseError> for Error {
    #[cold]
    fn from(src: ParseError) -> Self {
        Self::Parse(src)
    }
}
