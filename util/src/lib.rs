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
//This file is based on utils/src/lib.rs from
// <https://github.com/mimblewimble/grin>,
// originally developed by The Grin Developers and distributed under the
// Apache License, Version 2.0. You may obtain a copy of the License at
// <http://www.apache.org/licenses/LICENSE-2.0>.

//! Logging, as well as various low-level utilities that factor Rust
//! patterns that are frequent within the codebase.

#![deny(non_upper_case_globals)]
#![deny(non_camel_case_types)]
#![deny(non_snake_case)]
#![deny(unused_mut)]
#![deny(missing_docs)]

#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;

#[macro_use]
extern crate lazy_static;

extern crate serde;
#[macro_use]
extern crate serde_derive;

// Re-export SECP crate so only has to be included once
pub extern crate secp256k1zkp as secp_;
pub use secp_ as secp;

// other utils
use std::cell::{Ref, RefCell};

pub mod hex;
pub use hex::{from_hex, to_hex};

/// Encapsulation of a RefCell<Option<T>> for one-time initialization after
/// construction. This implementation will purposefully fail hard if not used
/// properly, for example if it's not initialized before being first used
/// (borrowed).
#[derive(Clone)]
pub struct OneTime<T> {
    /// inner
    inner: RefCell<Option<T>>,
}

unsafe impl<T> Sync for OneTime<T> {}
unsafe impl<T> Send for OneTime<T> {}

impl<T> OneTime<T> {
    /// Builds a new uninitialized OneTime.
    pub fn new() -> OneTime<T> {
        OneTime {
            inner: RefCell::new(None),
        }
    }

    /// Initializes the OneTime, should only be called once after construction.
    pub fn init(&self, value: T) {
        let mut inner_mut = self.inner.borrow_mut();
        *inner_mut = Some(value);
    }

    /// Whether the OneTime has been initialized
    pub fn is_initialized(&self) -> bool {
        match self.inner.try_borrow() {
            Ok(inner) => inner.is_some(),
            Err(_) => false,
        }
    }

    /// Borrows the OneTime, should only be called after initialization.
    pub fn borrow(&self) -> Ref<T> {
        Ref::map(self.inner.borrow(), |o| o.as_ref().unwrap())
    }
}

// Logging related
pub mod logger;
pub use logger::{init_logger, init_test_logger, LOGGER};

pub mod types;
pub use types::{LoggingConfig, LogLevel};
