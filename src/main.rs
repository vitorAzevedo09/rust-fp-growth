use clap::Parser;
use std::collections::HashMap;
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

struct Itemset {
    support: f64,
}

fn make_itemsets(dataset: String) -> (HashMap<String, Itemset>, f64) {
    let mut itemsets: HashMap<String, Itemset> = HashMap::new();
    let mut len: f64 = 0.0;
    for row in dataset.split("\n") {
        for item in row.split(",") {
            if item != "" {
                let i = itemsets
                    .entry(String::from(item))
                    .or_insert(Itemset { support: 0.0 });
                i.support += 1.0;
            }
        }
        len += 1.0;
    }
    (itemsets, len)
}

fn filter_itemsets(
    itemsets: HashMap<String, Itemset>,
    len: f64,
    min_sup: f64,
) -> HashMap<String, Itemset> {
    let mut filtered_itemsets: HashMap<String, Itemset> = HashMap::new();
    for (hash, itemset) in itemsets {
        let sup = itemset.support / len;
        if sup >= min_sup {
            filtered_itemsets.insert(hash, Itemset { support: sup });
        }
    }
    filtered_itemsets
}

fn main() {
    let args = Cli::parse();

    let transactions =
        fs::read_to_string(args.input).expect("Something went wrong reading the file");

    let (itemsets, len) = make_itemsets(transactions);

    let filtered_itemsets = filter_itemsets(itemsets, len, args.minimun_sup);

    let mut output = fs::File::create(args.output).unwrap();

    for (hash, i) in &filtered_itemsets {
        output
            .write(format!("{} - {:?}\n", hash, i.support).as_bytes())
            .unwrap();
    }
}
