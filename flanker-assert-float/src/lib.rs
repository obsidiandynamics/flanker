//! Assertions for floating-point numbers.

#![no_std]

extern crate alloc;

use alloc::format;
use alloc::string::String;

/// Asserts that the quotient of the difference between the two floats over the largest (by magnitude)
/// of the two floats, taken as an absolute value, is within the supplied `delta` threshold.
///
/// # Panics
/// If the quotient (`gamma`) exceeds `delta`.
pub fn assert_relative(left: f64, right: f64, delta: f64) {
    check_relative(left, right, delta)
        .map_err(|err| String::from("assertion failed: ") + &err)
        .unwrap();
}

/// The absolute value of an `f64`. This is not available in a `no_std` context.
fn abs(val: f64) -> f64 {
    if val >= 0.0 {
        val
    } else {
        -val
    }
}

/// Checks whether the quotient of the difference between the two floats over the largest (by magnitude)
/// of the two floats, taken as an absolute value, is within the supplied `delta` threshold.
///
/// # Errors
/// If the quotient (`gamma`) exceeds `delta`.
pub fn check_relative(left: f64, right: f64, delta: f64) -> Result<(), String> {
    if left != 0.0 || right != 0.0 {
        let denom = f64::max(abs(left), abs(right));
        let gamma = abs((left - right) / denom);
        if gamma > delta {
            return Err(format!(
                "(left ≉ right) left: {}, right: {}, gamma: {}",
                left, right, gamma
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests;