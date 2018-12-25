use crate::nodes::*;
use crate::edges::*;
use crate::graph::*;
use crate::path::*;

pub struct NaivePathFinder {

}

impl NaivePathFinder {
    pub fn find_path<'a, N, E>(graph : &Graph<'a, N, E>, from : &'a N, to : &N) -> Path<'a, N, E> 
        where N : BaseNode + Eq + std::hash::Hash, E : Edge<'a, N>
    {
        let mut edges : Vec<&E> = vec!{};
        let mut nodes : Vec<&N> = vec!{};
        let mut weigth = 0.0;
        let mut cur_from : &'a N = from;

        loop {
            match graph.edges.get(cur_from) {
                None => break,
                Some(edges_list) => {
                    let edge = edges_list[0];
                    cur_from = edge.get_finish();
                    edges.push(edge);
                    nodes.push(cur_from);
                    weigth += edge.get_weight();
                    if cur_from == to {
                        break;
                    }
                }
            }
        }

        Path {
            edges : edges,
            nodes : nodes,
            sum_weigth : weigth,
        }
    }
}