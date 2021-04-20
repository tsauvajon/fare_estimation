extern crate fare_estimation;

use fare_estimation::fare_estimation::{estimate_fare, MainError};
use std::fs::File;

fn main() -> Result<(), MainError> {
    let input = File::open("paths.csv")?;
    let output = File::create("out.csv")?;
    estimate_fare(input, output)
}
