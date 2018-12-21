use crate::coordinates::*;
use crate::nodes::*;

pub trait Edge<'a, T> where T : BaseNode {
    fn get_start(&self) -> &T;
    fn get_finish(&self) -> &T;
    fn get_weight(&self) -> f32;
}

pub struct BaseEdge<'a, T> where T : BaseNode {
    pub start: &'a T,
    pub finish: &'a T,
    pub weight: f32,
}

pub type Edge2dInt<'a> = BaseEdge<'a, Node2dInt>;

impl<'a> Edge<'a, Node2dInt> for Edge2dInt<'a> {
    fn get_start(&self) -> &Node2dInt {
        self.start
    }

    fn get_finish(&self) -> &Node2dInt {
        self.finish
    }

    fn get_weight(&self) -> f32 {
        self.weight
    }
}

impl<'a> Edge2dInt<'a> {
    pub fn new(start: &'a Node2dInt, finish: &'a Node2dInt) -> Edge2dInt<'a> {
        Edge2dInt {
            start: &start,
            finish: &finish,
            weight: Coordinate2DintCalculator::distance(&start.position, &finish.position),
        }
    }
}