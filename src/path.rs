use crate::nodes::*;
use crate::edges::*;

pub struct Path<'a, N, E> where N : BaseNode, E : Edge<'a, N> {
    pub edges : Vec<&'a E>,
    pub nodes : Vec<&'a N>,
    pub sum_weigth : f32,
}