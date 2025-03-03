use crate::{args::parse_args, run::run};
use std::{env, error::Error};

mod args;
mod format;
mod run;
mod syntax;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let (files, arg_table) = parse_args(args[1..].to_vec())?;

    run(files, arg_table);
    Ok(())
}
