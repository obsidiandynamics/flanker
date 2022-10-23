//! Assertions for strings.

#![no_std]

extern crate alloc;

use alloc::string::ToString;
use core::fmt::Debug;
use core::str::FromStr;

/// Asserts that a value that implements both [`FromStr`] and [`ToString`] may be marshalled
/// to a string and back again to an equivalent (by a [`PartialEq`] comparison) value.
///
/// # Panics
/// If the assertion fails.
pub fn assert_loopback<T>(value: &T)
    where
        T::Err: Debug,
        T: Debug + FromStr + ToString + PartialEq,
{
    let encoded = value.to_string();
    let decoded = T::from_str(&encoded);
    assert!(decoded.is_ok(), "error: '{:?}'", decoded.err().unwrap());
    assert_eq!(value, &decoded.unwrap());
}

#[cfg(test)]
mod tests;