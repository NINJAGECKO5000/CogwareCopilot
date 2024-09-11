use crate::{
    info,
    mailbox::{self, SET_CLOCK_RATE},
};
const RPI_IO_BASE_ADDR: usize = 0x3F00_0000; // Replace with actual base address
const V3D_OFFSET: usize = 0xc00000;
const V3D_IDENT0: usize = 0x000;

#[derive(Debug)]
pub enum V3DError {
    InitFailed,
    CheckFailed,
}

impl core::fmt::Display for V3DError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                V3DError::InitFailed => "V3D Core Initialization failed!",
                V3DError::CheckFailed => "V3D Check Failed!",
            }
        )
    }
}

impl core::error::Error for V3DError {}

pub fn init() -> Result<(), V3DError> {
    let mut ret = [0u32; 13];
    ret[0] = (13 * core::mem::size_of::<u32>()) as u32;
    ret[1] = 0;
    ret[2] = SET_CLOCK_RATE; //set clock
    ret[3] = 8;
    ret[4] = 8;
    ret[5] = 5; //channel
    ret[6] = 250_000_000; //V3D Clock rate
    ret[2] = 0x00030012; // enable QPU
    ret[3] = 4;
    ret[4] = 4;
    ret[5] = 1;
    ret[6] = 0;
    ret[7] = 0;
    let transfer = mailbox::Message::new(ret);
    mailbox::send_message_sync(mailbox::Channel::PROP, &transfer)
        .map_err(|_| V3DError::InitFailed)?;
    // if mailbox::send_message_sync(mailbox::Channel::PROP, &transfer) {
    info!("message: {:?}", transfer);
    check_v3d_ident0()?;
    // if check_v3d_ident0() {
    info!("We Passed V3D Check!");
    //     return true;
    // } else {
    //     info!("V3D check FAILED!!");
    //     return false;
    // }
    // } else {
    //     info!("Failed to sending message to init V3D");
    //     return false;
    // }

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
        return Err(V3DError::CheckFailed);
    }
    // }
    Ok(())
}
