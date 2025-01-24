use colored::Colorize;
use std::collections::HashMap;

use crate::args::File;

pub fn format_content(table: &HashMap<String, bool>, file: &mut File) {
    let mut content = file.content.clone();
    let first = file.range.0;

    let max_count = (content.lines().count() + first).to_string().len();

    if !table["no_number"] {
        content = add_number(&content, max_count, first);
    }

    if !table["no_header"] {
        content = add_header(&content, &file.name, table["no_number"], max_count);
    }

    if !table["no_new_line"] {
        content = format!("\n{content}");
    }

    file.content = content
}

fn add_number(content: &str, max_count: usize, first: usize) -> String {
    content
        .lines()
        .enumerate()
        .map(|(number, line)| {
            format!(
                "  {}{}  {}",
                " ".repeat(max_count - (number + first).to_string().len()),
                format!("{}   │", number + first).bright_black(),
                line
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn add_header(content: &str, file: &String, no_number: bool, max_count: usize) -> String {
    let dashes = "─".repeat(file.len() + 2);
    let addon: String = if !no_number {
        let spaces = " ".repeat(max_count);
        format!(
            "    {spaces}   {}\n    {spaces} {}",
            &file,
            format!("┌{dashes}").bright_black()
        )
    } else {
        format!(" {}\n{}", &file, dashes.bright_black())
    };

    format!("{}\n{}", addon, content)
}
