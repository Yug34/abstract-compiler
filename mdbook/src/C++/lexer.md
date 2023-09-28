## Executing the program

Make sure there's a `example.cpp` file in the `/lexer` directory. Then: `cargo run`.

## Testing the program

`cargo test`

### Current progress

The lexer is ready and produces the lexemes for C++ and JavaScript correctly, I'm currently working on classifying the lexemes according to their types.

## Overview

Currently, the tokenizing is done through an FSA with the following state variables:

```rust
pub struct Token {
    token: String,
    line: u64,
    position: u64,
    token_type: TokenTypes
}

pub struct State {
    token: String,
    previous_char: Option<char>,
    pre_previous_char: Option<char>,
    token_stream: Vec<Token>,
    current_line: u64,
    current_position: u64,
    string_type: Option<char>
}
```

Executing the program on an example file should output a stream of tokens, along with their location (line and position) in the source code.

Most of the code should be straightforward I think, there's some refactoring to do later but the interfacing of the FSA with the state is kept simple through a bunch of interfacing methods defined on `impl State`.

## State

I think `State` has a clean interface that'd be useful for parsing any language; it lets you develop an FSA and has a bunch of handy methods and functions to help you with it.

Ideally, using the same program for any other language should only consist of making changes to the `process_file` function in `main.rs`.
