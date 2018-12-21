use std::collections::HashMap;
use std::hash::Hash;

use crate::nodes::*;
use crate::edges::*;

pub struct Graph<'a, N, E> where N : BaseNode, E : Edge<'a, N>
{
    pub edges: HashMap<&'a N, Vec<&'a E>>,
}

pub type Graph2dInt<'a> = Graph<'a, Node2dInt, Edge2dInt<'a>>;

pub struct GraphFactory {

}

impl GraphFactory {
    pub fn from_node_and_edges<'a, N, E>(nodes : &'a [N], edges : &'a [E]) -> Graph<'a, N, E>
        where N : BaseNode + Eq + PartialEq + Hash, E : Edge<'a, N>
    {
        let mut hashmap = HashMap::new();

        for n in nodes {
            hashmap.insert(n, edges.iter().filter(|x| x.get_start() == n).collect());
        };

        Graph {
            edges : hashmap,
        }
    }
}