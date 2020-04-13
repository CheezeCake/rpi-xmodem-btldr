use super::gpio;
use super::timer;

const AUX_ENABLES: isize = 0x04 >> 2; // Auxiliary enables
const AUX_MU_IO_REG: isize = 0x40 >> 2; // Mini Uart I/O Data
const AUX_MU_IER_REG: isize = 0x44 >> 2; // Mini Uart Interrupt Enable
const AUX_MU_IIR_REG: isize = 0x48 >> 2; // Mini Uart Interrupt Identify
const AUX_MU_LCR_REG: isize = 0x4c >> 2; // Mini Uart Line Control
const AUX_MU_MCR_REG: isize = 0x50 >> 2; // Mini Uart Modem Control
const AUX_MU_LSR_REG: isize = 0x54 >> 2; // Mini Uart Line Status
const AUX_MU_CNTL_REG: isize = 0x60 >> 2; // Mini Uart Extra Control
const AUX_MU_BAUD_REG: isize = 0x68 >> 2; // Mini Uart Baudrate

const AUX_BASE: *mut u32 = (super::PERIPHERALS_BASE + 0x0021_5000) as *mut u32;

#[derive(Debug)]
pub enum Error {
    Timeout,
}

pub struct Uart;

impl Uart {
    pub fn new() -> Self {
        init();
        Self
    }

    pub fn putc(&self, c: u8) -> Result<(), Error> {
        unsafe {
            while AUX_BASE.offset(AUX_MU_LSR_REG).read_volatile() & 0x20 == 0 {}
            AUX_BASE.offset(AUX_MU_IO_REG).write_volatile(c as u32);
        }
        Ok(())
    }

    #[allow(dead_code)]
    pub fn getc(&self) -> Result<u8, Error> {
        self.getc_timeout(core::u32::MAX)
    }

    pub fn getc_timeout(&self, timeout: u32) -> Result<u8, Error> {
        let start = timer::now();

        while unsafe { AUX_BASE.offset(AUX_MU_LSR_REG).read_volatile() } & 0x1 == 0 {
            let now = timer::now();
            if start < now && now - start >= timeout {
                return Err(Error::Timeout);
            }
        }

        let c = unsafe { AUX_BASE.offset(AUX_MU_IO_REG).read_volatile() };
        Ok((c & 0xff) as u8)
    }

    pub fn puts(&self, s: &str) {
        for &c in s.as_bytes() {
            self.putc(c).unwrap();
        }
    }
}

fn init() {
    gpio::configure(14, gpio::GPIO_FN5, gpio::GPIO_PUD_OFF);
    gpio::configure(15, gpio::GPIO_FN5, gpio::GPIO_PUD_OFF);

    unsafe {
        AUX_BASE.offset(AUX_ENABLES).write_volatile(1);
        AUX_BASE.offset(AUX_MU_IER_REG).write_volatile(0);
        AUX_BASE.offset(AUX_MU_CNTL_REG).write_volatile(0);
        AUX_BASE.offset(AUX_MU_LCR_REG).write_volatile(3); // 8 bit
        AUX_BASE.offset(AUX_MU_MCR_REG).write_volatile(0);
        AUX_BASE.offset(AUX_MU_IER_REG).write_volatile(0);
        AUX_BASE.offset(AUX_MU_IIR_REG).write_volatile(0xc6); // clear receive and transmit FIFOs
        AUX_BASE.offset(AUX_MU_BAUD_REG).write_volatile(270); // ((250000000 / 115200) / 8) − 1 = ~270
    }

    unsafe {
        AUX_BASE.offset(AUX_MU_CNTL_REG).write_volatile(3); // enable receiver and transmitter
    }
}
