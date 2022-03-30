#[no_mangle]
pub extern "C" fn hello() -> u32 {
    100
}

#[no_mangle]
pub extern "C" fn add(x: f64, y: f64) -> f64 {
    x + y
}
