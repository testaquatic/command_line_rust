use std::process;

use grepr::args::Args;

fn main() {
    let args = Args::parse();
    if let Err(err) = args.run() {
        eprintln!("{}", err);
        process::exit(-1);
    }
}
