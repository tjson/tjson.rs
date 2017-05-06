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

//! A set containing unique `tjson::Value` types
//!
//! By default the map is backed by a [`BTreeMap`]. To preserve insertion
//! order, enable the `preserve_order` feature which will use the
//! [`LinkedHashMap`] type instead.
//!
//! [`BTreeMap`]: https://doc.rust-lang.org/std/collections/struct.BTreeMap.html
//! [`LinkedHashMap`]: https://docs.rs/linked-hash-map/*/linked_hash_map/struct.LinkedHashMap.html

#[cfg(feature = "preserve_order")]
use linked_hash_map::{self, LinkedHashMap};
use serde::ser;
#[cfg(not(feature = "preserve_order"))]
use std::collections::btree_map::{self, BTreeMap};
use std::fmt::Debug;
use std::hash::Hash;
use value::Value;

#[cfg(not(feature = "preserve_order"))]
type SetImpl<T> = BTreeMap<T, ()>;

#[cfg(feature = "preserve_order")]
type SetImpl<T> = LinkedHashMap<T, ()>;

/// Represents a TJSON set type.
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Set<T: Hash + Debug + Eq + PartialEq + PartialOrd + Ord> {
    set: SetImpl<T>,
}

impl<T: Hash + Debug + Eq + PartialEq + PartialOrd + Ord> Set<T> {
    /// Create a new empty set
    pub fn new() -> Set<T> {
        Set { set: SetImpl::new() }
    }
}

impl Set<Value> {
    /// Returns the number of elements in the set.
    #[inline]
    pub fn len(&self) -> usize {
        self.set.len()
    }

    /// Returns true if the set contains no elements.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.set.is_empty()
    }

    /// Gets an iterator over the entries of the set.
    #[inline]
    pub fn iter(&self) -> Iter {
        Iter { iter: self.set.iter() }
    }
}

impl ser::Serialize for Set<Value> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let result: Vec<&Value> = self.iter().collect();
        result.serialize(serializer)
    }
}

//////////////////////////////////////////////////////////////////////////////

impl<'a> IntoIterator for &'a Set<Value> {
    type Item = &'a Value;
    type IntoIter = Iter<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Iter { iter: self.set.iter() }
    }
}

/// An iterator over a tjson::Set's entries.
pub struct Iter<'a> {
    iter: IterImpl<'a>,
}

#[cfg(not(feature = "preserve_order"))]
type IterImpl<'a> = btree_map::Iter<'a, Value, ()>;

#[cfg(feature = "preserve_order")]
type IterImpl<'a> = linked_hash_map::Iter<'a, Value, ()>;

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Value;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|v| v.0)
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a> DoubleEndedIterator for Iter<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(|v| v.0)
    }
}

impl<'a> ExactSizeIterator for Iter<'a> {
    #[inline]
    fn len(&self) -> usize {
        self.iter.len()
    }
}

//////////////////////////////////////////////////////////////////////////////

impl IntoIterator for Set<Value> {
    type Item = Value;
    type IntoIter = IntoIter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter { iter: self.set.into_iter() }
    }
}

/// An owning iterator over a tjson::Set's entries.
pub struct IntoIter {
    iter: IntoIterImpl,
}

#[cfg(not(feature = "preserve_order"))]
type IntoIterImpl = btree_map::IntoIter<Value, ()>;
#[cfg(feature = "preserve_order")]
type IntoIterImpl = linked_hash_map::IntoIter<Value, ()>;

impl Iterator for IntoIter {
    type Item = Value;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|v| v.0)
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a> DoubleEndedIterator for IntoIter {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(|v| v.0)
    }
}

impl<'a> ExactSizeIterator for IntoIter {
    #[inline]
    fn len(&self) -> usize {
        self.iter.len()
    }
}

//////////////////////////////////////////////////////////////////////////////
