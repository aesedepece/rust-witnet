//This file is part of Rust-Witnet.
//
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
//This file is based on p2p/src/peers.rs from
// <https://github.com/mimblewimble/grin>,
// originally developed by The Grin Developers and distributed under the
// Apache License, Version 2.0. You may obtain a copy of the License at
// <http://www.apache.org/licenses/LICENSE-2.0>.

use rand::thread_rng;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};

use core::core::Transaction;
use peer::Peer;
use store::PeerStore;
use types::*;
use util::LOGGER;

#[derive(Clone)]
pub struct Peers {
    pub adapter: Arc<ChainAdapter>,
    store: Arc<PeerStore>,
    peers: Arc<RwLock<HashMap<SocketAddr, Arc<RwLock<Peer>>>>>,
    config: P2PConfig,
}

unsafe impl Send for Peers {}
unsafe impl Sync for Peers {}

impl Peers {
    /// Broadcasts the provided transaction to PEER_PREFERRED_COUNT of our peers.
	/// We may be connected to PEER_MAX_COUNT peers so we only
	/// want to broadcast to a random subset of peers.
	/// A peer implementation may drop the broadcast request
	/// if it knows the remote peer already has the transaction.
    pub fn broadcast_transaction(&self, tx: &Transaction) {
        let peers = self.connected_peers();
        for p in peers.iter().take(8) {
            let p = p.read().unwrap();
            if p.is_connected() {
                if let Err(e) = p.send_transaction(tx) {
                    debug!(LOGGER, "Error sending block to peer: {:?}", e);
                }
            }
        }
    }

    /// Get vec of peers we are currently connected to.
    pub fn connected_peers(&self) -> Vec<Arc<RwLock<Peer>>> {
        let mut res = self.peers
            .read()
            .unwrap()
            .values()
            .cloned()
            .collect::<Vec<_>>();
        thread_rng().shuffle(&mut res);
        res
    }
}