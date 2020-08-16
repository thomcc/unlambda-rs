# `unlambda`

Rust unlambda interpreter library. The code is okay, mostly.

Written on a whim. I had intended to write a rust unlambda binary, and maybe
I'll get to that but this project does nobody any good sitting in my `~/code`
folder.

It's rough around the edges and the docs are almost non-existent. The important
parts of the api are exposed at the top level but it's been long enough since I
looked that really I just fixed obvious warts and exposed more-or-less
everything.

If you want to use a library to evaluate unlambda, then go for it. Far be it
from me to stop you, even if you must access things deep in this code's guts.

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
