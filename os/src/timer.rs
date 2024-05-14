//! RISC-V timer-related functionality

use crate::config::CLOCK_FREQ;
use crate::sbi::set_timer;
use riscv::register::time;

const TIME_SLICE_MS: usize = 1;

/// read the `mtime` register
pub fn get_time() -> usize {
    time::read()
}

/// get current time in milliseconds
pub fn get_time_ms() -> usize {
    time::read() / (CLOCK_FREQ/1000)
}

/// set the next timer interrupt
pub fn set_next_trigger() {
    set_timer(get_time() + CLOCK_FREQ / 1000 * TIME_SLICE_MS);
}
