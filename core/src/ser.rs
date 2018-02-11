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
//This file is based on core/src/ser.rs from
// <https://github.com/mimblewimble/grin>,
// originally developed by The Grin Developers and distributed under the
// Apache License, Version 2.0. You may obtain a copy of the License at
// <http://www.apache.org/licenses/LICENSE-2.0>.

//! Serialization and deserialization layer specialized for binary encoding.
//! Ensures consistency and safety. Basically a minimal subset or
//! rustc_serialize customized for our need.
//!
//! To use it simply implement `Writeable` or `Readable` and then use the
//! `serialize` or `deserialize` functions on them as appropriate.

use std::fmt;
use std::io::{self, Read, Write};
use byteorder::{BigEndian, ByteOrder};

use consensus;
use core::hash::Hashed;
use keychain::{Identifier, IDENTIFIER_SIZE};

/// Possible errors deriving from serializing or deserializing.
#[derive(Debug)]
pub enum Error {
    /// Wraps an io error produced when reading or writing
    IOErr(io::Error),
    /// Expected a given value that wasn't found
    UnexpectedData {
        /// What we wanted
        expected: Vec<u8>,
        /// What we got
        received: Vec<u8>,
    },
    /// Data wasn't in a consumable format
    CorruptedData,
    /// When asked to read too much data
    TooLargeReadErr,
    /// Consensus rule failure
    ConsensusError(consensus::Error),
    /// Error from from_hex deserialization
    HexError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::IOErr(ref e) => write!(f, "{}", e),
            Error::UnexpectedData {
                expected: ref e,
                received: ref r,
            } => write!(f, "expected {:?}, got {:?}", e, r),
            Error::CorruptedData => f.write_str("corrupted data"),
            Error::TooLargeReadErr => f.write_str("too large read"),
            Error::ConsensusError(ref e) => write!(f, "consensus error {:?}", e),
            Error::HexError(ref e) => write!(f, "hex error {:?}", e),
        }
    }
}

/// Signal to a serializable object how much of its data should be serialized
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SerializationMode {
    /// Serialize everything sufficiently to fully reconstruct the object
    Full,
    /// Serialize the data that defines the object
    Hash,
    /// Serialize everything that a signer of the object should know
    SigHash,
}

/// Implementations defined how different numbers and binary structures are
/// written to an underlying stream or container (depending on implementation).
pub trait Writer {
    /// The mode this serializer is writing in
    fn serialization_mode(&self) -> SerializationMode;

    /// Writes a u8 as bytes
    fn write_u8(&mut self, n: u8) -> Result<(), Error> {
        self.write_fixed_bytes(&[n])
    }

    /// Writes a u16 as bytes
    fn write_u16(&mut self, n: u16) -> Result<(), Error> {
        let mut bytes = [0; 2];
        BigEndian::write_u16(&mut bytes, n);
        self.write_fixed_bytes(&bytes)
    }

    /// Writes a u32 as bytes
    fn write_u32(&mut self, n: u32) -> Result<(), Error> {
        let mut bytes = [0; 4];
        BigEndian::write_u32(&mut bytes, n);
        self.write_fixed_bytes(&bytes)
    }

    /// Writes a u64 as bytes
    fn write_u64(&mut self, n: u64) -> Result<(), Error> {
        let mut bytes = [0; 8];
        BigEndian::write_u64(&mut bytes, n);
        self.write_fixed_bytes(&bytes)
    }

    /// Writes a i64 as bytes
    fn write_i64(&mut self, n: i64) -> Result<(), Error> {
        let mut bytes = [0; 8];
        BigEndian::write_i64(&mut bytes, n);
        self.write_fixed_bytes(&bytes)
    }

    /// Writes a variable number of bytes. The length is encoded as a 64-bit
    /// prefix.
    fn write_bytes<T: AsFixedBytes>(&mut self, bytes: &T) -> Result<(), Error> {
        try!(self.write_u64(bytes.as_ref().len() as u64));
        self.write_fixed_bytes(bytes)
    }

    /// Writes a fixed number of bytes from something that can turn itself into
    /// a `&[u8]`. The reader is expected to know the actual length on read.
    fn write_fixed_bytes<T: AsFixedBytes>(&mut self, fixed: &T) -> Result<(), Error>;
}

/// Implementations defined how different numbers and binary structures are
/// read from an underlying stream or container (depending on implementation).
pub trait Reader {
    /// Read a u8 from the underlying Read
    fn read_u8(&mut self) -> Result<u8, Error>;
    /// Read a u16 from the underlying Read
    fn read_u16(&mut self) -> Result<u16, Error>;
    /// Read a u32 from the underlying Read
    fn read_u32(&mut self) -> Result<u32, Error>;
    /// Read a u64 from the underlying Read
    fn read_u64(&mut self) -> Result<u64, Error>;
    /// Read a i32 from the underlying Read
    fn read_i64(&mut self) -> Result<i64, Error>;
    /// first before the data bytes.
    fn read_vec(&mut self) -> Result<Vec<u8>, Error>;
    /// first before the data bytes limited to max bytes.
    fn read_limited_vec(&mut self, max: usize) -> Result<Vec<u8>, Error>;
    /// Read a fixed number of bytes from the underlying reader.
    fn read_fixed_bytes(&mut self, length: usize) -> Result<Vec<u8>, Error>;
    /// Consumes a byte from the reader, producing an error if it doesn't have
    /// the expected value
    fn expect_u8(&mut self, val: u8) -> Result<u8, Error>;
}

/// Trait that every type that can be serialized as binary must implement.
/// Writes directly to a Writer, a utility type thinly wrapping an
/// underlying Write implementation.
pub trait Writeable {
    /// Write the data held by this Writeable to the provided writer
    fn write<W: Writer>(&self, writer: &mut W) -> Result<(), Error>;
}

/// Trait to allow a collection of Writeables to be written in lexicographical sort order.
pub trait WriteableSorted {
    /// Write the data but sort it first.
    fn write_sorted<W: Writer>(&mut self, writer: &mut W) -> Result<(), Error>;
}

/// Reads a collection of serialized items into a Vec
/// and verifies they are lexicographically ordered.
///
/// A consensus rule requires everything is sorted lexicographically to avoid
/// leaking any information through specific ordering of items.
pub fn read_and_verify_sorted<T>(reader: &mut Reader, count: u64) -> Result<Vec<T>, Error>
    where
        T: Readable + Hashed + Writeable,
{
    let result: Vec<T> = try!((0..count).map(|_| T::read(reader)).collect());
    result.verify_sort_order()?;
    Ok(result)
}

/// Trait that every type that can be deserialized from binary must implement.
/// Reads directly to a Reader, a utility type thinly wrapping an
/// underlying Read implementation.
pub trait Readable
    where
        Self: Sized,
{
    /// Reads the data necessary to this Readable from the provided reader
    fn read(reader: &mut Reader) -> Result<Self, Error>;
}


/// Utility wrapper for an underlying byte Writer. Defines higher level methods
/// to write numbers, byte vectors, hashes, etc.
struct BinWriter<'a> {
    sink: &'a mut Write,
}

impl<'a> Writer for BinWriter<'a> {
    fn serialization_mode(&self) -> SerializationMode {
        SerializationMode::Full
    }

    fn write_fixed_bytes<T: AsFixedBytes>(&mut self, fixed: &T) -> Result<(), Error> {
        let bs = fixed.as_ref();
        try!(self.sink.write_all(bs));
        Ok(())
    }
}

macro_rules! impl_int {
    ($int: ty, $w_fn: ident, $r_fn: ident) => {
        impl Writeable for $int {
            fn write<W: Writer>(&self, writer: &mut W) -> Result<(), Error> {
                writer.$w_fn(*self)
            }
        }

        impl Readable for $int {
            fn read(reader: &mut Reader) -> Result<$int, Error> {
                reader.$r_fn()
            }
        }
    }
}

impl_int!(u8, write_u8, read_u8);
impl_int!(u16, write_u16, read_u16);
impl_int!(u32, write_u32, read_u32);
impl_int!(u64, write_u64, read_u64);
impl_int!(i64, write_i64, read_i64);

impl<T> Readable for Vec<T>
    where
        T: Readable,
{
    fn read(reader: &mut Reader) -> Result<Vec<T>, Error> {
        let mut buf = Vec::new();
        loop {
            let elem = T::read(reader);
            match elem {
                Ok(e) => buf.push(e),
                Err(Error::IOErr(ref ioerr)) if ioerr.kind() == io::ErrorKind::UnexpectedEof => {
                    break
                }
                Err(e) => return Err(e),
            }
        }
        Ok(buf)
    }
}

impl<T> Writeable for Vec<T>
    where
        T: Writeable,
{
    fn write<W: Writer>(&self, writer: &mut W) -> Result<(), Error> {
        for elmt in self {
            elmt.write(writer)?;
        }
        Ok(())
    }
}

impl<T> WriteableSorted for Vec<T>
    where
        T: Writeable + Ord,
{
    fn write_sorted<W: Writer>(&mut self, writer: &mut W) -> Result<(), Error> {
        self.sort();
        for elmt in self {
            elmt.write(writer)?;
        }
        Ok(())
    }
}

impl<'a, A: Writeable> Writeable for &'a A {
    fn write<W: Writer>(&self, writer: &mut W) -> Result<(), Error> {
        Writeable::write(*self, writer)
    }
}

impl<A: Writeable, B: Writeable> Writeable for (A, B) {
    fn write<W: Writer>(&self, writer: &mut W) -> Result<(), Error> {
        try!(Writeable::write(&self.0, writer));
        Writeable::write(&self.1, writer)
    }
}

impl<A: Readable, B: Readable> Readable for (A, B) {
    fn read(reader: &mut Reader) -> Result<(A, B), Error> {
        Ok((try!(Readable::read(reader)), try!(Readable::read(reader))))
    }
}

impl<A: Writeable, B: Writeable, C: Writeable> Writeable for (A, B, C) {
    fn write<W: Writer>(&self, writer: &mut W) -> Result<(), Error> {
        try!(Writeable::write(&self.0, writer));
        try!(Writeable::write(&self.1, writer));
        Writeable::write(&self.2, writer)
    }
}

impl<A: Writeable, B: Writeable, C: Writeable, D: Writeable> Writeable for (A, B, C, D) {
    fn write<W: Writer>(&self, writer: &mut W) -> Result<(), Error> {
        try!(Writeable::write(&self.0, writer));
        try!(Writeable::write(&self.1, writer));
        try!(Writeable::write(&self.2, writer));
        Writeable::write(&self.3, writer)
    }
}

impl<A: Readable, B: Readable, C: Readable> Readable for (A, B, C) {
    fn read(reader: &mut Reader) -> Result<(A, B, C), Error> {
        Ok((
            try!(Readable::read(reader)),
            try!(Readable::read(reader)),
            try!(Readable::read(reader)),
        ))
    }
}

impl<A: Readable, B: Readable, C: Readable, D: Readable> Readable for (A, B, C, D) {
    fn read(reader: &mut Reader) -> Result<(A, B, C, D), Error> {
        Ok((
            try!(Readable::read(reader)),
            try!(Readable::read(reader)),
            try!(Readable::read(reader)),
            try!(Readable::read(reader)),
        ))
    }
}

impl Writeable for [u8; 4] {
    fn write<W: Writer>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write_bytes(self)
    }
}

/// Useful marker trait on types that can be sized byte slices
pub trait AsFixedBytes: Sized + AsRef<[u8]> {
    /// The length in bytes
    fn len(&self) -> usize;
}

impl<'a> AsFixedBytes for &'a [u8] {
    fn len(&self) -> usize {
        return 1;
    }
}
impl AsFixedBytes for Vec<u8> {
    fn len(&self) -> usize {
        return self.len();
    }
}
impl AsFixedBytes for [u8; 1] {
    fn len(&self) -> usize {
        return 1;
    }
}
impl AsFixedBytes for [u8; 2] {
    fn len(&self) -> usize {
        return 2;
    }
}
impl AsFixedBytes for [u8; 4] {
    fn len(&self) -> usize {
        return 4;
    }
}
impl AsFixedBytes for [u8; 6] {
    fn len(&self) -> usize {
        return 6;
    }
}
impl AsFixedBytes for [u8; 8] {
    fn len(&self) -> usize {
        return 8;
    }
}
impl AsFixedBytes for [u8; 20] {
    fn len(&self) -> usize {
        return 20;
    }
}
impl AsFixedBytes for [u8; 32] {
    fn len(&self) -> usize {
        return 32;
    }
}
impl AsFixedBytes for String {
    fn len(&self) -> usize {
        return self.len();
    }
}
impl AsFixedBytes for ::core::hash::Hash {
    fn len(&self) -> usize {
        return 32;
    }
}
impl AsFixedBytes for ::util::secp::key::SecretKey {
    fn len(&self) -> usize {
        return 1;
    }
}
impl AsFixedBytes for ::util::secp::Signature {
    fn len(&self) -> usize {
        return 64;
    }
}
impl AsFixedBytes for Identifier {
    fn len(&self) -> usize {
        return IDENTIFIER_SIZE;
    }
}