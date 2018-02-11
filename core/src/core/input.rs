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

//! Transaction inputs structures.

// TODO Adapt to what will be defined in one of the first WIPs

use std::cmp::Ordering;

#[macro_use]
use core::output::{Output, OutputFeatures};
use core::hash::{Hash, ZERO_HASH};
#[macro_use]
use core::transaction;
use ser::{self, Readable, Reader, Writeable, Writer};

/// Don't seem to be able to define an Ord implementation for Hash due to
/// Ord being defined on all pointers, resorting to a macro instead
macro_rules! hashable_ord {
  ($hashable: ident) => {
    impl Ord for $hashable {
      fn cmp(&self, other: &$hashable) -> Ordering {
        self.hash().cmp(&other.hash())
      }
    }
    impl PartialOrd for $hashable {
      fn partial_cmp(&self, other: &$hashable) -> Option<Ordering> {
        Some(self.hash().cmp(&other.hash()))
      }
    }
    impl PartialEq for $hashable {
      fn eq(&self, other: &$hashable) -> bool {
        self.hash() == other.hash()
      }
    }
    impl Eq for $hashable {}
  }
}

/// A transaction input.
///
/// Primarily a reference to an output being spent by the transaction.
/// But also information required to verify coinbase maturity through
/// the lock_height hashed in the switch_commit_hash.
#[derive(Debug, Clone, Copy)]
pub struct Input{
    /// The output being spent.
    pub tx_out: Output,
    /// The hash of the block the output originated from.
    pub out_block: Option<Hash>,
}

hashable_ord!(Input);

/// Implementation of Writeable for a transaction Input, defines how to write
/// an Input as binary.
impl Writeable for Input {
    fn write<W: Writer>(&self, writer: &mut W) -> Result<(), ser::Error> {
        writer.write_u8(self.features.bits())?;
        writer.write_fixed_bytes(&self.commit)?;

        if self.features.contains(OutputFeatures::COINBASE_OUTPUT) {
            writer.write_fixed_bytes(&self.out_block.unwrap_or(ZERO_HASH))?;
        }

        Ok(())
    }
}

/// Implementation of Readable for a transaction Input, defines how to read
/// an Input from a binary stream.
impl Readable for Input {
    fn read(reader: &mut Reader) -> Result<Input, ser::Error> {
        let features = OutputFeatures::from_bits(reader.read_u8()?).ok_or(
            ser::Error::CorruptedData,
        )?;

        let tx_out = Output::read(reader)?;

        let out_block = if features.contains(OutputFeatures::COINBASE_OUTPUT) {
            Some(Hash::read(reader)?)
        } else {
            None
        };

        Ok(Input::new(
            tx_out,
            out_block,
        ))
    }
}

/// The input for a transaction, which spends a pre-existing unspent output.
/// The input commitment is a reproduction of the commitment of the output being spent.
/// Input must also provide the original output features and the hash of the block
/// the output originated from.
impl Input {
    /// Build a new input from the data required to identify and verify an output beng spent.
    pub fn new(
        tx_out: Output,
        out_block: Option<Hash>,
    ) -> Input {
        Input {
            tx_out,
            out_block,
        }
    }

    /// The identifier for the output being spent.
    pub fn tx_out(&self) -> Output {
        self.tx_out
    }
}
