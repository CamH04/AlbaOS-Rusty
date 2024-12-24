#![no_std]
#![no_main]

use core::panic::PanicInfo;

// we have to define our own panic as we have disabled stack unwinding
// and are in a freestanding env
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// main have to be called _start in a frestanding env
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
