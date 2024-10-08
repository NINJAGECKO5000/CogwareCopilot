use crate::{
    fb_trait::{Color, FrameBufferInterface},
    mailbox::set_virtual_framebuffer_offset,
};
pub struct FrameBuffer {
    // this could be an array.
    pub framebuff: &'static mut [u32],
    pub width: u32,
    pub height: u32,
    pub pitch: u32,
    pub is_rgb: bool,
    pub is_brg: bool,
    /// crate::mailbox::FB_VIRTUAL_WIDTH
    pub fb_virtual_width: u32,
    /// Bits used by each pixel
    pub depth_bits: u32,
    pub current_index: u8,
}

impl FrameBufferInterface for FrameBuffer {
    fn raw_buffer(&mut self) -> &mut [u32] {
        let start = self.width() * self.current_height_offset();
        let end_of_buffer = start + self.single_screen_len();
        &mut self.framebuff[start..end_of_buffer]
    }

    fn width(&self) -> usize {
        self.width as usize
    }

    fn use_pixel(&mut self, x_usize: usize, y_usize: usize, color: Color) {
        let width = self.width();
        let slice_ptr = (&mut self.raw_buffer()[width * y_usize + x_usize..]).as_mut_ptr();
        unsafe {
            core::ptr::write_volatile(slice_ptr, color.rgb());
        }
    }

    fn clear_screen(&mut self) {
        let slice_ptr = (&mut self.raw_buffer()).as_mut_ptr();
        for i in 0..self.single_screen_len() {
            unsafe {
                core::ptr::write_volatile(slice_ptr.add(i), 0);
            }
        }
    }

    fn update(&mut self) {
        set_virtual_framebuffer_offset(self.current_index as u32 * self.height);
        self.current_index = Self::inverse(self.current_index);
    }
}

impl FrameBuffer {
    fn single_screen_len(&self) -> usize {
        (self.height * self.width) as usize
    }
    fn current_height_offset(&self) -> usize {
        self.height as usize * self.current_index as usize
    }
    fn inverse(index: u8) -> u8 {
        if index == 1 {
            0
        } else {
            1
        }
    }
}
