use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use clap::{Arg, ArgAction, Command};

/// 명령줄 매개변수
#[derive(Debug)]
struct Args {
    /// 파일 목록
    files: Vec<String>,
    /// 줄 번호 표시
    number_lines: bool,
    /// 빈 줄은 건너 뛰는 줄 번호 표시
    number_nonblank_lines: bool,
}

fn run(args: Args) -> Result<(), anyhow::Error> {
    let mut line_num = 1_usize;

    for file_name in &args.files {
        let file = match open(file_name) {
            Ok(file) => file,
            Err(err) => {
                eprintln!("Failed to open {}: {}", file_name, err);
                continue;
            }
        };

        for line in file.lines() {
            let line_string = line?;
            if args.number_lines || (args.number_nonblank_lines && !line_string.is_empty()) {
                print!("{:>6}\t", line_num);
                line_num += 1;
            }
            println!("{}", line_string);
        }
    }

    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>, std::io::Error> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(std::io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn get_args() -> Args {
    let matches = Command::new("catr")
        .version("0.1.0")
        .about("Rust version of `cat`")
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("With no FILE, or when FILE is -, read standard input.")
                .num_args(1..)
                .default_value("-"),
        )
        .arg(
            Arg::new("number_lines")
                .long("number")
                .short('n')
                .help("number all output lines")
                .action(ArgAction::SetTrue)
                .conflicts_with("number_nonblank_lines"),
        )
        .arg(
            Arg::new("number_nonblank_lines")
                .short('b')
                .long("number-nonblank")
                .help("number nonempty output lines")
                .action(ArgAction::SetTrue)
                .conflicts_with("number_lines"),
        )
        .get_matches();

    let files = matches
        .get_many("files")
        .expect("files는 기본값이 있으므로 도달할 수 없음")
        .cloned()
        .collect::<Vec<String>>();
    let number_lines = matches.get_flag("number_lines");
    let number_nonblank_lines = matches.get_flag("number_nonblank_lines");

    Args {
        files,
        number_lines,
        number_nonblank_lines,
    }
}

fn main() {
    let args = get_args();
    if let Err(err) = run(args) {
        eprint!("{}", err);
        std::process::exit(1);
    }
}
