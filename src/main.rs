use clap::Parser;
use ordered_float::OrderedFloat;
use std::collections::BTreeMap;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    #[clap(short, long, parse(from_os_str))]
    input: PathBuf,
    #[clap(short, long)]
    output: String,
    #[clap(short, long)]
    minimun_sup: f64,
}

fn make_transactions(dataset: String) -> Vec<Vec<String>> {
    dataset
        .split("\n")
        .map(|row| row.split(",").map(|i| i.to_string()).collect())
        .collect()
}

fn make_itemsets(transaction: &Vec<Vec<String>>) -> (BTreeMap<String, f64>, f64) {
    let mut itemsets: BTreeMap<String, f64> = BTreeMap::new();
    let mut len: f64 = 0.0;
    for row in transaction {
        for item in row {
            if item != "" {
                let i = itemsets.entry(String::from(item)).or_insert(0.0);
                *i += 1.0;
            }
        }
        len += 1.0;
    }
    (itemsets, len)
}

fn filter_and_order_itemsets(
    itemsets: &BTreeMap<String, f64>,
    len: f64,
    min_sup: f64,
) -> BTreeMap<&String, OrderedFloat<f64>> {
    let mut ordered_itemsets: BTreeMap<&String, OrderedFloat<f64>> = BTreeMap::new();
    for (hash, support) in itemsets {
        let sup = support / len;
        if sup >= min_sup {
            ordered_itemsets.insert(hash, OrderedFloat(sup));
        }
    }
    ordered_itemsets
}

fn order_row(
    mut row: Vec<String>,
    ordered_itemsets: &BTreeMap<&String, OrderedFloat<f64>>,
) -> Vec<String> {
    row.sort_by(|a, b| {
        ordered_itemsets
            .get(a)
            .cmp(&ordered_itemsets.get(b))
            .reverse()
    });
    row
}

fn main() {
    let args = Cli::parse();

    let dataset = fs::read_to_string(args.input).expect("Something went wrong reading the file");

    let transactions = make_transactions(dataset);

    let (itemsets, len) = make_itemsets(&transactions);

    let ordered_itemsets = filter_and_order_itemsets(&itemsets, len, args.minimun_sup);

    let mut output = fs::File::create(args.output).unwrap();

    for row in transactions {
        for i in order_row(row, &ordered_itemsets) {
            output.write(format!("{}\n", i).as_bytes()).unwrap();
        }
        output.write(format!("\n").as_bytes()).unwrap();
    }
}
