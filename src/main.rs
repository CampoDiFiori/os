#![no_std] // don't link the Rust standard library
#![no_main] // disable the Rust-level entry point

mod vga_buffer;

use core::panic::PanicInfo;

#[no_mangle]
extern "C" fn _start() -> ! {
    trace!("Hello");
    debug!("Hello");
    info!("Hello");
    warn!("Hello");
    error!("Hello");
    loop {}
}

/// Function called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    error!("{info}");
    loop {}
}
