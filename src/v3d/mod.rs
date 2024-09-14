use core::mem;

use crate::{
    info,
    mailbox::{self, send_message_sync, MailboxTag, LAST_TAG, MBOX_REQUEST},
};
const RPI_IO_BASE_ADDR: usize = 0x3F00_0000; // Replace with actual base address
const V3D_OFFSET: usize = 0xc00000;
const V3D_BASE_PTR: *mut u32 = (RPI_IO_BASE_ADDR + V3D_OFFSET) as *mut u32;

mod message;
mod registers;

pub use message::*;
pub use registers::V3DRegisters;

// Flags for allocate memory.
/*enum MemFlag {
    MEM_FLAG_DISCARDABLE = 1 << 0, /* can be resized to 0 at any time. Use for cached data */
    MEM_FLAG_NORMAL = 0 << 2,      /* normal allocating alias. Don't use from ARM */
    MEM_FLAG_DIRECT = 1 << 2,      /* 0xC alias uncached */
    MEM_FLAG_COHERENT = 2 << 2,    /* 0x8 alias. Non-allocating in L2 but coherent */
    MEM_FLAG_L1_NONALLOCATING = (MEM_FLAG_DIRECT | MEM_FLAG_COHERENT), /* Allocating in L2 */
    MEM_FLAG_ZERO = 1 << 4,        /* initialise buffer to all zeros */
    MEM_FLAG_NO_INIT = 1 << 5,     /* don't initialise (default is initialise to all ones */
    MEM_FLAG_HINT_PERMALOCK = 1 << 6, /* Likely to be locked for long periods of time. */
}

/* primitive type in the GL pipline */
enum PrimType {
    // is this needed?
    PRIM_POINT = 0,
    PRIM_LINE = 1,
    PRIM_LINE_LOOP = 2,
    PRIM_LINE_STRIP = 3,
    PRIM_TRIANGLE = 4,
    PRIM_TRIANGLE_STRIP = 5,
    PRIM_TRIANGLE_FAN = 6,
}
enum GLcommands {
    GL_HALT = 0,
    GL_NOP = 1,
    GL_FLUSH = 4,
    GL_FLUSH_ALL_STATE = 5,
    GL_START_TILE_BINNING = 6,
    GL_INCREMENT_SEMAPHORE = 7,
    GL_WAIT_ON_SEMAPHORE = 8,
    GL_BRANCH = 16,
    GL_BRANCH_TO_SUBLIST = 17,
    GL_RETURN_FROM_SUBLIST = 18,
    GL_STORE_MULTISAMPLE = 24,
    GL_STORE_MULTISAMPLE_END = 25,
    GL_STORE_FULL_TILE_BUFFER = 26,
    GL_RELOAD_FULL_TILE_BUFFER = 27,
    GL_STORE_TILE_BUFFER = 28,
    GL_LOAD_TILE_BUFFER = 29,
    GL_INDEXED_PRIMITIVE_LIST = 32,
    GL_VERTEX_ARRAY_PRIMITIVES = 33,
    GL_VG_COORDINATE_ARRAY_PRIMITIVES = 41,
    GL_COMPRESSED_PRIMITIVE_LIST = 48,
    GL_CLIP_COMPRESSD_PRIMITIVE_LIST = 49,
    GL_PRIMITIVE_LIST_FORMAT = 56,
    GL_SHADER_STATE = 64,
    GL_NV_SHADER_STATE = 65,
    GL_VG_SHADER_STATE = 66,
    GL_VG_INLINE_SHADER_RECORD = 67,
    GL_CONFIG_STATE = 96,
    GL_FLAT_SHADE_FLAGS = 97,
    GL_POINTS_SIZE = 98,
    GL_LINE_WIDTH = 99,
    GL_RHT_X_BOUNDARY = 100,
    GL_DEPTH_OFFSET = 101,
    GL_CLIP_WINDOW = 102,
    GL_VIEWPORT_OFFSET = 103,
    GL_Z_CLIPPING_PLANES = 104,
    GL_CLIPPER_XY_SCALING = 105,
    GL_CLIPPER_Z_ZSCALE_OFFSET = 106,
    GL_TILE_BINNING_CONFIG = 112,
    GL_TILE_RENDER_CONFIG = 113,
    GL_CLEAR_COLORS = 114,
    GL_TILE_COORDINATES = 115,
}

impl Deref for GLcommands {
    type Target = u32;

    fn deref(&self) -> Self::Target {
        self as u32
    }
}

*/
//commented for now since not being used, but here when needed
pub fn init() -> Result<(), V3DError> {
    let message = max_gpu_clock_rate_message();
    send_message_sync(mailbox::Channel::PROP, &message).map_err(|_| V3DError::MaxClockRequest)?;
    //let message = message.clone();
    let rate = message.get_idx(6);
    // let rate2 = *rate;
    // info!("R: {:?}", rate);
    info!(
        "Max clock speed for GPU CORE is: {:?}Mhz",
        rate as f64 / 1_000_000.0 // rate2 as f64 / 1_000_000.0
    );
    let mut ret = [0u32; 13];
    ret[0] = (13 * mem::size_of::<u32>()) as u32;
    ret[1] = 0;
    ret[2] = MailboxTag::SetClockRate; //set clock
    ret[3] = 8;
    ret[4] = 8;
    ret[5] = 5; //channel
    ret[6] = rate; //V3D Clock rate
                   // ret[6] = rate2; //V3D Clock rate
    ret[2] = 0x00030012; // enable QPU
    ret[3] = 4;
    ret[4] = 4;
    ret[5] = 1;
    ret[6] = 0;
    ret[7] = 0;

    let transfer = mailbox::Message::new(ret);
    mailbox::send_message_sync(mailbox::Channel::PROP, &transfer).map_err(|_| V3DError::Init)?;
    check_v3d_ident0()?;
    info!("We Passed V3D Check!");

    let message2 = get_current_gpu_clock_rate_message();
    send_message_sync(mailbox::Channel::PROP, &message2)
        .map_err(|_| V3DError::CurrentClockRequest)?;
    // let message2 = message2.clone();
    let rate = message2.get_idx(6);
    // let rate2 = *rate;
    // info!("R: {:?}", rate);
    info!(
        "Rate Readback to check GPU CORE is: {:?}Mhz",
        rate as f64 / 1_000_000.0 // rate2 as f64 / 1_000_000.0
    );

    Ok(())
}

// The memory address for the V3D base address
const fn get_v3d_ptr() -> *mut u32 {
    V3D_BASE_PTR
}

pub fn check_v3d_ident0() -> Result<(), V3DError> {
    // unsafe {
    // Get the pointer to the V3D registers
    let v3d_ptr = get_v3d_ptr();
    // Read the value at V3D_IDENT0 offset using volatile read
    let v3d_ident0_value =
        unsafe { core::ptr::read_volatile(v3d_ptr.add(V3DRegisters::Ident0 as usize / 4)) }; // Divide by 4 because u32 is 4 bytes
                                                                                             // Check if the value matches 0x02443356
    if v3d_ident0_value != 0x02443356 {
        return Err(V3DError::Check);
    }
    // }
    Ok(())
}

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
