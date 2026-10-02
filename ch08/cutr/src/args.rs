use std::{
    cmp::Ordering,
    fs,
    io::{self, BufRead, BufReader},
    process,
    str::FromStr,
};

use anyhow::anyhow;
use clap::{Arg, ArgGroup, Command};

#[derive(Debug)]
pub struct Args {
    files: Vec<String>,
    delimiter: String,
    extract: ArgExtract,
}

#[derive(Debug)]
pub struct ArgExtract {
    fields: Option<String>,
    bytes: Option<String>,
    chars: Option<String>,
}

impl Args {
    pub fn parse() -> Args {
        let matches = Command::new("cutr")
            .version(env!("CARGO_PKG_VERSION"))
            .about("Rust version of `cut`")
            .arg(
                Arg::new("files")
                    .value_name("FILES")
                    .num_args(0..)
                    .help("Input file(s)")
                    .default_value("-"),
            )
            .arg(
                Arg::new("delimiter")
                    .short('d')
                    .long("delim")
                    .value_name("DELIMITER")
                    .help("Field delimiter")
                    .default_value("\t"),
            )
            .arg(
                Arg::new("fields")
                    .short('f')
                    .long("fields")
                    .value_name("FIELDS")
                    .help("Select fields"),
            )
            .arg(
                Arg::new("bytes")
                    .short('b')
                    .long("bytes")
                    .value_name("BYTES")
                    .help("Select bytes"),
            )
            .arg(
                Arg::new("chars")
                    .short('c')
                    .long("chars")
                    .value_name("CHARS")
                    .help("Select chars"),
            )
            .group(
                ArgGroup::new("count")
                    .args(["fields", "bytes", "chars"])
                    .required(true)
                    .multiple(false),
            )
            .get_matches();

        let files = matches
            .get_many("files")
            .expect("기본값이 있는 arg")
            .cloned()
            .collect();

        let delimiter = matches
            .get_one::<String>("delimiter")
            .expect("기본값이 있는 arg")
            .to_owned();

        if delimiter.len() != 1 {
            eprintln!(r#"--delim "{}" must be a single byte"#, delimiter);
            process::exit(-1);
        }

        let fields = matches
            .get_many::<String>("fields")
            .map(|item| item.cloned().collect());

        let bytes = matches
            .get_many::<String>("bytes")
            .map(|item| item.cloned().collect());

        let chars = matches
            .get_many::<String>("chars")
            .map(|item| item.cloned().collect());

        Args {
            files,
            delimiter,
            extract: ArgExtract {
                fields,
                bytes,
                chars,
            },
        }
    }

    pub fn run(&self) -> Result<(), io::Error> {
        let get_range = |range_str: &str| match Range::from_str(range_str) {
            Ok(range) => range,
            Err(e) => {
                eprintln!("{}", e);
                process::exit(-1);
            }
        };

        let mut range = if let Some(fields) = &self.extract.fields {
            get_range(fields)
        } else if let Some(bytes) = &self.extract.bytes {
            get_range(bytes)
        } else if let Some(chars) = &self.extract.chars {
            get_range(chars)
        } else {
            unreachable!();
        };

        for filename in &self.files {
            let f = match open_file(filename) {
                Ok(f) => f,
                Err(err) => {
                    eprintln!("{}: {}", filename, err);
                    continue;
                }
            };
            for line in f.lines() {
                if self.extract.fields.is_some() {
                    range.fields(&line?, self.delimiter.as_bytes()[0])?;
                } else if self.extract.bytes.is_some() {
                    range.bytes(&line?)?;
                } else if self.extract.chars.is_some() {
                    range.chars(&line?)?
                }
            }
        }

        Ok(())
    }
}

fn open_file(file_path: &str) -> Result<Box<dyn BufRead>, io::Error> {
    match file_path {
        "-" => Ok(Box::new(BufReader::new(io::stdin().lock()))),
        _ => Ok(Box::new(BufReader::new(fs::File::open(file_path)?))),
    }
}

#[derive(Debug)]
pub struct Range(Vec<usize>);

impl Range {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn fields(&mut self, input: &str, delimiter: u8) -> Result<(), csv::Error> {
        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(delimiter)
            .has_headers(false)
            .from_reader(input.as_bytes());

        let mut next_iter = self.iter();
        let mut next = next_iter.next();
        for record in rdr.records() {
            let mut matched = false;
            for (idx, field) in record?.iter().enumerate() {
                match next {
                    Some(val) => match val.cmp(&(idx + 1)) {
                        Ordering::Less => {
                            next = next_iter.next();
                        }
                        Ordering::Equal => loop {
                            print!(
                                "{}{}",
                                if matched {
                                    format!("{}", delimiter as char)
                                } else {
                                    String::new()
                                },
                                field
                            );
                            matched = true;
                            next = next_iter.next();

                            let Some(next_val) = next else {
                                break;
                            };
                            if *next_val != idx + 1 {
                                break;
                            }
                        },
                        Ordering::Greater => {}
                    },
                    None => break,
                }
            }
            if matched {
                println!();
            }
        }

        Ok(())
    }

    fn bytes(&mut self, input: &str) -> Result<(), io::Error> {
        let mut buf = Vec::new();
        let bytes = input.as_bytes();

        for idx in self.iter() {
            if let Some(byte) = bytes.get(idx - 1) {
                buf.push(*byte);
            }
        }
        if !buf.is_empty() {
            println!("{}", String::from_utf8_lossy(&buf));
        }

        Ok(())
    }

    fn chars(&mut self, input: &str) -> Result<(), io::Error> {
        let mut next_iter = self.iter();
        let mut next = next_iter.next();
        let mut matched = false;

        for (idx, ch) in input.chars().enumerate() {
            match next {
                Some(val) => match val.cmp(&(idx + 1)) {
                    Ordering::Less => {
                        next = next_iter.next();
                    }
                    Ordering::Equal => loop {
                        print!("{}", ch);
                        matched = true;
                        next = next_iter.next();
                        let Some(next_val) = next else {
                            break;
                        };
                        if *next_val != idx + 1 {
                            break;
                        }
                    },
                    Ordering::Greater => {}
                },
                None => break,
            }
        }
        if matched {
            println!()
        }

        Ok(())
    }

    fn push(&mut self, num: usize) {
        self.0.push(num);
    }

    fn extend(&mut self, other: impl IntoIterator<Item = usize>) {
        self.0.extend(other);
    }

    fn iter(&self) -> impl Iterator<Item = &usize> {
        self.0.iter()
    }

    fn sort(&mut self) {
        self.0.sort();
    }
}

impl FromStr for Range {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut range = Range::new();
        let error = || anyhow!(r#"illegal list value: "{}""#, s);
        for item in s.trim().split(",") {
            let start_to_end = item
                .split("-")
                .take(3)
                .map(|num| {
                    if num.starts_with("+") {
                        Err(error())
                    } else {
                        num.parse::<usize>()
                            .map_err(|_| error())
                            .and_then(|num| if num == 0 { Err(error()) } else { Ok(num) })
                    }
                })
                .collect::<Result<Vec<usize>, anyhow::Error>>()?;
            match start_to_end.len() {
                0 => return Err(error()),
                1 => {
                    range.push(start_to_end[0]);
                }
                2 => {
                    let start = start_to_end[0];
                    let end = start_to_end[1];
                    if start >= end {
                        anyhow::bail!(
                            "First number in range ({}) must be lower than second number ({})",
                            start,
                            end
                        );
                    }
                    range.extend(start..=end);
                }
                _ => return Err(error()),
            }
        }

        range.sort();

        Ok(range)
    }
}
