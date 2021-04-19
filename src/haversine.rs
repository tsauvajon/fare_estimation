#[derive(Clone, Debug)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
}

#[test]
fn distance_test() {
    assert_eq!(
        0.5491557912038084,
        distance(
            Location {
                latitude: 38.898556,
                longitude: -77.037852,
            },
            Location {
                latitude: 38.897147,
                longitude: -77.043934,
            },
        ),
    );

    assert_eq!(
        15328.17837221522,
        distance(
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
        distance(
            Location {
                latitude: 37.990832,
                longitude: 23.7032341
            },
            Location {
                latitude: -33.8883368,
                longitude: 151.1931148
            },
        ),
        distance(
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

pub fn distance(start: Location, end: Location) -> f64 {
    let r: f64 = 6371.0;

    let d_lat: f64 = (end.latitude - start.latitude).to_radians();
    let d_lon: f64 = (end.longitude - start.longitude).to_radians();
    let lat1: f64 = (start.latitude).to_radians();
    let lat2: f64 = (end.latitude).to_radians();

    let a: f64 = ((d_lat / 2.0).sin()) * ((d_lat / 2.0).sin())
        + ((d_lon / 2.0).sin()) * ((d_lon / 2.0).sin()) * (lat1.cos()) * (lat2.cos());
    let c: f64 = 2.0 * ((a.sqrt()).atan2((1.0 - a).sqrt()));

    r * c
}
