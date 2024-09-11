use crate::framebuffer::FrameBuffer; // videocoremboxbase: 3F00B880 resp-successful: 0
use crate::{info, mailbox::ReqResp::ResponseSuccessful};
use core::ops::Deref;
use core::{mem, ops::BitAnd};
// use log::info;
// use space_invaders::{SCREEN_HEIGHT, SCREEN_WIDTH}; // we hard set these here for now, should
// really ask the HVS for the screen H and W

pub const SCREEN_HEIGHT: u32 = 480;
pub const SCREEN_WIDTH: u32 = 480;
// const ResponseSuccessful: u32 = 0;
const VIDEOCORE_MBOX_BASE: u32 = 0x3F00B880;

use alloc::fmt::format;
use alloc::string::String;
use tock_registers::{
    interfaces::{Readable, Writeable},
    registers::{ReadOnly, WriteOnly},
};

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

#[derive(Debug)]
pub enum MailboxError {
    SendMessage(String),
    SetVirtFB,
    SetClockSpeed,
    GetMaxSpeed,
    QuerySerial,
    LfbInit { addr: u32 },
}

impl core::fmt::Display for MailboxError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let msg = match self {
            MailboxError::SendMessage(c) => {
                &format(format_args!("Failed to send mailbox message: {}", c))
            }
            MailboxError::SetVirtFB => {
                "Failed to sending message to set virtual framebuffer offset."
            }
            MailboxError::GetMaxSpeed => "Failed to get max clock speed",
            MailboxError::LfbInit { addr } => &format(format_args!(
                "Something went wrong setting up lfb. lfb address: {}",
                addr
            )),
            MailboxError::SetClockSpeed => "Failed to sending message to set clock speed.",
            MailboxError::QuerySerial => "Failed to sending message to query the board serial.",
        };

        write!(f, "{}", msg)
    }
}

impl core::error::Error for MailboxError {}

// TODO: wrap into registers map lib
#[repr(C)]
struct RawMailbox {
    read: ReadOnly<u32>,
    _unused: u32,
    _unused2: u32,
    _unused3: u32,
    poll: u32,
    sender: u32,
    status: ReadOnly<u32>,
    config: u32,
    write: WriteOnly<u32>,
}

impl RawMailbox {
    pub(crate) fn get() -> &'static mut RawMailbox {
        let raw_mailbox_ptr = VIDEOCORE_MBOX_BASE as *mut RawMailbox;
        unsafe { &mut *raw_mailbox_ptr }
    }

    pub(crate) fn is_empty(&self) -> bool {
        let status = self.get_status();
        status & STATUS_EMPTY == STATUS_EMPTY
    }

    fn is_full(&self) -> bool {
        let status = self.get_status();
        status & STATUS_FULL == STATUS_FULL
    }

    pub(crate) fn get_read(&self) -> u32 {
        self.read.get()
    }

    pub(crate) fn write_address(&mut self, address: usize) {
        self.write.set(address as u32)
    }

    fn get_status(&self) -> u32 {
        self.status.get()
    }
}

const STATUS_FULL: u32 = 0x80000000;
const STATUS_EMPTY: u32 = 0x40000000;

impl RawMailbox {}

#[derive(Debug, Copy, Clone)]
enum ReqResp {
    ResponseSuccessful,
    ResponseError,
    Request,
}

impl PartialEq<Self> for ReqResp {
    fn eq(&self, other: &Self) -> bool {
        let other = *other as u32;
        (*self as u32).eq(&other)
    }
}

impl Eq for ReqResp {}

impl Into<u32> for ReqResp {
    fn into(self) -> u32 {
        use ReqResp::*;
        match self {
            Request => 0x00000000,
            ResponseSuccessful => 0x80000000,
            ResponseError => 0x80000001,
        }
    }
}
impl From<u32> for ReqResp {
    fn from(val: u32) -> Self {
        use ReqResp::*;
        match val {
            0x00000000 => Request,
            0x80000000 => ResponseSuccessful,
            _ => ResponseError,
        }
    }
}
pub const MBOX_REQUEST: u32 = 0;
pub const BOARD_SERIAL_REQ: u32 = 0x00010004;
pub const GET_MAX_CLOCK_RATE: u32 = 0x00030004;
pub const SET_CLOCK_RATE: u32 = 0x00038002;
pub const GET_CURRENT_CLOCK_RATE: u32 = 0x00030002;
pub const SET_VIRTUAL_BUFFER_OFFSET_TAG: u32 = 0x00048009;
pub const TEST_SET_VIRTUAL_BUFFER_OFFSET_TAG: u32 = 0x00044009;
pub const LAST_TAG: u32 = 0;

#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct Message<const T: usize>([u32; T]);

impl<const T: usize> Deref for Message<T> {
    type Target = [u32; T];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const T: usize> Message<T> {
    pub fn new(message: [u32; T]) -> Message<T> {
        Message(message)
    }

    pub fn response_status(&self) -> ReqResp {
        ReqResp::from(self.0[1])
    }

    pub fn is_response_successfull(&self) -> bool {
        self.response_status() == ResponseSuccessful
    }
}

pub fn query_board_serial() -> Result<u64, MailboxError> {
    info!("Preparing board message..");
    let message = board_serial_message();
    info!("Sending message to channel PROP: {:?}", message);

    send_message_sync(Channel::PROP, &message).map_err(|_| MailboxError::QuerySerial)?;
    // return if send_message_sync(Channel::PROP, &message) {
    info!("Serial number is: {:#04x}/{:#04x}", message[5], message[4]);
    let b = message[4].to_ne_bytes();
    let c = message[5].to_ne_bytes();
    let single = [b[0], b[1], b[2], b[3], c[0], c[1], c[2], c[3]];
    info!("Single: {:?}", single);
    //     Some(u64::from_ne_bytes(single))
    // } else {
    //     info!("Failed to sending message to query the board serial.");
    //     None
    // };
    Ok(u64::from_ne_bytes(single))
}

const fn lfb_message() -> Message<LFB_MESSAGE_SIZE> {
    let mut ret = [0u32; LFB_MESSAGE_SIZE];
    ret[0] = (LFB_MESSAGE_SIZE * mem::size_of::<u32>()) as u32;
    ret[1] = MBOX_REQUEST;
    // set phy width:
    ret[2] = FB_PHYSICAL_WH_TAG;
    ret[3] = 8;
    ret[4] = 8;
    // FrameBufferInfo.width
    ret[5] = FB_PHYSICAL_WIDTH;
    // FrameBufferInfo.height
    ret[6] = FB_PHYSICAL_HEIGHT;

    // set virt wh
    ret[7] = FB_VIRTUAL_WH_TAG;
    ret[8] = 8;
    ret[9] = 8;
    // FrameBufferInfo.virtual_width
    ret[10] = FB_VIRTUAL_WIDTH;
    // FrameBufferInfo.virtual_height
    ret[11] = FB_VIRTUAL_HEIGHT;

    // set virt offset
    ret[12] = FB_VIRTUAL_OFFSET_TAG;
    ret[13] = 8;
    ret[14] = 8;
    ret[15] = FB_VIRTUAL_OFFSET_X;
    ret[16] = FB_VIRTUAL_OFFSET_Y;

    ret[17] = 0x48005; // set depth
    ret[18] = 4;
    ret[19] = 4;
    ret[20] = 32; // FrameBufferInfo.depth

    ret[21] = 0x48006; // set pixel order
    ret[22] = 4;
    ret[23] = 4;
    ret[24] = 1; // RGB, not BGR preferably

    ret[25] = 0x40001; // Allocate buffer
    ret[26] = 8;
    ret[27] = 8;
    ret[28] = 4096; // FrameBufferInfo.pointer
    ret[29] = 0; // FrameBufferInfo.size

    ret[30] = 0x40008; // get pitch
    ret[31] = 4;
    ret[32] = 4;
    ret[33] = 0; // FrameBufferInfo.pitch

    ret[34] = LAST_TAG;
    Message(ret)
}

// pub fn lfb_init<'a: 'static>(tentative: usize) -> Result<FrameBuffer, MailboxError> {
pub fn lfb_init<'a: 'static>() -> Result<FrameBuffer, MailboxError> {
    let message = lfb_message();
    send_message_sync(Channel::PROP, &message)?;

    if message[28] == 0 {
        return Err(MailboxError::LfbInit { addr: message[28] });
    }

    // convert GPU address to ARM address
    let fb_ptr_raw = (message[28] & 0x3FFFFFFF) as usize;
    info!("fb_ptr_raw: {}", fb_ptr_raw);

    // get actual physical width
    let width = message[5];
    // get actual physical height
    let height = message[6];
    // get number of bytes per line:
    let pitch = message[33];
    // get the pixel depth TODO: is this correct? Missin from: https://github.com/bztsrc/raspi3-tutorial/blob/master/09_framebuffer/lfb.c
    let depth = message[20];
    // get the actual channel order. brg = 0, rgb > 0
    let is_rgb = message[24] != 0;

    let casted = fb_ptr_raw as *const u32 as *mut u32;
    let casted = unsafe { &mut *casted };
    let framebuff: &mut [u32] =
        unsafe { core::slice::from_raw_parts_mut(casted, TOTAL_FB_BUFFER_LEN) };
    let fb = FrameBuffer {
        framebuff,
        width,
        height,
        pitch,
        depth_bits: depth,
        is_rgb,
        is_brg: !is_rgb,
        fb_virtual_width: FB_VIRTUAL_WIDTH,
        current_index: 0,
    };
    info!(
            "All good, setting up the frame buffer now: {}, height: {}, pitch: {}, depth:{}, is_rgb: {}",
            width, height, pitch, depth, is_rgb
        );
    Ok(fb)
}

pub fn set_clock_speed(new_clock: u32) -> Result<(), MailboxError> {
    let message = get_set_clock_rate_message(new_clock);
    // info!(
    //    "Sending message to channel PROP to set clock speed: {:?}",
    //    message
    //);
    send_message_sync(Channel::PROP, &message).map_err(|_| MailboxError::SetClockSpeed)?;
    //  info!("message: {:?}", message);
    let rate = message[6];
    let ratecalc: f64 = rate.into();
    info!(
        "New rate for ARM CORE is: {:?}Ghz",
        ratecalc / 1_000_000_000.0
    );
    let message2 = get_current_clock_rate_message();

    send_message_sync(Channel::PROP, &message2).map_err(|_| MailboxError::SetClockSpeed)?;
    info!("message: {:?}", message2);
    let rate = message2[6];
    let ratecalc: f64 = rate.into();

    info!(
        "Rate Readback to check ARM CORE is: {:?}Ghz",
        ratecalc / 1_000_000_000.0
    );
    Ok(())
}

#[allow(non_snake_case)]
pub fn set_virtual_framebuffer_offset(offset: u32) -> Result<(), MailboxError> {
    let message = get_set_virtual_framebuffer_offset_message(offset);
    send_message_sync(Channel::PROP, &message).map_err(|_| MailboxError::SetVirtFB)?;

    Ok(())
}

pub fn test_set_virtual_framebuffer_offset(offset: u32) -> Result<(), MailboxError> {
    let message = get_test_virtual_fb_offset_message(offset);

    send_message_sync(Channel::PROP, &message).map_err(|_| MailboxError::SetVirtFB)?;

    let offset_x = message[5];
    let offset_y = message[6];
    info!(
        " requested offset: {} new offset: {}, y{}",
        offset, offset_x, offset_y
    );
    Ok(())
}

pub fn max_clock_speed() -> Result<u32, MailboxError> {
    // command 0x00030004 ARM clock ID = 0x3
    // BCM2835_MAILBOX_TAG_GET_MAX_CLOCK_RATE 0x00030004
    let message2 = get_current_clock_rate_message();
    // info!(
    //   "Sending message to channel PROP to read clock speed: {:?}",
    //   message2
    //);
    send_message_sync(Channel::PROP, &message2).map_err(|_| MailboxError::GetMaxSpeed)?;

    info!("message: {:?}", message2);
    let rate = message2[6];
    let ratecalc: f64 = rate.into();

    info!(
        "Current ARM CORE rate is: {:?}Ghz",
        ratecalc / 1_000_000_000.0
    );

    let message = max_clock_rate_message();

    send_message_sync(Channel::PROP, &message).map_err(|_| MailboxError::GetMaxSpeed)?;

    info!("message: {:?}", message);
    let max_speed_hz = message[6];
    let ratecalc: f64 = max_speed_hz.into();
    info!(
        "Max clock speed for ARM CORE is : {:?}Ghz",
        ratecalc / 1_000_000_000.0
    );
    Ok(max_speed_hz)
}

const SET_VIRTUAL_FRAMEBUFFER_OFFSET_MESSAGE_SIZE: usize = 8;
fn get_set_virtual_framebuffer_offset_message(
    offset_y: u32,
) -> Message<SET_VIRTUAL_FRAMEBUFFER_OFFSET_MESSAGE_SIZE> {
    let mut ret = [0u32; SET_VIRTUAL_FRAMEBUFFER_OFFSET_MESSAGE_SIZE];
    ret[0] = (SET_VIRTUAL_FRAMEBUFFER_OFFSET_MESSAGE_SIZE * mem::size_of::<u32>()) as u32;
    ret[1] = MBOX_REQUEST;
    ret[2] = SET_VIRTUAL_BUFFER_OFFSET_TAG; // set virtual buffer offset
    ret[3] = 2 * mem::size_of::<u32>() as u32; // value buffer size in bytes
    ret[4] = 0; // :b 31 clear: request, | b31 set: response b30-b0: value length in bytes
    ret[5] = 0; // x in pixels
    ret[6] = offset_y; // y in pixels
    ret[7] = LAST_TAG;
    Message(ret)
}

const TEST_SET_VIRTUAL_FRAMEBUFFER_OFFSET_MESSAGE_SIZE: usize = 8;
fn get_test_virtual_fb_offset_message(
    offset_y: u32,
) -> Message<TEST_SET_VIRTUAL_FRAMEBUFFER_OFFSET_MESSAGE_SIZE> {
    let mut ret = [0u32; TEST_SET_VIRTUAL_FRAMEBUFFER_OFFSET_MESSAGE_SIZE];
    ret[0] = (TEST_SET_VIRTUAL_FRAMEBUFFER_OFFSET_MESSAGE_SIZE * mem::size_of::<u32>()) as u32;
    ret[1] = MBOX_REQUEST;
    ret[2] = SET_VIRTUAL_BUFFER_OFFSET_TAG; // set virtual buffer offset
    ret[3] = 2 * mem::size_of::<u32>() as u32; // value buffer size in bytes
    ret[4] = 0; // :b 31 clear: request, | b31 set: response b30-b0: value length in bytes
    ret[5] = 0; // x in pixels
    ret[6] = offset_y; // y in pixels
    ret[7] = LAST_TAG;
    Message(ret)
}
const GET_CURRENT_CLOCK_RATE_MESSAGE_SIZE: usize = 9;
fn get_current_clock_rate_message() -> Message<GET_CURRENT_CLOCK_RATE_MESSAGE_SIZE> {
    let mut ret = [0u32; GET_CURRENT_CLOCK_RATE_MESSAGE_SIZE];
    ret[0] = (GET_CURRENT_CLOCK_RATE_MESSAGE_SIZE * mem::size_of::<u32>()) as u32;
    ret[1] = MBOX_REQUEST;

    ret[2] = GET_CURRENT_CLOCK_RATE; // set clock rate
    ret[3] = 8; // value buffer size in bytes
    ret[4] = 8; // clock id
    ret[5] = 0x3; // rate in hz
    ret[6] = 0; // skip setting turbo
    ret[7] = LAST_TAG;
    Message(ret)
}

const GET_CLOCK_RATE_MESSAGE_SIZE: usize = 10;
fn get_set_clock_rate_message(new_clock_hz: u32) -> Message<GET_CLOCK_RATE_MESSAGE_SIZE> {
    let mut ret = [0u32; GET_CLOCK_RATE_MESSAGE_SIZE];
    ret[0] = (GET_CLOCK_RATE_MESSAGE_SIZE * mem::size_of::<u32>()) as u32;
    ret[1] = MBOX_REQUEST;

    ret[2] = SET_CLOCK_RATE; // set clock rate
    ret[3] = 8; // value buffer size in bytes
    ret[4] = 8; // clock id
    ret[5] = 0x3; // rate in hz
    ret[6] = new_clock_hz; // skip setting turbo
    ret[7] = LAST_TAG;
    Message(ret)
}

/// rate in hz.
const MAX_CLOCK_RATE_MESSAGE_SIZE: usize = 9;
fn max_clock_rate_message() -> Message<MAX_CLOCK_RATE_MESSAGE_SIZE> {
    let mut ret = [0u32; MAX_CLOCK_RATE_MESSAGE_SIZE];
    ret[0] = (MAX_CLOCK_RATE_MESSAGE_SIZE * mem::size_of::<u32>()) as u32;
    ret[1] = MBOX_REQUEST;

    // tag:
    ret[2] = GET_MAX_CLOCK_RATE; // get serial number command
    ret[3] = 8; // value buffer size in bytes
    ret[4] = 8; // :b 31 clear: request, | b31 set: response b30-b0: value length in bytes

    ret[5] = 0x3; // clock id
    ret[6] = 0; // used by the response.
    ret[7] = LAST_TAG;
    Message(ret)
}

const SERIAL_MESSAGE_SIZE: usize = 9;
fn board_serial_message() -> Message<SERIAL_MESSAGE_SIZE> {
    const SERIAL_MESSAGE_TAG: u32 = 0x00010004;
    let mut ret = [0u32; SERIAL_MESSAGE_SIZE];
    ret[0] = (SERIAL_MESSAGE_SIZE * mem::size_of::<u32>()) as u32;
    ret[1] = MBOX_REQUEST;

    ret[2] = SERIAL_MESSAGE_TAG; // tag identifier
    ret[3] = 8; // value buffer size in bytes
    ret[4] = 8; // Request codes:b 31 clear: request
    ret[5] = 8; // clear output buffer
    ret[6] = 0;

    ret[7] = LAST_TAG;
    Message(ret)
}

pub fn send_message_sync<const T: usize>(
    channel: Channel,
    message: &Message<T>,
) -> Result<(), MailboxError> {
    let raw_ptr = message.as_ptr();
    // This is needed because slices are fat pointers and I need to convert it to a thin pointer
    // first.
    let raw_ptr_addr = raw_ptr.cast::<usize>();
    let raw_ptr_addr = raw_ptr_addr as usize;
    // !0x0F is 1...10000
    let addr_clear_last_4_bits = raw_ptr_addr.bitand(!0x0F);
    let ch_clear_everything_but_last_4_vits = channel as usize & 0xF;
    let final_addr = addr_clear_last_4_bits | ch_clear_everything_but_last_4_vits;

    let raw_mailbox_ptr = VIDEOCORE_MBOX_BASE as *mut RawMailbox;
    let raw_mailbox = unsafe { &mut *raw_mailbox_ptr };

    // wait until we can write to the mailbox
    while raw_mailbox.is_full() {
        core::hint::spin_loop();
    }

    raw_mailbox.write_address(final_addr);

    // now wait for the response
    loop {
        // is there a response?
        while raw_mailbox.is_empty() {
            core::hint::spin_loop();
        }

        if raw_mailbox.get_read() == final_addr as u32 {
            return match message.response_status() {
                ReqResp::Request => Err(MailboxError::SendMessage(String::from(
                    "Message still contains a request?!",
                ))),
                ReqResp::ResponseError => Err(MailboxError::SendMessage(String::from(
                    "Something failed, the response is an error",
                ))),
                ReqResp::ResponseSuccessful => Ok(()),
            };
        }
    }
}

fn mailbox_tag_message<const N: usize>(
    channel: Channel,
    buf: &[u32; N],
) -> Result<(), MailboxError> {
    let mut ret = [0; N];
    ret[0] = (ret.len() * mem::size_of::<u32>()) as u32;
    ret[ret.len() + 2] = 0;
    ret[1] = 0;
    for i in buf {
        let val = *i as usize;
        ret[val + 2] = buf[val];
    }
    let transfer = Message(ret);

    let raw_ptr = transfer.0.as_ptr();
    // This is needed because slices are fat pointers and I need to convert it to a thin pointer
    // first.
    let raw_ptr_addr = raw_ptr.cast::<usize>();
    let raw_ptr_addr = raw_ptr_addr as usize;
    // !0x0F is 1...10000
    let addr_clear_last_4_bits = raw_ptr_addr.bitand(!0x0F);
    let ch_clear_everything_but_last_4_vits = channel as usize & 0xF;
    let final_addr = addr_clear_last_4_bits | ch_clear_everything_but_last_4_vits;

    let raw_mailbox_ptr = VIDEOCORE_MBOX_BASE as *mut RawMailbox;
    let raw_mailbox = unsafe { &mut *raw_mailbox_ptr };

    // wait until we can write to the mailbox
    while raw_mailbox.is_full() {
        core::hint::spin_loop();
    }

    raw_mailbox.write_address(final_addr);

    // now wait for the response
    loop {
        // is there a response?
        while raw_mailbox.is_empty() {
            core::hint::spin_loop();
        }

        if raw_mailbox.get_read() == final_addr as u32 {
            return match transfer.response_status() {
                ReqResp::Request => Err(MailboxError::SendMessage(String::from(
                    "Message still contains a request?!",
                ))),
                ReqResp::ResponseError => Err(MailboxError::SendMessage(String::from(
                    "Something failed, the response is an error",
                ))),
                ReqResp::ResponseSuccessful => Ok(()),
            };
        }
    }
}

#[derive(Copy, Clone)]
pub enum Channel {
    POWER = 0,
    FB = 1,
    VUART = 2,
    VCHIQ = 3,
    LEDS = 4,
    BTNS = 5,
    TOUCH = 6,
    COUNT = 7,
    PROP = 8,
}
