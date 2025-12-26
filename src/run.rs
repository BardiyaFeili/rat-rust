use crate::{args::File, format::format_content, syntax::highlight_file};
use std::collections::HashMap;

pub fn run(mut files: Vec<File>, table: HashMap<String, bool>) {
    if table["help"] {
        let help_message = "
Rat — a simple cat alternative

Usage:
    rat [options] [file1] [file2] ...

Options:
    -H, --help                      Show this help message
    -n, --no-number                 Disable line numbers
    -h, --no-header                 Disable the file header
    -s, --no-syntax-highlighting    Disable syntax highlighting
    -f, --no-formatting             Disable all formatting
    --no-new-line                   Do not print a trailing newline
    -r, --range start:end           Print only the specified line range for a file

Notes:
    - Ranges are applied per file; each file can have a different range or no range.
    - Example usage with range:
        rat file.txt -r 10:50";
        println!("{}", help_message);
        return;
    };

    for file in files.iter_mut() {
        if !table["no_syntax_highlighting"] {
            highlight_file(file);
        };

        if !table["no_formatting"] {
            format_content(&table, file);
        }

        println!("{}", file.content)
    }
}
