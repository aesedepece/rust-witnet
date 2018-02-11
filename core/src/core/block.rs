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
//This file is based on core/src/core/block.rs from
// <https://github.com/mimblewimble/grin>,
// originally developed by The Grin Developers and distributed under the
// Apache License, Version 2.0. You may obtain a copy of the License at
// <http://www.apache.org/licenses/LICENSE-2.0>.

//! Block and blockheader structures.

use time;

use global;
use core::{Hash, ShortId, Transaction, ZERO_HASH};

/// Errors thrown by Block validation.
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    //TODO
    /// Any other error thrown by block validation code.
    Other(String),
}

/// Block header, fairly standard compared to other blockchains.
#[derive(Clone, Debug, PartialEq)]
pub struct BlockHeader {
    /// Version of the block
    pub version: u16,
    /// Height of this block since the genesis block (epoch 0)
    pub epoch: u64,
    /// Hash of the block previous to this in the chain.
    pub previous: Hash,
    /// Timestamp at which the block was built.
    pub timestamp: time::Tm,
    /// Merkle root of all the transactions
    pub merkle_root: Hash,
    ///// Proof of stake data.
    // TODO pub pos: Proof,
}

impl Default for BlockHeader {
    fn default() -> BlockHeader {
        BlockHeader {
            version: 1,
            epoch: 0,
            previous: ZERO_HASH,
            timestamp: time::at_utc(time::Timespec { sec: 0, nsec: 0 }),
            merkle_root: ZERO_HASH,
            // TODO pos: Proof::zero(),
        }
    }
}

/// Compact representation of a full block.
/// Each input or output is represented as a short_id.
/// A node is reasonably likely to have already seen all tx data (tx broadcast before block)
/// and can go request missing tx data from peers if necessary to hydrate a compact block
/// into a full block.
#[derive(Debug, Clone)]
pub struct CompactBlock {
    /// The header with metadata and commitments to the rest of the data
    pub header: BlockHeader,
    /// List of transaction short_ids
    pub transaction_ids: Vec<ShortId>,
}


/// A block as expressed in the Witnet protocol. The reward is
/// non-explicit, assumed to be deducible from block height (similar to
/// bitcoin's schedule).
#[derive(Debug, Clone)]
pub struct Block {
    /// The header with metadata and commitments to the rest of the data
    pub header: BlockHeader,
    /// List of transactions
    pub transactions: Vec<Transaction>
}