mod models;
mod distance;

use models::{Location, RouteProblem};
use distance::haversine;

fn main () {
    let locations = vec![
            Location {
                lat: 11.5564,
                lon: 104.9282
            },
            Location {
                lat: 11.5621,
                lon: 104.9175,
            },
            Location {
                lat: 11.5489,
                lon: 104.9214,
            },
        ];


    let distance = haversine(locations[0], locations[1]);

    print!("Distance {:.2} km", distance)
}
