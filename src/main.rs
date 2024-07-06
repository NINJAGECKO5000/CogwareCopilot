#![allow(clippy::upper_case_acronyms)]
#![feature(asm_const)]
#![feature(const_option)]
#![feature(format_args_nl)]
#![feature(trait_alias)]
#![feature(alloc_error_handler)]
#![no_main]
#![no_std]

extern crate alloc;

mod bsp;
mod console;
mod cpu;
mod driver;
mod fb_trait;
mod framebuffer;
mod mailbox;
mod panic_wait;
mod print;
mod synchronization;
mod time;

use alloc::{format, string::String, vec};
use bsp::memory::initialize_heap;
use core::time::Duration;
use embedded_sdmmc::{
    sdcard::{EMMCController, SdResult},
    time::DummyTimesource,
    Mode, VolumeManager,
};

use crate::mailbox::{max_clock_speed, set_clock_speed};
use fb_trait::FrameBufferInterface;
use framebuffer::FrameBuffer;

/// Early init code.
///
/// # Safety
///
/// - Only a single core must be active and running this function.
/// - The init calls in this function must appear in the correct order.

unsafe fn kernel_init() -> ! {
    // Initialize the BSP driver subsystem.
    if let Err(x) = bsp::driver::init() {
        panic!("Error initializing BSP driver subsystem: {}", x);
    }
    use core::mem::MaybeUninit;
    {
        let mut u: MaybeUninit<FrameBuffer> = MaybeUninit::uninit();
        driver::driver_manager().init_drivers();
        initialize_heap();
        info!("kernel_init");
        let max_clock_speed = max_clock_speed();
        set_clock_speed(max_clock_speed.unwrap());
        let mut fb = mailbox::lfb_init(0).expect("Failed to init framebuffer");
        fb.display_boot_image();
        // let u = u.assume_init();
    }
    // Initialize all device drivers.
    // driver::driver_manager().init_drivers();
    // println! is usable from here on.

    // Transition from unsafe to safe.
    kernel_main()
}

/// The main function running after the early init.
fn kernel_main() -> ! {
    // use core::mem::MaybeUninit;
    // static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
    // unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }

    info!(
        "{} version {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );
    info!("Booting on: {}", bsp::board_name());

    info!(
        "Architectural timer resolution: {} ns",
        time::time_manager().resolution().as_nanos()
    );

    info!("Initializing EMMC Controller...");
    let mut card = EMMCController::new();
    card.emmc_init_card();

    info!("EMMC Controller initialized!");

    info!("Initializing Volume Manager...");
    let time_source = DummyTimesource::default();
    let mut volume_mgr = VolumeManager::new(card, time_source);
    info!("Volume Manager initialized!");

    let out = match open_file(&mut volume_mgr, "CONFIG.TXT") {
        Ok(f) => f,
        Err(e) => format!("{:?}", e),
    };

    info!("CONFIG.TXT:\n{}", out);

    info!("Drivers loaded:");
    driver::driver_manager().enumerate();

    // Test a failing timer case.
    time::time_manager().spin_for(Duration::from_nanos(1));

    loop {
        info!("Spinning for 1 second");
        time::time_manager().spin_for(Duration::from_secs(1));
    }
}
fn open_file(
    volume_mgr: &mut VolumeManager<EMMCController, DummyTimesource>,
    file: &str,
) -> Result<String, embedded_sdmmc::Error<SdResult>> {
    info!("Opening Volume 0...");
    let mut volume0 = volume_mgr.open_volume(embedded_sdmmc::VolumeIdx(0))?;

    info!("Done!");
    info!("Volume 0: {:?}", volume0);

    info!("Opening Volume 0...");
    let mut root_dir = volume0.open_root_dir()?;

    info!("Done!");
    info!("Root directory: {:#?}", root_dir);

    let mut config_file = root_dir.open_file_in_dir(file, Mode::ReadOnly)?;

    info!("Reading {}", file);

    // let mut out = String::with_capacity(config_file.length());
    // let mut out = String::new();
    let mut buf = vec![0u8; config_file.length() as _];
    info!("Vec len: {}", buf.len());

    while !config_file.is_eof() {
        config_file.read(&mut buf)?;
        // for b in &buf[0..num_read] {
        //     out.push(*b as char)
        // }
    }

    let out = String::from_utf8(buf).unwrap();

    Ok(out)
}
