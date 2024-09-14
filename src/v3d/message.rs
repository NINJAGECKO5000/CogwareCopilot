use super::*;

pub const fn get_current_gpu_clock_rate_message() -> mailbox::Message<9> {
    let mut ret = [0u32; 9];
    ret[0] = (9 * mem::size_of::<u32>()) as u32;
    ret[1] = MBOX_REQUEST;

    ret[2] = MailboxTag::GetClockRate; // set clock rate
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
    ret[2] = MailboxTag::GetMaxClockRate; // get serial number command
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
    ret[2] = MailboxTag::AllocateMemory;
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
    ret[2] = MailboxTag::ReleaseMemory;
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
    ret[2] = MailboxTag::LockMemory;
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
    ret[2] = MailboxTag::UnlockMemory;
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
    ret[2] = MailboxTag::ExecuteCode;
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
    ret[2] = MailboxTag::ExecuteQpu;
    ret[3] = 16;
    ret[4] = 16;
    ret[5] = num_qpus;
    ret[6] = control;
    ret[7] = noflush;
    ret[8] = timeout;
    ret[9] = LAST_TAG;
    mailbox::Message::new(ret)
}
