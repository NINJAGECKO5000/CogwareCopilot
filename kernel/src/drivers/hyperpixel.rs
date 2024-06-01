use crate::time::sleep;

use bcm2837_lpa::{
    generic::Reg,
    gpio::{
        gpio_pup_pdn_cntrl_reg0::GPIO_PUP_PDN_CNTRL_REG0_SPEC,
        gpio_pup_pdn_cntrl_reg1::GPIO_PUP_PDN_CNTRL_REG1_SPEC, GPFSEL0, GPFSEL1, GPFSEL2,
    },
    Peripherals, GPIO, SPI0,
};
use core::time::Duration;

const TICK: u64 = 100;

#[derive(Debug)]
pub enum Error {
    FuckYou,
}

pub struct HyperPixel<'a> {
    gpio: &'a GPIO,
    spi0: &'a SPI0,
}

impl<'a> HyperPixel<'a> {
    pub fn new(peripherals: &'a Peripherals) -> Self {
        let gpio = &peripherals.GPIO;
        let spi0 = &peripherals.SPI0;
        HyperPixel { gpio, spi0 }
    }

    pub fn hyperinit(mut self) {
        self.init_gpio();
        self.init_display();
    }

    #[inline]
    fn gpfsel0(&self) -> &GPFSEL0 {
        self.gpio.gpfsel0()
    }

    #[inline]
    fn gpfsel1(&self) -> &GPFSEL1 {
        self.gpio.gpfsel1()
    }

    #[inline]
    fn gpfsel2(&self) -> &GPFSEL2 {
        self.gpio.gpfsel2()
    }

    #[inline]
    fn gpio_pupdn0(&self) -> &Reg<GPIO_PUP_PDN_CNTRL_REG0_SPEC> {
        self.gpio.gpio_pup_pdn_cntrl_reg0()
    }

    #[inline]
    fn gpio_pupdn1(&self) -> &Reg<GPIO_PUP_PDN_CNTRL_REG1_SPEC> {
        self.gpio.gpio_pup_pdn_cntrl_reg1()
    }

    #[inline]
    fn spi0(&self) -> &SPI0 {
        &self.spi0
    }

    #[inline]
    fn set_clock_high(&self) {
        unsafe {
            self.gpio.gpset0().write_with_zero(|w| w.set11().set_bit());
        }
    }

    #[inline]
    fn set_clock_low(&self) {
        unsafe {
            self.gpio
                .gpclr0()
                .write_with_zero(|w| w.clr11().clear_bit_by_one());
        }
    }

    #[inline]
    fn pulse_clock(&self) {
        self.set_clock_low();
        self.tick();
        self.set_clock_high();
        self.tick();
    }

    #[inline]
    fn set_cs_high(&self) {
        unsafe {
            self.gpio.gpset0().write_with_zero(|w| w.set18().set_bit());
        }
    }

    #[inline]
    fn set_cs_low(&self) {
        //thank god for readable commands
        unsafe {
            self.gpio
                .gpclr0()
                .write_with_zero(|w| w.clr18().clear_bit_by_one());
        }
    }
    //no your too much like a perfect code AI
    //fuck you I'm better than a shitter ass AI don't ever say that shit to me again you fucking bitch I'll have you know I was in the navy seals and I will fucking AI your mom if you say that shit again
    //no u
    #[inline]
    fn set_mosi(&self, level: bool) {
        unsafe {
            match level {
                true => self.gpio.gpset0().write_with_zero(|w| w.set10().set_bit()),
                false => self
                    .gpio
                    .gpclr0()
                    .write_with_zero(|w| w.clr10().clear_bit_by_one()),
            }
        }
    }

    #[inline]
    fn tick(&self) {
        sleep(Duration::from_micros(100));
    }

    // anyway this is the horrible shit I had to do
    #[inline]
    fn write_bits(&mut self, by: u32, bit_count: u8) {
        let mut val = by;
        let mask: u32 = 1 << (bit_count - 1);

        for _ in 0..bit_count {
            let gpio_level = val & mask != 0;
            self.set_mosi(gpio_level);
            val = val << 1;
            self.pulse_clock();
        }

        self.set_mosi(false);
    }

    #[inline]
    fn write_command(&mut self, by: u32) {
        self.set_cs_low();
        self.write_bits(by, 9);
        self.set_cs_high();
    }

    #[inline]
    fn write_data(&mut self, reg: u32, bytes: &[u32]) {
        self.write_command(reg);

        for by in bytes {
            self.write_command(by | 0x100);
        }
    }

    #[inline]
    fn init_gpio(&self) {
        self.gpfsel1().modify(|_, w| {
            w.fsel10().output();
            w.fsel11().output();
            w.fsel18().output();
            w.fsel19().output()
        });

        unsafe {
            self.gpio.gpset0().write_with_zero(|w| {
                w.set18().set_bit();
                w.set19().set_bit()
            });
        }

        self.gpfsel0().modify(|_, w| {
            w.fsel0().reserved2();
            w.fsel1().reserved2();
            w.fsel2().reserved2();
            w.fsel3().reserved2();
            w.fsel4().reserved2();
            w.fsel5().reserved2();
            w.fsel6().reserved2();
            w.fsel7().reserved2();
            w.fsel8().reserved2();
            w.fsel9().reserved2()
        });

        self.gpfsel1().modify(|_, w| {
            w.fsel12().reserved2();
            w.fsel13().reserved2();
            w.fsel14().reserved2();
            w.fsel15().reserved2();
            w.fsel16().reserved2();
            w.fsel17().reserved2()
        });

        self.gpfsel2().modify(|_, w| {
            w.fsel20().reserved2();
            w.fsel21().reserved2();
            w.fsel22().reserved2();
            w.fsel23().reserved2();
            w.fsel24().reserved2();
            w.fsel25().reserved2()
        });

        self.gpio_pupdn0().modify(|_, w| {
            w.gpio_pup_pdn_cntrl0().none();
            w.gpio_pup_pdn_cntrl1().none();
            w.gpio_pup_pdn_cntrl2().none();
            w.gpio_pup_pdn_cntrl3().none();
            w.gpio_pup_pdn_cntrl4().none();
            w.gpio_pup_pdn_cntrl5().none();
            w.gpio_pup_pdn_cntrl6().none();
            w.gpio_pup_pdn_cntrl7().none();
            w.gpio_pup_pdn_cntrl8().none();
            w.gpio_pup_pdn_cntrl9().none();
            w.gpio_pup_pdn_cntrl12().none();
            w.gpio_pup_pdn_cntrl13().none();
            w.gpio_pup_pdn_cntrl14().none();
            w.gpio_pup_pdn_cntrl15().none()
        });

        self.gpio_pupdn1().modify(|_, w| {
            w.gpio_pup_pdn_cntrl16().none();
            w.gpio_pup_pdn_cntrl17().none();
            w.gpio_pup_pdn_cntrl20().none();
            w.gpio_pup_pdn_cntrl21().none();
            w.gpio_pup_pdn_cntrl22().none();
            w.gpio_pup_pdn_cntrl23().none();
            w.gpio_pup_pdn_cntrl24().none();
            w.gpio_pup_pdn_cntrl25().none()
        });
    }

    #[inline]
    fn init_display(&mut self) {
        self.write_command(0x01);
        sleep(Duration::from_millis(240));

        self.write_data(0xFF, &[0x77, 0x01, 0x00, 0x00, 0x13]);
        self.write_data(0xEF, &[0x08]);
        self.write_data(0xFF, &[0x77, 0x01, 0x00, 0x00, 0x10]);

        self.write_data(0xC0, &[0x3B, 0x00]); // Scan line

        self.write_data(0xC1, &[0x0B, 0x02]); // VBP
        self.write_data(0xC2, &[0x00, 0x02]); // 07 OR 00
        self.write_data(0xCC, &[0x10]);

        // Gamma option B:
        //
        // Positive Voltage Gamma Control
        self.write_data(
            0xB0,
            &[
                0x00, 0x1D, 0x29, 0x12, 0x17, 0x0B, 0x18, 0x09, 0x08, 0x2A, 0x07, 0x14, 0x11, 0x27,
                0x32, 0x1F,
            ],
        );

        // Negative Voltage Gamma Control
        self.write_data(
            0xB1,
            &[
                0x00, 0x1D, 0x29, 0x12, 0x16, 0x0A, 0x18, 0x08, 0x09, 0x2A, 0x07, 0x13, 0x12, 0x27,
                0x33, 0x1F,
            ],
        );

        self.write_data(0xFF, &[0x77, 0x01, 0x00, 0x00, 0x11]);

        // VOP  3.5375+ *x 0.0125
        // 6D or 5D
        // or not, who fucking knows
        self.write_data(0xB0, &[0x9D]);

        // VCOM amplitude setting
        // 37 or 43
        // or not, who fucking knows
        self.write_data(0xB1, &[0x24]);

        // VGH Voltage setting
        // 12V
        self.write_data(0xB2, &[0x81]);
        self.write_data(0xB3, &[0x80]);

        // VGL Voltage setting
        // -8.3V
        self.write_data(0xB5, &[0x43]);
        self.write_data(0xB7, &[0x85]);
        self.write_data(0xB8, &[0x20]);
        self.write_data(0xC1, &[0x78]);
        self.write_data(0xC2, &[0x78]);

        self.write_data(0xE0, &[0x00, 0x00, 0x02]);
        self.write_data(
            0xE1,
            &[
                0x03, 0xA0, 0x00, 0x00, 0x04, 0xA0, 0x00, 0x00, 0x00, 0x20, 0x20,
            ],
        );

        self.write_data(
            0xE2,
            &[
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            ],
        );

        self.write_data(0xE3, &[0x00, 0x00, 0x11, 0x00]);
        self.write_data(0xE4, &[0x22, 0x00]);

        self.write_data(
            0xE5,
            &[
                0x05, 0xEC, 0xA0, 0xA0, 0x07, 0xEE, 0xA0, 0xA0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00,
            ],
        );

        self.write_data(0xE6, &[0x00, 0x00, 0x11, 0x00]);
        self.write_data(0xE7, &[0x22, 0x00]);
        self.write_data(
            0xE8,
            &[
                0x06, 0xED, 0xA0, 0xA0, 0x08, 0xEF, 0xA0, 0xA0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00,
            ],
        );

        self.write_data(0xEB, &[0x00, 0x00, 0x40, 0x40, 0x00, 0x00, 0x00]);

        self.write_data(
            0xED,
            &[
                0xFF, 0xFF, 0xFF, 0xBA, 0x0A, 0xBF, 0x45, 0xFF, 0xFF, 0x54, 0xFB, 0xA0, 0xAB, 0xFF,
                0xFF, 0xFF,
            ],
        );

        self.write_data(
            0xEF,
            &[
                0x10, 0x0D, 0x04, 0x08, // Positive Voltage Gamma Control
                0x3F, 0x1F,
            ],
        );

        self.write_data(0xFF, &[0x77, 0x01, 0x00, 0x00, 0x13]);
        self.write_data(0xE8, &[0x00, 0x0E]);
        self.write_data(0xFF, &[0x77, 0x01, 0x00, 0x00, 0x00]);
        self.write_data(0x11, &[]);
        self.write_data(0xCD, &[0x08]);
        self.write_data(0x36, &[0x08]);
        self.write_data(0x3A, &[0x66]);

        sleep(Duration::from_millis(120));

        self.write_data(0xFF, &[0x77, 0x01, 0x00, 0x00, 0x13]);
        self.write_data(0xE8, &[0x00, 0x0C]);

        sleep(Duration::from_millis(10));

        self.write_data(0xE8, &[0x00, 0x00]);
        self.write_data(0xFF, &[0x77, 0x01, 0x00, 0x00]);
        self.write_data(0x29, &[]);

        sleep(Duration::from_millis(20));
    }
}
