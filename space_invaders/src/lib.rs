#![feature(let_chains)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "no_std", feature(format_args_nl))]
#![warn(clippy::pedantic)]

extern crate core;

mod framebuffer;
mod time;

use log::debug;

use crate::framebuffer::color;
pub use crate::framebuffer::fb_trait::FrameBufferInterface;
use crate::framebuffer::fb_trait::LETTER_WIDTH;
pub use crate::framebuffer::{Color, Coordinates};
pub use crate::time::TimeManagerInterface;
use core::fmt::Debug;
// use crate::{Color, Coordinates, FrameBufferInterface};
use core::mem;

pub const SCREEN_WIDTH: u32 = 480;
pub const SCREEN_WIDTH_NO_MARGIN: u32 = SCREEN_WIDTH - SCREEN_MARGIN;
pub const SCREEN_HEIGHT: u32 = 480;
pub const SCREEN_HEIGHT_NO_MARGIN: u32 = SCREEN_HEIGHT - SCREEN_MARGIN;
pub const SCREEN_MARGIN: u32 = 20;
pub const UI_SCORE_COLOR: Color = color::WHITE_COLOR;

// todo: in STD, if FPS is very low (i.e. no sleep at the end of the loop) enemies are stopped
// because the speedup rounds to 0.

#[macro_use]
mod macros {
    #[repr(C)] // guarantee 'bytes' comes after '_align'
    pub struct AlignedAs<Align, Bytes: ?Sized> {
        pub _align: [Align; 0],
        pub bytes: Bytes,
    }
    #[macro_export]
    macro_rules! include_bytes_align_as {
        ($align_ty:ty, $path:literal) => {{
            // const block expression to encapsulate the static
            use $crate::macros::AlignedAs;

            // this assignment is made possible by CoerceUnsized
            static ALIGNED: &AlignedAs<$align_ty, [u8]> = &AlignedAs {
                _align: [],
                bytes: *include_bytes!($path),
            };

            let as_u8 = &ALIGNED.bytes;
            // safety: the alignment is guaranteed by the above const block expression
            unsafe { core::slice::from_raw_parts(as_u8.as_ptr().cast::<u32>(), as_u8.len() / 4) }
        }};
    }
}
pub fn run_test<F, D>(mut fb: F, something: D)
where
    F: FrameBufferInterface,
    D: Debug,
{
    loop {
        fb.clear_screen();
        draw(&mut fb, &something);
        fb.update();
    }
}

fn draw(fb: &mut impl FrameBufferInterface, something: &impl Debug) {
    let mut message_buf = [0u8; 12 * mem::size_of::<char>()];
    let text =
        format_to_buffer(&mut message_buf, something).expect("Failed to format message to buffer");

    let mut x = (SCREEN_WIDTH / 2) - 60;
    let y = (SCREEN_HEIGHT / 2) - 10;
    for c in text.chars() {
        // right distance after each character
        x += LETTER_WIDTH as u32;
        fb.write_char(c, Coordinates::new(x, y), UI_SCORE_COLOR);
    }
}

fn format_to_buffer<'a>(
    buffer: &'a mut [u8],
    something: &'a impl Debug,
) -> Result<&'a str, core::fmt::Error> {
    use core::fmt::Write;
    let mut output = BufferWrite::new(buffer);
    write!(output, "{:?}", something)?;

    // Convert the buffer slice into a &str
    let written_length = output.written_length();
    let formatted_str = core::str::from_utf8(&buffer[..written_length]).unwrap();
    Ok(formatted_str)
}

// A custom implementation of core::fmt::Write for writing into a buffer
struct BufferWrite<'a> {
    buffer: &'a mut [u8],
    position: usize,
}

impl<'a> BufferWrite<'a> {
    fn new(buffer: &'a mut [u8]) -> Self {
        BufferWrite {
            buffer,
            position: 0,
        }
    }

    // Get the total number of bytes written so far
    fn written_length(&self) -> usize {
        self.position
    }
}

// Implement the Write trait for BufferWrite
impl<'a> core::fmt::Write for BufferWrite<'a> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let bytes = s.as_bytes();
        let remaining_space = self.buffer.len() - self.position;

        if bytes.len() <= remaining_space {
            self.buffer[self.position..self.position + bytes.len()].copy_from_slice(bytes);
            self.position += bytes.len();
            Ok(())
        } else {
            Err(core::fmt::Error)
        }
    }
}
