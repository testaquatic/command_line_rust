use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader},
    path::{Path, PathBuf},
};

use clap::{Arg, ArgAction, Command};
use regex::{Regex, RegexBuilder};

use crate::error::GreprError;

#[derive(Debug)]
pub struct Args {
    /// 패턴
    pattern: String,
    /// 파일
    files: Vec<String>,
    /// 대소문자 구분하지 않음
    insensitive: bool,
    /// 디렉터리를 재귀적 탐색
    recursive: bool,
    /// 일치하는 줄의 개수만 출력
    count: bool,
    /// 일치하지 않는 줄만 출력
    invert: bool,
}

impl Args {
    pub fn parse() -> Args {
        let matches = Command::new("grepr")
            .version(env!("CARGO_PKG_VERSION"))
            .arg(
                Arg::new("pattern")
                    .value_name("PATTERN")
                    .required(true)
                    .help("Search pattern"),
            )
            .arg(
                Arg::new("files")
                    .value_name("FILE")
                    .num_args(1..)
                    .default_value("-")
                    .help("Input file(s)"),
            )
            .arg(
                Arg::new("insensitive")
                    .short('i')
                    .long("insensitive")
                    .help("Case-insensitive search")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("recursive")
                    .short('r')
                    .long("recursive")
                    .help("Search directories recursively")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("count")
                    .short('c')
                    .long("count")
                    .help("Count matching lines")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("invert")
                    .short('v')
                    .long("invert")
                    .help("Invert match")
                    .action(ArgAction::SetTrue),
            )
            .get_matches();

        let pattern = matches
            .get_one::<String>("pattern")
            .cloned()
            .expect("기본값이 있는 매개변수");
        let files = matches
            .get_many::<String>("files")
            .expect("기본값이 있는 매개변수")
            .cloned()
            .collect::<Vec<_>>();
        let insensitive = matches.get_flag("insensitive");
        let recursive = matches.get_flag("recursive");
        let count = matches.get_flag("count");
        let invert = matches.get_flag("invert");

        Args {
            pattern,
            files,
            insensitive,
            recursive,
            count,
            invert,
        }
    }

    pub fn run(&self) -> Result<(), GreprError> {
        let regex = RegexBuilder::new(&self.pattern)
            .case_insensitive(self.insensitive)
            .build()
            .map_err(|err| GreprError::RegexError {
                regexp_error: err,
                pattern: self.pattern.to_string(),
            })?;

        for file in self.files.iter() {
            if file == "-" {
                process_buf_reader(
                    BufReader::new(io::stdin().lock()),
                    &regex,
                    self.invert,
                    self.count,
                    None,
                )
                .map_err(|err| GreprError::IoError {
                    file: file.to_string(),
                    error: err,
                })?;
            } else {
                let path = Path::new(file);
                if self.recursive {
                    if let Err(err) =
                        process_dir(path, &regex, self.invert, self.count).map_err(|err| {
                            GreprError::IoError {
                                file: path.to_string_lossy().to_string(),
                                error: err,
                            }
                        })
                    {
                        eprintln!("{err}");
                    };
                } else {
                    if path.is_dir() {
                        eprintln!("{} is a directory", path.to_string_lossy());
                        continue;
                    }

                    process_buf_reader(
                        open_file(path).map_err(|err| GreprError::IoError {
                            file: path.to_string_lossy().to_string(),
                            error: err,
                        })?,
                        &regex,
                        self.invert,
                        self.count,
                        if self.files.len() > 1 {
                            Some(path)
                        } else {
                            None
                        },
                    )
                    .map_err(|err| GreprError::IoError {
                        file: path.to_string_lossy().to_string(),
                        error: err,
                    })?;
                }
            }
        }

        Ok(())
    }
}

fn open_dir(path: &Path) -> Result<impl Iterator<Item = Result<PathBuf, io::Error>>, io::Error> {
    let read_dir = fs::read_dir(path)?;

    Ok(read_dir
        .into_iter()
        .map(|entry| entry.map(|item| item.path())))
}

fn process_dir(path: &Path, regex: &Regex, invert: bool, count: bool) -> Result<(), io::Error> {
    for path_result in open_dir(path)? {
        let path = path_result?;
        if path.is_dir() {
            process_dir(&path, regex, invert, count)?;
        } else {
            let buf_reader = BufReader::new(File::open(&path)?);
            process_buf_reader(buf_reader, regex, invert, count, Some(&path))?;
        }
    }

    Ok(())
}

fn open_file(path: &Path) -> Result<impl BufRead, io::Error> {
    Ok(BufReader::new(File::open(path)?))
}

fn process_buf_reader(
    mut buf_reader: impl BufRead,
    regex: &Regex,
    invert: bool,
    count: bool,
    print_path: Option<&Path>,
) -> Result<(), io::Error> {
    let print_line = |line: &str| {
        print_path
            .as_ref()
            .map(|path| print!("{}:{}", path.to_string_lossy(), line))
            .unwrap_or_else(|| print!("{}", line));
    };

    let mut matched_count = 0;
    let mut line = String::new();

    loop {
        if buf_reader.read_line(&mut line)? == 0 {
            break;
        }
        let is_matched = regex.is_match(&line);

        if count && is_matched {
            matched_count += 1;
        } else if is_matched ^ invert {
            print_line(&line);
        }

        line.clear();
    }

    if count {
        print_line(&matched_count.to_string());
        println!();
    }

    Ok(())
}
