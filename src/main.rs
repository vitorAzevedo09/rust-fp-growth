use clap::Parser;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    #[clap(parse(from_os_str))]
    input: PathBuf,
    output: String,
    minimun_sup: f64,
}

struct Itemset {
    support: f64,
}

fn make_itemsets(transactions: String) -> HashMap<String, Itemset> {
    let mut itemsets: HashMap<String, Itemset> = HashMap::new();
    for row in transactions.split("\n") {
        for item in row.split(",") {
            if item != "" {
                let i = itemsets
                    .entry(String::from(item))
                    .or_insert(Itemset { support: 0.0 });
                i.support += 1.0;
            }
        }
    }
    itemsets
}

fn filter_minimun_support(
    itemsets: HashMap<String, Itemset>,
    min_sup: f64,
) -> HashMap<String, Itemset> {
    let mut filtered_itemsets: HashMap<String, Itemset> = HashMap::new();
    for (hash, itemset) in itemsets {
        if itemset.support >= min_sup {
            filtered_itemsets.insert(hash, itemset);
        }
    }
    filtered_itemsets
}

fn main() {
    let args = Cli::parse();

    let transactions =
        fs::read_to_string(args.input).expect("Something went wrong reading the file");

    let itemsets = filter_minimun_support(make_itemsets(transactions), args.minimun_sup);

    let mut output = fs::File::create(args.output).unwrap();

    for (hash, itemset) in &itemsets {
        output
            .write(format!("{} - {:?}\n", hash, itemset.support).as_bytes())
            .unwrap();
    }
}
