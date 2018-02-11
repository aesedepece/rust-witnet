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
//This file is based on chain/src/chain.rs from
// <https://github.com/mimblewimble/grin>,
// originally developed by The Grin Developers and distributed under the
// Apache License, Version 2.0. You may obtain a copy of the License at
// <http://www.apache.org/licenses/LICENSE-2.0>.

//! Facade and handler for the rest of the blockchain implementation
//! and mostly the chain pipeline.
// TODO: we may not need orphan-related structures at all thanks to our DAG-like chain.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::time::Instant;

use core::core::{Block, BlockHeader, Hash};
use fast_merkle;
use types::*;

#[derive(Debug, Clone)]
struct Orphan {
    block: Block,
    opts: Options,
    added: Instant,
}

struct OrphanBlockPool {
    // blocks indexed by their hash
    orphans: RwLock<HashMap<Hash, Orphan>>,
    // additional index of previous -> hash
    // so we can efficiently identify a child block (ex-orphan) after processing a block
    prev_idx: RwLock<HashMap<Hash, Hash>>,
}

/// Facade to the blockchain block processing pipeline and storage. Provides
/// the current view of the UTXO set according to the chain state. Also
/// maintains locking for the pipeline to avoid conflicting processing.
pub struct Chain {
    store: Arc<ChainStore>,
    adapter: Arc<ChainAdapter>,

    head: Arc<Mutex<Tip>>,
    orphans: Arc<OrphanBlockPool>,
    merkle_tree: Arc<RwLock<fast_merkle::Trees>>,

    // POW verification function
    pow_verifier: fn(&BlockHeader, u32) -> bool,
}
