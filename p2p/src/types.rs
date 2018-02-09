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
//This file is based on p2p/src/types.rs from
// <https://github.com/mimblewimble/grin>,
// originally developed by The Grin Developers and distributed under the
// Apache License, Version 2.0. You may obtain a copy of the License at
// <http://www.apache.org/licenses/LICENSE-2.0>.

use std::io;
use std::net::IpAddr;

use core::core::hash::Hash;
use core::ser;
use store;

#[derive(Debug)]
pub enum Error {
    Serialization(ser::Error),
    Connection(io::Error),
    /// Header type does not match the expected message type
    BadMessage,
    Banned,
    ConnectionClose,
    Timeout,
    Store(store::Error),
    PeerWithSelf,
    ProtocolMismatch {
        us: u32,
        peer: u32,
    },
    GenesisMismatch {
        us: Hash,
        peer: Hash,
    },
}

/// Type of seeding the server will use to find other peers on the network.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Seeding {
    /// No seeding, mostly for tests that programmatically connect
    None,
    /// A list of seed addresses provided to the server
    List,
    /// Automatically download a text file with a list of server addresses
    WebStatic,
    /// Mostly for tests, where connections are initiated programmatically
    Programmatic,
}

impl Default for Seeding {
    fn default() -> Seeding {
        Seeding::None
    }
}


/// Configuration for the peer-to-peer server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct P2PConfig {
    /// P2P server listening address.
    pub host: IpAddr,
    /// P2P server listening port.
    pub port: u16,
    /// Which method will be used to get the list of seed nodes for initial bootstrap.
    #[serde(default)]
    pub seeding_type: Seeding,
    /// Peers to connect on start.
    pub seeding_peers: Option<Vec<String>>,
    /// Peers whitelist.
    pub peers_allow: Option<Vec<String>>,
    /// Peers blacklist.
    pub peers_deny: Option<Vec<String>>,
}

/// Default address for peer-to-peer connections.
impl Default for P2PConfig {
    fn default() -> P2PConfig {
        let ipaddr = "0.0.0.0".parse().unwrap();
        P2PConfig {
            host: ipaddr,
            port: 11337,
            seeding_type: Seeding::default(),
            seeding_peers: None,
            peers_allow: None,
            peers_deny: None,
        }
    }
}