// Point on the surface of the earth in rad
pub struct PointS2 {
    pub theta: f64,     // latitude
    pub phi: f64,       // longitude
}


pub struct PointR3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

pub fn abs(p: PointR3)-> f64 {
    return (p.x.powf(2.0)+p.y.powf(2.0)+p.z.powf(2.0)).sqrt();
}

// Express point on the unit sphere in cartesian coordinates
pub fn s2_to_r3(p: PointS2) -> PointR3 {
    let x: f64 = p.theta.sin() * p.phi.cos();
    let y: f64 = p.theta.sin() * p.phi.sin();
    let z: f64 = p.theta.cos();

    return PointR3 { x, y, z }
}