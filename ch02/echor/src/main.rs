use clap::{Arg, ArgAction, ArgMatches, Command};

struct EchoRCommand {
    text: Vec<String>,
    omit_newline: bool,
}

impl EchoRCommand {
    fn print(&self) {
        let ending = if self.omit_newline { "" } else { "\n" };
        print!("{}{}", self.text.join(" "), ending);
    }
}

impl From<ArgMatches> for EchoRCommand {
    fn from(matches: ArgMatches) -> Self {
        let text = matches
            .get_many("text")
            .expect("입력이 없음: 지금 코드의 경우에는 반드시 Some<T>이다.")
            .cloned()
            .collect::<Vec<String>>();
        let omit_newline = matches.get_flag("omit_newline");

        Self { text, omit_newline }
    }
}

fn main() {
    let matches = Command::new("echor")
        .version("0.1.0")
        .about("Rust version of `echor`")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("input text")
                .required(true)
                .num_args(1..),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .action(ArgAction::SetTrue)
                .help("Do not print newline"),
        )
        .get_matches();

    let echo_r_command = EchoRCommand::from(matches);
    echo_r_command.print();
}
