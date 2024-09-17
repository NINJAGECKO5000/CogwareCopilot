use alloc::string::String;
use mailbox::{MailboxError, MailboxTag, RawMailbox};

use crate::bail;

use super::*;

#[repr(C, align(16))]
#[derive(Debug, Copy, Clone)]
pub struct MailboxTagMessage<const PAYLOAD_SIZE: usize> {
    size: u32,
    status: u32,
    tag: MailboxTag,
    payload: [u32; PAYLOAD_SIZE],
}

#[derive(Debug)]
enum MailboxResponse {
    Request,
    Success,
    Error,
}

impl From<u32> for MailboxResponse {
    fn from(value: u32) -> Self {
        match value {
            0x00000000 => Self::Request,
            0x80000000 => Self::Success,
            _ => Self::Error,
        }
    }
}

pub struct MailboxMessage;

#[allow(dead_code, non_upper_case_globals)]
impl MailboxMessage {
    pub const CurrentArmClockRate: MailboxTagMessage<6> = MailboxTagMessage {
        size: (9 * mem::size_of::<u32>() as u32),
        status: 0,
        tag: MailboxTag::GetClockRate,
        payload: [8, 8, 0x3, 0, LAST_TAG, 0],
    };

    pub const MaxArmClockRate: MailboxTagMessage<6> = MailboxTagMessage {
        size: (9 * mem::size_of::<u32>() as u32),
        status: 0,
        tag: MailboxTag::GetMaxClockRate,
        payload: [8, 8, 0x3, 0, LAST_TAG, 0],
    };

    pub const CurrentGpuClockRate: MailboxTagMessage<6> = MailboxTagMessage {
        size: (9 * mem::size_of::<u32>() as u32),
        status: 0,
        tag: MailboxTag::GetClockRate,
        payload: [
            8, // value buffer size in bytes
            8, 0x5, // clock id
            0,
            0,
            LAST_TAG,// padding
        ],
    };

    pub const MaxGpuClockRate: MailboxTagMessage<6> = MailboxTagMessage {
        size: (9 * mem::size_of::<u32>() as u32),
        status: 0,
        tag: MailboxTag::GetMaxClockRate,
        payload: [
            8,   // ?
            8,   // ?
            0x5, // GPU Channel
            0,   // used by the response
            0,
            LAST_TAG,  // padding
        ],
    };

    // pub const Lfb: MailboxTagMessage<32> = MailboxTagMessage {
    //     size: (35 * mem::size_of::<u32>() as u32),
    //     status: 0,
    // }

    pub const fn mem_alloc(size: u32, align: u32, flags: u32) -> MailboxTagMessage<7> {
        MailboxTagMessage {
            size: (10 * mem::size_of::<u32>()) as u32,
            tag: MailboxTag::AllocateMemory,
            status: 0,
            payload: [12, 12, size, align, flags, 0, LAST_TAG],
        }
    }

    pub const fn mem_free(val: u32) -> MailboxTagMessage<4> {
        MailboxTagMessage {
            size: (7 * mem::size_of::<u32>()) as u32,
            tag: MailboxTag::ReleaseMemory,
            status: 0,
            payload: [4, 4, val, LAST_TAG],
        }
    }

    pub const fn mem_lock(val: u32) -> MailboxTagMessage<5> {
        MailboxTagMessage {
            size: (8 * size_of::<u32>()) as u32,
            tag: MailboxTag::LockMemory,
            status: 0,
            payload: [4, 4, val, 0, LAST_TAG],
        }
    }

    pub const fn mem_unlock(val: u32) -> MailboxTagMessage<4> {
        MailboxTagMessage {
            size: (7 * size_of::<u32>()) as u32,
            tag: MailboxTag::UnlockMemory,
            status: 0,
            payload: [4, 4, val, LAST_TAG],
        }
    }

    pub const fn execute_code(
        code: u32,
        r0: u32,
        r1: u32,
        r2: u32,
        r3: u32,
        r4: u32,
        r5: u32,
    ) -> MailboxTagMessage<10> {
        MailboxTagMessage {
            size: (13 * mem::size_of::<u32>()) as u32,
            status: 0,
            tag: MailboxTag::ExecuteCode,
            payload: [23, 23, code, r0, r1, r2, r3, r4, r5, LAST_TAG],
        }
    }

    pub const fn execute_qpu(
        num_qpus: u32,
        control: u32,
        noflush: u32,
        timeout: u32,
    ) -> MailboxTagMessage<7> {
        MailboxTagMessage {
            size: (10 * mem::size_of::<u32>()) as u32,
            status: 0,
            tag: MailboxTag::EnableQpu,
            payload: [16, 16, num_qpus, control, noflush, timeout, LAST_TAG],
        }
    }
}

#[allow(dead_code, non_upper_case_globals)]
impl<const TAGS: usize> MailboxTagMessage<TAGS> {
    pub const CurrentArmClockRate: MailboxTagMessage<6> = MailboxTagMessage {
        size: (9 * mem::size_of::<u32>() as u32),
        status: 0,
        tag: MailboxTag::GetClockRate,
        payload: [8, 8, 0x3, 0, LAST_TAG, 0],
    };

    pub const MaxArmClockRate: MailboxTagMessage<6> = MailboxTagMessage {
        size: (9 * mem::size_of::<u32>() as u32),
        status: 0,
        tag: MailboxTag::GetMaxClockRate,
        payload: [8, 8, 0x3, 0, LAST_TAG, 0],
    };

    pub const CurrentGpuClockRate: MailboxTagMessage<6> = MailboxTagMessage {
        size: (9 * mem::size_of::<u32>() as u32),
        status: 0,
        tag: MailboxTag::GetClockRate,
        payload: [
            8, // value buffer size in bytes
            8, 0x5, // clock id
            0, LAST_TAG, 0, // padding
        ],
    };

    pub const MaxGpuClockRate: MailboxTagMessage<6> = MailboxTagMessage {
        size: (9 * mem::size_of::<u32>() as u32),
        status: 0,
        tag: MailboxTag::GetMaxClockRate,
        payload: [
            8,   // ?
            8,   // ?
            0x5, // ?
            0,   // used by the response
            LAST_TAG, 0, // padding
        ],
    };

    // pub const Lfb: MailboxTagMessage<32> = MailboxTagMessage {
    //     size: (35 * mem::size_of::<u32>() as u32),
    //     status: 0,
    // }

    pub const fn mem_alloc(size: u32, align: u32, flags: u32) -> MailboxTagMessage<7> {
        MailboxTagMessage {
            size: (10 * mem::size_of::<u32>()) as u32,
            tag: MailboxTag::AllocateMemory,
            status: 0,
            payload: [12, 12, size, align, flags, 0, LAST_TAG],
        }
    }

    pub const fn mem_free(val: u32) -> MailboxTagMessage<4> {
        MailboxTagMessage {
            size: (7 * mem::size_of::<u32>()) as u32,
            tag: MailboxTag::ReleaseMemory,
            status: 0,
            payload: [4, 4, val, LAST_TAG],
        }
    }

    pub const fn mem_lock(val: u32) -> MailboxTagMessage<5> {
        MailboxTagMessage {
            size: (8 * size_of::<u32>()) as u32,
            tag: MailboxTag::LockMemory,
            status: 0,
            payload: [4, 4, val, 0, LAST_TAG],
        }
    }

    pub const fn mem_unlock(val: u32) -> MailboxTagMessage<4> {
        MailboxTagMessage {
            size: (7 * size_of::<u32>()) as u32,
            tag: MailboxTag::UnlockMemory,
            status: 0,
            payload: [4, 4, val, LAST_TAG],
        }
    }

    pub const fn execute_code(
        code: u32,
        r0: u32,
        r1: u32,
        r2: u32,
        r3: u32,
        r4: u32,
        r5: u32,
    ) -> MailboxTagMessage<10> {
        MailboxTagMessage {
            size: (13 * mem::size_of::<u32>()) as u32,
            status: 0,
            tag: MailboxTag::ExecuteCode,
            payload: [23, 23, code, r0, r1, r2, r3, r4, r5, LAST_TAG],
        }
    }

    pub const fn execute_qpu(
        num_qpus: u32,
        control: u32,
        noflush: u32,
        timeout: u32,
    ) -> MailboxTagMessage<7> {
        MailboxTagMessage {
            size: (10 * mem::size_of::<u32>()) as u32,
            status: 0,
            tag: MailboxTag::EnableQpu,
            payload: [16, 16, num_qpus, control, noflush, timeout, LAST_TAG],
        }
    }

    pub fn to_mailbox_addr(&self) -> u32 {
        // magic number 8 is the Prop channel on the mailbox
        //
        // we never send anything to the other mailboxes for tag messages so there's no reason to
        // take it in as a dynamic argument
        (((self as *const _ as *const () as usize) & !0x0F) | (8 as usize & 0xF)) as u32
    }

    pub fn send(self) -> Result<(), MailboxError> {
        self.send_sync()
    }

    pub fn send_and_read(self, resp_idx: usize) -> Result<u32, MailboxError> {
        self.send_sync()?;

        Ok(unsafe {
            core::ptr::read_volatile(match resp_idx {
                0 => &self.size,
                1 => &self.status,
                i if (resp_idx >= 2) && ((resp_idx - 2) < self.payload.len()) => {
                    &self.payload[i - 2]
                }
                _ => bail!(MailboxError::ReadResponse(String::from(
                    "index out of bounds"
                ))),
            } as *const u32)
        })
    }

    fn send_sync(&self) -> Result<(), MailboxError> {
        let raw_mailbox = RawMailbox::get();

        while raw_mailbox.is_full() {
            core::hint::spin_loop();
        }

        let addr = self.to_mailbox_addr();
        raw_mailbox.write_address(addr);

        loop {
            while raw_mailbox.is_empty() {
                core::hint::spin_loop();
            }

            if raw_mailbox.get_read() == addr {
                return match MailboxResponse::from(self.status) {
                    MailboxResponse::Request => Err(MailboxError::SendMessage(String::from(
                        "Message still contains a request?!",
                    ))),
                    MailboxResponse::Success => Ok(()),
                    MailboxResponse::Error => Err(MailboxError::SendMessage(String::from(
                        "Something failed, the response is an error",
                    ))),
                };
            }
        }
    }
}

pub const fn get_current_gpu_clock_rate_message() -> mailbox::Message<9> {
    let mut ret = [0u32; 9];
    ret[0] = (9 * mem::size_of::<u32>()) as u32;
    ret[1] = MBOX_REQUEST;

    ret[2] = MailboxTag::GetClockRate as _; // set clock rate
    ret[3] = 8; // value buffer size in bytes
    ret[4] = 8; // clock id
    ret[5] = 0x5; // rate in hz
    ret[6] = 0; // skip setting turbo
    ret[7] = LAST_TAG;
    mailbox::Message::new(ret)
}

pub const fn max_gpu_clock_rate_message() -> mailbox::Message<9> {
    let mut ret = [0u32; 9];
    ret[0] = (9 * mem::size_of::<u32>()) as u32;
    ret[1] = MBOX_REQUEST;

    // tag:
    ret[2] = MailboxTag::GetMaxClockRate as _; // get serial number command
    ret[3] = 8; // value buffer size in bytes
    ret[4] = 8; // :b 31 clear: request, | b31 set: response b30-b0: value length in bytes

    ret[5] = 0x5; // clock id
    ret[6] = 0; // used by the response.
    ret[7] = LAST_TAG;
    mailbox::Message::new(ret)
}

pub const fn mem_alloc(size: u32, align: u32, flags: u32) -> mailbox::Message<10> {
    let mut ret = [0u32; 10];
    ret[0] = (10 * mem::size_of::<u32>()) as u32;
    ret[1] = MBOX_REQUEST;
    ret[2] = MailboxTag::AllocateMemory as _;
    ret[3] = 12;
    ret[4] = 12;
    ret[5] = size;
    ret[6] = align;
    ret[7] = flags;
    ret[8] = 0; //response
    ret[9] = LAST_TAG;
    mailbox::Message::new(ret)
}

pub const fn mem_free(val: u32) -> mailbox::Message<7> {
    let mut ret = [0u32; 7];
    ret[0] = (7 * mem::size_of::<u32>()) as u32;
    ret[1] = MBOX_REQUEST;
    ret[2] = MailboxTag::ReleaseMemory as _;
    ret[3] = 4;
    ret[4] = 4;
    ret[5] = val;
    ret[6] = LAST_TAG;
    mailbox::Message::new(ret)
}

pub const fn mem_lock(val: u32) -> mailbox::Message<8> {
    let mut ret = [0u32; 8];
    ret[0] = (8 * mem::size_of::<u32>()) as u32;
    ret[1] = MBOX_REQUEST;
    ret[2] = MailboxTag::LockMemory as _;
    ret[3] = 4;
    ret[4] = 4;
    ret[5] = val;
    ret[6] = 0; //response
    ret[7] = LAST_TAG;
    mailbox::Message::new(ret)
}

pub const fn mem_unlock(val: u32) -> mailbox::Message<7> {
    let mut ret = [0u32; 7];
    ret[0] = (7 * mem::size_of::<u32>()) as u32;
    ret[1] = MBOX_REQUEST;
    ret[2] = MailboxTag::UnlockMemory as _;
    ret[3] = 4;
    ret[4] = 4;
    ret[5] = val;
    ret[6] = LAST_TAG;
    mailbox::Message::new(ret)
}

pub const fn execute_code(
    code: u32,
    r0: u32,
    r1: u32,
    r2: u32,
    r3: u32,
    r4: u32,
    r5: u32,
) -> mailbox::Message<13> {
    let mut ret = [0u32; 13];
    ret[0] = (13 * mem::size_of::<u32>()) as u32;
    ret[1] = MBOX_REQUEST;
    ret[2] = MailboxTag::ExecuteCode as _;
    ret[3] = 28;
    ret[4] = 28;
    ret[5] = code;
    ret[6] = r0;
    ret[7] = r1;
    ret[8] = r2;
    ret[9] = r3;
    ret[10] = r4;
    ret[11] = r5;
    ret[12] = LAST_TAG;
    mailbox::Message::new(ret)
}

pub const fn execute_qpu(
    num_qpus: u32,
    control: u32,
    noflush: u32,
    timeout: u32,
) -> mailbox::Message<10> {
    let mut ret = [0u32; 10];
    ret[0] = (10 * mem::size_of::<u32>()) as u32;
    ret[1] = MBOX_REQUEST;
    ret[2] = MailboxTag::ExecuteQpu as _;
    ret[3] = 16;
    ret[4] = 16;
    ret[5] = num_qpus;
    ret[6] = control;
    ret[7] = noflush;
    ret[8] = timeout;
    ret[9] = LAST_TAG;
    mailbox::Message::new(ret)
}
