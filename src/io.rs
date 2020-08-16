//! Various input and output types.

use super::*;
use std::{char::REPLACEMENT_CHARACTER, io::{Stdout, Write, Read, ErrorKind}, path::Path};

#[non_exhaustive]
pub enum Output {
    Buffer(Vec<u8>),
    Writer(Box<dyn Write + Sync + Send + 'static>),
}
impl Output {
    fn putc(&mut self, c: char) -> std::io::Result<()> {
        let mut a = [0u8; 4];
        let bytes = c.encode_utf8(&mut a).as_bytes();
        match self {
            Self::Buffer(b) => b.extend_from_slice(&bytes),
            Self::Writer(w) => w.write_all(&bytes)?,
        }
        Ok(())
    }
}

impl From<Vec<u8>> for Output {
    fn from(v: Vec<u8>) -> Self {
        Self::Buffer(v)
    }
}
impl From<Box<dyn Write + Sync + Send + 'static>> for Output {
    fn from(w: Box<dyn Write + Sync + Send + 'static>) -> Self {
        Self::Writer(w)
    }
}
impl From<Stdout> for Output {
    fn from(o: Stdout) -> Self {
        Self::Writer(Box::new(o))
    }
}

pub struct Ctx {
    pub(crate) stdin: Box<dyn std::io::Read + Send + 'static>,
    pub(crate) stdout: Output,
    last_char: Option<char>,
}

impl Ctx {
    pub fn new<I, O>(stdin: I, stdout: O) -> Self
    where
        I: Read + 'static + Sync + Send,
        O: Into<Output>,
    {
        Self {
            stdin: Box::new(stdin),
            stdout: stdout.into(),
            last_char: None,
        }
    }

    pub(crate) fn putc(&mut self, c: char) -> Result<(), Error> {
        self.stdout.putc(c).map_err(Into::into)
    }

    pub(crate) fn getc(&mut self) -> Result<Option<char>, Error> {
        let c = Self::read_single_char(&mut self.stdin)?;
        self.last_char = c;
        Ok(c)
    }

    pub(crate) fn last_char(&self) -> Option<char> {
        self.last_char
    }
    pub fn execute(&mut self, expr: P<Expr>) -> Result<(), Error> {
        let mut task = Task::Eval(expr, p(Cont::Final));
        while let Some(t) = task.run(self)? {
            task = t;
        }
        Ok(())
    }

    fn read_single_char(mut r: impl Read) -> Result<Option<char>, Error> {
        let mut first = 0u8;
        if let Err(e) = r.read_exact(std::slice::from_mut(&mut first)) {
            return if e.kind() == ErrorKind::UnexpectedEof {
                Ok(None)
            } else {
                Err(e.into())
            };
        }

        let width = match utf8_char_width(first) {
            1 => return Ok(Some(first as char)),
            0 => return Ok(Some(REPLACEMENT_CHARACTER)),
            n => n as usize,
        };

        let mut buf = [first, 0, 0, 0];
        if let Err(e) = r.read_exact(&mut buf[1..width]) {
            return if e.kind() == ErrorKind::UnexpectedEof {
                return Ok(Some(REPLACEMENT_CHARACTER));
            } else {
                Err(e.into())
            };
        }

        Ok(Some(
            std::str::from_utf8(&buf[..width])
                .ok()
                .and_then(|s| s.chars().next())
                .unwrap_or(REPLACEMENT_CHARACTER),
        ))
    }
}



fn utf8_char_width(first_byte: u8) -> usize {
    match first_byte {
        0b0000_0000..=0b0111_1111 => 1,
        0b1000_0000..=0b1011_1111 => 0,
        0b1100_0000..=0b1101_1111 => 2,
        0b1110_0000..=0b1110_1111 => 3,
        0b1111_0000..=0b1111_0111 => 4,
        0b1111_1000..=0b1111_1111 => 0,
    }
}

/// The "stdin" for an unlambda program.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Input<'a> {
    Str(&'a str),
    File(&'a Path),
    Stdin,
}

impl Input<'_> {
    pub fn parse(&self) -> Result<P<Expr>, ParseError> {
        let o = crate::ParseOptions {
            log_warnings: true,
            strict: false,
            ..Default::default()
        };
        match self {
            Self::Str(s) => parse_from_str(*s, o),
            Self::File(s) => parse_from_file(*s, o),
            Self::Stdin => parse_from_stdin(o),
        }
    }
}

impl<'a> From<&'a str> for Input<'a> {
    fn from(s: &'a str) -> Self {
        Self::Str(s)
    }
}

impl<'a> From<&'a Path> for Input<'a> {
    fn from(s: &'a Path) -> Self {
        Self::File(s)
    }
}

impl Default for Input<'_> {
    #[inline]
    fn default() -> Self {
        Self::Str("")
    }
}

