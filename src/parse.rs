//! The parse API. Somewhat useful.

use super::*;

/// Options for parsing. Use with [`parse_from_str`], [`parse_from_file`],
/// [`parse_from_reader`], or [`parse_from_stdin`], ...
#[derive(Default, Clone, Debug)]
#[non_exhaustive]
pub struct ParseOptions {
    pub strict: bool,
    pub log_warnings: bool,
}

pub fn parse_from_str(s: &str, o: ParseOptions) -> Result<P<Expr>, ParseError> {
    Parser::new(s, None, o).parse()
}

pub fn parse_from_file(
    path: impl AsRef<std::path::Path>,
    o: ParseOptions,
) -> Result<P<Expr>, ParseError> {
    let path = path.as_ref();
    let s = std::fs::read_to_string(path).map_err(|io| ParseError::io(io, Some(path)))?;
    Parser::new(&s, Some(path), o).parse()
}

pub fn parse_from_reader<R: Read>(mut r: R, o: ParseOptions) -> Result<P<Expr>, ParseError> {
    let mut s = String::new();
    r.read_to_string(&mut s)
        .map_err(|io| ParseError::io(io, None))?;
    Parser::new(&s, None, o).parse()
}

/// Note: just wraps [`parse_from_reader`], nothing fancy.
pub fn parse_from_stdin(o: ParseOptions) -> Result<P<Expr>, ParseError> {
    parse_from_reader(std::io::stdin().lock(), o)
}

#[derive(Debug)]
#[non_exhaustive]
pub enum ParseErrorKind {
    UnexpectedEnd,
    UnexpectedChar(char),
    Io(std::io::Error),
}

use std::io::Read;
use ParseErrorKind::{UnexpectedChar, UnexpectedEnd};

#[derive(Debug)]
struct ParseErrorInfo {
    kind: ParseErrorKind,
    line: usize,
    col: usize,
    file: Option<std::path::PathBuf>,
}

#[derive(Debug)]
pub struct ParseError(Box<ParseErrorInfo>);

impl ParseError {
    pub fn file(&self) -> Option<&std::path::Path> {
        self.0.file.as_deref()
    }

    pub fn line_col(&self) -> (usize, usize) {
        (self.0.line, self.0.col)
    }

    pub fn is_eof(&self) -> bool {
        match self.kind() {
            UnexpectedEnd => true,
            ParseErrorKind::Io(io) => io.kind() == std::io::ErrorKind::UnexpectedEof,
            _ => false,
        }
    }

    pub fn kind(&self) -> &ParseErrorKind {
        &self.0.kind
    }

    #[cold]
    pub(crate) fn new(
        kind: ParseErrorKind,
        line: usize,
        col: usize,
        file: Option<&std::path::Path>,
    ) -> Self {
        Self(Box::new(ParseErrorInfo {
            kind,
            line,
            col,
            file: file.map(ToOwned::to_owned),
        }))
    }
    #[cold]
    pub(crate) fn io(io: std::io::Error, file: Option<&std::path::Path>) -> Self {
        Self::new(ParseErrorKind::Io(io), 0, 0, file)
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind() {
            ParseErrorKind::UnexpectedEnd => f.write_str("unexpected end of input")?,
            ParseErrorKind::UnexpectedChar(c) => write!(f, "unexpected character {:?}", c)?,
            ParseErrorKind::Io(io) => io.fmt(f)?,
        };
        match self.line_col() {
            (0, 0) => {}
            (0, c) => write!(f, " at column {}", c)?,
            (l, c) => write!(f, " at line {}, column {}", l, c)?,
        }
        if let Some(p) = self.file() {
            write!(f, " in `{}`", p.display())?;
        }
        Ok(())
    }
}

impl std::error::Error for ParseError {
    #[cold]
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self.kind() {
            ParseErrorKind::Io(e) => Some(e),
            _ => None,
        }
    }
}

struct Parser<'a> {
    input: &'a str,
    src: Option<&'a std::path::Path>,
    pos: usize,
    opts: ParseOptions,
}

impl<'a> Parser<'a> {
    fn new(s: &'a str, src: Option<&'a std::path::Path>, opts: ParseOptions) -> Self {
        Self {
            input: s,
            src,
            pos: 0,
            opts,
        }
    }

    fn line_col(&self) -> (usize, usize) {
        self.line_col_at(self.pos)
    }

    fn line_col_at(&self, p: usize) -> (usize, usize) {
        let s = &self.input[..p];
        let l = s.lines().count();
        let r_start = self.input.rfind('\n').map(|v| v + 1).unwrap_or(0);
        let row = &self.input[r_start..];
        // chars isn't perfect but it's probably good enough.
        let chars = row.chars().count().saturating_sub(1);
        (l, chars)
    }

    fn char_at(&self, p: usize) -> Option<char> {
        self.input.get(p..).and_then(|s| s.chars().next())
    }

    fn try_next(&mut self) -> Option<char> {
        let mut c;
        loop {
            c = self.char_at(self.pos)?;
            self.pos += c.len_utf8();
            if c == '#' {
                while c != '\n' {
                    c = self.char_at(self.pos)?;
                    self.pos += c.len_utf8();
                }
            }
            if !c.is_whitespace() {
                break;
            }
        }
        Some(c)
    }

    #[cold]
    fn error(&self, kind: ParseErrorKind) -> ParseError {
        let (line, col) = self.line_col();
        ParseError::new(kind, line, col, self.src)
    }

    fn next_c(&mut self) -> Result<char, ParseError> {
        self.try_next().ok_or_else(|| self.error(UnexpectedEnd))
    }

    fn parse_expr(&mut self) -> Result<P<Expr>, ParseError> {
        let ch = self.next_c()?;
        match ch {
            '`' => {
                let operator = self.parse_expr()?;
                let operand = self.parse_expr()?;
                Ok(p(Expr::App(operator, operand)))
            }
            'i' | 'I' => Ok(p(Expr::Func(Func::I))),
            'k' | 'K' => Ok(p(Expr::Func(Func::K))),
            's' | 'S' => Ok(p(Expr::Func(Func::S))),
            'd' | 'D' => Ok(p(Expr::Func(Func::D))),
            'e' | 'E' => Ok(p(Expr::Func(Func::E))),
            'c' | 'C' => Ok(p(Expr::Func(Func::C))),
            'v' | 'V' => Ok(p(Expr::Func(Func::V))),
            'r' | 'R' => Ok(p(Expr::Func(Func::Dot('\n')))),
            '@' => Ok(p(Expr::Func(Func::At))),
            '|' => Ok(p(Expr::Func(Func::Pipe))),
            '.' => Ok(p(Expr::Func(Func::Dot(self.raw_next()?)))),
            '?' => Ok(p(Expr::Func(Func::Q(self.raw_next()?)))),
            other => Err(self.error(UnexpectedChar(other))),
        }
    }

    fn raw_next(&mut self) -> Result<char, ParseError> {
        self.char_at(self.pos)
            .ok_or_else(|| self.error(UnexpectedEnd))
            .map(|c| {
                self.pos += c.len_utf8();
                c
            })
    }

    fn parse(mut self) -> Result<P<Expr>, ParseError> {
        let e = self.parse_expr()?;
        if let Some(v) = self.try_next() {
            if self.opts.strict {
                return Err(self.error(UnexpectedChar(v)));
            } else if self.opts.log_warnings {
                let _lc = self.error(UnexpectedChar(v)).to_string();
                #[cfg(feature = "log")]
                {
                    log::warn!("Ignoring trailing garbage after expression: {}", _lc);
                }
            }
        }
        Ok(e)
    }
}
