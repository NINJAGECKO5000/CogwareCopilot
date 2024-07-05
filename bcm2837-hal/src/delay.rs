use crate::pac;

use core::time::Duration;
use embedded_hal::delay::DelayNs;

const PTR: *const pac::systmr::RegisterBlock = pac::SYSTMR::PTR;

pub struct Timer {}

impl core::ops::Deref for Timer {
    type Target = pac::systmr::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*(PTR) }
    }
}

impl Timer {
    pub const fn new() -> Timer {
        Timer {}
    }

    #[inline(always)]
    pub fn now(&self) -> Duration {
        let lower = self.clo().read().bits() as u64;
        let upper = self.chi().read().bits() as u64;
        let us = upper << 32 | lower;

        Duration::from_micros(us)
    }
}

impl DelayNs for Timer {
    fn delay_ns(&mut self, ns: u32) {
        let target = self.now() + Duration::from_nanos(ns as u64);

        while self.now() <= target {
            core::hint::spin_loop()
        }
    }
}
