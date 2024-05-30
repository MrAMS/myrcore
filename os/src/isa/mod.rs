//! ISA

/// Trap Type
#[derive(Debug)]
pub enum TrapCause {
    /// ecall
    UserEnvCall,
    /// store fault
    StoreFault,
    /// store page fault
    StorePageFault,
    /// ill instruction
    IllegalInstruction,
    /// timer timeout
    SupervisorTimer,
    /// unkown cause
    Unknown,
}

/// ISA API
pub trait ISAMethod {
    /// ISA word type
    type WordType;
    /// set trap handle address
    fn set_trap_entry(entry: Self::WordType);
    /// get trap cause
    fn get_trap_cause() -> (TrapCause, Self::WordType);
    /// which gpr id to store return value
    fn abi_gpr_id_ret() -> Self::WordType;
    /// enable timer interrupt
    fn int_timer_enable();
    /// get timer value
    fn get_timer_val() -> Self::WordType;
}

mod risc_v;
pub use risc_v::{ISA, TrapContext};


