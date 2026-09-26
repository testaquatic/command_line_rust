use std::process;

use uniqr::args::Args;

fn main() {
    let args = Args::parse();
    if let Err(err) = args.run() {
        eprint!("{}: {}", args.in_file, err);
        process::exit(1);
    }
}
