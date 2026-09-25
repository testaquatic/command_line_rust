use std::{
    fs,
    io::{self, BufReader},
};

use clap::{
    Arg,
    ArgAction::{self},
    Command,
};

use crate::utils::{Count, process_line};

#[derive(Debug)]
pub struct Args {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

impl Args {
    pub fn parse() -> Args {
        let matches = Command::new("wcr")
            .version("0.1.0")
            .about("Rust version of wc.")
            .arg(
                Arg::new("files")
                    .value_name("FILE")
                    .num_args(0..)
                    .default_value("-")
                    .help("파일. \"-\"은 STDIN으로 연결된다."),
            )
            .arg(
                Arg::new("lines")
                    .short('l')
                    .long("lines")
                    .help("줄 수를 표시한다")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("words")
                    .short('w')
                    .long("words")
                    .help("단어의 수를 표시한다.")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("bytes")
                    .short('c')
                    .long("bytes")
                    .help("바이트의 수를 표시한다.")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("chars")
                    .short('m')
                    .long("chars")
                    .help("문자의 수를 표시한다.")
                    .action(ArgAction::SetTrue),
            )
            .get_matches();

        let files = matches
            .get_many("files")
            .expect("None이 될 수 없음")
            .cloned()
            .collect();

        let mut lines = matches.get_flag("lines");
        let mut words = matches.get_flag("words");
        let mut bytes = matches.get_flag("bytes");
        let chars = matches.get_flag("chars");
        if !(lines || words || bytes || chars) {
            (lines, words, bytes) = (true, true, true);
        }

        Args {
            files,
            lines,
            words,
            bytes,
            chars,
        }
    }

    pub fn run(&self) -> Result<(), io::Error> {
        let mut total = Count::new(self.lines, self.words, self.bytes, self.chars);

        for filename in self.files.iter() {
            let count = if filename == "-" {
                process_line(
                    BufReader::new(io::stdin().lock()),
                    self.lines,
                    self.words,
                    self.bytes,
                    self.chars,
                )
            } else {
                let file = match fs::File::open(filename) {
                    Ok(file) => file,
                    Err(e) => {
                        eprintln!("{filename}: {e}");
                        continue;
                    }
                };
                process_line(
                    BufReader::new(file),
                    self.lines,
                    self.words,
                    self.bytes,
                    self.chars,
                )
            }?;

            count.print_line(filename, &mut io::stdout().lock())?;

            total = total.sum(&count);
        }

        if self.files.len() >= 2 {
            total.print_line("total", &mut io::stdout().lock())?;
        }

        Ok(())
    }
}
