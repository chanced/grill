//! Big numeric data structures (re-exported from `num` crate) and parsers
//! for `BigInt` and `BigRational`

pub use num;
pub use num::{BigInt, BigRational};

use num::FromPrimitive;

mod rational;
pub use rational::parse_rational;

mod int;
pub use int::parse_int;

lazy_static::lazy_static! {
    static ref TEN: BigInt = BigInt::from_u8(10).unwrap();
}

use crate::error::OverflowError;

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
pub(crate) fn u64_to_usize(v: u64) -> Result<usize, OverflowError<u64, { usize::MAX as u64 }>> {
    v.try_into().map_err(|_| OverflowError(v))
}
