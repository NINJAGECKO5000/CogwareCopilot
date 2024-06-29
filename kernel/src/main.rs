#![no_std]
#![no_main]
#![allow(missing_docs)]
#![feature(format_args_nl)]
#![feature(alloc_error_handler)]
// #![feature(panic_info_message)]
#![feature(asm_const)]

use core::panic::PanicInfo;

use crate::logger::IrisLogger;

use bcm2837_hal as hal;
use cortex_a::asm;
use embedded_sdmmc::time::DummyTimesource;
// use bcm2837_hal::pac::emmc;
use embedded_sdmmc::VolumeManager;
use hal::pac;

use cortex_a::registers::SCTLR_EL1;
use drivers::HyperPixel;
use embedded_sdmmc::sdcard::EMMCController;
use pac::Peripherals;
use space_invaders::run_test;
mod boot;
mod framebuffer;
mod logger;
mod mailbox;
mod print;
mod uart_pl011;

use crate::mailbox::{max_clock_speed, set_clock_speed};
use crate::mmio::PL011_UART_START;
// use crate::time::TIME_MANAGER;
use crate::uart_pl011::PL011Uart;
use log::{debug, error, info};
use tock_registers::interfaces::ReadWriteable;

static IRIS_LOGGER: IrisLogger = IrisLogger::new();
pub static PL011_UART: PL011Uart = unsafe { PL011Uart::new(PL011_UART_START) };

pub mod drivers;

mod mmio {
    pub const IO_BASE: usize = 0x3F00_0000;
    pub const UART_OFFSET: usize = 0x0020_1000;
    pub const VIDEOCORE_MBOX_OFFSET: usize = 0x0000_B880;
    pub const PL011_UART_START: usize = IO_BASE + UART_OFFSET;
    pub const VIDEOCORE_MBOX_BASE: usize = IO_BASE + VIDEOCORE_MBOX_OFFSET;
}

#[inline]
unsafe fn kernel_init() -> ! {
    SCTLR_EL1.modify(SCTLR_EL1::C::Cacheable + SCTLR_EL1::I::Cacheable);
    unsafe {
        PL011_UART.init().unwrap();
    }
    info!("kernel_init");
    IRIS_LOGGER.init().unwrap();
    let max_clock_speed = max_clock_speed();
    info!("Kernel speed: {:?}", max_clock_speed);
    set_clock_speed(1_000_000_000);
    main();
    panic!()
}

fn main() {
    info!("main");
    let fb = mailbox::lfb_init(0).expect("Failed to init framebuffer");
    let peripherals = unsafe { Peripherals::steal() };
    let gpio = peripherals.GPIO;

    info!("Starting Drivers!");
    info!("Initializing EMMC Controller...");
    let mut card = EMMCController::new();
    let result = card.emmc_init_card();
    info!("EMMC Controller initialized!");

    info!("Initializing Volume Manager...");
    let time_source = DummyTimesource::default();
    let mut volume_mgr = VolumeManager::new(card, time_source);
    info!("Volume Manager initialized!");

    info!("Opening Volume 0...");
    let mut volume0 = volume_mgr
        .open_volume(embedded_sdmmc::VolumeIdx(0))
        .expect("failed to open volume 0");

    info!("Done!");
    info!("Volume 0: {:?}", volume0);

    info!("Opening Volume 0...");
    let root_dir = volume0
        .open_root_dir()
        .expect("failed to open root directory");

    info!("Done!");
    info!("Root directory: {:#?}", root_dir);

    // let mut timer = hal::delay::Timer::new();
    // let hp = HyperPixel::new(gpio, &mut timer);
    // hp.init();
    info!("we made it past initialization yay fdsg");
    info!("hyperpixel is inited in theory");
    // where to add the rest of the program
    run_test(fb, result);
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    error!("PANIC! {}", info);
    loop {
        asm::wfe()
    }
}
