//! The Value enum, a loosely typed way of representing any valid TJSON value.

pub use chrono::datetime::DateTime;
pub use chrono::offset::utc::UTC;
pub use map::Map;
pub use number::Number;
pub use set::Set;

/// Represents any valid TJSON value.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum Value {
    /// Represents a TJSON boolean.
    Bool(bool),

    /// Represents TJSON binary data (8-bit clean).
    Data(Vec<u8>),

    /// Represents a TJSON number: either a signed integer (`Number::Int`),
    /// unsigned integer (`Number::UInt`), or floating point (`Number::Float`)
    Number(Number),

    /// Represents a TJSON Unicode String.
    String(String),

    /// Represents a TJSON timestamp (always UTC).
    Timestamp(DateTime<UTC>),

    /// Represents a TJSON array.
    Array(Vec<Value>),

    /// Represents a TJSON set.
    ///
    /// By default the set is backed by a `BTreeSet`. Enable the
    /// `preserve_order` feature of this crate to use `LinkedHashMap` instead,
    /// which preserves entries in the order they are inserted into the set.
    Set(Set<Value>),

    /// Represents a TJSON object.
    ///
    /// By default the map is backed by a `BTreeMap`. Enable the
    /// `preserve_order` feature of this crate to use `LinkedHashMap` instead,
    /// which preserves entries in the order they are inserted into the map.
    Object(Map<String, Value>),
}
