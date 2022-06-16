use clap::Parser;
use csv::Reader;
use std::error::Error;
use std::fs::read_to_string;

#[derive(Parser)]
struct Cli {
    #[clap(parse(from_os_str))]
    input: std::path::PathBuf,
    output: String,
}

fn csv_reader(output: String) -> Result<(), Box<dyn Error>> {
    let mut rdr = Reader::from_path(output)?;
    println!("{:?}", rdr);
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    let args = Cli::parse();
    let input = read_to_string(&args.input).expect("could not read file");
    csv_reader(input).unwrap();
}
