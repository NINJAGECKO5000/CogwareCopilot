pub struct Writer {
    buf: *mut [u8],
    offset: usize,
}

impl Writer {
    pub fn new(arm_addr: u32, len: usize) -> Writer {
        let ptr = (arm_addr & !0xC0000000) as *mut u8;
        let start = unsafe { core::slice::from_raw_parts_mut(ptr, len) } as *mut _;

        Writer {
            buf: start,
            offset: 0,
        }
    }

    fn get_slice(&self, len: usize) -> &mut [u8] {
        &mut (unsafe { &mut *self.buf })[self.offset..len + self.offset]
    }

    pub fn write(&mut self, data: &[u8]) {
        let slice = self.get_slice(data.len());
        data.iter().enumerate().for_each(|(i, val)| slice[i] = *val);
        self.offset += data.len();
    }

    pub fn bytes_written(&self) -> usize {
        self.offset
    }
}
