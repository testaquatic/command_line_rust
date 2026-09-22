use clap::{
    Arg, Command,
    builder::{RangedU64ValueParser, ValueParser},
};

#[derive(Debug)]
pub struct Args {
    pub files: Vec<String>,
    pub lines: u64,
    pub bytes: Option<u64>,
}

pub fn get_args() -> Args {
    let matches = Command::new("headr")
        .version("0.1.0")
        .about("Rust version of `head`.")
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .num_args(1..)
                .help("Input File(s)")
                .default_value("-"),
        )
        .arg(
            Arg::new("lines")
                .short('n')
                .long("lines")
                .value_name("LINES")
                .help("Output the first LINES lines")
                .conflicts_with("bytes")
                .default_value("10")
                .value_parser(ValueParser::new(
                    RangedU64ValueParser::<u64>::new().range(1..),
                )),
        )
        .arg(
            Arg::new("bytes")
                .short('c')
                .long("bytes")
                .value_name("BYTES")
                .help("Output the first BYTES bytes")
                .conflicts_with("lines")
                .value_parser(ValueParser::new(
                    RangedU64ValueParser::<u64>::new().range(1..),
                )),
        )
        .get_matches();

    let files = matches
        .get_many("files")
        .expect("None이 아님")
        .cloned()
        .collect();
    let lines = matches.get_one("lines").copied().expect("None이 아님");
    let bytes = matches.get_one("bytes").copied();

    Args {
        files,
        lines,
        bytes,
    }
}
