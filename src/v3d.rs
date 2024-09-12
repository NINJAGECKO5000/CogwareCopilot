use core::mem;

use crate::{
    info,
    mailbox::{self, send_message_sync, MailboxTag, LAST_TAG, MBOX_REQUEST},
};
const RPI_IO_BASE_ADDR: usize = 0x3F00_0000; // Replace with actual base address
const V3D_OFFSET: usize = 0xc00000;

#[allow(dead_code)]
#[repr(u32)]
pub enum V3DRegisters {
    /// V3D Identification 0 (V3D block identity)
    Ident0 = 0x000,
    /// V3D Identification 1 (V3D Configuration A)
    Ident1 = 0x004,
    /// V3D Identification 1 (V3D Configuration B)
    Ident2 = 0x008,

    /// Scratch Register
    Scratch = 0x010,

    /// 2 Cache Control
    L2CacheCtrl = 0x020,
    /// Slices Cache Control
    SliceCacheCtrl = 0x024,

    /// Interrupt Control
    InterruptCtrl = 0x030,
    /// Interrupt Enables
    InterruptEnable = 0x034,
    /// Interrupt Disables
    InterruptDisable = 0x038,

    /// Control List Executor Thread 0 Control and Status.
    ControlList0CS = 0x100,
    /// Control List Executor Thread 1 Control and Status.
    ControlList1CS = 0x104,
    /// Control List Executor Thread 0 End Address.
    ControlList0EA = 0x108,
    /// Control List Executor Thread 1 End Address.
    ControlList1EA = 0x10c,
    /// Control List Executor Thread 0 Current Address.
    ControlList0CA = 0x110,
    /// Control List Executor Thread 1 Current Address.
    ControlList1CA = 0x114,
    /// Control List Executor Thread 0 Return Address.
    ControlList00RA0 = 0x118,
    /// Control List Executor Thread 1 Return Address.
    ControlList01RA0 = 0x11c,
    /// Control List Executor Thread 0 List Counter
    ControlList0LC = 0x120,
    /// Control List Executor Thread 1 List Counter
    ControlList1LC = 0x124,
    /// Control List Executor Thread 0 Primitive List Counter
    ControlList0PC = 0x128,
    /// Control List Executor Thread 1 Primitive List Counter
    ControlList1PC = 0x12c,

    /// V3D Pipeline Control and Status
    PipelineCS = 0x130,
    /// Binning Mode Flush Count
    BinningFlushCnt = 0x134,
    /// Rendering Mode Frame Count
    RenderFrameCnt = 0x138,

    /// Current Address of Binning Memory Pool
    BinningMemPool = 0x300,
    /// Remaining Size of Binning Memory Pool
    FreeBinningMemPool = 0x304,
    /// Address of Overspill Binning Memory Block
    BinningOverspill = 0x308,
    /// Size of Overspill Binning Memory Block
    BinningOverspillSize = 0x30c,
    /// Binner Debug
    BinnerDebug = 0x310,

    /// Reserve QPUs 0-7
    ReserveQpuBank0 = 0x410,
    /// Reserve QPUs 8-15
    ReserveQpuBank1 = 0x414,
    /// QPU Scheduler Control
    QpuSchedCtrl = 0x418,

    // these are awful and should be probably broken out into their own enum to keep their names
    // from being a novel
    /// QPU User Program Request Program Address
    QpuUserProgReqProgAddr = 0x430,
    /// QPU User Program Request Uniforms Address
    QpuUserProgReqUniformsAddr = 0x434,
    /// QPU User Program Request Uniforms Length
    QpuUserProgReqUniformsLen = 0x438,
    /// QPU User Program Request Control and Status
    QpuUserProgReqCS = 0x43c,

    /// VPM Allocator Control
    VpmAllocCtrl = 0x500,
    /// VPM base (user) memory reservation
    VpmBase = 0x504,

    /// Performance Counter Clear
    PerfCntrClr = 0x670,
    /// Performance Counter Enables
    PerfCntrEnable = 0x674,

    /// Performance Counter Count 0
    PerfCntrCnt0 = 0x680,
    /// Performance Counter Mapping 0
    PerfCntrMap0 = 0x684,
    /// Performance Counter Count 1
    PerfCntrCnt1 = 0x688,
    /// Performance Counter Mapping 1
    PerfCntrMap1 = 0x68c,
    /// Performance Counter Count 2
    PerfCntrCnt2 = 0x690,
    /// Performance Counter Mapping 2
    PerfCntrMap2 = 0x694,
    /// Performance Counter Count 3
    PerfCntrCnt3 = 0x698,
    /// Performance Counter Mapping 3
    PerfCntrMap3 = 0x69c,
    /// Performance Counter Count 4
    PerfCntrCnt4 = 0x6a0,
    /// Performance Counter Mapping 4
    PerfCntrMap4 = 0x6a4,
    /// Performance Counter Count 5
    PerfCntrCnt5 = 0x6a8,
    /// Performance Counter Mapping 5
    PerfCntrMap5 = 0x6ac,
    /// Performance Counter Count 6
    PerfCntrCnt6 = 0x6b0,
    /// Performance Counter Mapping 6
    PerfCntrMap6 = 0x6b4,
    /// Performance Counter Count 7
    PerfCntrCnt7 = 0x6b8,
    /// Performance Counter Mapping 7
    PerfCntrMap7 = 0x6bc,
    /// Performance Counter Count 8
    PerfCntrCnt8 = 0x6c0,
    /// Performance Counter Mapping 8
    PerfCntrMap8 = 0x6c4,
    /// Performance Counter Count 9
    PerfCntrCnt9 = 0x6c8,
    /// Performance Counter Mapping 9
    PerfCntrMap9 = 0x6cc,
    /// Performance Counter Count 10
    PerfCntrCnt10 = 0x6d0,
    /// Performance Counter Mapping 10
    PerfCntrMap10 = 0x6d4,
    /// Performance Counter Count 11
    PerfCntrCnt11 = 0x6d8,
    /// Performance Counter Mapping 11
    PerfCntrMap11 = 0x6dc,
    /// Performance Counter Count 12
    PerfCntrCnt12 = 0x6e0,
    /// Performance Counter Mapping 12
    PerfCntrMap12 = 0x6e4,
    /// Performance Counter Count 13
    PerfCntrCnt13 = 0x6e8,
    /// Performance Counter Mapping 13
    PerfCntrMap13 = 0x6ec,
    /// Performance Counter Count 14
    PerfCntrCnt14 = 0x6f0,
    /// Performance Counter Mapping 14
    PerfCntrMap14 = 0x6f4,
    /// Performance Counter Count 15
    PerfCntrCnt15 = 0x6f8,
    /// Performance Counter Mapping 15
    PerfCntrMap15 = 0x6fc,

    /// PSE Error Signals
    PseErrors = 0xf00,
    /// FEP Overrun Error Signals
    FepOverrunErrors = 0xf04,
    /// FEP Interface Ready and Stall Signals, FEP Busy Signals
    FepInterfaceStatus = 0xf08,
    /// FEP Internal Ready Signals
    FepInternalReadySignals = 0xf0c,
    /// FEP Internal Stall Input Signals
    FepInternalStallSignals = 0xf10,

    /// Miscellaneous Error Signals = VPM, VDW, VCD, VCM, L2C)
    MiscErrors = 0xf20,
}

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

/* primitive typo\e in the GL pipline */
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
}*/
//commented for now since not being used, but here when needed

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
    let message = message.clone();
    let rate = &message[6];
    let rate2 = *rate;
    info!("R: {:?}", rate);
    info!(
        "Max clock speed for GPU CORE is: {:?}Mhz",
        rate2 as f64 / 1_000_000.0
    );
    let mut ret = [0u32; 13];
    ret[0] = (13 * mem::size_of::<u32>()) as u32;
    ret[1] = 0;
    ret[2] = MailboxTag::SetClockRate as u32; //set clock
    ret[3] = 8;
    ret[4] = 8;
    ret[5] = 5; //channel
    ret[6] = rate2; //V3D Clock rate
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
    let message2 = message2.clone();
    let rate = &message2[6];
    let rate2 = *rate;
    info!("R: {:?}", rate);
    info!(
        "Rate Readback to check GPU CORE is: {:?}Mhz",
        rate2 as f64 / 1_000_000.0
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

const GET_CURRENT_CLOCK_RATE_MESSAGE_SIZE: usize = 9;
fn get_current_gpu_clock_rate_message() -> mailbox::Message<GET_CURRENT_CLOCK_RATE_MESSAGE_SIZE> {
    let mut ret = [0u32; GET_CURRENT_CLOCK_RATE_MESSAGE_SIZE];
    ret[0] = (GET_CURRENT_CLOCK_RATE_MESSAGE_SIZE * mem::size_of::<u32>()) as u32;
    ret[1] = MBOX_REQUEST;

    ret[2] = MailboxTag::GetClockRate as u32; // set clock rate
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
    ret[2] = MailboxTag::GetMaxClockRate as u32; // get serial number command
    ret[3] = 8; // value buffer size in bytes
    ret[4] = 8; // :b 31 clear: request, | b31 set: response b30-b0: value length in bytes

    ret[5] = 0x5; // clock id
    ret[6] = 0; // used by the response.
    ret[7] = LAST_TAG;
    mailbox::Message::new(ret)
}
