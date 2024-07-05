use core::sync::atomic::{compiler_fence, Ordering};

use crate::interrupt;
use interrupts::Guard;

static mut GUARD: Option<Guard> = None;

struct SingleCoreCs;
critical_section::set_impl!(SingleCoreCs);

unsafe impl critical_section::Impl for SingleCoreCs {
    unsafe fn acquire() {
        // NOTE: Fence guarantees are provided by interrupt::disable(), which performs a `compiler_fence(SeqCst)`.
        let guard = interrupt::disable();

        compiler_fence(Ordering::SeqCst);
        GUARD = Some(guard);
    }

    unsafe fn release(_: ()) {
        compiler_fence(Ordering::SeqCst);
        GUARD = None;
        // // Only re-enable interrupts if they were enabled before the critical section.
        // if was_active {
        //     // NOTE: Fence guarantees are provided by interrupt::enable(), which performs a
        //     // `compiler_fence(SeqCst)`.
        //     interrupt::enable()
        // }
    }
}
