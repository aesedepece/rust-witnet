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
//This file is based on core/src/core/transaction.rs from
// <https://github.com/mimblewimble/grin>,
// originally developed by The Grin Developers and distributed under the
// Apache License, Version 2.0. You may obtain a copy of the License at
// <http://www.apache.org/licenses/LICENSE-2.0>.

//! Transaction outputs structures.

bitflags! {
	/// Options for block validation
	#[derive(Serialize, Deserialize)]
	pub struct OutputFeatures: u8 {
		/// No flags
		const DEFAULT_OUTPUT = 0b00000000;
		/// Output is a coinbase output, must not be spent until maturity
		const COINBASE_OUTPUT = 0b00000001;
	}
}

/// Output for a transaction, defining the new ownership of coins that are being
/// transferred.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Output {
    /// Options for an output's structure or use
    pub features: OutputFeatures,
    // TODO script?
}

/// An output_identifier can be build from either an input _or_ and output and
/// contains everything we need to uniquely identify an output being spent.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct OutputIdentifier {
    /// Output features (coinbase vs. regular transaction output)
    /// We need to include this when hashing to ensure coinbase maturity can be enforced.
    pub features: OutputFeatures,
    // TODO reference to output?
}