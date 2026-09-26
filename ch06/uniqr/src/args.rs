use std::{
    fs::File,
    io::{self, BufRead, BufReader, BufWriter, Write},
};

use clap::{Arg, ArgAction, Command};

use crate::context::UniqrContext;

/// 명령줄 인수
#[derive(Debug)]
pub struct Args {
    /// 입력 파일
    pub in_file: String,
    /// 출력 파일
    out_file: Option<String>,
    /// 각 반복되는 줄의 개수를 출력할지 여부
    count: bool,
}

impl Args {
    /// 인수를 파싱
    pub fn parse() -> Args {
        let matches = Command::new("uniqr")
            .about("러스트 버전 `uniq`. 줄의 반복을 출력한다.")
            .arg(
                Arg::new("in_file")
                    .value_name("입력")
                    .help("입력. 지정하지 않으면 STDIN. \"-\"은 STDIN")
                    .default_value("-"),
            )
            .arg(
                Arg::new("out_file")
                    .value_name("출력")
                    .help("출력. 지정하지 않으면 STDOUT."),
            )
            .arg(
                Arg::new("count")
                    .value_name("count")
                    .short('c')
                    .long("count")
                    .help("반복된 줄의 수를 표시한다.")
                    .action(ArgAction::SetTrue),
            )
            .get_matches();

        let in_file = matches.get_one("in_file").cloned().expect("`None`이 아님");
        let out_file = matches.get_one("out_file").cloned();
        let count = matches.get_flag("count");

        Args {
            in_file,
            out_file,
            count,
        }
    }

    pub fn run(&self) -> Result<(), io::Error> {
        match self.in_file.as_str() {
            "-" => {
                let buf_reader = BufReader::new(io::stdin().lock());
                if let Some(out_file) = &self.out_file {
                    let output = BufWriter::new(File::create(out_file)?);
                    process_file(buf_reader, output, self.count)
                } else {
                    let output = BufWriter::new(io::stdout().lock());
                    process_file(buf_reader, output, self.count)
                }
            }
            in_file => {
                let buf_reader = BufReader::new(File::open(in_file)?);
                if let Some(out_file) = &self.out_file {
                    let output = BufWriter::new(File::create(out_file)?);
                    process_file(buf_reader, output, self.count)
                } else {
                    let output = BufWriter::new(io::stdout().lock());
                    process_file(buf_reader, output, self.count)
                }
            }
        }
    }
}

fn process_file(
    mut buf_reader: impl BufRead,
    mut writer: impl Write,
    is_count_print: bool,
) -> Result<(), io::Error> {
    let mut line = String::new();
    let mut prev_line = String::new();
    let mut context = UniqrContext::new();
    let mut prev_count = context.count();

    loop {
        let n = buf_reader.read_line(&mut line)?;
        // 파일의 끝이라면 종료
        if n == 0 {
            // 파일 내용이 비어 있다면 인쇄하지 않고 종료
            if let UniqrContext::Start = context {
                return Ok(());
            }
            return print_line(
                &mut writer,
                if is_count_print {
                    Some(prev_count)
                } else {
                    None
                },
                &prev_line,
            );
        }

        context = context.add_line(&line);
        if let UniqrContext::NewLine { .. } = context {
            if prev_line != "" {
                print_line(
                    &mut writer,
                    if is_count_print {
                        Some(prev_count)
                    } else {
                        None
                    },
                    &prev_line,
                )?;
            }
            prev_line = line;
            line = String::new();
        }
        prev_count = context.count();
        line.clear();
    }
}

fn print_line(writer: &mut impl Write, count: Option<usize>, line: &str) -> Result<(), io::Error> {
    writer.write_fmt(format_args!(
        "{}{}\n",
        count
            .map(|c| format!("{:>7} ", c))
            .unwrap_or("".to_string()),
        line.trim(),
    ))
}
