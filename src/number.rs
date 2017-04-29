//! Number types available in TJSON

/// Represents a TJSON number: either signed int, unsigned int, or float
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Number {
    /// Signed integer (not necessarily less than zero).
    Int(i64),

    /// Unsigned integer
    UInt(u64),

    /// Floating point (always finite).
    Float(f64),
}
