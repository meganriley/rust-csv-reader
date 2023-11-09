use std::error::Error;

use csv;

//comprised of two functions

//first function
fn read_from_file(path: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;

    for result in reader.records() {
        let record = result?;

        println!("{:?}", record)
    }
    Ok(())
}

//second function
fn main() {
    if let Err(e) = read_from_file("./business-file.csv") {
        eprintln!("{}", e)
    }
}