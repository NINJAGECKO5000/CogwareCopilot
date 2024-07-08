#![no_std]
#![feature(pointer_byte_offsets)]
#![allow(missing_docs)]
pub use bcm2837_lpa as pac;
#[cfg(feature = "critical-section-impl")]
mod critical_section;

pub mod delay;
pub mod gpio;
pub mod interrupt;
pub(crate) mod macros;

pub use embedded_hal::delay::DelayNs;
