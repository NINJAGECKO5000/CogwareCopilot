use bcm2837_lpa::SPI0;
use embedded_hal::{
    delay::DelayNs,
    digital::OutputPin,
    spi::{ErrorKind, ErrorType, Operation, SpiBus, SpiDevice},
};
use embedded_hal_0_2::blocking::spi::Transfer;
use fugit::{Hertz, HertzU32};

use crate::delay::Timer;

pub static REFERENCE_FREQ: u32 = 250_000_000;

#[derive(Debug)]
pub enum SpiZeroError {
    ClearRxFifo,
    ClearTxFifo,
    ClearBothFifo,
    EnableDma,
    DisableDma,
    SetCspol(bool),
    SetBuiltinCs(BuiltinCS),
    SetBuiltinCsPol0(bool),
    SetBuiltinCsPol1(bool),
    SetBuiltinCsPol2(bool),
    SetRxInterrupt(bool),
    SetDoneInterrupt(bool),
    SetBuiltinDmaCS(bool),
    SetRenEnable(bool),
    SetLenEnable(bool),
    SetDmaLen(bool),
    SetLenLong(bool),
}

impl core::fmt::Display for SpiZeroError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Failed to ")?;

        match self {
            SpiZeroError::ClearRxFifo => write!(f, "clear RX FIFO"),
            SpiZeroError::ClearTxFifo => write!(f, "clear TX FIFO"),
            SpiZeroError::ClearBothFifo => write!(f, "clear RX/TX FIFO"),
            SpiZeroError::EnableDma => write!(f, "enable DMA"),
            SpiZeroError::DisableDma => write!(f, "disable DMA"),
            SpiZeroError::SetCspol(mode) => write!(f, "set CS polarity (arg: {})", mode),
            SpiZeroError::SetBuiltinCs(mode) => write!(f, "set builtin CS mode to {}", mode),
            SpiZeroError::SetBuiltinCsPol0(mode) => {
                write!(f, "set builtin CS polarity 0 (arg: {})", mode)
            }
            SpiZeroError::SetBuiltinCsPol1(mode) => {
                write!(f, "set builtin CS polarity 1 (arg: {})", mode)
            }
            SpiZeroError::SetBuiltinCsPol2(mode) => {
                write!(f, "set builtin CS polarity 2 (arg: {})", mode)
            }
            SpiZeroError::SetRxInterrupt(mode) => write!(f, "set RX interrupt (arg: {})", mode),

            SpiZeroError::SetDoneInterrupt(mode) => write!(f, "set done interrupt (arg: {})", mode),
            SpiZeroError::SetBuiltinDmaCS(mode) => write!(f, "set builtin DMA CS (arg: {})", mode),
            SpiZeroError::SetRenEnable(mode) => write!(f, "set REN enable (arg: {})", mode),
            SpiZeroError::SetLenEnable(mode) => write!(f, "set len enable (arg: {})", mode),
            SpiZeroError::SetDmaLen(mode) => write!(f, "set DMA len (arg: {})", mode),
            SpiZeroError::SetLenLong(mode) => write!(f, "set len long (arg: {})", mode),
        }
    }
}

impl core::error::Error for SpiZeroError {}

pub struct SPIZero<'a> {
    spi0: &'a SPI0,
}

#[derive(Debug, Clone, Copy)]
pub enum BuiltinCS {
    Cs0 = 00,
    Cs1 = 01,
    Cs2 = 10,
}

impl core::fmt::Display for BuiltinCS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BuiltinCS::Cs0 => "CS0",
                BuiltinCS::Cs1 => "CS1",
                BuiltinCS::Cs2 => "CS2",
            }
        )
    }
}

impl ErrorType for SPIZero<'_> {
    type Error = ErrorKind;
}
#[allow(dead_code)]
impl<'a> SPIZero<'a> {
    pub fn new(spi0: &'a SPI0) -> Self {
        SPIZero { spi0 }
    }
    //
    // fn registers(&self) -> &RegisterBlock {
    //     unsafe { &*self.registers }
    // }

    pub fn set_mode(&mut self, mode: embedded_hal::spi::Mode) {
        //let registers = unsafe { &*self.registers };
        let cpol = match mode.polarity {
            embedded_hal::spi::Polarity::IdleLow => 0,
            embedded_hal::spi::Polarity::IdleHigh => 1,
        };
        let cpha = match mode.phase {
            embedded_hal::spi::Phase::CaptureOnFirstTransition => 0,
            embedded_hal::spi::Phase::CaptureOnSecondTransition => 1,
        };
        self.spi0.cs().modify(|_, w| {
            w.cpol().bit(cpol != 0);
            w.cpha().bit(cpha != 0);
            w
        });
    }

    pub fn set_frequency(&mut self, freq_hz: Hertz<u32>) {
        let divider = round_clock(freq_hz); // Must power of 2, fn returns a val to the power of 2 based on ref clock, if excedes goes to 1 locking it at around 4khz
        unsafe {
            self.spi0
                .clk()
                .write_with_zero(|w| w.cdiv().bits(divider as u16))
        };
    }

    pub fn send(&mut self, bytes: u8) {
        unsafe {
            self.spi0
                .fifo()
                .write_with_zero(|w| w.data().bits(bytes.into()))
        };
    }

    pub fn read_data(&self) -> u32 {
        self.spi0.fifo().read().data().bits()
    }
    pub fn init(&mut self, mode: embedded_hal::spi::Mode, freq_hz: HertzU32) {
        let _ = self.set_mode(mode);
        let _ = self.set_frequency(freq_hz);
        let _ = self.clear_both_fifo();
        let _ = self.set_transfer_active();
        let _ = self.clear_transfer_active();
    }
    fn is_ready_to_send(&self) -> bool {
        self.spi0.cs().read().txd().bit_is_set()
    }

    fn data_to_be_read(&self) -> bool {
        self.spi0.cs().read().rxr().bit_is_set()
    }

    fn rx_fifo_full(&self) -> bool {
        self.spi0.cs().read().rxf().bit_is_set()
    }

    fn rx_fifo_has_data(&self) -> bool {
        self.spi0.cs().read().rxd().bit_is_set()
    }

    fn clear_rx_fifo(&self) -> Result<(), SpiZeroError> {
        unsafe { self.spi0.cs().modify(|_, w| w.clear().bits(0b10)) };
        if self.spi0.cs().read().clear().bits() == 0b10 {
            return Ok(());
        } else {
            return Err(SpiZeroError::ClearRxFifo);
        }
    }

    fn clear_tx_fifo(&self) -> Result<(), SpiZeroError> {
        unsafe { self.spi0.cs().modify(|_, w| w.clear().bits(0b01)) };
        if self.spi0.cs().read().clear().bits() == 0b01 {
            return Ok(());
        } else {
            return Err(SpiZeroError::ClearTxFifo);
        }
    }

    fn clear_both_fifo(&self) -> Result<(), SpiZeroError> {
        unsafe { self.spi0.cs().modify(|_, w| w.clear().bits(0b11)) };
        if self.spi0.cs().read().clear().bits() == 0b11 {
            return Ok(());
        } else {
            return Err(SpiZeroError::ClearBothFifo);
        }
    }

    fn read_transfer_active(&self) -> bool {
        self.spi0.cs().read().ta().bit_is_set()
    }

    fn set_transfer_active(&self) {
        self.spi0.cs().modify(|_, w| w.ta().bit(true));
    }

    fn clear_transfer_active(&self) {
        self.spi0.cs().modify(|_, w| w.ta().bit(false));
    }

    fn transfer_done(&self) -> bool {
        self.spi0.cs().read().done().bit_is_set()
    }

    fn dma_enable(&self) -> Result<(), SpiZeroError> {
        self.spi0.cs().modify(|_, w| w.dmaen().bit(true));
        if self.spi0.cs().read().dmaen() == true {
            return Ok(());
        } else {
            return Err(SpiZeroError::EnableDma);
        }
    }

    fn dma_disable(&self) -> Result<(), SpiZeroError> {
        self.spi0.cs().modify(|_, w| w.dmaen().bit(false));
        if self.spi0.cs().read().dmaen() == false {
            return Ok(());
        } else {
            return Err(SpiZeroError::DisableDma);
        }
    }

    fn set_cspol(&self, mode: bool) -> Result<(), SpiZeroError> {
        self.spi0.cs().modify(|_, w| w.cspol().bit(mode));
        if self.spi0.cs().read().cspol() == mode {
            return Ok(());
        } else {
            return Err(SpiZeroError::SetCspol(mode));
        }
    }

    fn set_builtin_cs(&self, mode: BuiltinCS) -> Result<(), SpiZeroError> {
        let val = mode as u8;
        unsafe { self.spi0.cs().modify(|_, w| w.cs().bits(val)) };
        if self.spi0.cs().read().cs().bits() == val {
            return Ok(());
        } else {
            return Err(SpiZeroError::SetBuiltinCs(mode));
        }
    }

    fn set_builtin_cspol0(&self, mode: bool) -> Result<(), SpiZeroError> {
        self.spi0.cs().modify(|_, w| w.cspol0().bit(mode));
        if self.spi0.cs().read().cspol0() == mode {
            return Ok(());
        } else {
            return Err(SpiZeroError::SetBuiltinCsPol0(mode));
        }
    }

    fn set_builtin_cspol1(&self, mode: bool) -> Result<(), SpiZeroError> {
        self.spi0.cs().modify(|_, w| w.cspol1().bit(mode));
        if self.spi0.cs().read().cspol1() == mode {
            return Ok(());
        } else {
            return Err(SpiZeroError::SetBuiltinCsPol1(mode));
        }
    }

    fn set_builtin_cspol2(&self, mode: bool) -> Result<(), SpiZeroError> {
        self.spi0.cs().modify(|_, w| w.cspol2().bit(mode));
        if self.spi0.cs().read().cspol2() == mode {
            return Ok(());
        } else {
            return Err(SpiZeroError::SetBuiltinCsPol2(mode));
        }
    }

    fn set_rx_interupt(&self, mode: bool) -> Result<(), SpiZeroError> {
        self.spi0.cs().modify(|_, w| w.intr().bit(mode));
        if self.spi0.cs().read().intr() == mode {
            return Ok(());
        } else {
            return Err(SpiZeroError::SetRxInterrupt(mode));
        }
    }

    fn set_done_interupt(&self, mode: bool) -> Result<(), SpiZeroError> {
        self.spi0.cs().modify(|_, w| w.intd().bit(mode));
        if self.spi0.cs().read().intd() == mode {
            return Ok(());
        } else {
            return Err(SpiZeroError::SetDoneInterrupt(mode));
        }
    }

    fn set_builtin_dma_cs(&self, mode: bool) -> Result<(), SpiZeroError> {
        self.spi0.cs().modify(|_, w| w.adcs().bit(mode));
        if self.spi0.cs().read().adcs() == mode {
            return Ok(());
        } else {
            return Err(SpiZeroError::SetBuiltinDmaCS(mode));
        }
    }

    fn set_ren_enable(&self, mode: bool) -> Result<(), SpiZeroError> {
        // from docs: read enable if you are using bidirectional mode.
        //If this bit is set, the SPI peripheral will be able to
        //send data to this device.

        self.spi0.cs().modify(|_, w| w.ren().bit(mode));
        if self.spi0.cs().read().ren() == mode {
            return Ok(());
        } else {
            return Err(SpiZeroError::SetRenEnable(mode));
        }
    }

    fn set_len_enable(&self, mode: bool) -> Result<(), SpiZeroError> {
        //LoSSI master enable

        self.spi0.cs().modify(|_, w| w.len().bit(mode));
        if self.spi0.cs().read().len() == mode {
            return Ok(());
        } else {
            return Err(SpiZeroError::SetLenEnable(mode));
        }
    }

    fn set_dma_len(&self, mode: bool) -> Result<(), SpiZeroError> {
        //DMA Enable in LoSSi

        self.spi0.cs().modify(|_, w| w.dma_len().bit(mode));
        if self.spi0.cs().read().dma_len() == mode {
            return Ok(());
        } else {
            return Err(SpiZeroError::SetDmaLen(mode));
        }
    }

    fn set_len_long(&self, mode: bool) -> Result<(), SpiZeroError> {
        //Enable Long data word in Lossi mode if
        //DMA_LEN is set
        //0= writing to the FIFO will write a single byte
        //1= writing to the FIFO will write a 32 bit word

        self.spi0.cs().modify(|_, w| w.len().bit(mode));
        if self.spi0.cs().read().len() == mode {
            return Ok(());
        } else {
            return Err(SpiZeroError::SetLenLong(mode));
        }
    }

    fn set_dlen_bytes(&self, bytes: u16) {
        //Data Length
        //The number of bytes to transfer.
        //This field is only valid for DMA mode (DMAEN
        //set) and controls how many bytes to transmit
        //(and therefore receive).

        unsafe { self.spi0.dlen().write_with_zero(|w| w.bits(bytes as u32)) };
    }

    //unimplemented: LTOH Reg and DC Reg
}

impl<'a> Transfer<u8> for SPIZero<'a> {
    // type Error = ErrorKind;
    type Error = SpiZeroError;

    fn transfer<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8], Self::Error> {
        let _ = self.clear_both_fifo();
        self.set_transfer_active();
        while !self.is_ready_to_send() {}

        for word in words.iter_mut() {
            self.set_ren_enable(false)?;
            self.send(*word);
            while !self.transfer_done() {}
            self.set_ren_enable(true)?;
            if self.rx_fifo_has_data() {
                *word = self.read_data() as u8;
            }
        }
        self.clear_transfer_active();

        Ok(words)
    }
}

impl<'a> SpiBus<u8> for SPIZero<'a> {
    fn read(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        let _ = self.clear_both_fifo();
        self.set_transfer_active();

        for w in words.iter_mut() {
            if self.rx_fifo_has_data() {
                *w = self.read_data() as u8;
            } else {
                break;
            }
        }
        self.clear_transfer_active();

        Ok(())
    }

    /// This expects `words` to be a slice of `u8`s in LITTLE ENDIAN ORDER.
    ///
    /// If you fuck this up it's your fault and I don't care
    fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        let _ = self.clear_both_fifo();
        self.set_transfer_active();
        while !self.is_ready_to_send() {}
        for w in words.iter() {
            self.send(*w);
            while !self.transfer_done() {}
        }
        self.clear_transfer_active();

        Ok(())
    }

    fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Self::Error> {
        let _ = self.clear_both_fifo();
        self.set_transfer_active();
        while !self.is_ready_to_send() {}

        for w in write.iter() {
            self.send(*w);
            while !self.transfer_done() {}
        }

        for w in read.iter_mut() {
            if self.rx_fifo_has_data() {
                *w = self.read_data() as u8;
            } else {
                break;
            }
        }

        self.clear_transfer_active();

        Ok(())
    }

    fn transfer_in_place(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        let _ = self.clear_both_fifo();
        self.set_transfer_active();
        while !self.is_ready_to_send() {}

        for word in words.iter_mut() {
            self.send(*word);
            while !self.transfer_done() {}
            if self.rx_fifo_has_data() {
                *word = self.read_data() as u8;
            } else {
                break;
            }
        }
        self.clear_transfer_active();

        Ok(())
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        self.set_transfer_active();
        while !self.transfer_done() {}
        self.clear_transfer_active();
        Ok(())
    }
}

// Example SpiDevice implementation assuming CS pin is managed externally
pub struct SPI0Device<'a, CS> {
    bus: &'a mut SPIZero<'a>,
    cs: CS,
}

impl<'a, CS> ErrorType for SPI0Device<'a, CS> {
    type Error = ErrorKind;
}

impl<'a, CS> SPI0Device<'a, CS>
where
    CS: OutputPin<Error = ()>,
{
    pub fn new(bus: &'a mut SPIZero<'a>, cs: CS) -> Self {
        SPI0Device { bus, cs }
    }
}

impl<'a, CS> SpiDevice<u8> for SPI0Device<'a, CS>
where
    CS: OutputPin<Error = ()>,
{
    fn transaction(&mut self, operations: &mut [Operation<'_, u8>]) -> Result<(), Self::Error> {
        self.cs.set_low().map_err(|_| ErrorKind::ChipSelectFault)?;

        for op in operations {
            match op {
                Operation::Read(rx_buf) => match SpiBus::read(&mut self.bus, rx_buf) {
                    Ok(()) => {}
                    Err(e) => {
                        self.cs.set_high().map_err(|_| ErrorKind::ChipSelectFault)?;
                        return Err(e);
                    }
                },
                Operation::Write(tx_buf) => self.bus.write(tx_buf)?,
                Operation::Transfer(rx_buf, tx_buf) => {
                    SpiBus::transfer(&mut self.bus, rx_buf, tx_buf)?
                }
                Operation::TransferInPlace(buf) => self.bus.transfer_in_place(buf)?,
                Operation::DelayNs(ns) => Timer::new().delay_ns(*ns),
            }
        }

        self.cs.set_high().map_err(|_| ErrorKind::ChipSelectFault)?;

        Ok(())
    }
}

fn round_clock(freq: Hertz<u32>) -> u32 {
    // Calculate the desired divider
    let divider = (REFERENCE_FREQ / freq.to_Hz()).max(1);

    // Find the closest power of 2
    let mut rounded_divider = 1u32;
    while rounded_divider < divider {
        rounded_divider <<= 1;
    }

    // Check if rounding down is closer
    let lower_bound = rounded_divider >> 1;
    if rounded_divider - divider > divider - lower_bound {
        rounded_divider = lower_bound;
    }

    rounded_divider
}
