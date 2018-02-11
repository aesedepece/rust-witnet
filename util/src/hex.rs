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
//This file is based on utils/src/hex.rs from
// <https://github.com/mimblewimble/grin>,
// originally developed by The Grin Developers and distributed under the
// Apache License, Version 2.0. You may obtain a copy of the License at
// <http://www.apache.org/licenses/LICENSE-2.0>.

//! Implements hex-encoding from bytes to string and decoding of strings
//! to bytes. Given that rustc-serialize is deprecated and serde doesn't
//! provide easy hex encoding, hex is a bit in limbo right now in Rust-
//! land. It's simple enough that we can just have our own.

use std::fmt::Write;
use std::num;

/// Encode the provided bytes into a hex string
pub fn to_hex(bytes: Vec<u8>) -> String {
    let mut s = String::new();
    for byte in bytes {
        write!(&mut s, "{:02x}", byte).expect("Unable to write");
    }
    s
}

/// Decode a hex string into bytes.
pub fn from_hex(hex_str: String) -> Result<Vec<u8>, num::ParseIntError> {
    let hex_trim = if &hex_str[..2] == "0x" {
        hex_str[2..].to_owned()
    } else {
        hex_str.clone()
    };
    split_n(&hex_trim.trim()[..], 2)
        .iter()
        .map(|b| u8::from_str_radix(b, 16))
        .collect::<Result<Vec<u8>, _>>()
}

fn split_n(s: &str, n: usize) -> Vec<&str> {
    (0..(s.len() - n + 1) / 2 + 1)
        .map(|i| &s[2 * i..2 * i + n])
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_hex() {
        assert_eq!(to_hex(vec![0, 0, 0, 0]), "00000000");
        assert_eq!(to_hex(vec![10, 11, 12, 13]), "0a0b0c0d");
        assert_eq!(to_hex(vec![0, 0, 0, 255]), "000000ff");
    }

    #[test]
    fn test_from_hex() {
        assert_eq!(from_hex("00000000".to_string()).unwrap(), vec![0, 0, 0, 0]);
        assert_eq!(
            from_hex("0a0b0c0d".to_string()).unwrap(),
            vec![10, 11, 12, 13]
        );
        assert_eq!(
            from_hex("000000ff".to_string()).unwrap(),
            vec![0, 0, 0, 255]
        );
    }
}