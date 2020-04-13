pub fn now() -> u32 {
    const COUNTER_LOW: *const u32 = (super::PERIPHERALS_BASE + 0x3004) as *const u32;
    unsafe { COUNTER_LOW.read_volatile() }
}
