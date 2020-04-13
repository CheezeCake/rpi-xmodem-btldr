// GPIO Function Select
const GPFSEL0: isize = 0x00 >> 2;

// GPIO Pin Pull-up/down Enable
const GPPUD: isize = 0x94 >> 2;

// GPIO Pin Pull-up/down Enable Clock
const GPPUDCLK0: isize = 0x98 >> 2;

// pull up/down
pub const GPIO_PUD_OFF: u32 = 0;

pub const GPIO_FN5: u32 = 2;

const GPIO_BASE: *mut u32 = (super::PERIPHERALS_BASE + 0x0020_0000) as *mut u32;

pub fn configure(pin: u32, func: u32, pud: u32) {
    let gppudclk_n = unsafe { GPIO_BASE.offset(GPPUDCLK0).add((pin / 32) as usize) };

    select_function(pin, func);

    // 1. write control signal
    unsafe {
        GPIO_BASE.offset(GPPUD).write_volatile(pud);
    }
    // 2. wait 150 cycles
    super::delay(150);

    // 3. clock the control signal
    unsafe {
        gppudclk_n.write_volatile(1 << (pin % 32));
    }
    // 4. wait 150 cycles
    super::delay(150);

    // 5. remove the control signal
    unsafe {
        GPIO_BASE.offset(GPPUD).write_volatile(GPIO_PUD_OFF);
    }

    // 6. remove the clock
    unsafe {
        gppudclk_n.write_volatile(0);
    }
}

fn select_function(pin: u32, func: u32) {
    let gpfsel = unsafe { GPIO_BASE.offset(GPFSEL0).add((pin / 10) as usize) };
    let val = unsafe { gpfsel.read_volatile() };
    let shift: u32 = (pin % 10) * 3;

    unsafe {
        gpfsel.write_volatile((val & !(7 << shift)) | (func << shift));
    }
}
