#![allow(clippy::upper_case_acronyms)]
// #![feature(asm_const)]
#![feature(const_option)]
#![feature(format_args_nl)]
#![feature(trait_alias)]
#![feature(alloc_error_handler)]
#![no_std]

extern crate alloc;

pub mod bsp;
pub mod console;
pub mod driver;
pub mod fb_trait;
pub mod framebuffer;
pub mod gl;
pub mod hvs;
pub mod hyperpixel;
pub mod mailbox;
pub mod panic_wait;
pub mod print;
pub mod synchronization;
pub mod time;
pub mod v3d;
