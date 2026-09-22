use headr::{args::get_args, run};

fn main() {
    let args = get_args();
    if let Err(err) = run(args) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
