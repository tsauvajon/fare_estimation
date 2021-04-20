mod haversine;

use chrono::prelude::*;
use chrono::{DateTime, Utc};
use rayon::prelude::*;
use serde::{Serialize, Serializer};
use std::convert::From;
use std::io;
use std::io::BufReader;

const MAX_SPEED: f64 = 100.0;
const IDLE_SPEED: f64 = 10.0;
const FARE_PER_SECOND_IDLE: f64 = 11.90 / (60.0 * 60.0);
const FARE_PER_KM_NIGHT: f64 = 1.30;
const FARE_PER_KM_DAY: f64 = 0.74;
const STANDARD_FLAG: f64 = 1.30;
const MINIMUM_FARE: f64 = 3.47;

#[derive(Debug)]
pub enum MainError {
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

pub fn estimate_fare(input: impl io::Read, output: impl io::Write) -> Result<(), MainError> {
    let rides = read_csv(input)?;
    let fares = calculate_all_fares(rides);
    write_csv(output, &fares)?;

    Ok(())
}

#[derive(Clone, Debug)]
struct Position {
    datetime: DateTime<Utc>,
    location: haversine::Location,
}

struct Segment {
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    distance_km: f64,
}

#[test]
fn segment_speed() {
    let day_segment = Segment {
        start: Utc.ymd(2019, 1, 1).and_hms(0, 0, 0),
        end: Utc.ymd(2019, 1, 1).and_hms(2, 0, 0),
        distance_km: 50.0,
    };
    assert_eq!(25.0, day_segment.speed());
    let night_segment = Segment {
        start: Utc.ymd(2019, 1, 1).and_hms(0, 0, 0),
        end: Utc.ymd(2019, 1, 1).and_hms(0, 30, 0),
        distance_km: 200.0,
    };
    assert_eq!(400.0, night_segment.speed());
}

#[test]
fn segment_duration() {
    let day_segment = Segment {
        start: Utc.ymd(2019, 1, 1).and_hms(0, 0, 0),
        end: Utc.ymd(2019, 1, 1).and_hms(2, 0, 0),
        distance_km: 50.0,
    };
    assert_eq!(7200, day_segment.duration_seconds());

    let night_segment = Segment {
        start: Utc.ymd(2019, 1, 1).and_hms(0, 0, 0),
        end: Utc.ymd(2019, 1, 1).and_hms(0, 30, 0),
        distance_km: 200.0,
    };
    assert_eq!(1800, night_segment.duration_seconds());
}

#[test]
fn segment_fare() {
    let day_segment = Segment {
        start: Utc.ymd(2019, 1, 1).and_hms(10, 0, 0),
        end: Utc.ymd(2019, 1, 1).and_hms(12, 0, 0),
        distance_km: 50.0,
    };
    assert_eq!(37.0, day_segment.get_fare());

    let idle_day_segment = Segment {
        start: Utc.ymd(2019, 1, 1).and_hms(10, 0, 0),
        end: Utc.ymd(2019, 1, 1).and_hms(11, 0, 0),
        distance_km: 0.0,
    };
    assert_eq!(11.90, idle_day_segment.get_fare());

    let night_segment = Segment {
        start: Utc.ymd(2019, 1, 1).and_hms(1, 0, 0),
        end: Utc.ymd(2019, 1, 1).and_hms(1, 30, 0),
        distance_km: 200.0,
    };
    assert_eq!(260.0, night_segment.get_fare());
}

#[test]
fn segment_is_idle() {
    let idle_segment = Segment {
        start: Utc.ymd(2019, 1, 1).and_hms(10, 0, 0),
        end: Utc.ymd(2019, 1, 1).and_hms(11, 0, 0),
        distance_km: 0.0,
    };
    assert_eq!(true, idle_segment.is_idle());

    let barely_idle_segment = Segment {
        start: Utc.ymd(2019, 1, 1).and_hms(10, 0, 0),
        end: Utc.ymd(2019, 1, 1).and_hms(11, 0, 0),
        distance_km: 10.0,
    };
    assert_eq!(true, barely_idle_segment.is_idle());

    let moving_idle_segment = Segment {
        start: Utc.ymd(2019, 1, 1).and_hms(10, 0, 0),
        end: Utc.ymd(2019, 1, 1).and_hms(11, 0, 0),
        distance_km: 50.0,
    };
    assert_eq!(false, moving_idle_segment.is_idle());
}

#[test]
fn segment_is_day() {
    let day_segment = Segment {
        start: Utc.ymd(2019, 1, 1).and_hms(10, 0, 0),
        end: Utc.ymd(2019, 1, 1).and_hms(12, 0, 0),
        distance_km: 50.0,
    };
    assert_eq!(true, day_segment.is_day());

    let late_day_segment = Segment {
        start: Utc.ymd(2019, 1, 1).and_hms(0, 0, 0),
        end: Utc.ymd(2019, 1, 1).and_hms(0, 30, 0),
        distance_km: 200.0,
    };
    assert_eq!(true, late_day_segment.is_day());

    let night_segment = Segment {
        start: Utc.ymd(2019, 1, 1).and_hms(5, 0, 0),
        end: Utc.ymd(2019, 1, 1).and_hms(20, 30, 0),
        distance_km: 200.0,
    };
    assert_eq!(false, night_segment.is_day());

    let early_night_segment = Segment {
        start: Utc.ymd(2019, 1, 1).and_hms(0, 0, 1),
        end: Utc.ymd(2019, 1, 1).and_hms(0, 30, 0),
        distance_km: 200.0,
    };
    assert_eq!(false, early_night_segment.is_day());
}

impl Segment {
    fn speed(&self) -> f64 {
        if self.distance_km == 0.0 {
            return 0.0;
        }
        let dt = self.duration_seconds();
        if dt == 0 {
            return f64::INFINITY;
        }

        let hours = dt as f64 / 3600.0;
        let kmph_speed = self.distance_km / hours;

        kmph_speed
    }

    fn duration_seconds(&self) -> i64 {
        self.end.timestamp() - self.start.timestamp()
    }

    fn get_fare(&self) -> f64 {
        if self.is_idle() {
            FARE_PER_SECOND_IDLE * self.duration_seconds() as f64
        } else if self.is_day() {
            FARE_PER_KM_DAY * self.distance_km
        } else {
            FARE_PER_KM_NIGHT * self.distance_km
        }
    }

    fn is_idle(&self) -> bool {
        self.speed() <= IDLE_SPEED
    }

    fn is_day(&self) -> bool {
        if self.start.num_seconds_from_midnight() == 0 {
            return true;
        }

        // how to declare that only once?
        let start_of_day = Utc
            .ymd(1, 1, 1)
            .and_hms(5, 0, 1)
            .num_seconds_from_midnight();

        self.start.num_seconds_from_midnight() >= start_of_day
    }
}

#[test]
fn it_is_too_fast() {
    for speed in vec![120.0, 150.0, 999999.999] {
        assert_eq!(true, is_too_fast(speed))
    }
}

#[test]
fn it_is_not_too_fast() {
    for speed in vec![0.1, 20.0, 50.3, 99.999] {
        assert_eq!(false, is_too_fast(speed))
    }
}

fn is_too_fast(speed: f64) -> bool {
    return speed > MAX_SPEED;
}

#[derive(Clone)]
struct Ride {
    id: u32,
    positions: Vec<Position>,
}

#[test]
fn test_calculate_all_fares() {
    let rides = vec![
        Ride {
            id: 1,
            positions: vec![],
        },
        Ride {
            id: 2,
            positions: vec![
                Position {
                    datetime: Utc.ymd(2020, 10, 20).and_hms(3, 0, 0),
                    location: haversine::Location {
                        latitude: 38.9,
                        longitude: -77.0,
                    },
                },
                Position {
                    datetime: Utc.ymd(2020, 10, 20).and_hms(5, 0, 0),
                    location: haversine::Location {
                        latitude: 38.9,
                        longitude: -78.0,
                    }, // ± 87km from previous position
                },
                Position {
                    datetime: Utc.ymd(2020, 10, 20).and_hms(6, 0, 0),
                    location: haversine::Location {
                        latitude: 38.9,
                        longitude: -77.0,
                    }, // ± 87km from previous position
                },
            ],
        },
    ];

    let want = vec![
        Fare {
            id: 1,
            amount: Amount::from(MINIMUM_FARE),
        },
        Fare {
            id: 2,
            amount: Amount::from(226.29426737040808),
        },
    ];

    let got = calculate_all_fares(rides);
    assert_eq!(want[0], got[0]);
    assert_eq!(want[1], got[1]);
}

fn calculate_all_fares(rides: Vec<Ride>) -> Vec<Fare> {
    rides
        .into_par_iter()
        .map(|ride| Fare {
            id: ride.id,
            amount: Amount::from(ride.calculate_fare()),
        })
        .collect()
}

#[test]
fn ride_fare() {
    for (ride, want) in vec![
        (
            Ride {
                id: 1,
                positions: vec![],
            },
            MINIMUM_FARE,
        ),
        (
            Ride {
                id: 1,
                positions: vec![
                    Position {
                        datetime: Utc.ymd(2020, 10, 20).and_hms(3, 0, 0),
                        location: haversine::Location {
                            latitude: 38.9,
                            longitude: -77.0,
                        },
                    },
                    Position {
                        datetime: Utc.ymd(2020, 10, 20).and_hms(5, 0, 0),
                        location: haversine::Location {
                            latitude: 38.9,
                            longitude: -78.0,
                        }, // ± 87km from previous position
                    },
                    Position {
                        datetime: Utc.ymd(2020, 10, 20).and_hms(6, 0, 0),
                        location: haversine::Location {
                            latitude: 38.9,
                            longitude: -77.0,
                        }, // ± 87km from previous position
                    },
                ],
            },
            226.29426737040808,
        ),
    ] {
        assert_eq!(want, ride.calculate_fare())
    }
}

impl Ride {
    fn calculate_fare(&self) -> f64 {
        let segments: Vec<Segment> = get_good_segments(&self);

        let mut fare_amount: f64 = STANDARD_FLAG;
        for segment in segments {
            fare_amount += segment.get_fare()
        }
        if fare_amount < MINIMUM_FARE {
            fare_amount = MINIMUM_FARE;
        }

        fare_amount
    }
}
#[test]
fn it_keeps_good_segments() {
    let ride = Ride {
        id: 1,
        positions: vec![
            Position {
                datetime: Utc.ymd(2020, 10, 20).and_hms(0, 0, 0),
                location: haversine::Location {
                    latitude: 38.898556,
                    longitude: -77.037852,
                },
            },
            Position {
                datetime: Utc.ymd(2020, 10, 20).and_hms(0, 1, 0),
                location: haversine::Location {
                    latitude: 38.897147,
                    longitude: -77.043934,
                }, // ± 0.55km from previous position, ± 33 km/h
            },
            Position {
                datetime: Utc.ymd(2020, 10, 20).and_hms(0, 2, 0),
                location: haversine::Location {
                    latitude: 38.898556,
                    longitude: -77.037852,
                }, // ± 0.55km from previous position, ± 33 km/h
            },
        ],
    };

    let segments = get_good_segments(&ride);
    assert_eq!(2, segments.len(),);
}

#[cfg(test)]
mod good_segment_tests {
    use super::*;

    #[test]
    fn it_ditches_bad_segments() {
        let ride = Ride {
            id: 1,
            positions: vec![
                Position {
                    datetime: Utc.ymd(2020, 10, 20).and_hms(0, 0, 0),
                    location: haversine::Location {
                        latitude: 38.898556,
                        longitude: -77.037852,
                    },
                },
                Position {
                    datetime: Utc.ymd(2020, 10, 20).and_hms(0, 1, 0),
                    location: haversine::Location {
                        latitude: 39.897147,
                        longitude: -77.043934,
                    }, // ± 111km from previous position, ± 6672 km/h
                },
                Position {
                    datetime: Utc.ymd(2020, 10, 20).and_hms(0, 2, 0),
                    location: haversine::Location {
                        latitude: 40.898556,
                        longitude: -77.037852,
                    },
                },
            ],
        };

        let segments = get_good_segments(&ride);
        assert_eq!(0, segments.len(),);
    }

    #[test]
    fn it_selects_correct_segments() {
        let ride = Ride {
            id: 1,
            positions: vec![
                Position {
                    datetime: Utc.ymd(2020, 10, 20).and_hms(0, 0, 0),
                    location: haversine::Location {
                        latitude: 38.898556,
                        longitude: -77.037852,
                    },
                },
                Position {
                    datetime: Utc.ymd(2020, 10, 20).and_hms(0, 0, 30),
                    location: haversine::Location {
                        latitude: 39.897147,
                        longitude: -77.043934,
                    }, // ± 111km from previous position, ± 6672 km/h
                },
                Position {
                    datetime: Utc.ymd(2020, 10, 20).and_hms(0, 1, 0),
                    location: haversine::Location {
                        latitude: 38.897147,
                        longitude: -77.043934,
                    }, // ± 0.55km from position 1, ± 33 km/h
                },
            ],
        };

        let segments = get_good_segments(&ride);
        assert_eq!(1, segments.len(),);
    }
}

fn get_good_segments(ride: &Ride) -> Vec<Segment> {
    let mut previous_position: Option<Position> = None;

    ride.positions
        .clone()
        .into_iter()
        .filter_map(|current_pos| {
            let prev_pos: Position = match previous_position.clone() {
                Some(prev_pos) => prev_pos,
                None => {
                    previous_position = Some(current_pos);
                    return None;
                }
            };

            let segment = Segment {
                start: prev_pos.datetime,
                end: current_pos.datetime,
                distance_km: haversine::distance_km(
                    prev_pos.location.clone(),
                    current_pos.location.clone(),
                ),
            };

            if is_too_fast(segment.speed()) {
                return None;
            }

            previous_position = Some(current_pos);
            Some(segment)
        })
        .collect()
}

#[derive(Debug)]
pub enum ReadError {
    MissingValueError { field: String },
    CSVError(csv::Error),
}

impl From<csv::Error> for ReadError {
    fn from(error: csv::Error) -> Self {
        ReadError::CSVError(error)
    }
}

type Record = (Option<u32>, Option<f64>, Option<f64>, Option<i64>);

type ParsedRecord = (Option<u32>, DateTime<chrono::Utc>, haversine::Location);

fn parse_record(record: Record) -> Result<ParsedRecord, ReadError> {
    let (id, lat, lon, datetime) = record;

    let datetime: DateTime<Utc> = match datetime {
        Some(ts) => Utc.timestamp(ts, 0),
        None => {
            return Err(ReadError::MissingValueError {
                field: "datetime".to_string(),
            })
        }
    };
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

    return Ok((id, datetime, loc));
}

fn read_csv(input: impl io::Read) -> Result<Vec<Ride>, ReadError> {
    let buffered = BufReader::new(input);

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(buffered);
    let mut rides: Vec<Ride> = vec![];
    let mut current_ride_id: Option<u32> = None;
    let mut positions: Vec<Position> = vec![];

    for record in reader.deserialize() {
        let record: Record = record?;
        let (id, datetime, location) = parse_record(record)?;

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

        positions.push(Position {
            datetime: datetime,
            location: location,
        });
        current_ride_id = Some(valid_id);
    }

    rides.push(Ride {
        id: current_ride_id.unwrap(),
        positions: positions,
    });

    Ok(rides)
}

#[derive(Serialize, Debug)]
struct Fare {
    id: u32,
    amount: Amount,
}

impl PartialEq for Fare {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.amount == other.amount
    }
}

// Amount get rounded to 2 decimal places when serialized
#[derive(Debug)]
struct Amount(f64);
impl PartialEq for Amount {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl std::convert::From<f64> for Amount {
    fn from(f: f64) -> Self {
        Self(f)
    }
}
impl Serialize for Amount {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{:.2}", self.0))
    }
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
