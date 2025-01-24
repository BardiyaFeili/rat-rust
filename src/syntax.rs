use syntect::{
    easy::HighlightLines,
    highlighting::{Style, ThemeSet},
    parsing::SyntaxSet,
    util::as_24_bit_terminal_escaped,
};

use crate::args::File;

pub fn highlight_file(file: &mut File) {
    let syntax_set = SyntaxSet::load_defaults_newlines();
    let theme_set = ThemeSet::load_defaults();

    let syntax = syntax_set
        .find_syntax_for_file(&file.name)
        .unwrap()
        .unwrap_or_else(|| syntax_set.find_syntax_plain_text());

    let mut highlight_line = HighlightLines::new(syntax, &theme_set.themes["base16-ocean.dark"]);

    let mut output = Vec::new();

    for line in file.content.as_str().lines() {
        let ranges: Vec<(Style, &str)> = highlight_line.highlight_line(line, &syntax_set).unwrap();
        let escaped = as_24_bit_terminal_escaped(&ranges[..], false);

        output.push(escaped);
    }

    file.content = output.join("\n");
}
