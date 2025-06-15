use crate::preview1::{GuestError, GuestMemory, GuestPtr, Memory, MemoryMut};

/// A simple in-memory backend for testing purposes.
///
/// `Vec<u8>` is used as the underlying memory. Bounds checks are performed for
/// every access and an error is returned if the pointer is out of range or
/// overflows.
impl Memory for Vec<u8> {
    fn copy_to_slice(&self, ptr: GuestPtr<[u8]>, dst: &mut [u8]) -> Result<(), GuestError> {
        let start = ptr.offset_base() as usize;
        let end = start.checked_add(ptr.len() as usize).ok_or(GuestError::PtrOverflow)?;
        if end > self.len() {
            return Err(GuestError::InvalidPointer);
        }
        dst.copy_from_slice(&self[start..end]);
        Ok(())
    }
}

impl MemoryMut for Vec<u8> {
    fn copy_from_slice(&mut self, ptr: GuestPtr<[u8]>, src: &[u8]) -> Result<(), GuestError> {
        let start = ptr.offset_base() as usize;
        let end = start.checked_add(ptr.len() as usize).ok_or(GuestError::PtrOverflow)?;
        if end > self.len() {
            return Err(GuestError::InvalidPointer);
        }
        self[start..end].copy_from_slice(src);
        Ok(())
    }
}

impl GuestMemory for Vec<u8> {}

