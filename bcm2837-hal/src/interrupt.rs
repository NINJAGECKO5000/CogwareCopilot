use core::sync::atomic::{compiler_fence, Ordering};

use interrupts::Guard;

#[derive(Debug)]
pub struct LicmaHandler;

impl core::ops::Deref for LicmaHandler {
    type Target = crate::pac::lic::RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(crate::pac::LIC::ptr() as *const Self::Target) }
    }
}

impl LicmaHandler {
    pub fn enable_mailbox() {
        Self.enable_basic().modify(|_, w| w.mailbox().bit(true));
    }

    pub fn disable_mailbox() {
        Self.disable_basic().modify(|_, w| w.mailbox().bit(true));
    }

    pub fn mailbox_pending() -> bool {
        Self.basic_pending().read().mailbox().bit_is_set()
    }
}

#[inline]
pub fn disable() -> Guard {
    let guard = interrupts::disable();
    // Ensure no subsequent memory accesses are reordered to before interrupts are disabled.
    compiler_fence(Ordering::SeqCst);

    guard
}

// /// Enables all the interrupts in the current core.
// ///
// /// # Safety
// ///
// /// - Do not call this function inside a critical section.
// #[inline]
// pub unsafe fn enable(guard: u64) {
//     // Ensure no preceeding memory accesses are reordered to after interrupts are enabled.
//     compiler_fence(Ordering::SeqCst);
// }
