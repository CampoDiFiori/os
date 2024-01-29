#![no_std] // don't link the Rust standard library
#![no_main] // disable the Rust-level entry point
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod io_ports;
mod serial;
mod vga_buffer;

use core::panic::PanicInfo;

#[no_mangle]
extern "C" fn _start() -> ! {
    #[cfg(test)]
    test_main();

    trace!("Hello");
    debug!("Hello");
    info!("Hello");
    warn!("Hello");
    error!("Hello");
    loop {}
}

/// Function called on panic
#[panic_handler]
#[cfg(not(test))]
fn panic(info: &PanicInfo) -> ! {
    error!("Error: {info}");
    loop {}
}

/// Function called on panic in test context
#[panic_handler]
#[cfg(test)]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]");
    serial_println!("");
    serial_println!("Error: {}", info);
    serial_println!("");
    io_ports::exit_qemu(io_ports::QemuExitCode::Failed);
    loop {}
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }

    io_ports::exit_qemu(io_ports::QemuExitCode::Success);
}

pub trait Testable {
    fn run(&self);
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("Running {}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
