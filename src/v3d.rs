use core::mem;

use crate::{
    info,
    mailbox::{
        self, send_message_sync, GET_CURRENT_CLOCK_RATE, GET_MAX_CLOCK_RATE, LAST_TAG,
        MBOX_REQUEST, SET_CLOCK_RATE,
    },
};
const RPI_IO_BASE_ADDR: usize = 0x3F00_0000; // Replace with actual base address
const V3D_OFFSET: usize = 0xc00000;
const V3D_IDENT0: usize = 0x000;

#[derive(Debug)]
pub enum V3DError {
    Init,
    Check,
    MaxClockRequest,
    CurrentClockRequest,
}

impl core::fmt::Display for V3DError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                V3DError::Init => "V3D Core Initialization failed",
                V3DError::Check => "V3D Check Failed",
                V3DError::MaxClockRequest => "Failed to request max clock speed",
                V3DError::CurrentClockRequest => "Failed to get current clock speed",
            }
        )
    }
}

impl core::error::Error for V3DError {}

pub fn init() -> Result<(), V3DError> {
    let message = max_gpu_clock_rate_message();
    send_message_sync(mailbox::Channel::PROP, &message).map_err(|_| V3DError::MaxClockRequest)?;

    let max_speed_hz = message[6];
    let ratecalc: f64 = max_speed_hz.into();
    info!(
        "Max clock speed for GPU CORE is : {:?}Ghz",
        ratecalc / 1_000_000_000.0
    );

    let mut ret = [0u32; 13];
    ret[0] = (13 * mem::size_of::<u32>()) as u32;
    ret[1] = 0;
    ret[2] = SET_CLOCK_RATE; //set clock
    ret[3] = 8;
    ret[4] = 8;
    ret[5] = 5; //channel
    ret[6] = max_speed_hz; //V3D Clock rate
    ret[2] = 0x00030012; // enable QPU
    ret[3] = 4;
    ret[4] = 4;
    ret[5] = 1;
    ret[6] = 0;
    ret[7] = 0;

    let transfer = mailbox::Message::new(ret);
    mailbox::send_message_sync(mailbox::Channel::PROP, &transfer).map_err(|_| V3DError::Init)?;
    info!("message: {:?}", transfer);
    check_v3d_ident0()?;
    info!("We Passed V3D Check!");

    let message2 = get_current_gpu_clock_rate_message();
    send_message_sync(mailbox::Channel::PROP, &message2)
        .map_err(|_| V3DError::CurrentClockRequest)?;

    let rate = message2[6];
    let ratecalc: f64 = rate.into();

    info!(
        "Rate Readback to check ARM CORE is: {:?}Ghz",
        ratecalc / 1_000_000_000.0
    );

    Ok(())
}

// The memory address for the V3D base address
pub fn get_v3d_ptr() -> *mut u32 {
    let address = RPI_IO_BASE_ADDR + V3D_OFFSET;
    address as *mut u32
}

pub fn check_v3d_ident0() -> Result<(), V3DError> {
    // unsafe {
    // Get the pointer to the V3D registers
    let v3d_ptr = get_v3d_ptr();
    info!("V3DPTR = {:?}", v3d_ptr);
    // Read the value at V3D_IDENT0 offset using volatile read
    let v3d_ident0_value = unsafe { core::ptr::read_volatile(v3d_ptr.add(V3D_IDENT0 / 4)) }; // Divide by 4 because u32 is 4 bytes
    info!("V3D IDENT0 VAL {:?}", v3d_ident0_value);
    // Check if the value matches 0x02443356
    if v3d_ident0_value != 0x02443356 {
        return Err(V3DError::Check);
    }
    // }
    Ok(())
}

const GET_CURRENT_CLOCK_RATE_MESSAGE_SIZE: usize = 9;
fn get_current_gpu_clock_rate_message() -> mailbox::Message<GET_CURRENT_CLOCK_RATE_MESSAGE_SIZE> {
    let mut ret = [0u32; GET_CURRENT_CLOCK_RATE_MESSAGE_SIZE];
    ret[0] = (GET_CURRENT_CLOCK_RATE_MESSAGE_SIZE * mem::size_of::<u32>()) as u32;
    ret[1] = MBOX_REQUEST;

    ret[2] = GET_CURRENT_CLOCK_RATE; // set clock rate
    ret[3] = 8; // value buffer size in bytes
    ret[4] = 8; // clock id
    ret[5] = 0x5; // rate in hz
    ret[6] = 0; // skip setting turbo
    ret[7] = LAST_TAG;
    mailbox::Message::new(ret)
}

const MAX_CLOCK_RATE_MESSAGE_SIZE: usize = 9;
fn max_gpu_clock_rate_message() -> mailbox::Message<MAX_CLOCK_RATE_MESSAGE_SIZE> {
    let mut ret = [0u32; MAX_CLOCK_RATE_MESSAGE_SIZE];
    ret[0] = (MAX_CLOCK_RATE_MESSAGE_SIZE * mem::size_of::<u32>()) as u32;
    ret[1] = MBOX_REQUEST;

    // tag:
    ret[2] = GET_MAX_CLOCK_RATE; // get serial number command
    ret[3] = 8; // value buffer size in bytes
    ret[4] = 8; // :b 31 clear: request, | b31 set: response b30-b0: value length in bytes

    ret[5] = 0x5; // clock id
    ret[6] = 0; // used by the response.
    ret[7] = LAST_TAG;
    mailbox::Message::new(ret)
}
