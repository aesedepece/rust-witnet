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
//This file is based on p2p/src/handshake.rs from
// <https://github.com/mimblewimble/grin>,
// originally developed by The Grin Developers and distributed under the
// Apache License, Version 2.0. You may obtain a copy of the License at
// <http://www.apache.org/licenses/LICENSE-2.0>.

use std::collections::VecDeque;
use std::sync::{Arc, RwLock};

use core::core::hash::Hash;
use types::*;

/// Handles the handshake negotiation when two peers connect and decides on
/// protocol.
pub struct Handshake {
    /// Ring buffer of nonces sent to detect self connections without requiring
    /// a node id.
    nonces: Arc<RwLock<VecDeque<u64>>>,
    /// The genesis block header of the chain seen by this node.
    /// We only want to connect to other nodes seeing the same chain (forks are ok).
    genesis: Hash,
    config: P2PConfig,
}