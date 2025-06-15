use wasui_memory::preview1::{GuestPtr, Memory, MemoryMut};

#[test]
fn read_and_write_u32() {
    let mut memory = vec![0u8; 8];
    let ptr = GuestPtr::<u32>::new(0);

    memory.write(ptr, 0x12345678).expect("write should succeed");
    let value = memory.read(ptr).expect("read should succeed");

    assert_eq!(value, 0x12345678);
}

#[test]
fn out_of_bounds_fails() {
    let memory = vec![0u8; 4];
    let ptr = GuestPtr::<u32>::new(2);

    assert!(memory.read(ptr).is_err());
}
