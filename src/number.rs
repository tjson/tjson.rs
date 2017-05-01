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

//! Number types available in TJSON

use error::Error;
use serde::de::{self, Visitor, Unexpected};
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use ordered_float::OrderedFloat;
use std::fmt::{self, Debug, Display};

/// Represents a TJSON number: either signed int, unsigned int, or float
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Number {
    n: N,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum N {
    /// Signed integer (not necessarily less than zero).
    Int(i64),

    /// Unsigned integer
    UInt(u64),

    /// Floating point (always finite).
    Float(OrderedFloat<f64>),
}

impl Number {
    /// Returns true if the `Number` is a signed integer.
    // TODO: example code
    #[inline]
    pub fn is_i64(&self) -> bool {
        match self.n {
            N::UInt(_) | N::Float(_) => false,
            N::Int(_) => true,
        }
    }

    /// Returns true if the `Number` is an unsigned integer.
    // TODO: example code
    #[inline]
    pub fn is_u64(&self) -> bool {
        match self.n {
            N::UInt(_) => true,
            N::Int(_) | N::Float(_) => false,
        }
    }

    /// Returns true if the `Number` is a floating point.
    // TODO: example code
    #[inline]
    pub fn is_f64(&self) -> bool {
        match self.n {
            N::Float(_) => true,
            N::UInt(_) | N::Int(_) => false,
        }
    }

    /// If the `Number` is a signed integer, represent it as i64 if possible. Returns
    /// None otherwise.
    // TODO: example code
    #[inline]
    pub fn as_i64(&self) -> Option<i64> {
        match self.n {
            N::UInt(n) => None,
            N::Int(n) => Some(n),
            N::Float(_) => None,
        }
    }

    /// If the `Number` is an unsigned integer, represent it as u64 if possible. Returns
    /// None otherwise.
    // TODO: example code
    #[inline]
    pub fn as_u64(&self) -> Option<u64> {
        match self.n {
            N::UInt(n) => Some(n),
            N::Int(n) => None,
            N::Float(_) => None,
        }
    }

    /// If the `Number` is a floating point, Represents the number as f64 if possible.
    /// Returns None otherwise.
    // TODO: example code
    #[inline]
    pub fn as_f64(&self) -> Option<f64> {
        match self.n {
            N::UInt(n) => None,
            N::Int(n) => None,
            N::Float(n) => Some(n.into()),
        }
    }

    /// Converts a finite `f64` to a `Number`. Infinite or NaN values are not JSON
    /// numbers.
    ///
    /// ```rust
    /// # use std::f64;
    /// #
    /// # use serde_json::Number;
    /// #
    /// assert!(Number::from_f64(256.0).is_some());
    ///
    /// assert!(Number::from_f64(f64::NAN).is_none());
    /// ```
    #[inline]
    pub fn from_f64(f: f64) -> Option<Number> {
        if f.is_finite() {
            Some(Number { n: N::Float(OrderedFloat::from(f)) })
        } else {
            None
        }
    }
}

impl fmt::Display for Number {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self.n {
            N::UInt(i) => Display::fmt(&i, formatter),
            N::Int(i) => Display::fmt(&i, formatter),
            N::Float(f) => Display::fmt(&f, formatter),
        }
    }
}

impl Debug for Number {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(&self.n, formatter)
    }
}

impl Serialize for Number {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        match self.n {
            N::UInt(i) => serializer.serialize_u64(i),
            N::Int(i) => serializer.serialize_i64(i),
            N::Float(f) => serializer.serialize_f64(f.into()),
        }
    }
}

impl<'de> Deserialize<'de> for Number {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Number, D::Error>
        where
            D: Deserializer<'de>,
    {
        struct NumberVisitor;

        impl<'de> Visitor<'de> for NumberVisitor {
            type Value = Number;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a number")
            }

            #[inline]
            fn visit_i64<E>(self, value: i64) -> Result<Number, E> {
                Ok(value.into())
            }

            #[inline]
            fn visit_u64<E>(self, value: u64) -> Result<Number, E> {
                Ok(value.into())
            }

            #[inline]
            fn visit_f64<E>(self, value: f64) -> Result<Number, E>
                where
                    E: de::Error,
            {
                Number::from_f64(value).ok_or_else(|| de::Error::custom("not a JSON number"))
            }
        }

        deserializer.deserialize_any(NumberVisitor)
    }
}

impl<'de> Deserializer<'de> for Number {
    type Error = Error;

    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Error>
        where
            V: Visitor<'de>,
    {
        match self.n {
            N::UInt(i) => visitor.visit_u64(i),
            N::Int(i) => visitor.visit_i64(i),
            N::Float(f) => visitor.visit_f64(f.into()),
        }
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string bytes
        byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

impl<'de, 'a> Deserializer<'de> for &'a Number {
    type Error = Error;

    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Error>
        where
            V: Visitor<'de>,
    {
        match self.n {
            N::UInt(i) => visitor.visit_u64(i),
            N::Int(i) => visitor.visit_i64(i),
            N::Float(f) => visitor.visit_f64(f.into()),
        }
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string bytes
        byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

macro_rules! from_signed {
    ($($signed_ty:ident)*) => {
        $(
            impl From<$signed_ty> for Number {
                #[inline]
                fn from(i: $signed_ty) -> Self {
                    if i < 0 {
                        Number { n: N::Int(i as i64) }
                    } else {
                        Number { n: N::UInt(i as u64) }
                    }
                }
            }
        )*
    };
}

macro_rules! from_unsigned {
    ($($unsigned_ty:ident)*) => {
        $(
            impl From<$unsigned_ty> for Number {
                #[inline]
                fn from(u: $unsigned_ty) -> Self {
                    Number { n: N::UInt(u as u64) }
                }
            }
        )*
    };
}

from_signed!(i8 i16 i32 i64 isize);
from_unsigned!(u8 u16 u32 u64 usize);

impl Number {
    // Not public API. Should be pub(crate).
    #[doc(hidden)]
    pub fn unexpected(&self) -> Unexpected {
        match self.n {
            N::UInt(u) => Unexpected::Unsigned(u),
            N::Int(i) => Unexpected::Signed(i),
            N::Float(f) => Unexpected::Float(f.into()),
        }
    }
}
