# monkelang

A programming language based on the cries of Monke

## Motivation

It is said that a monkey sitting at a typewriter for an infinite amount of time will eventually write the
entire works of Shakespeare. This project aims to find out if the transcribed cries of a monke for an infinite amount of time will eventually write a C compiler.

## The Language

The principles behind this language are identical to [Brainfuck](https://en.wikipedia.org/wiki/Brainfuck), the foremost inspiration for this project, other than Bananas, as well as [Ook](https://www.dangermouse.net/esoteric/ook.html), a very similar language designed for organutans instead of the general monkey populace.

This lanaguage is designed to be isomorphic to Brainfuck, and as such is also Turing complete<sup>[1](#monkenote)</sup>

## Syntax

monkelang has only 3 syntax elements

- ooh
- eee
- aah

These syntax elements will be paired up to form the following commands

| Command | BF equivalent | Meaning (taken from [Wikipedia](https://en.wikipedia.org/wiki/Brainfuck))                                                                                                         |
| ------- | ------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| ooh ooh | >             | increment the data pointer (to point to the next cell to the right).                                                                                                              |
| ooh eee | <             | decrement the data pointer (to point to the next cell to the left).                                                                                                               |
| ooh aah | +             | increment (increase by one) the byte at the data pointer.                                                                                                                         |
| eee eee | -             | decrement (decrease by one) the byte at the data pointer.                                                                                                                         |
| eee aah | .             | output the byte at the data pointer.                                                                                                                                              |
| aah ooh | ,             | accept one byte of input, storing its value in the byte at the data pointer.                                                                                                      |
| aah eee | [             | if the byte at the data pointer is zero, then instead of moving the instruction pointer forward to the next command, jump it forward to the command after the matching ] command. |
| aah aah | ]             | if the byte at the data pointer is nonzero, then instead of moving the instruction pointer forward to the next command, jump it back to the command after the matching [ command. |

Any characters which are not `'o', 'h', 'a', 'e'` are ignored, as monke is expected to produce other
types of noise, like `hooo` and `HA`

## Example programs

hello.monke (included with repo)

```
eee eee aah eee eee eee eee eee eee eee eee eee eee eee eee eee eee eee ooh
ooh ooh aah ooh eee aah aah ooh ooh eee eee eee aah eee eee aah eee eee eee
ooh ooh ooh aah ooh aah ooh aah ooh aah ooh aah ooh eee aah aah ooh ooh ooh
aah ooh aah eee aah ooh aah ooh aah ooh aah ooh aah ooh aah ooh aah ooh aah
eee aah eee aah ooh aah ooh aah ooh aah eee aah aah eee eee eee eee eee eee
eee ooh ooh ooh aah ooh eee aah aah ooh ooh eee eee eee eee eee eee eee eee
eee eee eee aah ooh aah ooh aah ooh aah ooh aah ooh aah ooh aah aah eee eee
eee ooh ooh ooh aah ooh aah ooh eee aah aah ooh ooh ooh aah eee aah aah eee
eee eee eee eee eee eee ooh ooh ooh aah ooh eee aah aah ooh ooh eee aah eee
eee eee aah eee eee eee eee eee eee eee aah eee eee eee eee eee eee eee eee
eee eee eee eee eee aah aah eee eee eee eee eee eee eee ooh ooh ooh aah ooh
eee aah aah ooh ooh ooh aah ooh aah eee aah
```

### Running the program

As a prerequisite, make sure you have `Rust` with `cargo` installed, see [here](https://doc.rust-lang.org/cargo/getting-started/installation.html).

Run `cargo run <path_to_src_file>` to run a monkelang interpreter on the src file direct from cargo

or `cargo build --release` followed by `target/release/monkelang <path_to_src_file>` to build and
run the binary

### Footnotes

<a name="monkenote">1</a>: monkelang isn't actually turing complete since it has a size limit of 30000 characters
