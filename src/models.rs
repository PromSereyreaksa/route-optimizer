#[derive(Debug, Clone, Copy)]
pub struct Location {
    pub lat: f64,
    pub lon: f64,
}

pub struct RouteProblem {
    pub locations: Vec<Location>,
}

#[derive(Debug)]
pub struct RouteSolution {
    pub order: Vec<usize>,
    pub total_distance: f64,
}
