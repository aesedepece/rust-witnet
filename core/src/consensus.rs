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
//This file is based on core/src/consensus.rs from
// <https://github.com/mimblewimble/grin>,
// originally developed by The Grin Developers and distributed under the
// Apache License, Version 2.0. You may obtain a copy of the License at
// <http://www.apache.org/licenses/LICENSE-2.0>.

//! All the rules required for a cryptocurrency to have reach consensus across
//! the whole network are complex and hard to completely isolate. Some can be
//! simple parameters (like block reward), others complex algorithms (like
//! Merkle sum trees or reorg rules). However, as long as they're simple
//! enough, consensus-relevant constants and short functions should be kept
//! here.

/// A wit is divisible to 10^9, following the SI prefixes
pub const WIT_BASE: u64 = 1_000_000_000;
/// Milliwit, a thousand of a wit
pub const MILLI_WIT: u64 = WIT_BASE / 1_000;
/// Microwit, a thousand of a milliwit
pub const MICRO_WIT: u64 = MILLI_WIT / 1_000;
/// Nanowit, smallest unit, takes a billion to make a wit
pub const NANO_WIT: u64 = 1;

/// The block subsidy amount, 50 wit per epoch
pub const REWARD: u64 = 50 * WIT_BASE;

/// Duration of each chain epoch, in seconds
pub const EPOCH_SEC: u64 = 90;

/// The maximum size we're willing to accept for any message. Enforced by the
/// peer-to-peer networking layer only for DoS protection.
pub const MAX_MSG_LEN: u64 = 20_000_000;

/// Total maximum block weight
pub const MAX_BLOCK_WEIGHT: usize = 80_000;

/// Consensus errors
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    /// Inputs/outputs/kernels must be sorted lexicographically.
    SortError,
}

/// Consensus rule that collections of items are sorted lexicographically.
pub trait VerifySortOrder<T> {
    /// Verify a collection of items is sorted as required.
    fn verify_sort_order(&self) -> Result<(), Error>;
}