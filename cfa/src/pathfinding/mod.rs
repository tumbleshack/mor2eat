#![deny(missing_docs)]
//! This crate implements several pathfinding, flow, and graph algorithms.
//!
//! Several algorithms require that the numerical types used to describe
//! edges weights implement `Ord`. If you wish to use Rust builtin
//! floating-point types (such as `f32`) which implement `PartialOrd`
//! in this context, you can wrap them into compliant types using the
//! [ordered-float](https://crates.io/crates/ordered-float) crate.

pub use num_traits;

pub mod directed;
pub mod grid;
pub mod kuhn_munkres;
pub mod matrix;
pub mod undirected;
pub mod utils;

/// Export all public functions and structures for an easy access.
pub mod prelude {
    pub use crate::pathfinding::directed::astar::*;
    pub use crate::pathfinding::directed::bfs::*;
    pub use crate::pathfinding::directed::dfs::*;
    pub use crate::pathfinding::directed::dijkstra::*;
    pub use crate::pathfinding::directed::edmonds_karp::*;
    pub use crate::pathfinding::directed::fringe::*;
    pub use crate::pathfinding::directed::idastar::*;
    pub use crate::pathfinding::directed::iddfs::*;
    pub use crate::pathfinding::directed::strongly_connected_components::*;
    pub use crate::pathfinding::directed::topological_sort::*;
    pub use crate::pathfinding::directed::yen::*;
    pub use crate::pathfinding::grid::*;
    pub use crate::pathfinding::kuhn_munkres::*;
    pub use crate::pathfinding::matrix::*;
    pub use crate::pathfinding::undirected::connected_components::*;
    pub use crate::pathfinding::undirected::kruskal::*;
    pub use crate::pathfinding::utils::*;
}
