use crate::math::geom::euclidean::{abs, s2_to_r3, PointR3, PointS2};

pub mod math;
mod optim;
mod ui;
mod plot;
mod physics;

fn main(){
    let p = PointS2{
        theta: 0.2,
        phi: 0.1
    };

    let p = s2_to_r3(p);
    println!("x = {}", p.x);
    println!("y = {}", p.y);
    println!("z = {}", p.z);
    println!("abs = {}", abs(p) );
}