#![no_std]
#![no_main]

use core::panic::PanicInfo;
use os::{serial_print, serial_println};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_fail();
    serial_println!("[test did not panic]");
    os::io_ports::exit_qemu(os::io_ports::QemuExitCode::Failed);
    loop {}
}

fn should_fail() {
    serial_print!("should_panic::should_fail...\t");
    assert_eq!(0, 1);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    os::io_ports::exit_qemu(os::io_ports::QemuExitCode::Success);
    loop {}
}
