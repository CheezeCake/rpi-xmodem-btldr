pub mod gpio;
mod timer;
pub mod uart;

#[cfg(feature = "rpi_1")]
const PERIPHERALS_BASE: usize = 0x2000_0000;
#[cfg(any(feature = "rpi_2", feature = "rpi_3"))]
const PERIPHERALS_BASE: usize = 0x3f00_0000;

extern "C" {
    pub fn dummy();
}

fn delay(cycles: usize) {
    for _ in 0..cycles {
        unsafe {
            dummy();
        }
    }
}

pub fn udelay(us: u32) {
    let start = timer::now();
    loop {
        let now = timer::now();
        if now < start || now - start >= us {
            break;
        }
    }
}
