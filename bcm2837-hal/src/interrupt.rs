use core::sync::atomic::{compiler_fence, Ordering};

use interrupts::Guard;

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
