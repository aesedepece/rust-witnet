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
use std::net::{IpAddr, SocketAddr};
use std::sync::mpsc;

use core::core::{self, Hash, CompactBlock};
use core::ser;
use store;
use witnet_store;

#[derive(Debug)]
pub enum Error {
    Serialization(ser::Error),
    Connection(io::Error),
    /// Header type does not match the expected message type
    BadMessage,
    Banned,
    ConnectionClose,
    Timeout,
    Store(witnet_store::Error),
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

bitflags! {
  /// Options for what type of interaction a peer supports.
  #[derive(Serialize, Deserialize)]
  pub struct Capabilities: u32 {
	/// We don't know (yet) what the peer can do.
	const UNKNOWN = 0b00000000;
	/// Full archival node, has the whole history without any pruning.
	const FULL_HIST = 0b00000001;
	/// Can provide block headers and the UTXO set for some recent-enough
	/// height.
	const UTXO_HIST = 0b00000010;
	/// Can provide a list of healthy peers
	const PEER_LIST = 0b00000100;

	const FULL_NODE = Capabilities::FULL_HIST.bits | Capabilities::UTXO_HIST.bits | Capabilities::PEER_LIST.bits;
  }
}

/// General information about a connected peer that's useful to other modules.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PeerInfo {
    pub capabilities: Capabilities,
    pub user_agent: String,
    pub version: u32,
    pub addr: SocketAddr,
}

/// Bridge between the networking layer and the rest of the system. Handles the
/// forwarding or querying of blocks and transactions from the network among
/// other things.
pub trait ChainAdapter: Sync + Send {
    /// Current total height
    fn total_height(&self) -> u64;

    /// A valid transaction has been received from one of our peers
    fn transaction_received(&self, tx: core::Transaction);

    /// A block has been received from one of our peers. Returns true if the
    /// block could be handled properly and is not deemed defective by the
    /// chain. Returning false means the block will never be valid and
    /// may result in the peer being banned.
    fn block_received(&self, b: core::Block, addr: SocketAddr) -> bool;

    fn compact_block_received(&self, cb: core::CompactBlock, addr: SocketAddr) -> bool;

    fn header_received(&self, bh: core::BlockHeader, addr: SocketAddr) -> bool;

    /// A set of block header has been received, typically in response to a
    /// block
    /// header request.
    fn headers_received(&self, bh: Vec<core::BlockHeader>, addr: SocketAddr);

    /// Finds a list of block headers based on the provided locator. Tries to
    /// identify the common chain and gets the headers that follow it
    /// immediately.
    fn locate_headers(&self, locator: Vec<Hash>) -> Vec<core::BlockHeader>;

    /// Gets a full block by its hash.
    fn get_block(&self, h: Hash) -> Option<core::Block>;
}

/// Additional methods required by the protocol that don't need to be
/// externally implemented.
pub trait NetAdapter: ChainAdapter {
    /// Find good peers we know with the provided capability and return their
    /// addresses.
    fn find_peer_addrs(&self, capab: Capabilities) -> Vec<SocketAddr>;

    /// A list of peers has been received from one of our peers.
    fn peer_addrs_received(&self, Vec<SocketAddr>);
}