//! RISC-V timer-related functionality

use crate::config::CLOCK_FREQ;
use crate::isa::{ISAMethod, ISA};
use crate::sbi::{SBIMethod, SBI};

const TIME_SLICE_MS: usize = 1;

/// read the `mtime` register
pub fn get_time() -> usize {
    ISA::get_timer_val() as usize
}

/// get current time in milliseconds
pub fn get_time_ms() -> usize {
    ISA::get_timer_val() as usize / (CLOCK_FREQ/1000)
}

/// set the next timer interrupt
pub fn set_next_trigger() {
    SBI::set_timer(get_time() + CLOCK_FREQ / 1000 * TIME_SLICE_MS);
}
