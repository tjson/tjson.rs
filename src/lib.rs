// tjson: Tagged JSON with rich types
//
// Copyright (c) 2017 Tony Arcieri
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
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
extern crate serde;

pub mod map;
pub mod number;
pub mod set;
pub mod value;
