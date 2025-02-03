use crate::{args::File, format::format_content, syntax::highlight_file};
use std::collections::HashMap;

pub fn run(mut files: Vec<File>, table: HashMap<String, bool>) {
    if table["help"] {
        println!("do you need help\nwell we are going to give you help\nRead the docs\nWhich docs? PRs are welcome");
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
