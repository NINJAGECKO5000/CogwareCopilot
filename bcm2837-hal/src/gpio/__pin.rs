use crate::{gpfsel, gpio_pins, gpio_pup_pdn, gpset, pac};

use paste::paste;

pub trait GpioExt {
    type Parts;

    fn split(self) -> Self::Parts;
}

impl GpioExt for pac::GPIO {
    type Parts = GPIO;

    fn split(self) -> Self::Parts {
        Self::Parts {
            gpio: self,
            pin_0: Pin0 {
                reg: pac::GPIO::ptr(),
            },
            pin_1: Pin1 {
                reg: pac::GPIO::ptr(),
            },
            pin_2: Pin2 {
                reg: pac::GPIO::ptr(),
            },
            pin_3: Pin3 {
                reg: pac::GPIO::ptr(),
            },
            pin_4: Pin4 {
                reg: pac::GPIO::ptr(),
            },
            pin_5: Pin5 {
                reg: pac::GPIO::ptr(),
            },
            pin_6: Pin6 {
                reg: pac::GPIO::ptr(),
            },
            pin_7: Pin7 {
                reg: pac::GPIO::ptr(),
            },
            pin_8: Pin8 {
                reg: pac::GPIO::ptr(),
            },
            pin_9: Pin9 {
                reg: pac::GPIO::ptr(),
            },
            pin_10: Pin10 {
                reg: pac::GPIO::ptr(),
            },
            pin_11: Pin11 {
                reg: pac::GPIO::ptr(),
            },
            pin_12: Pin12 {
                reg: pac::GPIO::ptr(),
            },
            pin_13: Pin13 {
                reg: pac::GPIO::ptr(),
            },
            pin_14: Pin14 {
                reg: pac::GPIO::ptr(),
            },
            pin_15: Pin15 {
                reg: pac::GPIO::ptr(),
            },
            pin_16: Pin16 {
                reg: pac::GPIO::ptr(),
            },
            pin_17: Pin17 {
                reg: pac::GPIO::ptr(),
            },
            pin_18: Pin18 {
                reg: pac::GPIO::ptr(),
            },
            pin_19: Pin19 {
                reg: pac::GPIO::ptr(),
            },
            pin_20: Pin20 {
                reg: pac::GPIO::ptr(),
            },
            pin_21: Pin21 {
                reg: pac::GPIO::ptr(),
            },
            pin_22: Pin22 {
                reg: pac::GPIO::ptr(),
            },
            pin_23: Pin23 {
                reg: pac::GPIO::ptr(),
            },
            pin_24: Pin24 {
                reg: pac::GPIO::ptr(),
            },
            pin_25: Pin25 {
                reg: pac::GPIO::ptr(),
            },
            pin_26: Pin26 {
                reg: pac::GPIO::ptr(),
            },
            pin_27: Pin27 {
                reg: pac::GPIO::ptr(),
            },
        }
    }
}

pub struct GPIO {
    gpio: pac::GPIO,
    pub pin_0: Pin0,
    pub pin_1: Pin1,
    pub pin_2: Pin2,
    pub pin_3: Pin3,
    pub pin_4: Pin4,
    pub pin_5: Pin5,
    pub pin_6: Pin6,
    pub pin_7: Pin7,
    pub pin_8: Pin8,
    pub pin_9: Pin9,
    pub pin_10: Pin10,
    pub pin_11: Pin11,
    pub pin_12: Pin12,
    pub pin_13: Pin13,
    pub pin_14: Pin14,
    pub pin_15: Pin15,
    pub pin_16: Pin16,
    pub pin_17: Pin17,
    pub pin_18: Pin18,
    pub pin_19: Pin19,
    pub pin_20: Pin20,
    pub pin_21: Pin21,
    pub pin_22: Pin22,
    pub pin_23: Pin23,
    pub pin_24: Pin24,
    pub pin_25: Pin25,
    pub pin_26: Pin26,
    pub pin_27: Pin27,
}

impl core::ops::Deref for GPIO {
    type Target = pac::gpio::RegisterBlock;

    fn deref(&self) -> &Self::Target {
        self.gpio.deref()
    }
}

gpio_pins!(
    Pin0, Pin1, Pin2, Pin3, Pin4, Pin5, Pin6, Pin7, Pin8, Pin9, Pin10, Pin11, Pin12, Pin13, Pin14,
    Pin15, Pin16, Pin17, Pin18, Pin19, Pin20, Pin21, Pin22, Pin23, Pin24, Pin25, Pin26, Pin27
);

gpset!(
    gpset0, gpclr0, [
        Pin0:  { set0, clr0 },
        Pin1:  { set1, clr1 },
        Pin2:  { set2, clr2 },
        Pin3:  { set3, clr3 },
        Pin4:  { set4, clr4 },
        Pin5:  { set5, clr5 },
        Pin6:  { set6, clr6 },
        Pin7:  { set7, clr7 },
        Pin8:  { set8, clr8 },
        Pin9:  { set9, clr9 },
        Pin10: { set10, clr10 },
        Pin11: { set11, clr11 },
        Pin12: { set12, clr12 },
        Pin13: { set13, clr13 },
        Pin14: { set14, clr14 },
        Pin15: { set15, clr15 },
        Pin16: { set16, clr16 },
        Pin17: { set17, clr17 },
        Pin18: { set18, clr18 },
        Pin19: { set19, clr19 },
        Pin20: { set20, clr20 },
        Pin21: { set21, clr21 },
        Pin22: { set22, clr22 },
        Pin23: { set23, clr23 },
        Pin24: { set24, clr24 },
        Pin25: { set25, clr25 },
        Pin26: { set26, clr26 },
        Pin27: { set27, clr27 }
    ]
);

gpfsel!(
    gpfsel0, [
        Pin0: { FSEL0_A, fsel0, sda0, sa5, reserved2, reserved3, reserved4, reserved5 },
        Pin1: { FSEL1_A, fsel1, scl0, sa4, reserved2, reserved3, reserved4, reserved5 },
        Pin2: { FSEL2_A, fsel2, sda1, sa3, reserved2, reserved3, reserved4, reserved5 },
        Pin3: { FSEL3_A, fsel3, scl1, sa2, reserved2, reserved3, reserved4, reserved5 },
        Pin4: { FSEL4_A, fsel4, gpclk0, sa1, reserved2, reserved3, reserved4, arm_tdi },
        Pin5: { FSEL5_A, fsel5, gpclk1, sa0, reserved2, reserved3, reserved4, arm_tdo },
        Pin6: { FSEL6_A, fsel6, gpclk2, soe_n, reserved2, reserved3, reserved4, arm_rtck },
        Pin7: { FSEL7_A, fsel7, spi0_ce1_n, swe_n, reserved2, reserved3, reserved4, reserved5 },
        Pin8: { FSEL8_A, fsel8, spi0_ce0_n, sd0, reserved2, reserved3, reserved4, reserved5 },
        Pin9: { FSEL9_A, fsel9, spi0_miso, sd1, reserved2, reserved3, reserved4, reserved5 }
    ]
);

gpfsel!(
    gpfsel1, [
        Pin10: { FSEL10_A, fsel10, spi0_mosi, sd2, reserved2, reserved3, reserved4, reserved5 },
        Pin11: { FSEL11_A, fsel11, spi0_sclk, sd3, reserved2, reserved3, reserved4, reserved5 },
        Pin12: { FSEL12_A, fsel12, pwm0_0, sd4, reserved2, reserved3, reserved4, arm_tms },
        Pin13: { FSEL13_A, fsel13, pwm0_1, sd5, reserved2, reserved3, reserved4, arm_tck },
        Pin14: { FSEL14_A, fsel14, txd0, sd6, reserved2, reserved3, reserved4, txd1 },
        Pin15: { FSEL15_A, fsel15, rxd0, sd7, reserved2, reserved3, reserved4, rxd1 },
        Pin16: { FSEL16_A, fsel16, reserved0, sd8, reserved2, cts0, spi1_ce2_n, cts1 },
        Pin17: { FSEL17_A, fsel17, reserved0, sd9, reserved2, rts0, spi1_ce1_n, rts1 },
        Pin18: { FSEL18_A, fsel18, pcm_clk, sd10, reserved2, reserved3, spi1_ce0_n, pwm0_0 },
        Pin19: { FSEL19_A, fsel19, pcm_fs, sd11, reserved2, reserved3, spi1_miso, pwm0_1 }
    ]
);

gpfsel!(
    gpfsel2, [
        Pin20: { FSEL20_A, fsel20, pcm_din, sd12, reserved2, reserved3, spi1_mosi, gpclk0 },
        Pin21: { FSEL21_A, fsel21, pcm_dout, sd13, reserved2, reserved3, spi1_sclk, gpclk1 },
        Pin22: { FSEL22_A, fsel22, reserved0, sd14, reserved2, sd1_clk, arm_trst, reserved5 },
        Pin23: { FSEL23_A, fsel23, reserved0, sd15, reserved2, sd1_cmd, arm_rtck, reserved5 },
        Pin24: { FSEL24_A, fsel24, reserved0, sd16, reserved2, sd1_dat0, arm_tdo, reserved5 },
        Pin25: { FSEL25_A, fsel25, reserved0, sd17, reserved2, sd1_dat1, arm_tck, reserved5 },
        Pin26: { FSEL26_A, fsel26, reserved0, reserved1, reserved2, sd1_dat2, arm_tdi, reserved5 },
        Pin27: { FSEL27_A, fsel27, reserved0, reserved1, reserved2, sd1_dat3, arm_tms, reserved5 }
    ]
);

gpio_pup_pdn!(
    gpio_pup_pdn_cntrl_reg0, [
        Pin0: { gpio_pup_pdn_cntrl0 },
        Pin1: { gpio_pup_pdn_cntrl1 },
        Pin2: { gpio_pup_pdn_cntrl2 },
        Pin3: { gpio_pup_pdn_cntrl3 },
        Pin4: { gpio_pup_pdn_cntrl4 },
        Pin5: { gpio_pup_pdn_cntrl5 },
        Pin6: { gpio_pup_pdn_cntrl6 },
        Pin7: { gpio_pup_pdn_cntrl7 },
        Pin8: { gpio_pup_pdn_cntrl8 },
        Pin9: { gpio_pup_pdn_cntrl9 },
        Pin10: { gpio_pup_pdn_cntrl10 },
        Pin11: { gpio_pup_pdn_cntrl11 },
        Pin12: { gpio_pup_pdn_cntrl12 },
        Pin13: { gpio_pup_pdn_cntrl13 },
        Pin14: { gpio_pup_pdn_cntrl14 },
        Pin15: { gpio_pup_pdn_cntrl15 }
    ]
);

gpio_pup_pdn!(
    gpio_pup_pdn_cntrl_reg1, [
        Pin16: { gpio_pup_pdn_cntrl16 },
        Pin17: { gpio_pup_pdn_cntrl17 },
        Pin18: { gpio_pup_pdn_cntrl18 },
        Pin19: { gpio_pup_pdn_cntrl19 },
        Pin20: { gpio_pup_pdn_cntrl20 },
        Pin21: { gpio_pup_pdn_cntrl21 },
        Pin22: { gpio_pup_pdn_cntrl22 },
        Pin23: { gpio_pup_pdn_cntrl23 },
        Pin24: { gpio_pup_pdn_cntrl24 },
        Pin25: { gpio_pup_pdn_cntrl25 },
        Pin26: { gpio_pup_pdn_cntrl26 },
        Pin27: { gpio_pup_pdn_cntrl27 }
    ]
);
