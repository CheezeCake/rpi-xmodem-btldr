#![no_std]

mod hw;
mod xmodem;

use hw::uart;
use hw::uart::Uart;
use xmodem::XmodemReceiver;

use core::panic::PanicInfo;
use core::slice;

#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    let panic_str = if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
        s
    } else {
        ""
    };
    let serial = uart::Uart::new();
    loop {
        serial.puts("panic: ");
        serial.puts(panic_str);
        serial.putc('\n' as u8).unwrap();

        hw::udelay(1_000_000);
    }
}

impl From<uart::Error> for xmodem::Error {
    fn from(error: uart::Error) -> Self {
        match error {
            uart::Error::Timeout => xmodem::Error::Timeout,
        }
    }
}

impl xmodem::Recv for uart::Uart {
    fn recv(&self, timeout: u32) -> Result<u8, xmodem::Error> {
        self.getc_timeout(timeout).map_err(From::from)
    }
}

impl xmodem::Send for uart::Uart {
    fn send(&self, c: u8) -> Result<(), xmodem::Error> {
        self.putc(c).map_err(From::from)
    }
}

#[no_mangle]
extern "C" fn run(r0: u32, r1: u32, r2: u32) -> ! {
    let serial = Uart::new();
    let xm = XmodemReceiver::new(serial);

    #[cfg(any(feature = "rpi_1", feature = "rpi_2"))]
    const CODE_START: usize = 0x0000_8000;
    #[cfg(any(feature = "rpi_3", feature = "rpi_4"))]
    const CODE_START: usize = 0x0008_0000;

    let mut dst = CODE_START as *mut u8;

    xm.start();
    loop {
        match xm.recv_packet(unsafe { slice::from_raw_parts_mut(dst, 128) }) {
            Ok(0) => break,
            Ok(n) => dst = unsafe { dst.add(n) },
            Err(_) => panic!("reception failed!"),
        }
    }

    hw::udelay(200_000);

    let entry: extern "C" fn(u32, u32, u32) -> ! =
        unsafe { core::mem::transmute(CODE_START as *const ()) };
    entry(r0, r1, r2);
}
