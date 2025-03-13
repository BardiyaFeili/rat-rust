use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs,
};

pub struct File {
    pub name: String,
    pub content: String,
    pub len: usize,
    pub range: (usize, usize),
}

impl File {
    fn from(name: String) -> Result<File, Box<dyn Error>> {
        let content = fs::read_to_string(&name)?;
        let len = content.lines().count();

        Ok(File {
            name,
            content,
            len,
            range: (1, len),
        })
    }

    fn add_range(&mut self, range: &str) {
        let (first, last) = parse_range(range, self.len);

        self.range = (first, last);
        self.content = self
            .content
            .lines()
            .skip(first - 1)
            .take(last - first + 1)
            .collect::<Vec<&str>>()
            .join("\n");
    }
}

type ParseArgsResult = Result<(Vec<File>, HashMap<String, bool>), Box<dyn Error>>;
pub fn parse_args(args: Vec<String>) -> ParseArgsResult {
    let mut flags: HashSet<String> = HashSet::new();
    let mut opts: HashSet<char> = HashSet::new();
    let mut files: Vec<File> = Vec::new();
    let mut is_range: bool = false;

    for arg in args {
        if arg.starts_with("---") {
            println!(
                "Arg {} is not correct, Arguments cannot have more than 2 dashes",
                arg
            );
            continue;
        }
        if let Some(rest) = arg.strip_prefix("--") {
            if rest.is_empty() {
                continue;
            } else if rest == "range" {
                is_range = true;
                continue;
            }
            flags.insert(rest.to_string());
        } else if let Some(rest) = arg.strip_prefix('-') {
            if rest.is_empty() {
                continue;
            }
            for letter in rest.chars() {
                if letter == 'r' {
                    is_range = true;
                    continue;
                }
                opts.insert(letter);
            }
        } else if is_range {
            if let Some(file) = files.last_mut() {
                file.add_range(&arg);
                is_range = false;
                continue;
            } else {
                println!("The range can't be before a file");
                continue;
            }
        } else {
            files.push(File::from(arg)?);
        }
    }

    Ok((files, gen_args(opts, flags)))
}

fn gen_args(opts: HashSet<char>, flags: HashSet<String>) -> HashMap<String, bool> {
    let mut args: HashMap<String, bool> = HashMap::from([
        ("help".to_string(), false),
        ("no_number".to_string(), false),
        ("no_header".to_string(), false),
        ("no_formatting".to_string(), false),
        ("no_syntax_highlighting".to_string(), false),
        ("no_new_line".to_string(), false),
    ]);

    for opt in opts {
        match opt {
            'H' => args.insert("help".to_string(), true),
            'n' => args.insert("no_number".to_string(), true),
            'h' => args.insert("no_header".to_string(), true),
            'f' => args.insert("no_formatting".to_string(), true),
            's' => args.insert("no_syntax_highlighting".to_string(), true),
            _ => {
                println!("arg {} is not valid.", opt);
                None
            }
        };
    }

    for flag in flags {
        match flag.as_str() {
            "help" => args.insert("help".to_string(), true),
            "no-number" => args.insert("no_number".to_string(), true),
            "no-header" => args.insert("no_header".to_string(), true),
            "no-formatting" => args.insert("no_formatting".to_string(), true),
            "no-syntax-highlighting" => args.insert("no_syntax_highlighting".to_string(), true),
            "no-new-line" => args.insert("no_new_line".to_string(), true),
            _ => {
                println!("flag {} is not valid", flag);
                None
            }
        };
    }

    args
}

fn parse_range(range: &str, len: usize) -> (usize, usize) {
    if !range.contains(':') {
        panic!("The input range must contain a ':' separator");
    }

    let parts: Vec<&str> = range.split(':').collect();
    if parts.len() != 2 {
        panic!("Invalid range format. Expected one ':' separator");
    }

    let first = if parts[0].is_empty() {
        1
    } else {
        parts[0].parse::<usize>().unwrap_or_else(|_| {
            panic!("Invalid start value: {}", parts[0]);
        })
    };

    let last = if parts[1].is_empty() {
        len
    } else {
        parts[1].parse::<usize>().unwrap_or_else(|_| {
            panic!("Invalid end value: {}", parts[1]);
        })
    };

    if first < 1 {
        panic!("Start Value cannot be smaller than 1");
    }
    if last == 0 {
        panic!("End value cannot be 0");
    }
    if last > len {
        panic!("End value cannot be greater than the length: {}", len);
    }
    if last < first {
        panic!("End value cannot be smaller than the start value");
    }

    (first, last)
}
