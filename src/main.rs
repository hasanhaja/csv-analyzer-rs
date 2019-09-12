//use serde::{
//    Serialize,
//    Deserialize,
//};
use csv;
use std::error::Error;
use std::io;
use std::process;

trait Statistical {
    fn mean(&self) -> f64;
    fn mode(&self) -> f64;
    fn median(&self) -> f64;
    fn stdev(&self) -> f64;
    fn variance(&self) -> f64;
}

impl Statistical for Vec<f64> {
    fn mean(&self) -> f64 {
        let n = self.len() as f64;
        let sum: f64 = self.iter().sum();
        sum/n
    }

    fn mode(&self) -> f64 {
        unimplemented!();
    }
    fn median(&self) -> f64 {
        unimplemented!();
    }

    fn stdev(&self) -> f64 {
        self.variance().sqrt()
    }

    fn variance(&self) -> f64 {
        let n = self.len() as f64;
        let square_summed = self.iter().fold(0.0, |sum, x| sum + x*x);
        let mean = self.mean();

        (square_summed/n) - (mean*mean)
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b';')
        .flexible(true)
        .from_reader(io::stdin());

    let mut prices = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let price: f64 = record[1].parse()?;

        prices.push(price);
    }

    if !prices.is_empty() {
        let mean = prices.mean();
        let stdev = prices.stdev();
        let variance = prices.variance();

        println!("The mean price is {}.", mean);
        println!("The standard deviation is {}.", stdev);
        println!("The variance is {}.", variance);
    }

    Ok(())
}


fn main() {

    if let Err(err) = run() {
        println!("error running example: {}", err);
        process::exit(1);
    }

}