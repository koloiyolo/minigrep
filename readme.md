# minigrep

This repository is CLI I/O Project, that was created based on Chapter 12 of *The Rust Programming Language* book, that is available at: https://doc.rust-lang.org/book/title-page.html.

The aim for this project is to showcase many valuable tricks and concepts of Rust that I learned by following the book, including:
* Structs
* Enums
* Match statements
* Borrow Checker fundamentals
* Rust Error handling
* Test Driven Development
* Args and Environment Variables handling

## Project Overview
This project is minimized `grep` tool implementation.
It is used to search file contents for specific phrases.

As of now it supports:
* passing file as argument
* querying for certain phrase
* outputing whole file when passing `_` as query

Planned to support (besides the project chapter contents):
* [x] output to file
* [ ] help
* [ ] Separate branch with state management using Rust Enums

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

If You want to create output file:
```bash
cargo run -- frog poem.txt -of output.txt
```
or:
```bash
cargo run -- frog poem.txt -output_file output.txt
```

If the operation is successfull this will create file named `output.txt` that will contain query result, and print them on the cli.

### Case Sensitivity
If you want your results to be case insensitive set `IGNORE_CASE` value to `1`
```bash
export IGNORE_CASE=1
```
If you want case sensivity back just set it to `0`
