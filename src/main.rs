
#![no_std] // unlink std libaray
#![no_main] // remove rust level entry point

use core::panic::PanicInfo;
mod vga_buffer;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle] // dont mangle name of the function
pub extern "C" fn _start() -> ! {
    vga_buffer::hello_world();

    loop {}
}



