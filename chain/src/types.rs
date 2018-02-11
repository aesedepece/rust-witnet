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
//This file is based on chain/src/types.rs from
// <https://github.com/mimblewimble/grin>,
// originally developed by The Grin Developers and distributed under the
// Apache License, Version 2.0. You may obtain a copy of the License at
// <http://www.apache.org/licenses/LICENSE-2.0>.

//! Base types that the block chain pipeline requires.

use serde;
use serde_derive;

use core::core::{Block, BlockHeader, Hash, Output};
use witnet_store as store;

bitflags! {
    /// Options for block validation
	pub struct Options: u32 {
		/// No flags
		const NONE = 0b00000000;
		/// Runs without checking the Proof of Stake, mostly to make testing easier.
		const SKIP_POS = 0b00000001;
		/// Adds block while in syncing mode.
		const SYNC = 0b00000010;
		/// Block validation on a block we mined ourselves
		const MINE = 0b00000100;
	}
}

/// Errors thrown by chain validation.
#[derive(Debug)]
pub enum Error {
    // TODO Stablish all possible chain errors
    /// Any other error thrown by chain validation code.
    Other(String),
}

/// The tip of a fork. A handle to the fork ancestry from its leaf in the
/// blockchain tree. References the max height and the latest and previous
/// blocks for convenience.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tip {
    /// Height of the tip (max height of the fork)
    pub height: u64,
    /// Last block pushed to the fork
    pub last_block_h: Hash,
    /// Block previous to last
    pub prev_block_h: Hash,
}

// TODO: Adapt to DAG requirements
/// Trait the chain pipeline requires an implementor for in order to process
/// blocks.
pub trait ChainStore: Send + Sync {
    /// Get the tip that's also the head of the chain
    fn head(&self) -> Result<Tip, store::Error>;

    /// Block header for the chain head
    fn head_header(&self) -> Result<BlockHeader, store::Error>;

    /// Save the provided tip as the current head of our chain
    fn save_head(&self, t: &Tip) -> Result<(), store::Error>;

    /// Save the provided tip as the current head of the body chain, leaving the
    /// header chain alone.
    fn save_body_head(&self, t: &Tip) -> Result<(), store::Error>;

    /// Gets a block header by hash
    fn get_block(&self, h: &Hash) -> Result<Block, store::Error>;

    /// Check whether we have a block without reading it
    fn block_exists(&self, h: &Hash) -> Result<bool, store::Error>;

    /// Gets a block header by hash
    fn get_block_header(&self, h: &Hash) -> Result<BlockHeader, store::Error>;

    /// Save the provided block in store
    fn save_block(&self, b: &Block) -> Result<(), store::Error>;

    /// Save the provided block header in store
    fn save_block_header(&self, bh: &BlockHeader) -> Result<(), store::Error>;

    /// Get the tip of the header chain
    fn get_header_head(&self) -> Result<Tip, store::Error>;

    /// Save the provided tip as the current head of the block header chain
    fn save_header_head(&self, t: &Tip) -> Result<(), store::Error>;

    /// Get the tip of the current sync header chain
    fn get_sync_head(&self) -> Result<Tip, store::Error>;

    /// Save the provided tip as the current head of the sync header chain
    fn save_sync_head(&self, t: &Tip) -> Result<(), store::Error>;

    /// Reset header_head and sync_head to head of current body chain
    fn reset_head(&self) -> Result<(), store::Error>;

    /// Gets the block header at the provided height
    fn get_header_by_height(&self, height: u64) -> Result<BlockHeader, store::Error>;

    /// Save a header as associated with its height
    fn save_header_height(&self, header: &BlockHeader) -> Result<(), store::Error>;

    /// Delete the block header at the height
    fn delete_header_by_height(&self, height: u64) -> Result<(), store::Error>;

    /// Is the block header on the current chain?
    /// Use the header_by_height index to verify the block header is where we think it is.
    fn is_on_current_chain(&self, header: &BlockHeader) -> Result<(), store::Error>;

    /// Saves the position of an output, represented by its commitment, in the
    /// UTXO MMR. Used as an index for spending and pruning.
    fn save_output_pos(&self, output: &Output, pos: u64) -> Result<(), store::Error>;

    /// Gets the position of an output, represented by its commitment, in the
    /// UTXO MMR. Used as an index for spending and pruning.
    fn get_output_pos(&self, output: &Output) -> Result<u64, store::Error>;

    /// Saves the provided block header at the corresponding height. Also check
    /// the consistency of the height chain in store by assuring previous
    /// headers are also at their respective heights.
    fn setup_height(&self, bh: &BlockHeader, old_tip: &Tip) -> Result<(), store::Error>;
}

/// Bridge between the chain pipeline and the rest of the system. Handles
/// downstream processing of valid blocks by the rest of the system, most
/// importantly the broadcasting of blocks to our peers.
pub trait ChainAdapter {
    /// The blockchain pipeline has accepted this block as valid and added
    /// it to our chain.
    fn block_accepted(&self, b: &Block, opts: Options);
}