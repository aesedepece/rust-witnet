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
//This file is based on pool/src/types.rs from
// <https://github.com/mimblewimble/grin>,
// originally developed by The Grin Developers and distributed under the
// Apache License, Version 2.0. You may obtain a copy of the License at
// <http://www.apache.org/licenses/LICENSE-2.0>.

//! The primary module containing the implementations of the transaction pool
//! and its top-level members.

use std::collections::HashMap;

pub use graph;
use core::consensus;
use core::core::{OutputIdentifier, Transaction};

/// Tranasction pool configuration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PoolConfig {
    /// Base fee for a transaction to be accepted by the pool. The transaction
    /// weight is computed from its number of inputs and outputs and multipled
    /// by the base fee to compare to the actual transaction fee.
    #[serde = "default_accept_fee_base"]
    pub accept_fee_base: u64,

    /// Maximum capacity of the pool in number of transactions
    #[serde = "default_max_pool_size"]
    pub max_pool_size: usize,
}

impl Default for PoolConfig {
    fn default() -> PoolConfig {
        PoolConfig {
            accept_fee_base: default_accept_fee_base(),
            max_pool_size: default_max_pool_size(),
        }
    }
}

fn default_accept_fee_base() -> u64 {
    consensus::MILLI_WIT
}
fn default_max_pool_size() -> usize {
    50_000
}

/// Bridge between the transaction pool and the rest of the system. Handles
/// downstream processing of valid transactions by the rest of the system, most
/// importantly the broadcasting of transactions to our peers.
pub trait PoolAdapter: Send + Sync {
    /// The transaction pool has accepted this transactions as valid and added
    /// it to its internal cache.
    fn tx_accepted(&self, tx: &Transaction);
}

/// Pool contains the elements of the graph that are connected, in full, to
/// the blockchain.
/// Reservations of outputs by orphan transactions (not fully connected) are
/// not respected.
/// Spending references (input -> output) exist in two structures: internal
/// graph references are contained in the pool edge sets, while references
/// sourced from the blockchain's UTXO set are contained in the
/// blockchain_connections set.
/// Spent by references (output-> input) exist in two structures: pool-pool
/// connections are in the pool edge set, while unspent (dangling) references
/// exist in the available_outputs set.
pub struct Pool {
    graph: graph::DirectedGraph,

    // available_outputs are unspent outputs of the current pool set,
    // maintained as edges with empty destinations, keyed by the
    // output's hash.
    available_outputs: HashMap<OutputIdentifier, graph::Edge>,

    // Consumed blockchain utxo's are kept in a separate map.
    consumed_blockchain_outputs: HashMap<OutputIdentifier, graph::Edge>,
}

/// Orphans contains the elements of the transaction graph that have not been
/// connected in full to the blockchain.
pub struct Orphans {
    graph: graph::DirectedGraph,

    // available_outputs are unspent outputs of the current orphan set,
    // maintained as edges with empty destinations.
    available_outputs: HashMap<OutputIdentifier, graph::Edge>,

    // missing_outputs are spending references (inputs) with missing
    // corresponding outputs, maintained as edges with empty sources.
    missing_outputs: HashMap<OutputIdentifier, graph::Edge>,

    // pool_connections are bidirectional edges which connect to the pool
    // graph. They should map one-to-one to pool graph available_outputs.
    // pool_connections should not be viewed authoritatively, they are
    // merely informational until the transaction is officially connected to
    // the pool.
    pool_connections: HashMap<OutputIdentifier, graph::Edge>,
}
