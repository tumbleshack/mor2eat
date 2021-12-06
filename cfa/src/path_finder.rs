use crossbeam_utils::thread;
use std::error::Error;
use std::sync::atomic::{AtomicUsize, Ordering};
use super::graph_builder;
use super::utils;
use super::pathfinding::directed::yen;

pub fn input_unnumbered_edges_from(path: String) -> Result<graph_builder::Edges, Box<dyn Error>> {
    println!("Reading valid connection lists to memory...");
    utils::input_from(&path[..])
}

pub fn yen_wrapper(source: &String, dest: &String, edges: &graph_builder::Edges, mut paths: Paths) -> Paths{
    if !source.eq(dest) {
        let yen_output = yen::yen(
            source,
            |c| edge_list(c, &edges),
            |c| (*c).eq(dest),
            9600
        );

        println!("Yen output for source {} and dest {}: {:?}", source, dest, yen_output.last());
        
        let take = 2;
        let actual_take: usize;
        if yen_output.len() >= take {
            actual_take = take;
        } else {
            actual_take = yen_output.len();
        }

        if paths.len() > 0 {
            let yen_rev = yen_output.iter().rev();
            for path in yen_rev.take(actual_take) {
                paths.push(path.clone());
            }
        }
    }

    paths
}

pub fn yen_for_each_dest(source: &String, edges: &graph_builder::Edges) -> Paths {
    let mut paths: Paths = Paths::new();
    for dest in edges.keys().clone() {
        paths = yen_wrapper(source, &dest, edges, paths);
    }
    paths
}

fn process_next_pair(
    atomic_idx: &AtomicUsize, 
    max_idx: usize, 
    to_process: &Vec<Endpoint>, 
    edges: &graph_builder::Edges, 
    mut paths: Paths) -> Paths 
{
    let mut idx: usize;
    while (idx = atomic_idx.fetch_add(1, Ordering::SeqCst)) == () && idx <= max_idx {
        let endpoint = to_process.get(idx).unwrap();
        paths = yen_wrapper(&endpoint.source, &endpoint.dest, edges, paths);
    }
    paths
} 

pub fn run_yen(edges: &graph_builder::Edges) -> Paths {

    let num_threads = 8;

    let path_chunks: Result<Vec<Paths>, _> = thread::scope(|s| {
        let paths_to_process = std::sync::Arc::new(source_dest_pairs(edges));
        let max_idx = paths_to_process.len() - 1;
        static CURR_IDX: AtomicUsize = AtomicUsize::new(0);

        let threads: Vec<_> = (0..num_threads).map(|_| {
            let my_paths = paths_to_process.clone();
            s.spawn(move |_| {
                let paths = Paths::new();
                process_next_pair(&CURR_IDX, max_idx, &my_paths, edges, paths)
            })
        }).collect();
        threads.into_iter().map(|t| {t.join()}).collect()
    }).unwrap();

    let path_chunks: Vec<Paths> = path_chunks.unwrap();

    let paths: Paths = path_chunks.into_iter().fold(Paths::new(), merge_paths);

    paths
}

fn merge_paths(mut a: Paths, b: Paths) -> Paths {
    for each in b {
        a.push(each);
    }
    a
}

fn source_dest_pairs(edges: &graph_builder::Edges) -> Vec<Endpoint> {
    let mut endpoints: Vec<Endpoint> = Vec::new();
    for source in edges.keys().clone() {
        for dest in edges.keys().clone() {
            if !source.eq(dest) {
                endpoints.push(Endpoint { 
                    source: source.clone(),
                    dest: dest.clone()
                })
            }
        }
    }
    endpoints
}

fn edge_list(node: &String, edges: &graph_builder::Edges) -> Vec<(String, i64)> {
    let mut edge_touples: Vec<(String, i64)> = Vec::new();
    match edges.get(node) {
        Some(unnumbered_edge_list) => {
            for edge in unnumbered_edge_list {
                edge_touples.push((edge.node.clone(), edge.distance.clone()));
            }
        },
        None => {
            println!("Error! Edges for node {} not found in map", node);
        }
    }
    edge_touples
}

pub type Paths = Vec<Path>;

pub type Path = (Vec<String>, i64);

#[derive(Debug, Clone)]
pub struct Endpoint {
    source: String,
    dest: String,
}