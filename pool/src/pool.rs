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
//This file is based on pool/src/pool.rs from
// <https://github.com/mimblewimble/grin>,
// originally developed by The Grin Developers and distributed under the
// Apache License, Version 2.0. You may obtain a copy of the License at
// <http://www.apache.org/licenses/LICENSE-2.0>.

//! Top-level Pool type, methods, and tests.

use std::collections::HashMap;
use std::sync::Arc;

use core::core::{hash, transaction};
use types::*;

/// The pool itself.
/// The transactions HashMap holds ownership of all transactions in the pool,
/// keyed by their transaction hash.
pub struct TransactionPool<T> {
    config: PoolConfig,
    /// All transactions in the pool
    pub transactions: HashMap<hash::Hash, Box<transaction::Transaction>>,
    /// The pool itself
    pub pool: Pool,
    /// Orphans in the pool
    pub orphans: Orphans,

    // blockchain is a DummyChain, for now, which mimics what the future
    // chain will offer to the pool
    blockchain: Arc<T>,
    adapter: Arc<PoolAdapter>,
}