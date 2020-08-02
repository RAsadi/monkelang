# monkelang

A programming language based on the cries of Monke

## Motivation

It is said that a monkey sitting at a typewriter for an infinite amount of time will eventually write the
entire works of Shakespeare. This project aims to find out if the transcribed cries of a monke for an infinite amount of time will eventually write a C compiler.

## The Language

The principles behind this language are identical to [Brainfuck](https://en.wikipedia.org/wiki/Brainfuck), the foremost inspiration for this project, other than Bananas.

This lanaguage is designed to be isomorphic to Brainfuck, and as such is also Turing complete

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
