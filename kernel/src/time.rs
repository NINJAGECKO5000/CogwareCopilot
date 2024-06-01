use crate::mmio::TIMER_REG_BASE;
use core::time::Duration;
use tock_registers::interfaces::Readable;
use tock_registers::register_bitfields;
use tock_registers::registers::ReadOnly;
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

pub fn now(reg: &mut ArmTimeRegisters) -> Duration {
    let lower = reg.counter_lower.get() as u64;
    let upper = reg.counter_higher.get() as u64;
    let microseconds = (upper << 32) | lower;
    Duration::from_micros(microseconds)
}

pub fn sleep(duration: Duration) {
    let ptr = TIMER_REG_BASE as *mut ArmTimeRegisters;
    let registers = unsafe { &mut *ptr };
    let target = now(registers) + duration;

    while now(registers) <= target {}
}
