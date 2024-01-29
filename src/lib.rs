#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod io_ports;
pub mod serial;
pub mod vga_buffer;

use core::panic::PanicInfo;

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]");
    serial_println!("");
    serial_println!("Error: {}", info);
    serial_println!("");
    io_ports::exit_qemu(io_ports::QemuExitCode::Failed);
    loop {}
}

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

/// Entry point for `cargo test`
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
