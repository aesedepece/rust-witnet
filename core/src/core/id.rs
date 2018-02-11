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
//This file is based on core/src/core/id.rs from
// <https://github.com/mimblewimble/grin>,
// originally developed by The Grin Developers and distributed under the
// Apache License, Version 2.0. You may obtain a copy of the License at
// <http://www.apache.org/licenses/LICENSE-2.0>.

//! short ids for compact blocks

use std::cmp::min;

use ser;

use util;

/// The size of a short id used to identify inputs and outputs (6 bytes)
pub const SHORT_ID_SIZE: usize = 6;

/// Short id for identifying inputs and outputs
#[derive(PartialEq, Clone, PartialOrd, Ord, Eq, Serialize, Deserialize)]
pub struct ShortId([u8; 6]);

impl ::std::fmt::Debug for ShortId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        try!(write!(f, "{}(", stringify!(ShortId)));
        try!(write!(f, "{}", self.to_hex()));
        write!(f, ")")
    }
}

impl ShortId {
    /// Build a new short_id from a byte slice
    pub fn from_bytes(bytes: &[u8]) -> ShortId {
        let mut hash = [0; SHORT_ID_SIZE];
        for i in 0..min(SHORT_ID_SIZE, bytes.len()) {
            hash[i] = bytes[i];
        }
        ShortId(hash)
    }

    /// Hex string representation of a short_id
    pub fn to_hex(&self) -> String {
        util::to_hex(self.0.to_vec())
    }

    /// Reconstructs a switch commit hash from a hex string.
    pub fn from_hex(hex: &str) -> Result<ShortId, ser::Error> {
        let bytes = util::from_hex(hex.to_string())
            .map_err(|_| ser::Error::HexError(format!("short_id from_hex error")))?;
        Ok(ShortId::from_bytes(&bytes))
    }

    /// The zero short_id, convenient for generating a short_id for testing.
    pub fn zero() -> ShortId {
        ShortId::from_bytes(&[0])
    }
}