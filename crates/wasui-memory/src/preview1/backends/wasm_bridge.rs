use crate::preview1::{GuestError, GuestMemory, GuestPtr, Memory, MemoryMut};
use wasm_bridge::Store;

struct DummyContext;

impl wasm_bridge::AsContext for DummyContext {
    type Data = ();
    fn as_context(&self) -> &Store<Self::Data> {
        unreachable!()
    }
}

impl wasm_bridge::AsContextMut for DummyContext {
    fn as_context_mut(&mut self) -> &mut Store<Self::Data> {
        unreachable!()
    }
}

impl Memory for wasm_bridge::Memory {
    fn copy_to_slice(&self, ptr: GuestPtr<[u8]>, dst: &mut [u8]) -> Result<(), GuestError> {
        wasm_bridge::Memory::read(self, DummyContext {}, ptr.offset_base() as usize, dst)
            .map_err(|_| GuestError::InvalidMemory)
    }
}

impl MemoryMut for wasm_bridge::Memory {
    fn copy_from_slice(&mut self, ptr: GuestPtr<[u8]>, src: &[u8]) -> Result<(), GuestError> {
        wasm_bridge::Memory::write(self, DummyContext {}, ptr.offset_base() as usize, src)
            .map_err(|_| GuestError::InvalidMemory)
    }
}

impl GuestMemory for wasm_bridge::Memory {}
