use std::{error::Error, process, fs::File};

fn example() -> Result<(), Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    let file = File::open("Organization_v3.csv")?;
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        let title = &record[0];
        println!("{}", title);
    }
    Ok(())
}

fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}