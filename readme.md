# minigrep

This repository is CLI I/O Project, that was created based on Chapter 12 of *The Rust Programming Language* book, that is available at: https://doc.rust-lang.org/book/title-page.html.

The aim for this project is to showcase many valuable tricks and concepts of Rust that I learned by following the book, including:
* Structs
* Match statements
* Borrow Checker fundamentals
* Rust Error handling
* Test Driven Development

## Project Overview
This project is minimized `grep` tool implementation.
It is used for 

As of now it supports:
* passing file as argument
* querying for certain phrase
* outputing whole file when passing `_` as query

Planned to support (besides the project chapter contents):
* output to file
* help

### Example usage

With query:
```bash
cargo run -- frog poem.txt
```
Output:
```
How public, like a frog
```

With query as `_`
```bash
cargo run -- _ poem.txt
```
Output:
```
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!
```