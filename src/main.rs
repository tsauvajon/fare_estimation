mod haversine;

use serde::Serialize;
use std::fs::File;
use std::io;
use std::io::BufReader;

// const IDLE_SPEED: f32 = 10.0;
// const FARE_PER_SECOND_IDLE: f32 = 11.90 / (60.0 * 60.0);
// const FARE_PER_KM_NIGHT: f32 = 1.30;
// const FARE_PER_KM_DAY: f32 = 0.74;
// const STANDARD_FLAG: f32 = 1.30;
// const MINIMUM_FARE: f32 = 3.47;

fn main() -> Result<(), io::Error> {
    let input = File::open("paths.csv")?;
    read_csv(input)?;

    let fares = vec![
        Fare {
            id: &1,
            amount: &12.34,
        },
        Fare {
            id: &2,
            amount: &45.67,
        },
        Fare {
            id: &3,
            amount: &89.34,
        },
    ];
    let output = File::create("out.csv")?;
    write_csv(output, &fares)?;
    write_csv(io::stdout(), &fares)?;

    Ok(())
}

fn read_csv(input: impl io::Read) -> Result<(), csv::Error> {
    let buffered = BufReader::new(input);

    let mut reader = csv::Reader::from_reader(buffered);
    let _distance = haversine::distance(
        haversine::Location {
            latitude: 37.0,
            longitude: 23.0,
        },
        haversine::Location {
            latitude: -33.0,
            longitude: 151.0,
        },
    );

    for _record in reader.records() {
        // let record = record?;
        // println!(
        //     "In {}, {} built the {} model. It is a {}.",
        //     &record[0], &record[1], &record[2], &record[3]
        // );
    }

    Ok(())
}

#[derive(Serialize)]
struct Fare<'a> {
    id: &'a u32,
    amount: &'a f32,
}

fn write_csv(output: impl io::Write, fares: &Vec<Fare>) -> Result<(), csv::Error> {
    let mut wtr = csv::Writer::from_writer(output);

    for fare in fares {
        wtr.serialize(fare)?;
    }

    wtr.flush()?;

    Ok(())
}
