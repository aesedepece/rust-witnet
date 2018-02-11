//Rust-Witnet is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
//Rust-Witnet is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
//You should have received a copy of the GNU General Public License
// along with Rust-Witnet. If not, see <http://www.gnu.org/licenses/>.
//
//This file is based on pool/src/graph.rs from
// <https://github.com/mimblewimble/grin>,
// originally developed by The Grin Developers and distributed under the
// Apache License, Version 2.0. You may obtain a copy of the License at
// <http://www.apache.org/licenses/LICENSE-2.0>.

//! Base types for the transaction pool's Directed Acyclic Graphs

use std::collections::HashMap;
use time;

use core::core::{Hash, OutputIdentifier};

/// An entry in the transaction pool.
/// These are the vertices of both of the graph structures
#[derive(Debug, PartialEq, Clone)]
pub struct PoolEntry {
    // Core data
    /// Unique identifier of this pool entry and the corresponding transaction
    pub transaction_hash: Hash,

    // Metadata
    /// Size estimate
    pub size_estimate: u64,
    /// Receive timestamp
    pub receive_ts: time::Tm,
}

/// An edge connecting graph vertices.
/// For various use cases, one of either the source or destination may be
/// unpopulated.
pub struct Edge {
    // Source and Destination are the vertex id's, the transaction (kernel)
    // hash.
    source: Option<Hash>,
    destination: Option<Hash>,

    // Output is the output hash which this input/output pairing corresponds
    // to.
    output: OutputIdentifier,
}

/// The generic graph container. Both graphs, the pool and orphans, embed this
/// structure and add additional capability on top of it.
pub struct DirectedGraph {
    edges: HashMap<OutputIdentifier, Edge>,
    vertices: Vec<PoolEntry>,
    roots: Vec<PoolEntry>,
}
