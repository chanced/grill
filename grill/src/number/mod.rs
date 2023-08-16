use crate::error::OverflowError;

#[inline]
pub(crate) fn usize_to_u32(v: usize) -> Result<u32, OverflowError> {
    if v > u32::MAX as usize {
        Err(OverflowError(v))
    } else {
        #[allow(clippy::cast_possible_truncation)]
        Ok(v as u32)
    }
}
