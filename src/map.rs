//! A map of String member names to `tjson::Value` types
//!
//! By default the map is backed by a [`BTreeMap`]. To preserve insertion
//! order, enable the `preserve_order` feature which will use the
//! [`LinkedHashMap`] type instead.
//!
//! [`BTreeMap`]: https://doc.rust-lang.org/std/collections/struct.BTreeMap.html
//! [`LinkedHashMap`]: https://docs.rs/linked-hash-map/*/linked_hash_map/struct.LinkedHashMap.html


#[cfg(feature = "preserve_order")]
use linked_hash_map::LinkedHashMap;
#[cfg(not(feature = "preserve_order"))]
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::hash::Hash;

#[cfg(not(feature = "preserve_order"))]
type MapImpl<K, V> = BTreeMap<K, V>;

#[cfg(feature = "preserve_order")]
type MapImpl<K, V> = LinkedHashMap<K, V>;

/// Represents a JSON key/value type.
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Map<K: Hash + Debug + Eq + PartialEq, V>(MapImpl<K, V>);
