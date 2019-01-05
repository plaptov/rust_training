pub trait Coordinate {

}

pub trait CoordinateCalculator<T> where T : Coordinate {
    
    fn distance(start: &T, finish: &T) -> f32;
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct Coordinate2Dint {
    pub x: i32,
    pub y: i32,
}

impl Coordinate for Coordinate2Dint {
}

impl Coordinate2Dint {
    pub fn new(x: i32, y: i32) -> Coordinate2Dint {
        Coordinate2Dint {
            x,
            y,
        }
    }
}

pub struct Coordinate2DintCalculator {

}

impl CoordinateCalculator<Coordinate2Dint> for Coordinate2DintCalculator {
    fn distance(start: &Coordinate2Dint, finish: &Coordinate2Dint) -> f32 {
        let xdif = finish.x - start.x;
        let ydif = finish.y - start.y;
        let f : f32 = (xdif * xdif + ydif * ydif) as f32;
        f.sqrt()
    }
}
