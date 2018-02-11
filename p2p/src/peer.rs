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
//This file is based on p2p/src/peer.rs from
// <https://github.com/mimblewimble/grin>,
// originally developed by The Grin Developers and distributed under the
// Apache License, Version 2.0. You may obtain a copy of the License at
// <http://www.apache.org/licenses/LICENSE-2.0>.

use std::sync::{Arc, RwLock};

use conn;
use core::core::{Hash, Transaction};
use msg;
use types::*;
use util::LOGGER;

const MAX_TRACK_SIZE: usize = 30;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Connected,
    Disconnected,
    Banned,
}

pub struct Peer {
    pub info: PeerInfo,
    state: Arc<RwLock<State>>,
    // set of all hashes known to this peer (so no need to send)
    tracking_adapter: TrackingAdapter,
    connection: Option<conn::Tracker>
}

impl Peer {
    fn check_connection(&self) -> bool {
        match self.connection.as_ref().unwrap().error_channel.try_recv() {
            Ok(Error::Serialization(e)) => {
                let mut state = self.state.write().unwrap();
                *state = State::Banned;
                info!(LOGGER, "Client {} corrupted, ban ({:?}).", self.info.addr, e);
                false
            }
            Ok(e) => {
                let mut state = self.state.write().unwrap();
                *state = State::Disconnected;
                debug!(LOGGER, "Client {} connection lost: {:?}", self.info.addr, e);
                false
            }
            Err(_) => true,
        }
    }

    /// Whether this peer is still connected.
    pub fn is_connected(&self) -> bool {
        if !self.check_connection() {
            return false
        }
        let state = self.state.read().unwrap();
        *state == State::Connected
    }

    /// Sends the provided transaction to the remote peer. The request may be
	/// dropped if the remote peer is known to already have the transaction.
    pub fn send_transaction(&self, tx: &Transaction) -> Result<(), Error> {
        if !self.tracking_adapter.has(tx.hash()) {
            debug!(LOGGER, "Send tx {} to {}", tx.hash(), self.info.addr);
            self.connection.as_ref().unwrap().send(tx, msg::Type::Transaction)
        } else {
            debug!(LOGGER, "Not sending tx {} to {} (already seen)", tx.hash(), self.info.addr);
            Ok(())
        }
    }
}

/// Adapter implementation that forwards everything to an underlying adapter
/// but keeps track of the block and transaction hashes that were received.
#[derive(Clone)]
struct TrackingAdapter {
    adapter: Arc<NetAdapter>,
    known: Arc<RwLock<Vec<Hash>>>,
}

impl TrackingAdapter {
    fn new(adapter: Arc<NetAdapter>) -> TrackingAdapter {
        TrackingAdapter {
            adapter: adapter,
            known: Arc::new(RwLock::new(vec![])),
        }
    }

    fn has(&self, hash: Hash) -> bool {
        let known = self.known.read().unwrap();
        // may become too slow, an ordered set (by timestamp for eviction) may
        // end up being a better choice
        known.contains(&hash)
    }

    fn push(&self, hash: Hash) {
        let mut known = self.known.write().unwrap();
        if known.len() > MAX_TRACK_SIZE {
            known.truncate(MAX_TRACK_SIZE);
        }
        known.insert(0, hash);
    }
}