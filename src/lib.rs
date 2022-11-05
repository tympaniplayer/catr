use clap::{Arg, ArgAction, Command};
use std::error::Error;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("catr")
        .version("0.1.0")
        .author("Nate Palmer <nate@natepalmer.dev")
        .about("Rust cat")
        .arg(
            Arg::new("file")
                .value_name("FILE")
                .help("Input file(s)")
                .num_args(1..)
                .required(true),
        )
        .arg(
            Arg::new("number")
                .short('n')
                .help("Number lines")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("number-nonblank")
                .short('b')
                .help("Number nonblank lines")
                .action(ArgAction::SetTrue),
        )
        .get_matches();
    let files = matches
        .get_many::<String>("file")
        .unwrap()
        .map(|v| v.to_string())
        .collect();
    Ok(Config {
        files,
        number_lines: matches.get_flag("number"),
        number_nonblank_lines: matches.get_flag("number-nonblank"),
    })
}

pub fn run() -> MyResult<()> {
    println!("Hello world");
    Ok(())
}
