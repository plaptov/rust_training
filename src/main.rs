mod coordinates;
mod nodes;
mod edges;
mod graph;
mod path;
mod naive_path_finder;
mod astar_path_finder;

use crate::coordinates::*;
use crate::nodes::*;
use crate::edges::*;
use crate::graph::*;
use crate::naive_path_finder::NaivePathFinder;
use crate::astar_path_finder::AStarPathFinder;

fn main() {
    let start = Coordinate2Dint::new(1, 1);
    let finish = Coordinate2Dint::new(2, 2);
    let intermediate = Coordinate2Dint::new(1, 2);

    let start_node = Node2dInt::new(start);
    let finish_node = Node2dInt::new(finish);
    let int_node = Node2dInt::new(intermediate);
    let edge = Edge2dInt::new(&start_node, &int_node);
    let edge2 = Edge2dInt::new(&int_node, &finish_node);
    

    let nodes = vec!{ &start_node, &finish_node, &int_node };
    let edges = vec!{ &edge, &edge2 };
    let graph = GraphFactory::from_nodes_and_edges(&nodes, &edges);

    let path = NaivePathFinder::find_path(&graph, &start_node, &finish_node);
    println!("naive");
    for ed in path.edges {
        println!("{}", ed);
    }

    let path = AStarPathFinder::find_path(&graph, &start_node, &finish_node,
        &| start, finish | Coordinate2DintCalculator::distance(start.get_position(), finish.get_position()));
    println!("A*");
    for ed in path.edges {
        println!("{}", ed);
    }
}
