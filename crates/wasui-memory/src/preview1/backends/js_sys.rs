use crate::preview1::{GuestError, GuestMemory, GuestPtr, Memory, MemoryMut};
use js_sys::wasm_bindgen::JsValue;
use js_sys::{Reflect, Uint8Array, WebAssembly};

trait AsBuffer {
    fn as_buffer(&self) -> Result<JsValue, GuestError>;
}

impl AsBuffer for WebAssembly::Memory {
    fn as_buffer(&self) -> Result<JsValue, GuestError> {
        thread_local! {
            static BUFFER_NAME: JsValue = "buffer".into();
        }

        BUFFER_NAME.with(|buffer_name| {
            Reflect::get(&self, buffer_name).map_err(|_| GuestError::InvalidMemory)
        })
    }
}

impl Memory for WebAssembly::Memory {
    fn copy_to_slice(&self, ptr: GuestPtr<[u8]>, dst: &mut [u8]) -> Result<(), GuestError> {
        let buffer = self.as_buffer()?;

        let mem = Uint8Array::new_with_byte_offset_and_length(
            &buffer,
            ptr.offset_base() as u32,
            ptr.len() as u32,
        );

        mem.copy_to(dst);

        Ok(())
    }
}

impl MemoryMut for WebAssembly::Memory {
    fn copy_from_slice(&mut self, ptr: GuestPtr<[u8]>, src: &[u8]) -> Result<(), GuestError> {
        let buffer = self.as_buffer()?;

        let mem = Uint8Array::new_with_byte_offset_and_length(
            &buffer,
            ptr.offset_base() as u32,
            ptr.len() as u32,
        );

        mem.copy_from(src);

        Ok(())
    }
}

impl GuestMemory for WebAssembly::Memory {}
