use crate::CodecError;

pub const MAX_FRAME_BYTES: usize = 64 * 1024;

pub(crate) fn limit(maximum: usize) -> Result<(), CodecError> {
    if maximum == 0 || maximum > MAX_FRAME_BYTES {
        return Err(CodecError::InvalidLimit);
    }
    Ok(())
}
