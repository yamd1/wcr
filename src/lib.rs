use clap::{Arg, ArgAction, ArgMatches, Command};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("wcr")
        .version("0.1.0")
        .about("Rust wc")
        .arg(
            Arg::new("files")
                .value_name("FILES")
                .help("Input file name")
                .action(ArgAction::Append)
                .default_value("-"),
        )
        .arg(
            Arg::new("lines")
                .value_name("LINES")
                .help("display lines")
                .short('n')
                .long("lines")
                .default_value("false")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("words")
                .value_name("WORDS")
                .short('w')
                .long("words")
                .default_value("false")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("bytes")
                .value_name("BYTES")
                .short('c')
                .long("bytes")
                .default_value("false")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("chars")
                .value_name("CHARS")
                .short('m')
                .long("chars")
                .default_value("false")
                .action(ArgAction::SetTrue)
                .conflicts_with("bytes"),
        )
        .get_matches();

    let files: Vec<_> = matches
        .get_many::<String>("files")
        .unwrap()
        .map(|v| v.to_owned())
        .collect();
    let mut lines = matches.get_flag("lines");
    let mut words = matches.get_flag("words");
    let mut bytes = matches.get_flag("bytes");
    let chars = matches.get_flag("chars");

    if [lines, words, bytes, chars].iter().all(|v| v == &false) {
        lines = true;
        words = true;
        bytes = true;
    }

    Ok(Config {
        files,
        lines,
        words,
        bytes,
        chars,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}", config);
    Ok(())
}
