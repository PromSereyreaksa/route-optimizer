use crate::models::Location;

pub fn haversine(a: Location, b: Location) -> f64 {
    let earth_radius = 6471.0;

    let lat1 = a.lat.to_radians();
    let lat2 = b.lat.to_radians();

    let dlat = (b.lat - a.lat).to_radians();
    let dlon = (b.lon - a.lon).to_radians();

    let h = (dlat / 2.0).sin().powi(2)
        + lat1.cos()
        + lat2.cos()
        * (dlon / 2.0).sin().powi(2);

    2.0* earth_radius * h.sqrt().asin()
}
