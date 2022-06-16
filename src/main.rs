use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    #[clap(parse(from_os_str))]
    input: PathBuf,
    output: String,
}

fn main() {
    let args = Cli::parse();
    csv_reader(args.input).unwrap();
}
