//! Big numeric data structures (re-exported from `num` crate) and parsers
//! for `BigInt` and `BigRational`

pub use num;
pub use num::{BigInt, BigRational};

use num::FromPrimitive;

mod rational;

mod int;

lazy_static::lazy_static! {
    static ref TEN: BigInt = BigInt::from_u8(10).unwrap();
}

/// Parses a string into a [`BigInt`]
pub fn parse_int(value: &str) -> Result<BigInt, NumberError> {
    int::Parser::parse(value)
}

/// Parses a string into a [`BigRational`]
pub fn parse_rational(value: &str) -> Result<BigRational, NumberError> {
    rational::Parser::parse(value)
}

use crate::error::{NumberError, OverflowError};

/// Attempts to convert a `usize` to `u32`
///
/// # Errors
/// Returns `OverflowError<usize, { u32::MAX as u64 }>` if `v` exceeds
/// `u32::MAX` (`4294967295`)
#[inline]
pub(crate) fn usize_to_u32(v: usize) -> Result<u32, OverflowError<usize, { u32::MAX as u64 }>> {
    v.try_into().map_err(|_| OverflowError(v))
}

/// Attempts to convert a `u64` to `usize`
///
/// # Errors
/// Returns `OverflowError<u64, { usize::MAX as u64 }>` if the architure is not
/// 64-bit and the value is too large
#[inline]
pub(crate) fn u64_to_usize(v: u64) -> Result<usize, OverflowError<u64, { usize::MAX as u64 }>> {
    v.try_into().map_err(|_| OverflowError(v))
}

fn ten() -> BigInt {
    TEN.clone()
}
