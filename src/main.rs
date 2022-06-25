use clap::Parser;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

mod preprocess;

#[derive(Parser)]
struct Cli {
    #[clap(short, long, parse(from_os_str))]
    input: PathBuf,
    #[clap(short, long)]
    output: String,
    #[clap(short, long)]
    minimun_sup: f64,
}

fn main() {
    let args = Cli::parse();

    let dataset = fs::read_to_string(args.input).expect("Something went wrong reading the file");

    let transactions = preprocess::make_transactions(dataset);

    let itemsets = preprocess::make_itemsets(&transactions);

    let ordered_itemsets =
        preprocess::filter_itemsets(itemsets, transactions.len(), args.minimun_sup);

    let mut output = fs::File::create(args.output).unwrap();

    for row in transactions {
        for i in preprocess::order_row(row, &ordered_itemsets) {
            output.write(format!("{}\n", i).as_bytes()).unwrap();
        }
        output.write(format!("\n").as_bytes()).unwrap();
    }
}
