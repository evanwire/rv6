#![no_std] // Avoids linking to standard lib
#![no_main] // Overwrite entry point

use core::panic::PanicInfo;

// Overwriting the OS entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}