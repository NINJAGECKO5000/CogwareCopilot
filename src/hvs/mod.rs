mod displaylist;
mod plane;

use core::mem;

use alloc::vec::Vec;
use bcm2837_hal::pac::VCMAILBOX;
use displaylist::DisplayList;
pub use plane::*;

pub const SCREEN_HEIGHT: u32 = 480;
pub const SCREEN_WIDTH: u32 = 480;

const LFB_MESSAGE_SIZE: usize = 35;
/// Set physical (display) width/height
const FB_PHYSICAL_WH_TAG: u32 = 0x00048003;
/// Width of the requested frame buffer
const FB_PHYSICAL_WIDTH: u32 = SCREEN_WIDTH;
/// Height of the requested frame buffer
const FB_PHYSICAL_HEIGHT: u32 = SCREEN_HEIGHT;

pub const FB_BUFFER_LEN: usize = FB_PHYSICAL_HEIGHT as usize * FB_PHYSICAL_WIDTH as usize;

/// Set virtual (buffer) width/height
const FB_VIRTUAL_WH_TAG: u32 = 0x00048004;
const FB_VIRTUAL_WIDTH: u32 = SCREEN_WIDTH;
const FB_VIRTUAL_HEIGHT: u32 = SCREEN_HEIGHT * 2;

pub const TOTAL_FB_BUFFER_LEN: usize = FB_VIRTUAL_HEIGHT as usize * FB_VIRTUAL_WIDTH as usize;

const FB_VIRTUAL_OFFSET_TAG: u32 = 0x48009;
const FB_VIRTUAL_OFFSET_X: u32 = 0;
const FB_VIRTUAL_OFFSET_Y: u32 = 0;

pub struct FrameBufferData {}

#[derive(Default)]
pub struct Hvs {
    planes: Vec<Plane>,
    display_list: DisplayList,
}

impl Hvs {
    pub fn new(mailbox: &VCMAILBOX) -> Hvs {
        Hvs::default()
    }

    fn init() {}

    /// Add a new plane to the display list.
    ///
    /// NOTE: The order in which planes are added will determine the order they are drawn to the
    /// display. Planes added later will be drawn on top of planes added before.
    pub fn add_plane(&mut self, plane: Plane) {
        self.planes.push(plane);
    }

    pub fn draw(&mut self) {
        self.display_list.write_planes(&self.planes)
    }
}

const fn lfb_message() -> [u32; LFB_MESSAGE_SIZE] {
    let mut msg = [0u32; LFB_MESSAGE_SIZE];
    msg[0] = (LFB_MESSAGE_SIZE * mem::size_of::<u32>()) as u32;
    msg[1] = 0; // says it's a request?
    msg[2] = FB_PHYSICAL_WH_TAG;
    msg[3] = 8;
    msg[4] = 8;
    msg[5] = FB_PHYSICAL_WIDTH;
    msg[6] = FB_PHYSICAL_HEIGHT;

    // set virt wh
    msg[7] = FB_VIRTUAL_WH_TAG;
    msg[8] = 8;
    msg[9] = 8;
    // FrameBufferInfo.virtual_width
    msg[10] = FB_VIRTUAL_WIDTH;
    // FrameBufferInfo.virtual_height
    msg[11] = FB_VIRTUAL_HEIGHT;

    // set virt offset
    msg[12] = FB_VIRTUAL_OFFSET_TAG;
    msg[13] = 8;
    msg[14] = 8;
    msg[15] = FB_VIRTUAL_OFFSET_X;
    msg[16] = FB_VIRTUAL_OFFSET_Y;

    msg[17] = 0x48005; // set depth
    msg[18] = 4;
    msg[19] = 4;
    msg[20] = 32; // FrameBufferInfo.depth

    msg[21] = 0x48006; // set pixel order
    msg[22] = 4;
    msg[23] = 4;
    msg[24] = 1; // RGB, not BGR preferably

    msg[25] = 0x40001; // Allocate buffer
    msg[26] = 8;
    msg[27] = 8;
    msg[28] = 4096; // FrameBufferInfo.pointer
    msg[29] = 0; // FrameBufferInfo.size

    msg[30] = 0x40008; // get pitch
    msg[31] = 4;
    msg[32] = 4;
    msg[33] = 0; // FrameBufferInfo.pitch

    msg[34] = LAST_TAG;

    todo!()
}
