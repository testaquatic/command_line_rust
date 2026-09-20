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
    println!("{:#?}", args);
}
