//! Unlambda interpreter library.
//!
//! Written on a whim. I had intended to write a rust unlambda binary, and maybe
//! I'll get to that but this project does nobody any good sitting in my
//! `~/code` folder.
//!
//! It's rough around the edges and the docs are almost non-existent. The
//! important parts of the api are exposed at the top level but it's been long
//! enough since I looked that really I just fixed obvious warts and exposed
//! more-or-less everything. If you want to use a library to evaluate unlambda,
//! then far be it from me to stop you from getting deep into my code's guts.
//!
//! # Example
//!
//! ```
//! # fn main() -> Result<(), unlambda::EvalError> {
//! use unlambda::Input;
//! let source = "`.!`.d`.l`.r`.o`.w`. `.,`.o`.l`.l`.e`.Hi";
//! // `unlambda::Input` is the "stdin", which can be a string,
//! // a file, actual stdin, ... It defaults to the empty string.
//! let input = unlambda::Input::default();
//! // Produces an error if we fail to parse, or if the
//! // unlambda program does some IO which itself produces an error.
//! let output = unlambda::eval_to_string(source, input)?;
//! assert_eq!(output, "Hello, world!");
//! # Ok(())
//! # }
//! ```

pub use eval::{eval_to_stdout, eval_to_string, eval_to_vec, Error as EvalError};
pub use io::Input;
pub use parse::{parse_from_file, parse_from_reader, parse_from_stdin, parse_from_str, ParseError};
pub use util::P;

pub mod eval;
pub mod internals;
pub mod io;
pub mod parse;
mod util;

pub(crate) use eval::*;
pub(crate) use internals::*;
pub(crate) use io::*;
pub(crate) use parse::*;
pub(crate) use util::p;
