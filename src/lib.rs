// tjson: Tagged JSON with rich types
//
// Copyright 2017 Tony Arcieri
//
// Includes portions of code from the Serde JSON project:
// https://github.com/serde-rs/json
//
// Copyright 2017 Serde Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # TJSON for Rust
//!
//! Tagged JSON (TJSON) is a microformat that extends the ubiquitous JSON
//! format with a set of tags that extend the types the format can express:
//!
//! ```json
//! {
//!   "array-example:A<O>": [
//!     {
//!       "string-example:s": "foobar",
//!       "binary-example:d": "QklOQVJZ",
//!       "float-example:f": 0.42,
//!       "int-example:i": "42",
//!       "timestamp-example:t": "2016-11-06T22:27:34Z",
//!       "boolean-example:b": true
//!     }
//!   ],
//!   "set-example:S<i>": [1, 2, 3]
//! }
//! ```

#![crate_name = "tjson"]
#![crate_type = "lib"]
#![doc(html_root_url = "https://docs.rs/tjson/")]

#![deny(missing_docs, missing_debug_implementations, missing_copy_implementations)]
#![deny(trivial_casts, trivial_numeric_casts)]
#![deny(unsafe_code, unstable_features, unused_import_braces, unused_qualifications)]

extern crate chrono;
extern crate dtoa;
extern crate itoa;
extern crate ordered_float;
#[macro_use]
extern crate serde;

#[cfg(feature = "preserve_order")]
extern crate linked_hash_map;

#[macro_use]
mod macros;

pub mod de;
pub mod error;
pub mod map;
pub mod ser;
pub mod set;
pub mod value;

mod iter;
mod number;
mod read;
