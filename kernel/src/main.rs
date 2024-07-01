#![no_std]
#![no_main]
#![allow(missing_docs)]
#![feature(format_args_nl)]
#![feature(alloc_error_handler)]
// #![feature(panic_info_message)]
#![feature(asm_const)]

extern crate alloc;

use core::panic::PanicInfo;

use crate::logger::IrisLogger;

use alloc::string::String;
use alloc::{format, vec};
use bcm2837_hal as hal;
use cortex_a::asm;
use embedded_alloc::Heap;
use embedded_sdmmc::time::DummyTimesource;
use embedded_sdmmc::{Mode, VolumeManager};
use hal::pac;

use cortex_a::registers::SCTLR_EL1;
use drivers::HyperPixel;
use embedded_sdmmc::sdcard::{EMMCController, SdResult};
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
use crate::uart_pl011::PL011Uart;
use log::{debug, error, info};
use tock_registers::interfaces::ReadWriteable;

// Allocating 384MB for the heap
// Should be fine, right?
const HEAP_SIZE: usize = 384_000_000;

#[global_allocator]
static HEAP: Heap = Heap::empty();

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
    // Initialize the allocator BEFORE you use it
    // idiot
    {
        use core::mem::MaybeUninit;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }

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
    let peripherals = Peripherals::take().expect("Failed to get peripherals");

    let gpio = peripherals.GPIO;

    info!("Starting Drivers!");

    // Initialize EMMC Controller
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

    info!("Initializing HyperPixel...");
    // Initialize HyperPixel display
    let mut timer = hal::delay::Timer::new();
    let hp = HyperPixel::new(gpio, &mut timer);
    hp.init();
    info!("hyperpixel is inited in theory");

    info!("we made it past initialization yay fdsg");
    // where to add the rest of the program
    run_test(fb, &out);
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

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    error!("PANIC! {}", info);
    loop {
        asm::wfe()
    }
}
