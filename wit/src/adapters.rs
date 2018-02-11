//Rust-Witnet is free software: you can redistribute it and/or modify
//it under the terms of the GNU General Public License as published by
//the Free Software Foundation, either version 3 of the License, or
//(at your option) any later version.
//
//Rust-Witnet is distributed in the hope that it will be useful,
//but WITHOUT ANY WARRANTY; without even the implied warranty of
//MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//GNU General Public License for more details.
//
//You should have received a copy of the GNU General Public License
//along with Rust-Witnet. If not, see <http://www.gnu.org/licenses/>.
//
//This file is based on grin/src/adapters.rs from
// <https://github.com/mimblewimble/grin>,
// originally developed by The Grin Developers and distributed under the
// Apache License, Version 2.0. You may obtain a copy of the License at
// <http://www.apache.org/licenses/LICENSE-2.0>.

use std::sync::{Arc, RwLock};
use std::sync::atomic::AtomicBool;

use core::core::Transaction;
use chain;
use p2p;
use pool;
use util::OneTime;

/// Implementation of the NetAdapter for the blockchain. Gets notified when new
/// blocks, transactions and other objects are received and forwards to the chain
/// and pool implementations.
pub struct NetToChainAdapter {
    currently_syncing: Arc<AtomicBool>,
    chain: Arc<chain::Chain>,
    tx_pool: Arc<RwLock<pool::TransactionPool<PoolToChainAdapter>>>,
    peers: OneTime<p2p::Peers>,
}

/// Implements the view of the blockchain required by the TransactionPool to
/// operate. Mostly needed to break any direct lifecycle or implementation
/// dependency between the pool and the chain.
#[derive(Clone)]
pub struct PoolToChainAdapter {
    chain: OneTime<Arc<chain::Chain>>,
}

impl PoolToChainAdapter {
    /// Create a new pool adapter
    pub fn new() -> PoolToChainAdapter {
        PoolToChainAdapter {
            chain: OneTime::new(),
        }
    }

    pub fn set_chain(&self, chain_ref: Arc<chain::Chain>) {
        self.chain.init(chain_ref);
    }
}

/// Implementation of the ChainAdapter for the network. Gets notified when the
/// blockchain accepted a new block, asking the pool to update its state and
/// the network to broadcast the block
pub struct ChainToPoolAndNetAdapter {
    tx_pool: Arc<RwLock<pool::TransactionPool<PoolToChainAdapter>>>,
    peers: OneTime<p2p::Peers>,
}

/// Adapter between the transaction pool and the network, to relay
/// transactions that have been accepted.
pub struct PoolToNetAdapter {
    peers: OneTime<p2p::Peers>,
}


impl pool::PoolAdapter for PoolToNetAdapter {
    fn tx_accepted(&self, tx: &Transaction) {
        self.peers.borrow().broadcast_transaction(tx);
    }
}

impl PoolToNetAdapter {
    /// Create a new pool to net adapter
    pub fn new() -> PoolToNetAdapter {
        PoolToNetAdapter {
            peers: OneTime::new(),
        }
    }

    /// Setup the p2p server on the adapter
    pub fn init(&self, peers: p2p::Peers) {
        self.peers.init(peers);
    }
}