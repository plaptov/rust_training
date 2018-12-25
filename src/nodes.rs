use std::fmt::*;
use crate::coordinates::*;

pub trait BaseNode {
    type P : Coordinate;

    fn get_position(&self) -> &Self::P;
}

#[derive(Eq, PartialEq, Hash)]
pub struct Node<T> where T : Coordinate {
    pub position: T,
}

impl <T> BaseNode for Node<T> where T : Coordinate {
    type P = T;

    fn get_position(&self) -> &T { &self.position }
}

impl <T> Node<T> where T : Coordinate {      

}

pub type Node2dInt = Node<Coordinate2Dint>;

impl Node2dInt {
    pub fn new(position: Coordinate2Dint) -> Node2dInt {
        Node2dInt {
            position : position
        }
    }
}

impl Display for Node2dInt {
    fn fmt(&self, f : &mut Formatter) -> std::fmt::Result {
        write!(f, "[{}, {}]", &self.position.x, &self.position.y)
    }
}
