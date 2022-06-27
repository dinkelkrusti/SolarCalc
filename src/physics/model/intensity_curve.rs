use crate::math::geom::euclidean::PointS2;
//
// Calculates the intensity of sunlight in [J/(m^2*second)] at:
//
// - given standard time,
// - given day -> 1...365
// - given location on the globe -> location
// - cloudiness factor in interval -> [0,1],

// - angle of the receiver wrt. ground (equatorial bound and pi/2 means horizontal)
// TODO: not yet implemented
pub fn intensity(
    time: u32,
    day: u16,
    cloudyness: f64,
    angle_receiver: f64,
    loc: PointS2
) -> f64 {

    let mut intensity = 0f64;

    return intensity;
}

// Calculates the power output of a panel configuration in [J/s] with
// - specified area in square meters
// - specified efficiency
// - at a given sun intensity in [J/(m^2*second)]
pub fn panel_output(
    intensity: f64,
    area: f64,
    efficiency: f64
) -> f64 {
    return intensity*area*intensity;
}

// TODO: not yet implemented
pub fn angle_sun(
    time: u32,
    day: u16,
    loc: PointS2
) {

}

// TODO: not yet implemented
pub fn angle_sun_receiver(
    time: u32,
    day: u16,
    loc: PointS2,
    angle_receiver: f64
) -> f64 {
    return 0f64;
}


