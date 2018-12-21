use crate::coordinates::*;

pub trait BaseNode {
    type P : Coordinate;

    fn get_position(&self) -> &Self::P;
}

#[derive(Eq, PartialEq)]
pub struct Node<T> where T : Coordinate {
    pub position: T,
}

impl <T> BaseNode for Node<T> where T : Coordinate {
    type P = T;

    fn get_position(&self) -> &T { &self.position }
}

impl <T> Node<T> where T : Coordinate {      
    pub fn new(position: T) -> Node<T> {
        Node {
            position : position,
        }
    }
}

pub type Node2dInt = Node<Coordinate2Dint>;