use std::collections::BTreeMap;
use std::collections::HashMap;

mod tree;

pub fn make_transactions(dataset: String) -> Vec<Vec<String>> {
    dataset
        .split("\n")
        .map(|row| row.split(",").map(|i| i.to_string()).collect())
        .collect()
}

pub fn make_itemsets(transaction: &Vec<Vec<String>>) -> HashMap<String, u64> {
    let mut itemsets: HashMap<String, u64> = HashMap::new();
    for row in transaction {
        for item in row {
            if item != "" {
                let i = itemsets.entry(String::from(item)).or_insert(0);
                *i += 1;
            }
        }
    }
    itemsets
}

pub fn filter_itemsets(
    itemsets: HashMap<String, u64>,
    len: usize,
    min_sup: f64,
) -> BTreeMap<String, u64> {
    let mut filtered_itemsets: BTreeMap<String, u64> = BTreeMap::new();
    for (hash, support) in itemsets {
        let sup = len as f64 * min_sup;
        if sup as u64 <= support {
            filtered_itemsets.entry(hash).or_insert(support);
        }
    }
    filtered_itemsets
}

pub fn order_row(mut row: Vec<String>, ordered_itemsets: &BTreeMap<String, u64>) -> Vec<String> {
    row.sort_by(|a, b| {
        ordered_itemsets
            .get(a)
            .cmp(&ordered_itemsets.get(b))
            .reverse()
    });
    row
}
