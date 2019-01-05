use std::collections::VecDeque;
use std::collections::HashMap;
use std::marker::PhantomData;

use crate::nodes::*;
use crate::edges::*;
use crate::graph::*;
use crate::path::*;

pub struct AStarPathFinder {

}

impl AStarPathFinder {
    pub fn find_path<'a, N, E, F>(
        graph : &Graph<'a, N, E>, 
        from : &'a N, 
        to : &N, 
        heuristic_distance_calculator : &F,
        ) -> Path<'a, N, E> 
        where   N : BaseNode + Eq + std::hash::Hash, 
                E : Edge<'a, N>,
                F : Fn(&N, &N) -> f32,
    {
        //let mut used_nodes = HashMap::new();
        let mut min_priority = std::i64::MAX;

        let mut edges : Vec<&E> = vec!{};
        let nodes : Vec<&N> = vec!{};
        let mut sum_weigth = 0.0;

        let mut path_storage = PartialPathStorage::new();
        let mut is_first_loop = true;

        loop {
            let mut cur_from : &N = from;

            if !is_first_loop && path_storage.is_empty() {
                break;
            }

            let deq = path_storage.get_mut(&min_priority);
            let p = match deq {
                None => None,
                Some(deq) => {
                    let v = deq.pop_front();
                    if deq.is_empty() {
                        path_storage.remove(&min_priority);
                    };
                    v
                }
            };

            if !is_first_loop && p.is_none() {
                min_priority = std::i64::MAX;
                for k in path_storage.keys() {
                    if *k < min_priority {
                        min_priority = *k;
                    }
                }
                continue;
            }

            is_first_loop = false;

            let prev_path : Option<&PartialPath<'a, N, E>> =
                match p {
                    None => None,
                    Some(ref path) => {
                        match path.last_node() {
                            None => (),
                            Some(node) => {
                                if node == to {
                                    edges = path.edges.clone();
                                    // nodes = ?
                                    sum_weigth = path.weight;
                                    break;
                                }
                            }
                        }
                        cur_from = match path.last_node() {
                            None => cur_from,
                            Some(n) => n,
                        };
                        Some(path)
                    },
                };

            match graph.edges.get(cur_from) {
                None => break,
                Some(edges_list) => {
                    for ed in edges_list {
                        let edge : &'a E = ed;
                        let p = PartialPath::new(prev_path, edge, to, heuristic_distance_calculator);
                        if p.get_priority() < min_priority {
                            min_priority = p.get_priority();
                        }
                        add_path_to_storage(&mut path_storage, p);
                    }
                }
            }
        }

        Path {
            edges,
            nodes,
            sum_weigth,
        }
    }
}

struct PartialPath<'a, N, E>
    where N : BaseNode, E : Edge<'a, N>
{
    pub edges : Vec<&'a E>,
    pub weight : f32,
    pub distance : f32,
    phantom : PhantomData<N>,
}

type PartialPathStorage<'a, N, E> = HashMap<i64, VecDeque<PartialPath<'a, N, E>>>;

impl <'a, N, E> PartialPath<'a, N, E>
    where N : BaseNode, E : Edge<'a, N>
{
    pub fn new<F>(prev_path : Option<&PartialPath<'a, N, E>>, new_edge : &'a E, to : &N,
        heuristic_distance_calculator : F) 
        -> PartialPath<'a, N, E>
        where F : Fn(&N, &N) -> f32,
    {
        let edges = match prev_path {
            None => vec!{new_edge},
            Some(path) => {
                let mut new_edges = path.edges.clone();
                new_edges.push(new_edge);
                new_edges
            }
        };

        let weight = match prev_path {
            None => new_edge.get_weight(),
            Some(path) => path.weight + new_edge.get_weight()
        };

        PartialPath {
            edges,
            weight,
            distance : heuristic_distance_calculator(new_edge.get_finish(), to),
            phantom : PhantomData
        }
    }

    pub fn get_priority(&self) -> i64 {
        ((self.weight + self.distance) * 1_000_000.0) as i64
    }

    pub fn last_node(&self) -> Option<&N> {
        match &self.edges.last() {
            None => None,
            Some(edge) => Some(edge.get_finish()),
        }
    }
}

fn add_path_to_storage<'a, N, E>(storage : &mut PartialPathStorage<'a, N, E>, path : PartialPath<'a, N, E>) 
    where N : BaseNode, E : Edge<'a, N>
{
    storage
        .entry(path.get_priority())
        .or_insert_with(VecDeque::new)
        .push_back(path);
}