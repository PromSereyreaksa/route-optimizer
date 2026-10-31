mod models;

use models::{Location, RouteProblem};

fn main () {
    let problem = RouteProblem {
        locations: vec![
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
        ],
    };

    print!("{:?}", problem.locations)
}
