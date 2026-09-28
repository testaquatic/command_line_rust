use std::{
    fs::{self},
    io,
    path::{Path, PathBuf},
};

use clap::{
    Arg, ArgAction, Command, ValueEnum,
    builder::{EnumValueParser, PossibleValue},
};
use regex::Regex;

#[derive(Debug)]
pub struct Args {
    paths: Vec<String>,
    names: Vec<Regex>,
    entry_types: Vec<EntryType>,
}

impl Args {
    pub fn parse() -> Args {
        let matches = Command::new("findr")
            .about("find의 러스트 버전, 파일을 찾는다.")
            .version(env!("CARGO_PKG_VERSION"))
            .arg(
                Arg::new("paths")
                    .value_name("경로")
                    .help("검색 경로")
                    .num_args(0..)
                    .default_value("."),
            )
            .arg(
                Arg::new("names")
                    .short('n')
                    .long("name")
                    .help("이름 패턴")
                    .num_args(0..)
                    .value_parser(Regex::new)
                    .action(ArgAction::Append)
                    .value_name("이름"),
            )
            .arg(
                Arg::new("entry_types")
                    .short('t')
                    .long("type")
                    .value_name("유형")
                    .help("대상의 유형")
                    .num_args(0..)
                    .action(ArgAction::Append)
                    .value_parser(EnumValueParser::<EntryType>::new()),
            )
            .get_matches();

        let paths = matches
            .get_many("paths")
            .unwrap_or_default()
            .cloned()
            .collect();

        let names = matches
            .get_many("names")
            .unwrap_or_default()
            .cloned()
            .collect();

        let entry_types = matches
            .get_many("entry_types")
            .unwrap_or_default()
            .cloned()
            .collect();

        Args {
            paths,
            names,
            entry_types,
        }
    }

    pub fn run(&self) -> Result<(), io::Error> {
        for path_string in self.paths.iter() {
            let path = PathBuf::from(path_string);
            error_printer(&path, find_from_path(&path, &self.names, &self.entry_types));
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EntryType {
    Directory,
    File,
    SymbolicLink,
    Unknown,
}

impl ValueEnum for EntryType {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            EntryType::Directory,
            EntryType::File,
            EntryType::SymbolicLink,
        ]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        let possible_value = match self {
            EntryType::Directory => PossibleValue::new("d"),
            EntryType::File => PossibleValue::new("f"),
            EntryType::SymbolicLink => PossibleValue::new("l"),
            EntryType::Unknown => return None,
        };

        Some(possible_value)
    }
}

fn find_from_path(
    path: &Path,
    names: &[Regex],
    entry_types: &[EntryType],
) -> Result<(), io::Error> {
    let last_string = path
        .iter()
        .next_back()
        .expect("입력이 반드시 있어야 함")
        .to_string_lossy();
    let entry_type = file_type(path)?;

    if is_types_match(&entry_type, entry_types) && is_names_match(&last_string, names) {
        println!("{}", path.to_string_lossy());
    }

    if entry_type == EntryType::Directory {
        for next in next_path(path)? {
            let next_path = next?;
            error_printer(&next_path, find_from_path(&next_path, names, entry_types));
        }
    }

    Ok(())
}

fn is_types_match(entry_type: &EntryType, types: &[EntryType]) -> bool {
    if types.is_empty() {
        return true;
    }
    types.iter().any(|item| entry_type == item)
}

fn is_names_match(path: &str, names: &[Regex]) -> bool {
    if names.is_empty() {
        return true;
    }
    names.iter().any(|regex| regex.is_match(path))
}

fn error_printer(path: &Path, result: Result<(), io::Error>) {
    if let Err(e) = result {
        eprintln!("{}: {}", path.display(), e);
    }
}

fn file_type(path: &Path) -> Result<EntryType, io::Error> {
    let meta = fs::symlink_metadata(path)?;
    let entry_type = if meta.is_dir() {
        EntryType::Directory
    } else if meta.is_file() {
        EntryType::File
    } else if meta.is_symlink() {
        EntryType::SymbolicLink
    } else {
        EntryType::Unknown
    };

    Ok(entry_type)
}

fn next_path(dir: &Path) -> Result<impl Iterator<Item = Result<PathBuf, io::Error>>, io::Error> {
    let dir = fs::read_dir(dir)?;
    let dir_iter = dir
        .into_iter()
        .map(|dir_entry_result| dir_entry_result.map(|dir_entry| dir_entry.path()));

    Ok(dir_iter)
}
