#[no_mangle]
pub extern "C" fn hello() -> u32 {
    // "hello from dylib: hello"
    100
}

#[no_mangle]
pub extern "C" fn add() -> u32 {
    // "hello from dylib: add"
    1
}
