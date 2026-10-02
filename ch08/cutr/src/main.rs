use std::process;

use cutr::args::Args;

fn main() {
    let args = Args::parse();
    if let Err(e) = args.run() {
        eprintln!("{}", e);
        process::exit(-1);
    }
}
