use serde::Serialize;
use std::fs::File;
use std::io;
use std::io::BufReader;

fn main() -> Result<(), io::Error> {
    let input = File::open("paths.csv")?;
    read_csv(input)?;

    let fares = vec![
        Fare { id: &1, amount: &12.34 },
        Fare { id: &2, amount: &45.67 },
        Fare { id: &3, amount: &89.34 },
    ];
    let output = File::create("out.csv")?;
    write_csv(output, &fares)?;
    write_csv(io::stdout(), &fares)?;

    Ok(())
}

fn read_csv(input: impl io::Read) -> Result<(), csv::Error> {
    let buffered = BufReader::new(input);

    let mut reader = csv::Reader::from_reader(buffered);
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

pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
}

#[test]
fn haversine_distance_test() {
    assert_eq!(
        0.5491557912038084,
        haversine_distance(
            Location {
                latitude: 38.898556,
                longitude: -77.037852
            },
            Location {
                latitude: 38.897147,
                longitude: -77.043934
            },
        ),
    );

    assert_eq!(
        15328.17837221522,
        haversine_distance(
            Location {
                latitude: -33.8883368,
                longitude: 151.1931148
            },
            Location {
                latitude: 37.990832,
                longitude: 23.7032341
            },
        ),
    );

    assert_eq!(
        haversine_distance(
            Location {
                latitude: 37.990832,
                longitude: 23.7032341
            },
            Location {
                latitude: -33.8883368,
                longitude: 151.1931148
            },
        ),
        haversine_distance(
            Location {
                latitude: -33.8883368,
                longitude: 151.1931148
            },
            Location {
                latitude: 37.990832,
                longitude: 23.7032341
            },
        ),
    );
}

pub fn haversine_distance(start: Location, end: Location) -> f64 {
    let r: f64 = 6371.0;

    let d_lat: f64 = (end.latitude - start.latitude).to_radians();
    let d_lon: f64 = (end.longitude - start.longitude).to_radians();
    let lat1: f64 = (start.latitude).to_radians();
    let lat2: f64 = (end.latitude).to_radians();

    let a: f64 = ((d_lat / 2.0).sin()) * ((d_lat / 2.0).sin())
        + ((d_lon / 2.0).sin()) * ((d_lon / 2.0).sin()) * (lat1.cos()) * (lat2.cos());
    let c: f64 = 2.0 * ((a.sqrt()).atan2((1.0 - a).sqrt()));

    return r * c;
}
