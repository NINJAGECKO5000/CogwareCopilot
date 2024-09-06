use embedded_hal::digital::{ErrorKind, ErrorType, OutputPin};
use paste::paste;

pub mod pin;

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum PinMode {
    Input,
    InputPullUp,
    InputPullDown,
    Output,
    AF0,
    AF1,
    AF2,
    AF3,
    AF4,
    AF5,
}

impl PinMode {
    fn value(&self) -> u32 {
        match self {
            PinMode::Input => 0,
            PinMode::InputPullUp => 0,
            PinMode::InputPullDown => 0,
            PinMode::Output => 1,
            PinMode::AF0 => 4,
            PinMode::AF1 => 5,
            PinMode::AF2 => 6,
            PinMode::AF3 => 7,
            PinMode::AF4 => 3,
            PinMode::AF5 => 2,
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum PullUpDownMode {
    None = 0,
    Up = 1,
    Down = 2,
}

pub trait GpioExt {
    type Parts;

    fn split(&self) -> Self::Parts;
}

impl GpioExt for crate::pac::GPIO {
    type Parts = Gpio;

    fn split(&self) -> Self::Parts {
        Gpio::new()
    }
}

pub struct Pin {
    pin_num: u8,
    mode: PinMode,
}
/*impl ErrorType for OutputPin {
    type Error = ErrorKind;
}
impl OutputPin for Pin {
    fn set_low(&mut self) -> Result<(), Self::Error> {
        todo!()
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        todo!()
    }

    fn set_state(&mut self, state: embedded_hal::digital::PinState) -> Result<(), Self::Error> {
        match state {
            embedded_hal::digital::PinState::Low => self.set_low(),
            embedded_hal::digital::PinState::High => self.set_high(),
        }
    }
}*/
impl Pin {
    pub fn new(pin_num: u8, mode: PinMode) -> Self {
        Self { pin_num, mode }
    }

    pub fn set_mode(&self, mode: PinMode) -> &Pin {
        let reg = self.fsel_ptr();
        let bank = (self.pin_num % 10) * 3;
        let val: u32 = mode.value() << bank;

        reg.modify(|r, w| unsafe { w.bits(r.bits() | val) });

        match mode {
            PinMode::InputPullUp => self.set_pupdn(PullUpDownMode::Up),
            PinMode::InputPullDown => self.set_pupdn(PullUpDownMode::Down),
            _ => self.set_pupdn(PullUpDownMode::None),
        };

        self
    }

    pub fn set_pupdn(&self, mode: PullUpDownMode) -> &Pin {
        let reg = self.pupdn_ptr();
        let bank = (self.pin_num % 16) * 2;
        let val: u32 = (mode as u32) << bank;

        reg.modify(|r, w| unsafe { w.bits(r.bits() | val) });

        self
    }

    pub fn set_high(&self) -> &Pin {
        let reg = self.set_ptr();
        let bank = self.pin_num % 32;

        unsafe {
            reg.write_with_zero(|w| w.bits(1 << bank));
        }

        self
    }

    pub fn set_low(&self) -> &Pin {
        let reg = self.clr_ptr();
        let bank = self.pin_num % 32;

        unsafe {
            reg.write_with_zero(|w| w.bits(1 << bank));
        }

        self
    }

    pub fn set_fen(&self) -> &Pin {
        // falling writer
        let reg = self.fen_ptr();
        let bank = self.pin_num % 32;

        unsafe {
            reg.write_with_zero(|w| w.bits(1 << bank));
        }

        self
    }

    pub fn set_afen(&self) -> &Pin {
        // falling async writer
        let reg = self.afen_ptr();
        let bank = self.pin_num % 32;

        unsafe {
            reg.write_with_zero(|w| w.bits(1 << bank));
        }

        self
    }

    pub fn set_ren(&self) -> &Pin {
        // rising writer
        let reg = self.ren_ptr();
        let bank = self.pin_num % 32;

        unsafe {
            reg.write_with_zero(|w| w.bits(1 << bank));
        }

        self
    }

    pub fn set_aren(&self) -> &Pin {
        // rising async writer
        let reg = self.aren_ptr();
        let bank = self.pin_num % 32;

        unsafe {
            reg.write_with_zero(|w| w.bits(1 << bank));
        }

        self
    }

    pub fn set_hen(&self) -> &Pin {
        // high detect enable writer
        let reg = self.hen_ptr();
        let bank = self.pin_num % 32;

        unsafe {
            reg.write_with_zero(|w| w.bits(1 << bank));
        }

        self
    }

    pub fn set_len(&self) -> &Pin {
        // low detect enable writer
        let reg = self.len_ptr();
        let bank = self.pin_num % 32;

        unsafe {
            reg.write_with_zero(|w| w.bits(1 << bank));
        }

        self
    }

    pub fn set_eds(&self) -> &Pin {
        // event detect status writer
        let reg = self.eds_ptr();
        let bank = self.pin_num % 32;

        unsafe {
            reg.write_with_zero(|w| w.bits(1 << bank));
        }

        self
    }

    // -------------------------------------------
    // Pointers
    // -------------------------------------------

    fn fen_ptr(&self) -> &crate::pac::gpio::GPFEN0 {
        // falling edge R/W
        let offset = (self.pin_num / 32) * 0x04;
        let offset_ptr = unsafe { crate::pac::GPIO::ptr().byte_add(offset as usize) } as usize;

        unsafe { &*(offset_ptr as *const crate::pac::gpio::GPFEN0) }
    }

    fn afen_ptr(&self) -> &crate::pac::gpio::GPAFEN0 {
        // async falling edge R/W
        let offset = (self.pin_num / 32) * 0x04;
        let offset_ptr = unsafe { crate::pac::GPIO::ptr().byte_add(offset as usize) } as usize;

        unsafe { &*(offset_ptr as *const crate::pac::gpio::GPAFEN0) }
    }

    fn ren_ptr(&self) -> &crate::pac::gpio::GPREN0 {
        // rising edge R/W
        let offset = (self.pin_num / 32) * 0x04;
        let offset_ptr = unsafe { crate::pac::GPIO::ptr().byte_add(offset as usize) } as usize;

        unsafe { &*(offset_ptr as *const crate::pac::gpio::GPREN0) }
    }

    fn aren_ptr(&self) -> &crate::pac::gpio::GPAREN0 {
        // async rising edge R/W
        let offset = (self.pin_num / 32) * 0x04;
        let offset_ptr = unsafe { crate::pac::GPIO::ptr().byte_add(offset as usize) } as usize;

        unsafe { &*(offset_ptr as *const crate::pac::gpio::GPAREN0) }
    }

    fn eds_ptr(&self) -> &crate::pac::gpio::GPEDS0 {
        // event detect status R/W
        let offset = (self.pin_num / 32) * 0x04;
        let offset_ptr = unsafe { crate::pac::GPIO::ptr().byte_add(offset as usize) } as usize;

        unsafe { &*(offset_ptr as *const crate::pac::gpio::GPEDS0) }
    }

    fn hen_ptr(&self) -> &crate::pac::gpio::GPHEN0 {
        // high detect enable R/W
        let offset = (self.pin_num / 32) * 0x04;
        let offset_ptr = unsafe { crate::pac::GPIO::ptr().byte_add(offset as usize) } as usize;

        unsafe { &*(offset_ptr as *const crate::pac::gpio::GPHEN0) }
    }

    fn len_ptr(&self) -> &crate::pac::gpio::GPLEN0 {
        // low detect enable R/W
        let offset = (self.pin_num / 32) * 0x04;
        let offset_ptr = unsafe { crate::pac::GPIO::ptr().byte_add(offset as usize) } as usize;

        unsafe { &*(offset_ptr as *const crate::pac::gpio::GPLEN0) }
    }

    fn lvl_ptr(&self) -> &crate::pac::gpio::GPLEV0 {
        // pin level reader R/O
        let offset = (self.pin_num / 32) * 0x04;
        let offset_ptr = unsafe { crate::pac::GPIO::ptr().byte_add(offset as usize) } as usize;

        unsafe { &*(offset_ptr as *const crate::pac::gpio::GPLEV0) }
    }

    fn pupdn_ptr(&self) -> &crate::pac::gpio::GPIO_PUP_PDN_CNTRL_REG0 {
        let offset = (self.pin_num / 16) * 0x04;
        let offset_ptr = unsafe {
            crate::pac::GPIO::ptr()
                .byte_add(0xE4)
                .byte_add(offset as usize)
        } as usize;

        unsafe { &*(offset_ptr as *const crate::pac::gpio::GPIO_PUP_PDN_CNTRL_REG0) }
    }

    fn set_ptr(&self) -> &crate::pac::gpio::GPSET0 {
        let offset = (self.pin_num / 32) * 0x04;
        let offset_ptr = unsafe {
            crate::pac::GPIO::ptr()
                .byte_add(0x1C)
                .byte_add(offset as usize)
        } as usize;

        unsafe { &*(offset_ptr as *const crate::pac::gpio::GPSET0) }
    }

    fn clr_ptr(&self) -> &crate::pac::gpio::GPCLR0 {
        let offset = (self.pin_num / 32) * 0x04;
        let offset_ptr = unsafe {
            crate::pac::GPIO::ptr()
                .byte_add(0x28)
                .byte_add(offset as usize)
        } as usize;

        unsafe { &*(offset_ptr as *const crate::pac::gpio::GPCLR0) }
    }

    fn fsel_ptr(&self) -> &crate::pac::gpio::GPFSEL0 {
        // unholy
        let offset = (self.pin_num / 10) * 0x04;
        let offset_ptr = unsafe { crate::pac::GPIO::ptr().byte_add(offset as usize) } as usize;

        unsafe { &*(offset_ptr as *const crate::pac::gpio::GPFSEL0) }
    }
}

macro_rules! gpio {
    ($($i:literal),+) => {
        paste! {
            pub struct Gpio {
                pub pins: [Pin; 45],
            }

            impl Gpio {
                fn new() -> Gpio {
                    let pins = [
                        $(
                        Pin::new($i, PinMode::Input),
                        )+
                    ];
                    Gpio {
                        pins
                    }
                }
            }
        }
    };
}

gpio! {
    0,
    1,
    2,
    3,
    4,
    5,
    6,
    7,
    8,
    9,
    10,
    11,
    12,
    13,
    14,
    15,
    16,
    17,
    18,
    19,
    20,
    21,
    22,
    23,
    24,
    25,
    26,
    27,
    28,
    29,
    30,
    31,
    32,
    33,
    34,
    35,
    36,
    37,
    38,
    39,
    40,
    41,
    42,
    43,
    44
}
