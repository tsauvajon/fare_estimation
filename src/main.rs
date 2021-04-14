mod haversine;

use serde::Serialize;
use std::convert::From;
use std::fs::File;
use std::io;
use std::io::BufReader;

// const IDLE_SPEED: f32 = 10.0;
// const FARE_PER_SECOND_IDLE: f32 = 11.90 / (60.0 * 60.0);
// const FARE_PER_KM_NIGHT: f32 = 1.30;
// const FARE_PER_KM_DAY: f32 = 0.74;
// const STANDARD_FLAG: f32 = 1.30;
// const MINIMUM_FARE: f32 = 3.47;

#[derive(Debug)]
enum ReadError {
    MissingValueError { field: String },
    // InvalidValueError { field: String, value: String },
    CSVError(csv::Error),
}

impl From<csv::Error> for ReadError {
    fn from(error: csv::Error) -> Self {
        ReadError::CSVError(error)
    }
}

#[derive(Debug)]
enum MainError {
    ReadError(ReadError),
    IOError(io::Error),
}

impl From<io::Error> for MainError {
    fn from(error: io::Error) -> Self {
        MainError::IOError(error)
    }
}

impl From<ReadError> for MainError {
    fn from(error: ReadError) -> Self {
        MainError::ReadError(error)
    }
}

fn main() -> Result<(), MainError> {
    let input = File::open("paths.csv")?;
    let rides = read_csv(input)?;

    let mut fares: Vec<Fare> = vec![];
    for ride in rides {
        fares.push(Fare {
            id: ride.id,
            amount: 12.34,
        })
    }

    let output = File::create("out.csv")?;
    write_csv(output, &fares)?;
    write_csv(io::stdout(), &fares)?;

    Ok(())
}

struct Ride {
    id: u32,
    positions: Vec<haversine::Location>,
}

type Record = (Option<u32>, Option<f64>, Option<f64>, Option<u32>);

fn read_csv(input: impl io::Read) -> Result<Vec<Ride>, ReadError> {
    let buffered = BufReader::new(input);

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(buffered);
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

    let mut rides: Vec<Ride> = vec![];
    let mut current_ride_id: Option<u32> = None;
    let mut positions: Vec<haversine::Location> = vec![];

    for record in reader.deserialize() {
        let record: Record = record?;
        let (id, lat, lon, _timestamp) = record;
        let loc = haversine::Location {
            latitude: match lat {
                Some(lat) => lat,
                None => {
                    return Err(ReadError::MissingValueError {
                        field: "latitude".to_string(),
                    })
                }
            },
            longitude: match lon {
                Some(lon) => lon,
                None => {
                    return Err(ReadError::MissingValueError {
                        field: "longitude".to_string(),
                    })
                }
            },
        };

        let valid_id = match id {
            Some(id) => id,
            None => {
                return Err(ReadError::MissingValueError {
                    field: "id".to_string(),
                })
            }
        };

        if let Some(cri) = current_ride_id {
            if cri != valid_id {
                rides.push(Ride {
                    id: cri,
                    positions: positions.clone(),
                });
                positions = vec![];
            }
        }

        positions.push(loc);
        current_ride_id = Some(valid_id);
    }

    rides.push(Ride {
        id: current_ride_id.unwrap(),
        positions: positions,
    });

    Ok(rides)
}

#[derive(Serialize)]
struct Fare {
    id: u32,
    amount: f32,
}

fn write_csv(output: impl io::Write, fares: &Vec<Fare>) -> Result<(), io::Error> {
    let mut writer = csv::WriterBuilder::new()
        .has_headers(false)
        .from_writer(output);

    for fare in fares {
        writer.serialize(fare)?;
    }

    writer.flush()?;

    Ok(())
}
