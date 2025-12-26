
# Rat, a simple cat alternative

A simple Rust application similar to `cat` or `bat`, written in Rust.  
Built mainly for learning and experimentation.

## Features

- Fast
- Syntax highlighting for most languages
- Support for reading file ranges
- Clean and readable output
- Line numbers

## How to build

First, clone the project and navigate to it in your terminal.

```bash
# To build
cargo build --release

# To run
cargo run --release
```

## How to use

rat [options] [file1] [file2] ...

Options:

- -H, --help
    Show the help message
- -n, --no-number
    Disable line numbers
- -h, --no-header
    Disable the file header
- -s, --no-syntax-highlighting
    Disable syntax highlighting
- -f, --no-formatting
    Disable all formatting
- --no-new-line
    Do not print a trailing newline

Ranges:

You can print only part of a file by specifying a range with -r or --range.

Format:

start:end

Ranges are applied per file, so each file can have a different range or no range at all.

Example:

rat file.txt -r 10:50
cat \[options\] \[file1\] -r starting:ending
