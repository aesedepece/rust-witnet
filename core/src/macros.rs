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
//This file is based on core/src/macros.rs from
// <https://github.com/mimblewimble/grin>,
// originally developed by The Grin Developers and distributed under the
// Apache License, Version 2.0. You may obtain a copy of the License at
// <http://www.apache.org/licenses/LICENSE-2.0>.

//! Generic macros used here and there to simplify and make code more
//! readable.

/// Eliminate some of the boilerplate of serialization (package ser) by
/// passing directly pairs of writer function and data to write.
/// Example before:
///   try!(reader.write_u64(42));
///   try!(reader.write_u32(100));
/// Example after:
///   ser_multiwrite!(writer, [write_u64, 42], [write_u32, 100]);
#[macro_export]
macro_rules! ser_multiwrite {
  ($wrtr:ident, $([ $write_call:ident, $val:expr ]),* ) => {
    $( try!($wrtr.$write_call($val)) );*
  }
}