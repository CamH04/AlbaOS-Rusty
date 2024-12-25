#![no_std]
#![no_main]

use core::panic::PanicInfo;

// we have to define our own panic as we have disabled stack unwinding
// and are in a freestanding env
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static START_TEXT: &[u8] = b"AlbaOS Boots!";

// main have to be called _start in a non specified target
#[no_mangle] //dont mangle the name _start bcz the compiler /bootloader will struggle to find out
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;
    //iterates every char in string and prints char to vga buffer
    for (i, &byte) in START_TEXT.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xF; //0xF means white
        }
    }


    loop {}
}
