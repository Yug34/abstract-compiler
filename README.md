## A Rust-based Abstract Compiler that works for multiple languages

Very long-term project, the goal here is to make a single compiler that compiles the source code of multiple different languages into a common AST interoperable with all the languages.

Where will this be useful? In static analysis of codebases that use multiple languages.

There's tons of better way to do things than I'm doing here, just hacking things together and learning on the way. Currently reading [Engineering a Compiler](https://dl.acm.org/doi/pdf/10.5555/2737838#:~:text=Engineering%20a%20Compiler%20is%20a,to%20build%20a%20modern%20compiler.%E2%80%9D&text=%E2%80%9CA%20wonderful%20introduction%20to%20the,lore%20of%20modern%20compil%2D%20ers.).

I've parked this project for now, and will start it again soon, in ~2 weeks.

## Technical details

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

## `State`

I think `State` has a clean interface that'd be useful for parsing any language; it lets you develop an FSA and has a bunch of handy methods and functions to help you with it.

Ideally, using the same program for any other language should only consist of making changes to the `process_file` function in `main.rs`.


# Current progress:

## - C++ Compiler:
- Tokenizing strings and chars is complete. `cargo test` to test it.
- Math operations and logic (=, ==, +, ++, /, *, -, --, +=, -=, /=, *=) also works perfect.
- Tokenizes classes and functions correctly as well.

## - JavaScript Compiler:
- Tokenizing strings and chars is complete except for template literals.
- Math operations and logic are parsed perfectly.
- Functions and arrow functions work.
- Classes somewhat work, but not yet complete.

## How to run
Make sure there's an `example.cpp` or an `example.js` file in the `/lexer` directory. Then: `cargo run`.

The compiler should run for the given `example.cpp` or the `example.js` file.

## Testing

```sh
cargo test
```

## Sample input and output:

If this is `example.cpp`:

[//]: # (I'm sorry for using CSS here :P)

```scss
using namespace std;

class ExampleClass: public RandomClass {
    public:
        string randomProperty;
        //std::cout also parses correctly
        void printname() { std::cout << "Some string" << randomProperty; }
};
int main() {
    ExampleClass obj1;
    obj1.randomProperty = "Example String";
    obj1.printname();
    return 0;
}

bool f = !!true;

int a = b;
if (a == b) {}
if (a != b) {}
int c = a / b;
```

The output should be:

```scss
Token { token: "using", line: 1, position: 6, token_type: Keywords(Using) }
Token { token: "namespace", line: 1, position: 16, token_type: Keywords(Namespace) }
Token { token: "std", line: 1, position: 20, token_type: Unknown }
Token { token: ";", line: 1, position: 20, token_type: SpecialCharacters(Semicolon) }
Token { token: "class", line: 3, position: 6, token_type: Keywords(Class) }
Token { token: "ExampleClass", line: 3, position: 19, token_type: Unknown }
Token { token: ":", line: 3, position: 19, token_type: Unknown }
Token { token: "public", line: 3, position: 27, token_type: Keywords(Public) }
Token { token: "RandomClass", line: 3, position: 39, token_type: Unknown }
Token { token: "{", line: 3, position: 40, token_type: SpecialCharacters(OpenCurly) }
Token { token: "public", line: 4, position: 11, token_type: Keywords(Public) }
Token { token: ":", line: 4, position: 11, token_type: Unknown }
Token { token: "string", line: 5, position: 15, token_type: Unknown }
Token { token: "randomProperty", line: 5, position: 30, token_type: Unknown }
Token { token: ";", line: 5, position: 30, token_type: SpecialCharacters(Semicolon) }
Token { token: "void", line: 7, position: 13, token_type: Keywords(Void) }
Token { token: "printname", line: 7, position: 23, token_type: Unknown }
Token { token: "(", line: 7, position: 23, token_type: SpecialCharacters(OpenParen) }
Token { token: ")", line: 7, position: 24, token_type: SpecialCharacters(CloseParen) }
Token { token: "{", line: 7, position: 26, token_type: SpecialCharacters(OpenCurly) }
Token { token: "std", line: 7, position: 31, token_type: Unknown }
Token { token: "::", line: 7, position: 31, token_type: Unknown }
Token { token: "cout", line: 7, position: 37, token_type: Unknown }
Token { token: "<<", line: 7, position: 38, token_type: Unknown }
Token { token: "\"Some string\"", line: 7, position: 53, token_type: Unknown }
Token { token: "<<", line: 7, position: 55, token_type: Unknown }
Token { token: "randomProperty", line: 7, position: 72, token_type: Unknown }
Token { token: ";", line: 7, position: 72, token_type: SpecialCharacters(Semicolon) }
Token { token: "}", line: 7, position: 74, token_type: SpecialCharacters(CloseCurly) }
Token { token: "}", line: 8, position: 1, token_type: SpecialCharacters(CloseCurly) }
Token { token: ";", line: 8, position: 2, token_type: SpecialCharacters(Semicolon) }
Token { token: "int", line: 9, position: 4, token_type: Keywords(Int) }
Token { token: "main", line: 9, position: 9, token_type: Unknown }
Token { token: "(", line: 9, position: 9, token_type: SpecialCharacters(OpenParen) }
Token { token: ")", line: 9, position: 10, token_type: SpecialCharacters(CloseParen) }
Token { token: "{", line: 9, position: 12, token_type: SpecialCharacters(OpenCurly) }
Token { token: "ExampleClass", line: 10, position: 17, token_type: Unknown }
Token { token: "obj1", line: 10, position: 22, token_type: Unknown }
Token { token: ";", line: 10, position: 22, token_type: SpecialCharacters(Semicolon) }
Token { token: "obj1.randomProperty", line: 11, position: 24, token_type: Unknown }
Token { token: "=", line: 11, position: 25, token_type: SpecialCharacters(Assignment) }
Token { token: "\"Example String\"", line: 11, position: 42, token_type: Unknown }
Token { token: ";", line: 11, position: 43, token_type: SpecialCharacters(Semicolon) }
Token { token: "obj1.printname", line: 12, position: 19, token_type: Unknown }
Token { token: "(", line: 12, position: 19, token_type: SpecialCharacters(OpenParen) }
Token { token: ")", line: 12, position: 20, token_type: SpecialCharacters(CloseParen) }
Token { token: ";", line: 12, position: 21, token_type: SpecialCharacters(Semicolon) }
Token { token: "return", line: 13, position: 11, token_type: Keywords(Return) }
Token { token: "0", line: 13, position: 13, token_type: Unknown }
Token { token: ";", line: 13, position: 13, token_type: SpecialCharacters(Semicolon) }
Token { token: "}", line: 14, position: 1, token_type: SpecialCharacters(CloseCurly) }
Token { token: "bool", line: 16, position: 5, token_type: Keywords(Bool) }
Token { token: "f", line: 16, position: 7, token_type: Unknown }
Token { token: "=", line: 16, position: 8, token_type: SpecialCharacters(Assignment) }
Token { token: "!!", line: 16, position: 10, token_type: SpecialCharacters(DoubleNegation) }
Token { token: "true", line: 16, position: 16, token_type: Keywords(True) }
Token { token: ";", line: 16, position: 16, token_type: SpecialCharacters(Semicolon) }
Token { token: "int", line: 18, position: 4, token_type: Keywords(Int) }
Token { token: "a", line: 18, position: 6, token_type: Unknown }
Token { token: "=", line: 18, position: 7, token_type: SpecialCharacters(Assignment) }
Token { token: "b", line: 18, position: 10, token_type: Unknown }
Token { token: ";", line: 18, position: 10, token_type: SpecialCharacters(Semicolon) }
Token { token: "if", line: 19, position: 3, token_type: Keywords(If) }
Token { token: "(", line: 19, position: 4, token_type: SpecialCharacters(OpenParen) }
Token { token: "a", line: 19, position: 6, token_type: Unknown }
Token { token: "==", line: 19, position: 7, token_type: SpecialCharacters(Equals) }
Token { token: "b", line: 19, position: 11, token_type: Unknown }
Token { token: ")", line: 19, position: 11, token_type: SpecialCharacters(CloseParen) }
Token { token: "{", line: 19, position: 13, token_type: SpecialCharacters(OpenCurly) }
Token { token: "}", line: 19, position: 14, token_type: SpecialCharacters(CloseCurly) }
Token { token: "if", line: 20, position: 3, token_type: Keywords(If) }
Token { token: "(", line: 20, position: 4, token_type: SpecialCharacters(OpenParen) }
Token { token: "a", line: 20, position: 6, token_type: Unknown }
Token { token: "!=", line: 20, position: 7, token_type: Unknown }
Token { token: "b", line: 20, position: 11, token_type: Unknown }
Token { token: ")", line: 20, position: 11, token_type: SpecialCharacters(CloseParen) }
Token { token: "{", line: 20, position: 13, token_type: SpecialCharacters(OpenCurly) }
Token { token: "}", line: 20, position: 14, token_type: SpecialCharacters(CloseCurly) }
Token { token: "int", line: 21, position: 4, token_type: Keywords(Int) }
Token { token: "c", line: 21, position: 6, token_type: Unknown }
Token { token: "=", line: 21, position: 7, token_type: SpecialCharacters(Assignment) }
Token { token: "a", line: 21, position: 10, token_type: Unknown }
Token { token: "/", line: 21, position: 12, token_type: SpecialCharacters(Divide) }
Token { token: "b", line: 21, position: 14, token_type: Unknown }
Token { token: ";", line: 21, position: 14, token_type: SpecialCharacters(Semicolon) }

```
