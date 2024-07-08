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
mod hvs;
mod mailbox;
mod panic_wait;
mod print;
mod synchronization;
mod time;

use alloc::{format, vec::Vec};
use bcm2837_hal::{delay::Timer, pac::Peripherals, DelayNs};
use bsp::memory::initialize_heap;
use core::time::Duration;
use embedded_sdmmc::{sdcard::EMMCController, time::DummyTimesource, Mode, VolumeManager};
use fb_trait::FrameBufferInterface;
use hvs::{Hvs, Plane};

use crate::mailbox::{max_clock_speed, set_clock_speed};
// use fb_trait::FrameBufferInterface;
// use framebuffer::FrameBuffer;

const BOOT_IMAGE_QOI: &[u8] = include_bytes!("CogWare.qoi");

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
    {
        // let mut u: MaybeUninit<FrameBuffer> = MaybeUninit::uninit();
        driver::driver_manager().init_drivers();
        initialize_heap();
        info!("kernel_init");
        let max_clock_speed = max_clock_speed();
        set_clock_speed(max_clock_speed.unwrap());

        info!("initializing hvs");
        let mut timer = Timer::new();
        let (header, image) =
            qoi::decode_to_vec(BOOT_IMAGE_QOI).expect("Failed to decode boot image (wtf?)");

        let mut hvs = Hvs::new();
        // this doesn't work
        // why
        // hvs.add_plane(Plane::from_qoi(header, image));
        hvs.add_plane(Plane::white());
        hvs.draw();

        timer.delay_ms(1000);

        hvs.add_plane(Plane::green_half_alpha());
        hvs.draw();

        timer.delay_ms(1000);
        hvs.add_plane(Plane::from_qoi(header, image));
        hvs.draw();

        timer.delay_ms(1000);
        info!("SUGMA");

        // loop {
        //     timer.delay_ns(500_000_000);
        //     hvs.reset();
        //     hvs.add_plane(Plane::from_qoi(header, image.clone()));
        //     hvs.draw();
        //     timer.delay_ns(500_000_000);
        // }
        // let mut fb = mailbox::lfb_init(0).expect("Failed to init framebuffer");
        // fb.display_boot_image();
        // let u = u.assume_init();
    }

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

    info!("Opening Volume 0...");
    let mut volume0 = volume_mgr
        .open_volume(embedded_sdmmc::VolumeIdx(0))
        .expect("Failed to open volume 0");

    info!("Done!");
    info!("Volume 0: {:?}", volume0);

    info!("Opening Volume 0...");
    let mut root_dir = volume0
        .open_root_dir()
        .expect("Failed to open root directory");

    info!("Done!");
    info!("Root directory: {:#?}", root_dir);

    let mut cfg_file = root_dir
        .open_file_in_dir("CONFIG.TXT", Mode::ReadOnly)
        .expect("Failed to open CONFIG.TXT");

    let out = match cfg_file.read_to_string() {
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
