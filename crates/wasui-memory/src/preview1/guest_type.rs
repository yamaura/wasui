use crate::preview1::{GuestError, GuestPtr, Memory, MemoryMut};

pub trait GuestErrorType {
    fn success() -> Self;
}

pub trait GuestType: Sized {
    fn guest_size() -> u32;
    fn guest_align() -> usize;

    fn read<M: Memory>(memory: &M, ptr: GuestPtr<Self>) -> Result<Self, GuestError>;
    fn write<M: MemoryMut>(
        memory: &mut M,
        ptr: GuestPtr<Self>,
        value: Self,
    ) -> Result<(), GuestError>;
}

impl GuestType for u8 {
    fn guest_size() -> u32 {
        1
    }

    fn guest_align() -> usize {
        1
    }

    fn read<M: Memory>(memory: &M, ptr: GuestPtr<Self>) -> Result<Self, GuestError> {
        let mut buf = [0u8; 1];
        memory.copy_to_slice(ptr.as_array(1), &mut buf)?;
        Ok(buf[0])
    }

    fn write<M: MemoryMut>(
        memory: &mut M,
        ptr: GuestPtr<Self>,
        value: Self,
    ) -> Result<(), GuestError> {
        memory.copy_from_slice(ptr.as_array(1), &[value])
    }
}

impl GuestType for u32 {
    #[inline(always)]
    fn guest_size() -> u32 {
        4
    }

    fn guest_align() -> usize {
        4
    }

    #[inline]
    fn read<M: Memory>(memory: &M, ptr: GuestPtr<Self>) -> Result<Self, GuestError> {
        let mut buf = [0u8; 4];
        memory.copy_to_slice(ptr.cast::<u8>().as_array(4), &mut buf)?;
        Ok(u32::from_le_bytes(buf))
    }

    #[inline]
    fn write<M: MemoryMut>(
        memory: &mut M,
        ptr: GuestPtr<Self>,
        value: Self,
    ) -> Result<(), GuestError> {
        memory.copy_from_slice(ptr.cast::<u8>().as_array(4), &value.to_le_bytes())
    }
}

impl GuestType for u64 {
    #[inline(always)]
    fn guest_size() -> u32 {
        8
    }

    fn guest_align() -> usize {
        8
    }

    #[inline]
    fn read<M: Memory>(memory: &M, ptr: GuestPtr<Self>) -> Result<Self, GuestError> {
        let mut buf = [0u8; 8];
        memory.copy_to_slice(ptr.cast::<u8>().as_array(8), &mut buf)?;
        Ok(u64::from_le_bytes(buf))
    }

    #[inline]
    fn write<M: MemoryMut>(
        memory: &mut M,
        ptr: GuestPtr<Self>,
        value: Self,
    ) -> Result<(), GuestError> {
        memory.copy_from_slice(ptr.cast::<u8>().as_array(8), &value.to_le_bytes())
    }
}
