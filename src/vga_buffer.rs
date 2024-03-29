use core::fmt::Write;

use lazy_static::lazy_static;
use volatile::Volatile;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: u8,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct VgaBuffer {
    /// TODO: Add volatile
    inner: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

impl VgaBuffer {
    fn print_char(&mut self, col: usize, c: u8, fcolor: Color, bcolor: Color) {
        self.inner[BUFFER_HEIGHT - 1][col].write(ScreenChar {
            ascii_character: c,
            color_code: ((bcolor as u8) << 4) | fcolor as u8,
        });
    }

    fn print_newline(&mut self) {
        for row in 0..BUFFER_HEIGHT - 1 {
            for col in 0..BUFFER_WIDTH {
                self.inner[row][col].write(self.inner[row + 1][col].read());
            }
        }

        for col in 0..BUFFER_WIDTH {
            self.inner[BUFFER_HEIGHT - 1][col] = Default::default();
        }
    }
}

pub struct VgaBufferWriter {
    col: usize,
    color: Color,
    buffer: &'static mut VgaBuffer,
}

impl Write for VgaBufferWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.bytes() {
            if c == b'\n' {
                self.buffer.print_newline();
                self.col = 0;
                continue;
            }

            if self.col == BUFFER_WIDTH {
                self.buffer.print_newline();
                self.col = 0;
            }

            self.buffer
                .print_char(self.col, c, self.color, Color::Black);
            self.col += 1;
        }

        Ok(())
    }
}

lazy_static! {
    pub static ref VGA_BUFFER_WRITER: spin::Mutex<VgaBufferWriter> =
        spin::Mutex::new(VgaBufferWriter {
            col: 0,
            color: Color::White,
            buffer: unsafe { &mut *(0xb8000 as *mut VgaBuffer) }
        });
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print($crate::vga_buffer::Color::White, format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        $crate::vga_buffer::_print($crate::vga_buffer::Color::Cyan, format_args!("[ TRACE ] "));
        $crate::println!($($arg)*);
    };
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::vga_buffer::_print($crate::vga_buffer::Color::LightBlue, format_args!("[ DEBUG ] "));
        $crate::println!($($arg)*);
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::vga_buffer::_print($crate::vga_buffer::Color::Green, format_args!("[ INFO  ] "));
        $crate::println!($($arg)*);
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::vga_buffer::_print($crate::vga_buffer::Color::Yellow, format_args!("[ WARN  ] "));
        $crate::println!($($arg)*);
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => (
        $crate::vga_buffer::_print($crate::vga_buffer::Color::Red, format_args!("[ ERROR ] "));
        $crate::println!($($arg)*);
    );
}

#[doc(hidden)]
pub fn _print(color: Color, args: core::fmt::Arguments) {
    x86_64::instructions::interrupts::without_interrupts(|| {
        let mut writer = VGA_BUFFER_WRITER.lock();
        writer.color = color;
        writer.write_fmt(args).unwrap();
    });
}

#[cfg(test)]
mod tests {
    use x86_64::instructions::interrupts;

    use crate::vga_buffer::{BUFFER_HEIGHT, BUFFER_WIDTH, VGA_BUFFER_WRITER};
    use core::fmt::Write;

    #[test_case]
    fn println_many() {
        for _ in 0..200 {
            println!("test_println_many output");
        }
    }

    #[test_case]
    fn println_output() {
        let s = "Some test string that fits on a single line";
        interrupts::without_interrupts(|| {
            let mut writer = VGA_BUFFER_WRITER.lock();
            writeln!(writer, "\n{s}").expect("writeln failed");
            for (i, c) in s.chars().enumerate() {
                let screen_char = writer.buffer.inner[BUFFER_HEIGHT - 2][i].read();
                assert_eq!(char::from(screen_char.ascii_character), c);
            }
        });
    }

    #[test_case]
    fn println_long_line() {
        let long_line = [b'A'; BUFFER_WIDTH + 10];
        let long_line = core::str::from_utf8(&long_line).unwrap();

        interrupts::without_interrupts(|| {
            let mut writer = VGA_BUFFER_WRITER.lock();
            writeln!(writer, "\n{long_line}").expect("writeln failed");
            let chars = long_line.chars().enumerate();
            for (i, c) in chars {
                let line_idx = if i < BUFFER_WIDTH {
                    BUFFER_HEIGHT - 3
                } else {
                    BUFFER_HEIGHT - 2
                };
                let screen_char = writer.buffer.inner[line_idx][i % BUFFER_WIDTH].read();
                assert_eq!(char::from(screen_char.ascii_character), c, "Index {i}");
            }
        });
    }
}
