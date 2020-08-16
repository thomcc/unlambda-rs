# `unlambda`

[![Build Status](https://github.com/thomcc/unlambda-rs/workflows/CI/badge.svg)](https://github.com/thomcc/unlambda-rs/actions)
[![Docs](https://docs.rs/unlambda-rs/badge.svg)](https://docs.rs/unlambda-rs)
[![Latest Version](https://img.shields.io/crates/v/unlambda-rs.svg)](https://crates.io/crates/unlambda-rs)

It's a Rust unlambda interpreter that you can use as a library. The code is
mostly okay, (by which I entirely mean it works, doesn't leak memory (AFAIK) and
the API doesn't disgust me). But has way less documentation that I usually want
to provide.

Anyway, this was written on a whim a *while* ago. I had intended to write a rust
unlambda binary, and maybe I'll get to that but this project does nobody any
good sitting in my `~/code` folder. I'm trying to get the >90% projects out
there and on to crates.io if they're any good at all, and so here we are.

Anyway, it's a little rough around the edges (some things I wouldn't do today),
exposes more of it's guts than you might expect, and the docs for anything
beyond the basics are almost non-existent.

That said, the important parts of the api are exposed at the top level, although
essentially almost everything is `pub`. I mean, if you want to use write a
program that evaluates unlambda and don't want to write the interpreter
yourself, then there's no real need for me to hold stuff back that might get in
your way? I don't know. It helps that unlambda is basically a fixed format and
doesn't get updates, so barring bugfixes this is probably final-ish.

## Example

```rust
let source = "`.!`.d`.l`.r`.o`.w`. `.,`.o`.l`.l`.e`.Hi";
// `unlambda::Input` is the "stdin", which can be a string,
// a file, actual stdin, ... It defaults to the empty string.
let input = unlambda::Input::default();
// Produces an error if we fail to parse, or if the
// unlambda program does some IO which itself produces an error.
let output = unlambda::eval_to_string(source, input)?;
assert_eq!(output, "Hello, world!");
```

## License

This code is public domain, as explained [./LICENSE-CC0].

It's potentially worth noting that the Unlambda distribution itself is
distributed under the GPL, but I don't belive I've done anything that would make
that apply to this code.
