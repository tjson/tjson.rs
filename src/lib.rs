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

#![deny(missing_docs, unstable_features, unused_import_braces)]

extern crate chrono;
extern crate dtoa;
extern crate itoa;
extern crate num_traits;
extern crate ordered_float;
#[macro_use]
extern crate serde;

#[cfg(feature = "preserve_order")]
extern crate linked_hash_map;

#[doc(inline)]
pub use self::de::{Deserializer, StreamDeserializer, from_reader, from_slice, from_str};
#[doc(inline)]
pub use self::error::{Error, Result};
#[doc(inline)]
pub use self::ser::{Serializer, to_string, to_string_pretty, to_vec, to_vec_pretty, to_writer,
                    to_writer_pretty};
#[doc(inline)]
pub use self::value::{Map, Set, Number, Value, from_value, to_value};

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
