use findr::args::Args;

fn main() {
    let args = Args::parse();
    if let Err(e) = args.run() {
        println!("{}", e);
    }
}
