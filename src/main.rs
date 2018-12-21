mod coordinates;
mod nodes;
mod edges;
mod graph;

use crate::coordinates::*;
use crate::nodes::*;
use crate::edges::*;

fn main() {
    let start = Coordinate2Dint::new(1, 1);
    let finish = Coordinate2Dint::new(2, 2);
    let dist = Coordinate2DintCalculator::distance(&start, &finish);
    println!("{}", dist);

    let start_node = Node2dInt::new(start);
    let finish_node = Node2dInt::new(finish);
    let edge = Edge2dInt::new(&start_node, &finish_node);
    println!("{}", edge.weight);
}
