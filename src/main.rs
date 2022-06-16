use csv::Reader;
use std::error::Error;

fn example() -> Result<(), Box<dyn Error>> {
    let mut rdr = Reader::from_path("./csv/market-basket-transactions.csv")?;
    println!("{:?}", rdr);
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    example().unwrap();
}
