use clap::Parser;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    #[clap(parse(from_os_str))]
    input: PathBuf,
    output: String,
}

fn main() {
    let args = Cli::parse();
    let transactions =
        fs::read_to_string(args.input).expect("Something went wrong reading the file");
    let mut output = fs::File::create(args.output).unwrap();
    output.write(transactions.as_bytes()).unwrap();
}
