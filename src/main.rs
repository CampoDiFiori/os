#![no_std] // don't link the Rust standard library
#![no_main] // disable the Rust-level entry point
#![feature(custom_test_frameworks)]
#![test_runner(os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

#[no_mangle]
extern "C" fn _start() -> ! {
    #[cfg(test)]
    test_main();

    os::init();

    os::trace!("Hello");
    os::debug!("Hello");
    os::info!("Hello");
    os::warn!("Hello");
    os::error!("Hello");

    os::hlt_loop();
}

/// Function called on panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os::error!("Error: {info}");
    os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os::test_panic_handler(info)
}
