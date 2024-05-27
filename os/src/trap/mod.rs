//! Trap handling functionality
//!
//! For rCore, we have a single trap entry point, namely `__alltraps`. At
//! initialization in [`init()`], we set the `stvec` CSR to point to it.
//!
//! All traps go through `__alltraps`, which is defined in `trap.S`. The
//! assembly language code does just enough work restore the kernel space
//! context, ensuring that Rust code safely runs, and transfers control to
//! [`trap_handler()`].
//!
//! It then calls different functionality based on what exactly the exception
//! was. For example, timer interrupts trigger task preemption, and syscalls go
//! to [`syscall()`].

use crate::syscall::syscall;
use crate::task::{exit_current_and_run_next, suspend_current_and_run_next};
use crate::timer::set_next_trigger;
use core::arch::global_asm;

use crate::isa::{ISAMethod, TrapCause, ISA};

global_asm!(include_str!("trap.S"));

/// initialize CSR `stvec` as the entry of `__alltraps`
pub fn init() {
    extern "C" {
        fn __alltraps();
    }
    ISA::set_trap_entry(__alltraps as u32);
        // stvec::write(__alltraps as usize, TrapMode::Direct);
}

/// timer interrupt enabled
pub fn enable_timer_interrupt() {
    ISA::int_timer_enable();
}

#[no_mangle]
/// handle an interrupt, exception, or system call from user space
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let (cause, extra_info) = ISA::get_trap_cause();
    // let stval = stval::read(); // get extra value
    match cause {
        TrapCause::UserEnvCall => {
            cx.sepc += 4;
            cx.set_func_ret(syscall(cx.get_syscall_id(), cx.get_func_args()) as usize);
        }
        TrapCause::StoreFault | TrapCause::StorePageFault => {
            println!("[kernel] PageFault in application, bad addr = {:#x}, bad instruction = {:#x}, kernel killed it.", extra_info, cx.sepc);
            exit_current_and_run_next();
        }
        TrapCause::IllegalInstruction => {
            println!("[kernel] IllegalInstruction in application, kernel killed it.");
            exit_current_and_run_next();
        }
        TrapCause::SupervisorTimer => {
            set_next_trigger();
            suspend_current_and_run_next();
        }
        _ => {
            panic!(
                "Unimplemented TrapCause {:?}", cause
            );
        }
    }
    cx
}

pub use crate::isa::TrapContext;
