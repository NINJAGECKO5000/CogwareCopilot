use crate::mmio::TIMER_REG_BASE;
use core::time::Duration;
use cortex_a::asm::barrier;
use cortex_a::registers::{CNTFRQ_EL0, CNTPCT_EL0};
use tock_registers::interfaces::Readable;
use tock_registers::register_bitfields;
use tock_registers::registers::ReadOnly;

const NS_PER_S: u64 = 1_000_000_000;
static mut REGISTERS: *mut ArmTimeRegisters = TIMER_REG_BASE as *mut ArmTimeRegisters;

register_bitfields! {
    u32,
}

#[repr(C)]
/// https://tc.gts3.org/cs3210/2020/spring/r/BCM2837-ARM-Peripherals.pdf cap 12 - pag 172
struct ArmTimeRegisters {
    /// CS register
    _controller_status: ReadOnly<u32>,
    /// CLO: counter lower 32 bits
    counter_lower: ReadOnly<u32>,
    /// CHI: System Timer Counter Higher 32 bits
    counter_higher: ReadOnly<u32>,
    /// system Timer compare registers - 4 in total
    _compare: [u32; 4],
}

pub fn now() -> Duration {
    let registers = unsafe { &mut *REGISTERS };
    let lower = registers.counter_lower.get() as u64;
    let upper = registers.counter_higher.get() as u64;
    let microseconds = (upper << 32) | lower;
    Duration::from_micros(microseconds)
}

pub fn sleep(duration: Duration) {
    let target = now() + duration;

    while now() <= target {}
}

pub fn get_sys_tick_count() -> u64 {
    barrier::isb(barrier::SY);
    CNTPCT_EL0.get()
}

pub fn resolution() -> Duration {
    Duration::from_nanos(NS_PER_S / (CNTFRQ_EL0.get() as u64))
}
