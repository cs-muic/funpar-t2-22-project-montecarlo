use std::{error::Error, io, process};
use rayon::prelude::*;

fn read_csv() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());

    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the error here.

        // https://doc.rust-lang.org/reference/expressions/operator-expr.html#the-question-mark-operator

        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn monte_carlo_simulator()
{
    // TODO:
}

// Use command below to run
// cargo run < ../test_cases/test1.csv

fn main() {
    if let Err(err) = read_csv() {
        // For handling read error
        println!("error running example: {}", err);
        process::exit(1);
    }
}