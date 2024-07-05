use crate::gpio::{Gpio, GpioExt, Pinmode};
use crate::pac;

pub struct SPI {
    gpio: Gpio,
}


impl SPI{
    pub fn new(gpio: pac::GPIO) -> Self {
        let gpio = gpio.split();
        SPI { gpio }
    }

pub fn spi_begin(&self)
{
    // Set the SPI0 pins to the Alt 0 function to enable SPI0 access on them
    self.gpio.pins[7..=11](PinMode, ALT0); 
    
    // Set the SPI CS register to the some sensible defaults
    //volatile u32_t* paddr = bcm2835_spi0 + BCM2835_SPI0_CS/4;
    //bcm2835_peri_write(paddr, 0); // All 0s
    
    // Clear TX and RX fifos
    //bcm2835_peri_write_nb(paddr, BCM2835_SPI0_CS_CLEAR);
}

pub fn spi_end(&self)
{  
    // Set all the SPI0 pins back to input
    self.gpio.pins [7..=11](PinMode, Input); // CE1
     // CE0
     // MISO
     // MOSI
     // CLK
}

fn spi_setClockDivider(divider: Divider as u16)
{
    volatile u32_t* paddr = bcm2835_spi0 + BCM2835_SPI0_CLK/4;
    bcm2835_peri_write(paddr, divider);
}

fn spi_setDataMode(u8_t mode)
{
    volatile u32_t* paddr = bcm2835_spi0 + BCM2835_SPI0_CS/4;
    // Mask in the CPO and CPHA bits of CS
    bcm2835_peri_set_bits(paddr, mode << 2, BCM2835_SPI0_CS_CPOL | BCM2835_SPI0_CS_CPHA);
}

u8_t spi_transfer(u8_t value)
{
    volatile u32_t* paddr = bcm2835_spi0 + BCM2835_SPI0_CS/4;
    volatile u32_t* fifo = bcm2835_spi0 + BCM2835_SPI0_FIFO/4;

    // This is Polled transfer as per section 10.6.1
    // BUG ALERT: what happens if we get interupted in this section, and someone else
    // accesses a different peripheral? 
    // Clear TX and RX fifos
    bcm2835_peri_set_bits(paddr, BCM2835_SPI0_CS_CLEAR, BCM2835_SPI0_CS_CLEAR);

    // Set TA = 1
    bcm2835_peri_set_bits(paddr, BCM2835_SPI0_CS_TA, BCM2835_SPI0_CS_TA);

    // Maybe wait for TXD
    while (!(bcm2835_peri_read(paddr) & BCM2835_SPI0_CS_TXD))
	;

    // Write to FIFO, no barrier
    bcm2835_peri_write_nb(fifo, value);

    // Wait for DONE to be set
    while (!(bcm2835_peri_read_nb(paddr) & BCM2835_SPI0_CS_DONE))
	;

    // Read any byte that was sent back by the slave while we sere sending to it
    u32_t ret = bcm2835_peri_read_nb(fifo);

    // Set TA = 0, and also set the barrier
    bcm2835_peri_set_bits(paddr, 0, BCM2835_SPI0_CS_TA);

    return ret;
}

fn spi_transfernb(char* tbuf, char* rbuf, u32_t len)
{
    volatile u32_t* paddr = bcm2835_spi0 + BCM2835_SPI0_CS/4;
    volatile u32_t* fifo = bcm2835_spi0 + BCM2835_SPI0_FIFO/4;
    u32_t TXCnt=0;
    u32_t RXCnt=0;

    // This is Polled transfer as per section 10.6.1
    // BUG ALERT: what happens if we get interupted in this section, and someone else
    // accesses a different peripheral? 

    // Clear TX and RX fifos
    bcm2835_peri_set_bits(paddr, BCM2835_SPI0_CS_CLEAR, BCM2835_SPI0_CS_CLEAR);

    // Set TA = 1
    bcm2835_peri_set_bits(paddr, BCM2835_SPI0_CS_TA, BCM2835_SPI0_CS_TA);

    // Use the FIFO's to reduce the interbyte times
    while((TXCnt < len)||(RXCnt < len))
    {
        // TX fifo not full, so add some more bytes
        while(((bcm2835_peri_read(paddr) & BCM2835_SPI0_CS_TXD))&&(TXCnt < len ))
        {
           bcm2835_peri_write_nb(fifo, tbuf[TXCnt]);
           TXCnt++;
        }
        //Rx fifo not empty, so get the next received bytes
        while(((bcm2835_peri_read(paddr) & BCM2835_SPI0_CS_RXD))&&( RXCnt < len ))
        {
           rbuf[RXCnt] = bcm2835_peri_read_nb(fifo);
           RXCnt++;
        }
    }
    // Wait for DONE to be set
    while (!(bcm2835_peri_read_nb(paddr) & BCM2835_SPI0_CS_DONE))
	;

    // Set TA = 0, and also set the barrier
    bcm2835_peri_set_bits(paddr, 0, BCM2835_SPI0_CS_TA);
}
// Writes an number of bytes to SPI
fn spi_writenb(char* tbuf, u32_t len)
{
    volatile u32_t* paddr = bcm2835_spi0 + BCM2835_SPI0_CS/4;
    volatile u32_t* fifo = bcm2835_spi0 + BCM2835_SPI0_FIFO/4;

    // This is Polled transfer as per section 10.6.1
    // BUG ALERT: what happens if we get interupted in this section, and someone else
    // accesses a different peripheral?

    // Clear TX and RX fifos
    bcm2835_peri_set_bits(paddr, BCM2835_SPI0_CS_CLEAR, BCM2835_SPI0_CS_CLEAR);

    // Set TA = 1
    bcm2835_peri_set_bits(paddr, BCM2835_SPI0_CS_TA, BCM2835_SPI0_CS_TA);

    u32_t i;
    for (i = 0; i < len; i++)
    {
	// Maybe wait for TXD
	while (!(bcm2835_peri_read(paddr) & BCM2835_SPI0_CS_TXD))
	    ;
	
	// Write to FIFO, no barrier
	bcm2835_peri_write_nb(fifo, tbuf[i]);
	
	// Read from FIFO to prevent stalling
	while (bcm2835_peri_read(paddr) & BCM2835_SPI0_CS_RXD)
	    (fn) bcm2835_peri_read_nb(fifo);
    }
    
    // Wait for DONE to be set
    while (!(bcm2835_peri_read_nb(paddr) & BCM2835_SPI0_CS_DONE)) {
	while (bcm2835_peri_read(paddr) & BCM2835_SPI0_CS_RXD)
		(fn) bcm2835_peri_read_nb(fifo);
    };

    // Set TA = 0, and also set the barrier
    bcm2835_peri_set_bits(paddr, 0, BCM2835_SPI0_CS_TA);
}

// Writes (and reads) an number of bytes to SPI
// Read bytes are copied over onto the transmit buffer
fn spi_transfern(char* buf, u32_t len)
{
    bcm2835_spi_transfernb(buf, buf, len);
}

fn spi_chipSelect(u8_t cs)
{
    volatile u32_t* paddr = bcm2835_spi0 + BCM2835_SPI0_CS/4;
    // Mask in the CS bits of CS
    bcm2835_peri_set_bits(paddr, cs, BCM2835_SPI0_CS_CS);
}

fn spi_setChipSelectPolarity(u8_t cs, u8_t active)
{
    volatile u32_t* paddr = bcm2835_spi0 + BCM2835_SPI0_CS/4;
    u8_t shift = 21 + cs;
    // Mask in the appropriate CSPOLn bit
    bcm2835_peri_set_bits(paddr, active << shift, 1 << shift);
}
}
