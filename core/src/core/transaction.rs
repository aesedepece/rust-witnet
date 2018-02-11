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

//! Transaction structures.

use core::input::Input;
use core::output::Output;
#[macro_use]
use macros;
use ser::{self, read_and_verify_sorted, Readable, Reader, Writeable, WriteableSorted, Writer};

/// Errors thrown by transaction validation.
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    //TODO
    /// Any other error thrown by TX validation code.
    Other(String),
}

/// A transaction
#[derive(Debug, Clone)]
pub struct Transaction {
    /// Set of inputs spent by the transaction.
    pub inputs: Vec<Input>,
    /// Set of outputs the transaction produces.
    pub outputs: Vec<Output>,
    /// Fee paid by the transaction.
    pub fee: u64,
    /// Transaction is not valid before this chain height.
    pub lock_height: u64,
}

/// Implementation of Writeable for a fully blinded transaction, defines how to
/// write the transaction as binary.
impl Writeable for Transaction {
    // TODO: adapt to what will be defined in of the first Witnet WIPs.
    fn write<W: Writer>(&self, writer: &mut W) -> Result<(), ser::Error> {
        ser_multiwrite!(
			writer,
			[write_u64, self.fee],
			[write_u64, self.lock_height]
		);
        ser_multiwrite!(
			writer,
			[write_u64, self.inputs.len() as u64],
			[write_u64, self.outputs.len() as u64]
		);

        // Consensus rule that everything is sorted in lexicographical order on the wire.
        let mut inputs = self.inputs.clone();
        let mut outputs = self.outputs.clone();

        inputs.write_sorted(writer)?;
        outputs.write_sorted(writer)?;

        Ok(())
    }
}
