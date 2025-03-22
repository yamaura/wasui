//! A trait that has an interface similar to `wiggle::GuestMemory`.
//!
//! This library provides abstractions for guest memory access

mod backends;
mod guest_type;
pub use guest_type::{GuestErrorType, GuestType};

#[derive(Debug, thiserror::Error)]
pub enum GuestError {
    #[error("Pointer overflow")]
    PtrOverflow,
    #[error("Invalid pointer")]
    InvalidPointer,
    #[error("Invalid data")]
    InvalidData,
    #[error("Invalid memory")]
    InvalidMemory,
}

pub trait Memory: Sized {
    fn copy_to_slice(&self, ptr: GuestPtr<[u8]>, dst: &mut [u8]) -> Result<(), GuestError>;

    fn copy_to_vec(&self, ptr: GuestPtr<[u8]>) -> Result<Vec<u8>, GuestError> {
        let mut data = vec![0; ptr.len() as usize];
        self.copy_to_slice(ptr, &mut data)?;
        Ok(data)
    }

    fn read<T>(&self, ptr: GuestPtr<T>) -> Result<T, GuestError>
    where
        T: GuestType,
    {
        T::read(self, ptr)
    }
}

pub trait MemoryMut: Sized {
    fn copy_from_slice(&mut self, ptr: GuestPtr<[u8]>, src: &[u8]) -> Result<(), GuestError>;
    fn write<T>(&mut self, ptr: GuestPtr<T>, value: T) -> Result<(), GuestError>
    where
        T: GuestType,
    {
        T::write(self, ptr, value)
    }
}

pub trait GuestMemory: Memory + MemoryMut {}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq)]
pub struct GuestPtr<T: ?Sized + Pointee> {
    pointer: T::Pointer,
}

impl<T: ?Sized + Pointee> GuestPtr<T> {
    pub fn new(pointer: T::Pointer) -> Self {
        Self { pointer }
    }

    pub fn offset(&self) -> T::Pointer {
        self.pointer
    }

    /// Casts this `GuestPtr` type to a different type.
    ///
    /// This is a safe method which is useful for simply reinterpreting the type
    /// parameter on this `GuestPtr`. Note that this is a safe method, where
    /// again there's no guarantees about alignment, validity, in-bounds-ness,
    /// etc of the returned pointer.
    pub fn cast<U>(&self) -> GuestPtr<U>
    where
        U: Pointee<Pointer = T::Pointer> + ?Sized,
    {
        GuestPtr::new(self.pointer)
    }

    /// Performs pointer arithmetic on this pointer, moving the pointer forward
    /// `amt` slots.
    ///
    /// This will either return the resulting pointer or `Err` if the pointer
    /// arithmetic calculation would overflow around the end of the address
    /// space.
    pub fn add(&self, amt: u32) -> Result<GuestPtr<T>, GuestError>
    where
        T: GuestType + Pointee<Pointer = u32>,
    {
        let offset = amt
            .checked_mul(T::guest_size())
            .and_then(|o| self.pointer.checked_add(o));
        let offset = match offset {
            Some(o) => o,
            None => return Err(GuestError::PtrOverflow),
        };
        Ok(GuestPtr::new(offset))
    }

    /// Returns a `GuestPtr` for an array of `T`s using this pointer as the
    /// base.
    pub fn as_array(&self, elems: u32) -> GuestPtr<[T]>
    where
        T: GuestType + Pointee<Pointer = u32>,
    {
        GuestPtr::new((self.pointer, elems))
    }
}

impl<T> GuestPtr<[T]> {
    pub fn offset_base(&self) -> u32 {
        self.pointer.0
    }

    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> u32 {
        self.pointer.1
    }

    pub fn iter(&self) -> impl ExactSizeIterator<Item = Result<GuestPtr<T>, GuestError>> + '_
    where
        T: GuestType,
    {
        let base = self.as_ptr();
        (0..self.len()).map(move |i| base.add(i))
    }

    pub fn as_ptr(&self) -> GuestPtr<T> {
        GuestPtr::new(self.pointer.0)
    }
}

impl GuestPtr<str> {
    pub fn as_bytes(&self) -> GuestPtr<[u8]> {
        GuestPtr::new((self.pointer.0, self.pointer.1))
    }
}

mod private {
    pub trait Sealed {}
    impl<T> Sealed for T {}
    impl<T> Sealed for [T] {}
    impl Sealed for str {}
}

pub trait Pointee: private::Sealed {
    #[doc(hidden)]
    type Pointer: Copy + PartialEq;
}

impl<T> Pointee for T {
    type Pointer = u32;
}

impl<T> Pointee for [T] {
    type Pointer = (u32, u32);
}

impl Pointee for str {
    type Pointer = (u32, u32);
}
