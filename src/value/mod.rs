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

//! The Value enum, a loosely typed way of representing any valid TJSON value.

use serde::ser::Serialize;
use serde::de::DeserializeOwned;

use error::Error;
pub use map::Map;
pub use set::Set;
pub use number::Number;

pub use chrono::datetime::DateTime;
pub use chrono::offset::utc::UTC;

pub use self::index::Index;

use self::ser::Serializer;

/// Represents any valid TJSON value.
#[derive(Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum Value {
    /// Since TJSON is non-nullable, this indicates cases where a requested
    /// value is not present, e.g. for non-panicing `Index`
    Undefined,

    /// Represents a TJSON boolean.
    ///
    /// ```rust
    /// # #[macro_use]
    /// # extern crate tjson;
    /// #
    /// # fn main() {
    /// let v = tjson!(true);
    /// # }
    /// ```
    Bool(bool),

    /// Represents TJSON binary data (8-bit clean).
    ///
    /// ```rust
    /// # #[macro_use]
    /// # extern crate tjson;
    /// #
    /// # fn main() {
    /// let v = tjson!(b"a bytestring");
    /// # }
    /// ```
    Data(Vec<u8>),

    /// Represents a TJSON number: either a signed integer (`Number::Int`),
    /// unsigned integer (`Number::UInt`), or floating point (`Number::Float`)
    ///
    /// ```rust
    /// # #[macro_use]
    /// # extern crate tjson;
    /// #
    /// # fn main() {
    /// let f = tjson!(12.5);
    /// let i = tjson!(-42);
    /// let u = tjson!(0xFFFFFFFFFFFFFFFF);
    /// # }
    /// ```
    Number(Number),

    /// Represents a TJSON Unicode String.
    ///
    /// ```rust
    /// # #[macro_use]
    /// # extern crate tjson;
    /// #
    /// # fn main() {
    /// let v = tjson!("a string");
    /// # }
    String(String),

    /// Represents a TJSON timestamp (always UTC).
    ///
    /// ```rust
    /// # extern crate chrono;
    /// # #[macro_use]
    /// # extern crate tjson;
    /// # use chrono::offset::utc::UTC;
    /// #
    /// # fn main() {
    /// let v = tjson!(UTC::now());
    /// # }
    Timestamp(DateTime<UTC>),

    /// Represents a TJSON array.
    ///
    /// ```rust
    /// # #[macro_use]
    /// # extern crate tjson;
    /// #
    /// # fn main() {
    /// let v = tjson!(["an", "array"]);
    /// # }
    /// ```
    Array(Vec<Value>),

    /// Represents a TJSON set.
    ///
    /// By default the set is backed by a `BTreeSet`. Enable the
    /// `preserve_order` feature of this crate to use `LinkedHashMap` instead,
    /// which preserves entries in the order they are inserted into the set.
    ///
    /// ```rust
    /// # #[macro_use]
    /// # extern crate tjson;
    /// # use tjson::set::Set;
    /// #
    /// # fn main() {
    /// let v = tjson!(Set::new());
    /// # }
    /// ```
    Set(Set<Value>),

    /// Represents a TJSON object.
    ///
    /// By default the map is backed by a `BTreeMap`. Enable the
    /// `preserve_order` feature of this crate to use `LinkedHashMap` instead,
    /// which preserves entries in the order they are inserted into the map.
    ///
    /// ```rust
    /// # #[macro_use]
    /// # extern crate tjson;
    /// #
    /// # fn main() {
    /// let v = tjson!({ "an": "object" });
    /// # }
    /// ```
    Object(Map<String, Value>),
}

fn parse_index(s: &str) -> Option<usize> {
    if s.starts_with('+') || (s.starts_with('0') && s.len() != 1) {
        return None;
    }
    s.parse().ok()
}

impl Value {
    /// Index into a TJSON array or map. A string index can be used to access a
    /// value in a map, and a usize index can be used to access an element of an
    /// array.
    ///
    /// Returns `None` if the type of `self` does not match the type of the
    /// index, for example if the index is a string and `self` is an array or a
    /// number. Also returns `None` if the given key does not exist in the map
    /// or the given index is not within the bounds of the array.
    ///
    /// ```rust
    /// # #[macro_use]
    /// # extern crate tjson;
    /// #
    /// # fn main() {
    /// let object = tjson!({ "A": 65, "B": 66, "C": 67 });
    /// assert_eq!(*object.get("A").unwrap(), tjson!(65));
    ///
    /// let array = tjson!([ "A", "B", "C" ]);
    /// assert_eq!(*array.get(2).unwrap(), tjson!("C"));
    ///
    /// assert_eq!(array.get("A"), None);
    /// # }
    /// ```
    ///
    /// Square brackets can also be used to index into a value in a more concise
    /// way. This returns `Value::Undefined` in cases where `get` would have returned
    /// `None`.
    ///
    /// ```rust
    /// # #[macro_use]
    /// # extern crate tjson;
    /// #
    /// # fn main() {
    /// let object = tjson!({
    ///     "A": ["a", "á", "à"],
    ///     "B": ["b", "b́"],
    ///     "C": ["c", "ć", "ć̣", "ḉ"],
    /// });
    /// assert_eq!(object["B"][0], tjson!("b"));
    ///
    /// assert_eq!(object["D"], tjson!(null));
    /// assert_eq!(object[0]["x"]["y"]["z"], tjson!(null));
    /// # }
    /// ```
    pub fn get<I: Index>(&self, index: I) -> Option<&Value> {
        index.index_into(self)
    }

    /// Mutably index into a TJSON array or map. A string index can be used to
    /// access a value in a map, and a usize index can be used to access an
    /// element of an array.
    ///
    /// Returns `None` if the type of `self` does not match the type of the
    /// index, for example if the index is a string and `self` is an array or a
    /// number. Also returns `None` if the given key does not exist in the map
    /// or the given index is not within the bounds of the array.
    ///
    /// ```rust
    /// # #[macro_use]
    /// # extern crate tjson;
    /// #
    /// # fn main() {
    /// let mut object = tjson!({ "A": 65, "B": 66, "C": 67 });
    /// *object.get_mut("A").unwrap() = tjson!(69);
    ///
    /// let mut array = tjson!([ "A", "B", "C" ]);
    /// *array.get_mut(2).unwrap() = tjson!("D");
    /// # }
    /// ```
    pub fn get_mut<I: Index>(&mut self, index: I) -> Option<&mut Value> {
        index.index_into_mut(self)
    }

    /// Returns true if the `Value` is an Object. Returns false otherwise.
    ///
    /// For any Value on which `is_object` returns true, `as_object` and
    /// `as_object_mut` are guaranteed to return the map representation of the
    /// object.
    ///
    /// ```rust
    /// # #[macro_use]
    /// # extern crate tjson;
    /// #
    /// # fn main() {
    /// let obj = tjson!({ "a": { "nested": true }, "b": ["an", "array"] });
    ///
    /// assert!(obj.is_object());
    /// assert!(obj["a"].is_object());
    ///
    /// // array, not an object
    /// assert!(!obj["b"].is_object());
    /// # }
    /// ```
    pub fn is_object(&self) -> bool {
        self.as_object().is_some()
    }

    /// If the `Value` is an Object, returns the associated Map. Returns None
    /// otherwise.
    ///
    /// ```rust
    /// # #[macro_use]
    /// # extern crate tjson;
    /// #
    /// # fn main() {
    /// let v = tjson!({ "a": { "nested": true }, "b": ["an", "array"] });
    ///
    /// // The length of `{"nested": true}` is 1 entry.
    /// assert_eq!(v["a"].as_object().unwrap().len(), 1);
    ///
    /// // The array `["an", "array"]` is not an object.
    /// assert_eq!(v["b"].as_object(), None);
    /// # }
    /// ```
    pub fn as_object(&self) -> Option<&Map<String, Value>> {
        match *self {
            Value::Object(ref map) => Some(map),
            _ => None,
        }
    }

    /// If the `Value` is an Object, returns the associated mutable Map.
    /// Returns None otherwise.
    ///
    /// ```rust
    /// # #[macro_use]
    /// # extern crate tjson;
    /// #
    /// # fn main() {
    /// let mut v = tjson!({ "a": { "nested": true } });
    ///
    /// v["a"].as_object_mut().unwrap().clear();
    /// assert_eq!(v, tjson!({ "a": {} }));
    /// # }
    ///
    /// ```
    pub fn as_object_mut(&mut self) -> Option<&mut Map<String, Value>> {
        match *self {
            Value::Object(ref mut map) => Some(map),
            _ => None,
        }
    }

    /// Returns true if the `Value` is an Array. Returns false otherwise.
    ///
    /// For any Value on which `is_array` returns true, `as_array` and
    /// `as_array_mut` are guaranteed to return the vector representing the
    /// array.
    ///
    /// ```rust
    /// # #[macro_use]
    /// # extern crate tjson;
    /// #
    /// # fn main() {
    /// let obj = tjson!({ "a": ["an", "array"], "b": { "an": "object" } });
    ///
    /// assert!(obj["a"].is_array());
    ///
    /// // an object, not an array
    /// assert!(!obj["b"].is_array());
    /// # }
    /// ```
    pub fn is_array(&self) -> bool {
        self.as_array().is_some()
    }

    /// If the `Value` is an Array, returns the associated vector. Returns None
    /// otherwise.
    ///
    /// ```rust
    /// # #[macro_use]
    /// # extern crate tjson;
    /// #
    /// # fn main() {
    /// let v = tjson!({ "a": ["an", "array"], "b": { "an": "object" } });
    ///
    /// // The length of `["an", "array"]` is 2 elements.
    /// assert_eq!(v["a"].as_array().unwrap().len(), 2);
    ///
    /// // The object `{"an": "object"}` is not an array.
    /// assert_eq!(v["b"].as_array(), None);
    /// # }
    /// ```
    pub fn as_array(&self) -> Option<&Vec<Value>> {
        match *self {
            Value::Array(ref array) => Some(&*array),
            _ => None,
        }
    }

    /// If the `Value` is an Array, returns the associated mutable vector.
    /// Returns None otherwise.
    ///
    /// ```rust
    /// # #[macro_use]
    /// # extern crate tjson;
    /// #
    /// # fn main() {
    /// let mut v = tjson!({ "a": ["an", "array"] });
    ///
    /// v["a"].as_array_mut().unwrap().clear();
    /// assert_eq!(v, tjson!({ "a": [] }));
    /// # }
    /// ```
    pub fn as_array_mut(&mut self) -> Option<&mut Vec<Value>> {
        match *self {
            Value::Array(ref mut list) => Some(list),
            _ => None,
        }
    }

    /// Returns true if the `Value` is a String. Returns false otherwise.
    ///
    /// For any Value on which `is_string` returns true, `as_str` is guaranteed
    /// to return the string slice.
    ///
    /// ```rust
    /// # #[macro_use]
    /// # extern crate tjson;
    /// #
    /// # fn main() {
    /// let v = tjson!({ "a": "some string", "b": false });
    ///
    /// assert!(v["a"].is_string());
    ///
    /// // The boolean `false` is not a string.
    /// assert!(!v["b"].is_string());
    /// # }
    /// ```
    pub fn is_string(&self) -> bool {
        self.as_str().is_some()
    }

    /// If the `Value` is a String, returns the associated str. Returns None
    /// otherwise.
    ///
    /// ```rust
    /// # #[macro_use]
    /// # extern crate tjson;
    /// #
    /// # fn main() {
    /// let v = tjson!({ "a": "some string", "b": false });
    ///
    /// assert_eq!(v["a"].as_str(), Some("some string"));
    ///
    /// // The boolean `false` is not a string.
    /// assert_eq!(v["b"].as_str(), None);
    /// # }
    /// ```
    pub fn as_str(&self) -> Option<&str> {
        match *self {
            Value::String(ref s) => Some(s),
            _ => None,
        }
    }

    /// Returns true if the `Value` is a Number. Returns false otherwise.
    ///
    /// ```rust
    /// # #[macro_use]
    /// # extern crate tjson;
    /// #
    /// # fn main() {
    /// let v = tjson!({ "a": 1, "b": "2" });
    ///
    /// assert!(v["a"].is_number());
    ///
    /// // The string `"2"` is a string, not a number.
    /// assert!(!v["b"].is_number());
    /// # }
    /// ```
    pub fn is_number(&self) -> bool {
        match *self {
            Value::Number(_) => true,
            _ => false,
        }
    }

    /// Returns true if the `Value` is an integer between `i64::MIN` and
    /// `i64::MAX`.
    ///
    /// For any Value on which `is_i64` returns true, `as_i64` is guaranteed to
    /// return the integer value.
    ///
    /// ```rust
    /// # #[macro_use]
    /// # extern crate tjson;
    /// #
    /// # use std::i64;
    /// #
    /// # fn main() {
    /// let big = i64::MAX as u64 + 10;
    /// let v = tjson!({ "a": 64, "b": big, "c": 256.0 });
    ///
    /// assert!(v["a"].is_i64());
    ///
    /// // Greater than i64::MAX.
    /// assert!(!v["b"].is_i64());
    ///
    /// // Numbers with a decimal point are not considered integers.
    /// assert!(!v["c"].is_i64());
    /// # }
    /// ```
    pub fn is_i64(&self) -> bool {
        match *self {
            Value::Number(ref n) => n.is_i64(),
            _ => false,
        }
    }

    /// Returns true if the `Value` is an integer between zero and `u64::MAX`.
    ///
    /// For any Value on which `is_u64` returns true, `as_u64` is guaranteed to
    /// return the integer value.
    ///
    /// ```rust
    /// # #[macro_use]
    /// # extern crate tjson;
    /// #
    /// # fn main() {
    /// let v = tjson!({ "a": 64, "b": -64, "c": 256.0 });
    ///
    /// assert!(v["a"].is_u64());
    ///
    /// // Negative integer.
    /// assert!(!v["b"].is_u64());
    ///
    /// // Numbers with a decimal point are not considered integers.
    /// assert!(!v["c"].is_u64());
    /// # }
    /// ```
    pub fn is_u64(&self) -> bool {
        match *self {
            Value::Number(ref n) => n.is_u64(),
            _ => false,
        }
    }

    /// Returns true if the `Value` is a number that can be represented by f64.
    ///
    /// For any Value on which `is_f64` returns true, `as_f64` is guaranteed to
    /// return the floating point value.
    ///
    /// Currently this function returns true if and only if both `is_i64` and
    /// `is_u64` return false but this is not a guarantee in the future.
    ///
    /// ```rust
    /// # #[macro_use]
    /// # extern crate tjson;
    /// #
    /// # fn main() {
    /// let v = tjson!({ "a": 256.0, "b": 64, "c": -64 });
    ///
    /// assert!(v["a"].is_f64());
    ///
    /// // Integers.
    /// assert!(!v["b"].is_f64());
    /// assert!(!v["c"].is_f64());
    /// # }
    /// ```
    pub fn is_f64(&self) -> bool {
        match *self {
            Value::Number(ref n) => n.is_f64(),
            _ => false,
        }
    }

    /// If the `Value` is an integer, represent it as i64 if possible. Returns
    /// None otherwise.
    ///
    /// ```rust
    /// # #[macro_use]
    /// # extern crate tjson;
    /// #
    /// # use std::i64;
    /// #
    /// # fn main() {
    /// let big = i64::MAX as u64 + 10;
    /// let v = tjson!({ "a": 64, "b": big, "c": 256.0 });
    ///
    /// assert_eq!(v["a"].as_i64(), Some(64));
    /// assert_eq!(v["b"].as_i64(), None);
    /// assert_eq!(v["c"].as_i64(), None);
    /// # }
    /// ```
    pub fn as_i64(&self) -> Option<i64> {
        match *self {
            Value::Number(ref n) => n.as_i64(),
            _ => None,
        }
    }

    /// If the `Value` is an integer, represent it as u64 if possible. Returns
    /// None otherwise.
    ///
    /// ```rust
    /// # #[macro_use]
    /// # extern crate tjson;
    /// #
    /// # fn main() {
    /// let v = tjson!({ "a": 64, "b": -64, "c": 256.0 });
    ///
    /// assert_eq!(v["a"].as_u64(), Some(64));
    /// assert_eq!(v["b"].as_u64(), None);
    /// assert_eq!(v["c"].as_u64(), None);
    /// # }
    /// ```
    pub fn as_u64(&self) -> Option<u64> {
        match *self {
            Value::Number(ref n) => n.as_u64(),
            _ => None,
        }
    }

    /// If the `Value` is a number, represent it as f64 if possible. Returns
    /// None otherwise.
    ///
    /// ```rust
    /// # #[macro_use]
    /// # extern crate tjson;
    /// #
    /// # fn main() {
    /// let v = tjson!({ "a": 256.0, "b": 64, "c": -64 });
    ///
    /// assert_eq!(v["a"].as_f64(), Some(256.0));
    /// assert_eq!(v["b"].as_f64(), Some(64.0));
    /// assert_eq!(v["c"].as_f64(), Some(-64.0));
    /// # }
    /// ```
    pub fn as_f64(&self) -> Option<f64> {
        match *self {
            Value::Number(ref n) => n.as_f64(),
            _ => None,
        }
    }

    /// Returns true if the `Value` is a Boolean. Returns false otherwise.
    ///
    /// For any Value on which `is_boolean` returns true, `as_bool` is
    /// guaranteed to return the boolean value.
    ///
    /// ```rust
    /// # #[macro_use]
    /// # extern crate tjson;
    /// #
    /// # fn main() {
    /// let v = tjson!({ "a": false, "b": "false" });
    ///
    /// assert!(v["a"].is_boolean());
    ///
    /// // The string `"false"` is a string, not a boolean.
    /// assert!(!v["b"].is_boolean());
    /// # }
    /// ```
    pub fn is_boolean(&self) -> bool {
        self.as_bool().is_some()
    }

    /// If the `Value` is a Boolean, returns the associated bool. Returns None
    /// otherwise.
    ///
    /// ```rust
    /// # #[macro_use]
    /// # extern crate tjson;
    /// #
    /// # fn main() {
    /// let v = tjson!({ "a": false, "b": "false" });
    ///
    /// assert_eq!(v["a"].as_bool(), Some(false));
    ///
    /// // The string `"false"` is a string, not a boolean.
    /// assert_eq!(v["b"].as_bool(), None);
    /// # }
    /// ```
    pub fn as_bool(&self) -> Option<bool> {
        match *self {
            Value::Bool(b) => Some(b),
            _ => None,
        }
    }

    /// Returns true if the `Value` is Undefined. Returns false otherwise.
    ///
    /// For any Value on which `is_undefined` returns true, `as_undefined` is guaranteed
    /// to return `Some(())`.
    ///
    /// ```rust
    /// # #[macro_use]
    /// # extern crate tjson;
    /// #
    /// # fn main() {
    /// let v = tjson!({ "a": null, "b": false });
    ///
    /// assert!(v["a"].is_undefined());
    ///
    /// // The boolean `false` is not null.
    /// assert!(!v["b"].is_undefined());
    /// # }
    /// ```
    pub fn is_undefined(&self) -> bool {
        self.as_undefined().is_some()
    }

    /// If the `Value` is Undefined, returns (). Returns None otherwise.
    ///
    /// ```rust
    /// # #[macro_use]
    /// # extern crate tjson;
    /// #
    /// # fn main() {
    /// let v = tjson!({ "a": null, "b": false });
    ///
    /// assert_eq!(v["a"].as_undefined(), Some(()));
    ///
    /// // The boolean `false` is not null.
    /// assert_eq!(v["b"].as_undefined(), None);
    /// # }
    /// ```
    pub fn as_undefined(&self) -> Option<()> {
        match *self {
            Value::Undefined => Some(()),
            _ => None,
        }
    }

    /// Looks up a value by a JSON Pointer.
    ///
    /// JSON Pointer defines a string syntax for identifying a specific value
    /// within a JavaScript Object Notation (JSON) document.
    ///
    /// A Pointer is a Unicode string with the reference tokens separated by `/`.
    /// Inside tokens `/` is replaced by `~1` and `~` is replaced by `~0`. The
    /// addressed value is returned and if there is no such value `None` is
    /// returned.
    ///
    /// For more information read [RFC6901](https://tools.ietf.org/html/rfc6901).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[macro_use]
    /// # extern crate tjson;
    /// #
    /// # fn main() {
    /// let data = tjson!({
    ///     "x": {
    ///         "y": ["z", "zz"]
    ///     }
    /// });
    ///
    /// assert_eq!(data.pointer("/x/y/1").unwrap(), &tjson!("zz"));
    /// assert_eq!(data.pointer("/a/b/c"), None);
    /// # }
    /// ```
    pub fn pointer<'a>(&'a self, pointer: &str) -> Option<&'a Value> {
        if pointer == "" {
            return Some(self);
        }
        if !pointer.starts_with('/') {
            return None;
        }
        let tokens = pointer
            .split('/')
            .skip(1)
            .map(|x| x.replace("~1", "/").replace("~0", "~"));
        let mut target = self;

        for token in tokens {
            let target_opt = match *target {
                Value::Object(ref map) => map.get(&token),
                Value::Array(ref list) => parse_index(&token).and_then(|x| list.get(x)),
                _ => return None,
            };
            if let Some(t) = target_opt {
                target = t;
            } else {
                return None;
            }
        }
        Some(target)
    }

    /// Looks up a value by a JSON Pointer and returns a mutable reference to
    /// that value.
    ///
    /// JSON Pointer defines a string syntax for identifying a specific value
    /// within a JavaScript Object Notation (JSON) document.
    ///
    /// A Pointer is a Unicode string with the reference tokens separated by `/`.
    /// Inside tokens `/` is replaced by `~1` and `~` is replaced by `~0`. The
    /// addressed value is returned and if there is no such value `None` is
    /// returned.
    ///
    /// For more information read [RFC6901](https://tools.ietf.org/html/rfc6901).
    ///
    /// # Example of Use
    ///
    /// ```rust
    /// extern crate tjson;
    ///
    /// use tjson::Value;
    /// use std::mem;
    ///
    /// fn main() {
    ///     let s = r#"{"x": 1.0, "y": 2.0}"#;
    ///     let mut value: Value = tjson::from_str(s).unwrap();
    ///
    ///     // Check value using read-only pointer
    ///     assert_eq!(value.pointer("/x"), Some(&1.0.into()));
    ///     // Change value with direct assignment
    ///     *value.pointer_mut("/x").unwrap() = 1.5.into();
    ///     // Check that new value was written
    ///     assert_eq!(value.pointer("/x"), Some(&1.5.into()));
    ///
    ///     // "Steal" ownership of a value. Can replace with any valid Value.
    ///     let old_x = value.pointer_mut("/x").map(|x| mem::replace(x, Value::Undefined)).unwrap();
    ///     assert_eq!(old_x, 1.5);
    ///     assert_eq!(value.pointer("/x").unwrap(), &Value::Undefined);
    /// }
    /// ```
    pub fn pointer_mut<'a>(&'a mut self, pointer: &str) -> Option<&'a mut Value> {
        if pointer == "" {
            return Some(self);
        }
        if !pointer.starts_with('/') {
            return None;
        }
        let tokens = pointer
            .split('/')
            .skip(1)
            .map(|x| x.replace("~1", "/").replace("~0", "~"));
        let mut target = self;

        for token in tokens {
            // borrow checker gets confused about `target` being mutably borrowed too many times because of the loop
            // this once-per-loop binding makes the scope clearer and circumvents the error
            let target_once = target;
            let target_opt = match *target_once {
                Value::Object(ref mut map) => map.get_mut(&token),
                Value::Array(ref mut list) => {
                    parse_index(&token).and_then(move |x| list.get_mut(x))
                }
                _ => return None,
            };
            if let Some(t) = target_opt {
                target = t;
            } else {
                return None;
            }
        }
        Some(target)
    }
}

mod index;
mod partial_eq;
mod from;
mod ser;
mod de;

/// The default value is `Value::Undefined`.
///
/// This is useful for handling omitted `Value` fields when deserializing.
///
/// # Examples
///
/// ```rust
/// # #[macro_use]
/// # extern crate serde_derive;
/// #
/// # extern crate tjson;
/// #
/// use tjson::Value;
///
/// #[derive(Deserialize)]
/// struct Settings {
///     level: i32,
///     #[serde(default)]
///     extras: Value,
/// }
///
/// # fn try_main() -> Result<(), tjson::Error> {
/// let data = r#" { "level": 42 } "#;
/// let s: Settings = tjson::from_str(data)?;
///
/// assert_eq!(s.level, 42);
/// assert_eq!(s.extras, Value::Undefined);
/// #
/// #     Ok(())
/// # }
/// #
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
impl Default for Value {
    fn default() -> Value {
        Value::Undefined
    }
}

/// Convert a `T` into `tjson::Value` which is an enum that can represent
/// any valid TJSON data.
///
/// ```rust
/// extern crate serde;
///
/// #[macro_use]
/// extern crate serde_derive;
///
/// #[macro_use]
/// extern crate tjson;
///
/// use std::error::Error;
///
/// #[derive(Serialize)]
/// struct User {
///     fingerprint: String,
///     location: String,
/// }
///
/// fn compare_json_values() -> Result<(), Box<Error>> {
///     let u = User {
///         fingerprint: "0xF9BA143B95FF6D82".to_owned(),
///         location: "Menlo Park, CA".to_owned(),
///     };
///
///     // The type of `expected` is `tjson::Value`
///     let expected = tjson!({
///                            "fingerprint": "0xF9BA143B95FF6D82",
///                            "location": "Menlo Park, CA",
///                          });
///
///     let v = tjson::to_value(u).unwrap();
///     assert_eq!(v, expected);
///
///     Ok(())
/// }
/// #
/// # fn main() {
/// #     compare_json_values().unwrap();
/// # }
/// ```
///
/// # Errors
///
/// This conversion can fail if `T`'s implementation of `Serialize` decides to
/// fail, or if `T` contains a map with non-string keys.
///
/// ```rust
/// extern crate tjson;
///
/// use std::collections::BTreeMap;
///
/// fn main() {
///     // The keys in this map are vectors, not strings.
///     let mut map = BTreeMap::new();
///     map.insert(vec![32, 64], "x86");
///
///     println!("{}", tjson::to_value(map).unwrap_err());
/// }
/// ```
pub fn to_value<T>(value: T) -> Result<Value, Error>
where
    T: Serialize,
{
    value.serialize(Serializer)
}

/// Interpret a `tjson::Value` as an instance of type `T`.
///
/// This conversion can fail if the structure of the Value does not match the
/// structure expected by `T`, for example if `T` is a struct type but the Value
/// contains something other than a TJSON map. It can also fail if the structure
/// is correct but `T`'s implementation of `Deserialize` decides that something
/// is wrong with the data, for example required struct fields are missing from
/// the TJSON map or some number is too big to fit in the expected primitive
/// type.
///
/// ```rust
/// #[macro_use]
/// extern crate tjson;
///
/// #[macro_use]
/// extern crate serde_derive;
///
/// extern crate serde;
///
/// #[derive(Deserialize, Debug)]
/// struct User {
///     fingerprint: String,
///     location: String,
/// }
///
/// fn main() {
///     // The type of `j` is `tjson::Value`
///     let j = tjson!({
///                     "fingerprint": "0xF9BA143B95FF6D82",
///                     "location": "Menlo Park, CA"
///                   });
///
///     let u: User = tjson::from_value(j).unwrap();
///     println!("{:#?}", u);
/// }
/// ```
pub fn from_value<T>(value: Value) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    T::deserialize(value)
}
